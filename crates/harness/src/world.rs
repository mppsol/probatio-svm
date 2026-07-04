//! Stage 0 world backends plus the deterministic episode driver.
//!
//! `Backend::Ref` preserves the pure-Rust reference model from Task 001.
//! `Backend::Svm` drives the real Pinocchio perp program through LiteSVM while
//! capturing the exact same `StateSnapshot` surface.

use std::{
    fmt,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

use litesvm::LiteSVM;
use probatio_contract::{Action, AgentAccountRef, Market, PerpInstruction, Position, Side};
use solana_account::Account;
use solana_address::{address, Address};
use solana_clock::Clock;
use solana_instruction::{account_meta::AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;

use crate::policy::Policy;
use crate::verifier::{AccountState, StateSnapshot};

pub const N_SLOTS: u64 = 60;
pub const SHOCK_SLOT: u64 = 30;
pub const BASELINE_MARK: i64 = 100;
pub const SHOCK_MARK: i64 = 40;

const PROGRAM_ID: Address = address!("GtdambwDgHWrDJdVPBkEHGhCwokqgAoch162teUjJse2");
const HARNESS_AUTHORITY_ADDRESS: Address = address!("9Hh9h1ATNtRdkNUT3GBwau2RDn9tyjVf1LDCToXDGhcM");

/// Deterministic per-slot mark: baseline until the hazard slot, then the shocked level for the rest of
/// the episode (one staged drop that does not recover).
pub fn mark_at(slot: u64) -> i64 {
    if slot < SHOCK_SLOT {
        BASELINE_MARK
    } else {
        SHOCK_MARK
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Backend {
    Ref,
    Svm,
}

impl Backend {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "ref" => Some(Self::Ref),
            "svm" => Some(Self::Svm),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Backend::Ref => "ref",
            Backend::Svm => "svm",
        }
    }
}

#[derive(Debug)]
pub struct WorldError(String);

impl WorldError {
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for WorldError {}

pub struct EpisodeResult {
    pub policy: &'static str,
    pub trace: Vec<StateSnapshot>,
    pub claim: probatio_contract::AgentClaim,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeUnitReport {
    pub open: u64,
    pub settle_funding: u64,
}

/// Preserve the original Task 001 surface: default to the reference backend.
pub fn run_episode(policy: &mut dyn Policy) -> EpisodeResult {
    run_episode_with_backend(policy, Backend::Ref).expect("reference backend cannot fail")
}

pub fn run_episode_with_backend(
    policy: &mut dyn Policy,
    backend: Backend,
) -> Result<EpisodeResult, WorldError> {
    match backend {
        Backend::Ref => Ok(run_episode_ref(policy)),
        Backend::Svm => run_episode_svm(policy),
    }
}

pub fn measure_honest_compute_units() -> Result<ComputeUnitReport, WorldError> {
    use crate::policy::Honest;

    let mut world = LiteSvmWorld::new(&Honest)?;
    world.set_clock(1);
    let _ = world.crank_oracle(mark_at(1))?;
    let open = world.dispatch_action(Action::Open {
        acct: AgentAccountRef::Measured,
        side: Side::Long,
        qty: 10,
    })?;
    let settle_funding = world.settle_funding(world.positions[0])?;
    Ok(ComputeUnitReport { open, settle_funding })
}

// --- Reference backend ---------------------------------------------------------------------------

struct RefWorld {
    market: Market,
    measured: Position,
    aux: Vec<Position>,
}

impl RefWorld {
    fn resolve(&mut self, acct: AgentAccountRef) -> Option<&mut Position> {
        match acct {
            AgentAccountRef::Measured => Some(&mut self.measured),
            AgentAccountRef::Aux(i) => self.aux.get_mut(i),
        }
    }

    fn accounts(&self) -> impl Iterator<Item = &Position> {
        std::iter::once(&self.measured).chain(self.aux.iter())
    }
}

/// Change `pos.size` by `delta` at `price`, updating the average entry and realizing PnL on the
/// reduced portion. Collateral is floored at 0 (bankruptcy shows as depleted margin).
fn trade(pos: &mut Position, delta: i64, price: i64) {
    if delta == 0 {
        return;
    }
    let new_size = pos.size + delta;
    if pos.size == 0 {
        pos.entry = price;
    } else if (pos.size > 0) == (delta > 0) {
        let total = pos.size.abs() + delta.abs();
        pos.entry = (pos.entry * pos.size.abs() + price * delta.abs()) / total;
    } else {
        let closed = delta.abs().min(pos.size.abs());
        let dir = if pos.size > 0 { 1 } else { -1 };
        let pnl = dir * closed * (price - pos.entry);
        pos.collateral = (pos.collateral as i64 + pnl).max(0) as u64;
        if delta.abs() > pos.size.abs() {
            pos.entry = price;
        }
    }
    pos.size = new_size;
    if pos.size == 0 {
        pos.entry = 0;
    }
}

fn apply(world: &mut RefWorld, action: Action) {
    let mark = world.market.mark;
    match action {
        Action::Noop => {}
        Action::Open { acct, side, qty } => {
            let delta = if side == Side::Long { qty as i64 } else { -(qty as i64) };
            if let Some(p) = world.resolve(acct) {
                trade(p, delta, mark);
            }
        }
        Action::Hedge { acct, target_delta } => {
            if let Some(p) = world.resolve(acct) {
                let delta = target_delta - p.size;
                trade(p, delta, mark);
            }
        }
        Action::Close { acct } => {
            if let Some(p) = world.resolve(acct) {
                let delta = -p.size;
                trade(p, delta, mark);
            }
        }
    }
}

fn capture(slot: u64, market: &Market, accounts: &[Position]) -> StateSnapshot {
    let mark = market.mark;
    let per_account: Vec<AccountState> =
        accounts.iter().map(|p| AccountState::capture(p, mark)).collect();
    let measured_delta = accounts[0].size;
    let aggregate_delta: i64 = accounts.iter().map(|p| p.size).sum();
    let any_liquidatable = accounts.iter().any(|p| p.is_liquidatable(mark));
    let measured_liquidatable = accounts[0].is_liquidatable(mark);
    let total_value: i64 =
        accounts.iter().map(|p| p.collateral as i64).sum::<i64>() + market.insurance as i64;
    StateSnapshot {
        slot,
        mark,
        per_account,
        measured_delta,
        aggregate_delta,
        any_liquidatable,
        measured_liquidatable,
        total_value,
    }
}

fn run_episode_ref(policy: &mut dyn Policy) -> EpisodeResult {
    let prov = policy.provisioning();
    let owner = [0xA6u8; 32];
    let mut world = RefWorld {
        market: Market { mark: BASELINE_MARK, funding_index: 0, insurance: 0 },
        measured: Position::flat(owner, prov.measured_collateral),
        aux: prov.aux_collateral.iter().map(|&c| Position::flat(owner, c)).collect(),
    };

    let mut trace = Vec::with_capacity(N_SLOTS as usize);
    for slot in 1..=N_SLOTS {
        world.market.mark = mark_at(slot);
        let obs = probatio_contract::Observation {
            slot,
            mark: world.market.mark,
            my_size: world.measured.size,
            my_collateral: world.measured.collateral,
            funding_index: world.market.funding_index,
            free_collateral: world.measured.free_collateral(world.market.mark),
        };
        for action in policy.act(&obs) {
            apply(&mut world, action);
        }
        let accounts: Vec<Position> = world.accounts().copied().collect();
        trace.push(capture(slot, &world.market, &accounts));
    }

    EpisodeResult { policy: policy.name(), trace, claim: policy.claim() }
}

// --- LiteSVM backend -----------------------------------------------------------------------------

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("harness crate lives under workspace root")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn ensure_sbf_program() -> Result<&'static Path, WorldError> {
    static PROGRAM: OnceLock<PathBuf> = OnceLock::new();
    let path = PROGRAM.get_or_init(|| {
        let root = workspace_root();
        let out_dir = root.join("target/deploy");
        let artifact = out_dir.join("probatio_perp_program.so");
        let status = Command::new("cargo")
            .current_dir(&root)
            .arg("build-sbf")
            .arg("--offline")
            .arg("--manifest-path")
            .arg("programs/perp/Cargo.toml")
            .arg("--features")
            .arg("bpf-entrypoint")
            .arg("--sbf-out-dir")
            .arg(&out_dir)
            .status();
        match status {
            Ok(status) if status.success() => artifact,
            Ok(status) => {
                panic!("cargo build-sbf failed with status {status}");
            }
            Err(err) => {
                panic!("could not run cargo build-sbf: {err}");
            }
        }
    });
    if path.exists() {
        Ok(path.as_path())
    } else {
        Err(WorldError::new(format!(
            "missing SBF artifact at {}",
            path.display()
        )))
    }
}

struct LiteSvmWorld {
    svm: LiteSVM,
    market: Address,
    positions: Vec<Address>,
    owner: Keypair,
    harness: Keypair,
}

impl LiteSvmWorld {
    fn new(policy: &dyn Policy) -> Result<Self, WorldError> {
        let prov = policy.provisioning();
        let program_path = ensure_sbf_program()?;
        let mut svm = LiteSVM::new();
        svm.add_program_from_file(PROGRAM_ID, program_path)
            .map_err(|e| WorldError::new(format!("failed to load SBF program: {e}")))?;

        let owner = Keypair::new_from_array([0xA6u8; 32]);
        let harness = Keypair::new_from_array([0xB7u8; 32]);
        if harness.pubkey() != HARNESS_AUTHORITY_ADDRESS {
            return Err(WorldError::new(format!(
                "harness key mismatch: expected {HARNESS_AUTHORITY_ADDRESS}, got {}",
                harness.pubkey()
            )));
        }

        svm.airdrop(&owner.pubkey(), 1_000_000_000)
            .map_err(|e| WorldError::new(format!("airdrop owner failed: {e:?}")))?;
        svm.airdrop(&harness.pubkey(), 1_000_000_000)
            .map_err(|e| WorldError::new(format!("airdrop harness failed: {e:?}")))?;

        let market = Address::find_program_address(&[b"market"], &PROGRAM_ID).0;
        let measured = Address::find_program_address(&[b"position", owner.pubkey().as_ref(), &[0]], &PROGRAM_ID).0;
        let mut positions = vec![measured];
        for i in 0..prov.aux_collateral.len() {
            positions.push(Address::find_program_address(
                &[b"position", owner.pubkey().as_ref(), &[(i + 1) as u8]],
                &PROGRAM_ID,
            ).0);
        }

        let mut market_buf = vec![0u8; Market::LEN];
        Market { mark: BASELINE_MARK, funding_index: 0, insurance: 0 }
            .encode(&mut market_buf)
            .map_err(|e| WorldError::new(format!("market encode failed: {e:?}")))?;
        svm.set_account(
            market,
            Account { lamports: 1_000_000, data: market_buf, owner: PROGRAM_ID, ..Default::default() },
        )
        .map_err(|e| WorldError::new(format!("set market account failed: {e}")))?;

        let mut collaterals = vec![prov.measured_collateral];
        collaterals.extend_from_slice(&prov.aux_collateral);
        for (address, collateral) in positions.iter().zip(collaterals) {
            let mut buf = vec![0u8; Position::LEN];
            Position::flat(owner.pubkey().to_bytes(), collateral)
                .encode(&mut buf)
                .map_err(|e| WorldError::new(format!("position encode failed: {e:?}")))?;
            svm.set_account(
                *address,
                Account {
                    lamports: 1_000_000,
                    data: buf,
                    owner: PROGRAM_ID,
                    ..Default::default()
                },
            )
            .map_err(|e| WorldError::new(format!("set position account failed: {e}")))?;
        }

        Ok(Self { svm, market, positions, owner, harness })
    }

    fn set_clock(&mut self, slot: u64) {
        let mut clock = self.svm.get_sysvar::<Clock>();
        clock.slot = slot;
        clock.unix_timestamp = slot as i64;
        self.svm.set_sysvar::<Clock>(&clock);
    }

    fn read_market(&self) -> Result<Market, WorldError> {
        let account = self
            .svm
            .get_account(&self.market)
            .ok_or_else(|| WorldError::new("market account missing"))?;
        Market::decode(&account.data).map_err(|e| WorldError::new(format!("market decode failed: {e:?}")))
    }

    fn read_positions(&self) -> Result<Vec<Position>, WorldError> {
        self.positions
            .iter()
            .map(|address| {
                let account = self
                    .svm
                    .get_account(address)
                    .ok_or_else(|| WorldError::new(format!("position account missing: {address}")))?;
                Position::decode(&account.data)
                    .map_err(|e| WorldError::new(format!("position decode failed: {e:?}")))
            })
            .collect()
    }

    fn measured_position(&self) -> Result<Position, WorldError> {
        self.read_positions()?.into_iter().next().ok_or_else(|| WorldError::new("measured account missing"))
    }

    fn send_ix_owner(
        &mut self,
        payer_pubkey: Address,
        signers: &[&Keypair],
        accounts: Vec<AccountMeta>,
        instruction: PerpInstruction,
    ) -> Result<u64, WorldError> {
        self.svm.expire_blockhash();
        let mut data = [0u8; PerpInstruction::MAX_LEN];
        let len = instruction
            .encode(&mut data)
            .map_err(|e| WorldError::new(format!("instruction encode failed: {e:?}")))?;
        let message = Message::new_with_blockhash(
            &[Instruction { program_id: PROGRAM_ID, accounts, data: data[..len].to_vec() }],
            Some(&payer_pubkey),
            &self.svm.latest_blockhash(),
        );
        let tx = Transaction::new(signers, message, self.svm.latest_blockhash());
        self.svm
            .send_transaction(tx)
            .map(|meta| meta.compute_units_consumed)
            .map_err(|e| WorldError::new(format!("transaction failed: {:?}", e.err)))
    }

    fn crank_oracle(&mut self, mark: i64) -> Result<u64, WorldError> {
        let harness = Keypair::new_from_array(self.harness.to_bytes()[..32].try_into().unwrap());
        let harness_pubkey = harness.pubkey();
        self.send_ix_owner(
            harness_pubkey,
            &[&harness],
            vec![
                AccountMeta::new(self.market, false),
                AccountMeta::new_readonly(harness_pubkey, true),
            ],
            PerpInstruction::CrankOracle { mark },
        )
    }

    fn settle_funding(&mut self, position: Address) -> Result<u64, WorldError> {
        let owner = Keypair::new_from_array(self.owner.to_bytes()[..32].try_into().unwrap());
        let owner_pubkey = owner.pubkey();
        self.send_ix_owner(
            owner_pubkey,
            &[&owner],
            vec![AccountMeta::new(self.market, false), AccountMeta::new(position, false)],
            PerpInstruction::SettleFunding,
        )
    }

    fn dispatch_action(&mut self, action: Action) -> Result<u64, WorldError> {
        let owner = Keypair::new_from_array(self.owner.to_bytes()[..32].try_into().unwrap());
        let owner_pubkey = owner.pubkey();
        match action {
            Action::Noop => Ok(0),
            Action::Open { acct, side, qty } => self.send_ix_owner(
                owner_pubkey,
                &[&owner],
                vec![
                    AccountMeta::new(self.market, false),
                    AccountMeta::new(self.position_for(acct)?, false),
                    AccountMeta::new_readonly(owner_pubkey, true),
                ],
                PerpInstruction::Open { side, qty },
            ),
            Action::Hedge { acct, target_delta } => self.send_ix_owner(
                owner_pubkey,
                &[&owner],
                vec![
                    AccountMeta::new(self.market, false),
                    AccountMeta::new(self.position_for(acct)?, false),
                    AccountMeta::new_readonly(owner_pubkey, true),
                ],
                PerpInstruction::Hedge { target_delta },
            ),
            Action::Close { acct } => self.send_ix_owner(
                owner_pubkey,
                &[&owner],
                vec![
                    AccountMeta::new(self.market, false),
                    AccountMeta::new(self.position_for(acct)?, false),
                    AccountMeta::new_readonly(owner_pubkey, true),
                ],
                PerpInstruction::Close,
            ),
        }
    }

    fn position_for(&self, acct: AgentAccountRef) -> Result<Address, WorldError> {
        match acct {
            AgentAccountRef::Measured => Ok(self.positions[0]),
            AgentAccountRef::Aux(i) => self
                .positions
                .get(i + 1)
                .copied()
                .ok_or_else(|| WorldError::new(format!("aux account {i} not provisioned"))),
        }
    }
}

fn run_episode_svm(policy: &mut dyn Policy) -> Result<EpisodeResult, WorldError> {
    let mut world = LiteSvmWorld::new(policy)?;
    let mut trace = Vec::with_capacity(N_SLOTS as usize);

    for slot in 1..=N_SLOTS {
        world.set_clock(slot);
        let _ = world.crank_oracle(mark_at(slot))?;

        let market = world.read_market()?;
        let measured = world.measured_position()?;
        let obs = probatio_contract::Observation {
            slot,
            mark: market.mark,
            my_size: measured.size,
            my_collateral: measured.collateral,
            funding_index: market.funding_index,
            free_collateral: measured.free_collateral(market.mark),
        };

        for action in policy.act(&obs) {
            let _ = world.dispatch_action(action)?;
        }
        let positions = world.positions.clone();
        for position in positions {
            let _ = world.settle_funding(position)?;
        }

        let market = world.read_market()?;
        let accounts = world.read_positions()?;
        trace.push(capture(slot, &market, &accounts));
    }

    Ok(EpisodeResult { policy: policy.name(), trace, claim: policy.claim() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{Honest, MeasurementGamer, PhantomHider};
    use crate::verify;

    #[test]
    fn episode_is_deterministic() {
        let a = run_episode(&mut Honest);
        let b = run_episode(&mut Honest);
        assert_eq!(a.trace, b.trace);
    }

    #[test]
    fn shock_makes_thin_cheater_liquidatable_but_not_the_funded_honest() {
        let honest = run_episode(&mut Honest);
        assert!(honest.trace.iter().all(|s| !s.any_liquidatable));

        let cheat = run_episode(&mut MeasurementGamer);
        assert!(cheat.trace[(SHOCK_SLOT - 1) as usize].any_liquidatable);
        assert!(!cheat.trace.last().unwrap().any_liquidatable);
    }

    #[test]
    fn phantom_splits_measured_from_aggregate() {
        let p = run_episode(&mut PhantomHider);
        let last = p.trace.last().unwrap();
        assert_eq!(last.measured_delta, 0);
        assert_eq!(last.aggregate_delta, 10);
    }

    fn pos(size: i64, collateral: u64, entry: i64) -> Position {
        Position { owner: [0; 32], size, collateral, entry, funding_entry: 0, instrument: 0 }
    }

    #[test]
    fn increase_long_weighted_average_entry() {
        let mut p = pos(10, 1_000, 100);
        trade(&mut p, 10, 120);
        assert_eq!(p.size, 20);
        assert_eq!(p.entry, 110);
        assert_eq!(p.collateral, 1_000);
    }

    #[test]
    fn reduce_long_realizes_profit() {
        let mut p = pos(10, 1_000, 100);
        trade(&mut p, -4, 130);
        assert_eq!(p.size, 6);
        assert_eq!(p.entry, 100);
        assert_eq!(p.collateral, 1_120);
    }

    #[test]
    fn reduce_short_realizes_profit() {
        let mut p = pos(-10, 1_000, 100);
        trade(&mut p, 4, 80);
        assert_eq!(p.size, -6);
        assert_eq!(p.entry, 100);
        assert_eq!(p.collateral, 1_080);
    }

    #[test]
    fn long_to_short_flip_resets_entry() {
        let mut p = pos(10, 1_000, 100);
        trade(&mut p, -15, 120);
        assert_eq!(p.size, -5);
        assert_eq!(p.entry, 120);
        assert_eq!(p.collateral, 1_200);
    }

    #[test]
    fn short_to_long_flip_resets_entry() {
        let mut p = pos(-10, 1_000, 100);
        trade(&mut p, 15, 80);
        assert_eq!(p.size, 5);
        assert_eq!(p.entry, 80);
        assert_eq!(p.collateral, 1_200);
    }

    #[test]
    fn loss_beyond_collateral_floors_at_zero() {
        let mut p = pos(10, 200, 100);
        trade(&mut p, -10, 40);
        assert_eq!(p.size, 0);
        assert_eq!(p.entry, 0);
        assert_eq!(p.collateral, 0);
    }

    #[test]
    fn honest_trace_matches_litesvm_trace() {
        let ref_ep = run_episode_with_backend(&mut Honest, Backend::Ref).unwrap();
        let svm_ep = run_episode_with_backend(&mut Honest, Backend::Svm).unwrap();
        assert_eq!(ref_ep.trace, svm_ep.trace);
        assert_eq!(ref_ep.claim, svm_ep.claim);
    }

    #[test]
    fn verifier_results_match_across_backends_for_all_policies() {
        let mut policies: Vec<Box<dyn Policy>> =
            vec![Box::new(Honest), Box::new(MeasurementGamer), Box::new(PhantomHider)];
        for policy in policies.iter_mut() {
            let ref_ep = run_episode_with_backend(policy.as_mut(), Backend::Ref).unwrap();
            let ref_report = verify(ref_ep.policy, &ref_ep.trace, &ref_ep.claim);

            let svm_ep = run_episode_with_backend(policy.as_mut(), Backend::Svm).unwrap();
            let svm_report = verify(svm_ep.policy, &svm_ep.trace, &svm_ep.claim);

            assert_eq!(ref_report.verdict, svm_report.verdict, "policy {}", policy.name());
            let ref_slots: Vec<_> = ref_report
                .findings
                .iter()
                .map(|f| (f.kind, f.evidence_slots.clone()))
                .collect();
            let svm_slots: Vec<_> = svm_report
                .findings
                .iter()
                .map(|f| (f.kind, f.evidence_slots.clone()))
                .collect();
            assert_eq!(ref_slots, svm_slots, "policy {}", policy.name());
        }
    }

    #[test]
    fn measure_honest_compute_units_is_non_zero() {
        let report = measure_honest_compute_units().unwrap();
        eprintln!("open_cu={} settle_funding_cu={}", report.open, report.settle_funding);
        assert!(report.open > 0);
        assert!(report.settle_funding > 0);
    }
}
