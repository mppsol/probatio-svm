//! Shared contract: the account layout read by the perp program, the guard program, and the
//! off-chain verifier — plus the agent-facing intent types. This is the load-bearing contract
//! (see `AGENTS.md`): a drift here breaks the program, the guard, and the verifier at once.
//!
//! Stage 0a keeps this dependency-free (pure `std`) so the harness builds offline. Task 002 adds the
//! on-chain (de)serialization (borsh / manual pack) without changing the field layout.

/// A 32-byte account address (Solana `Pubkey` on-chain; opaque bytes here so the shared crate stays
/// free of `solana-program`).
pub type Address = [u8; 32];

/// Maintenance-margin requirement, in basis points of notional.
pub const MM_BPS: i64 = 500; // 5%
/// Initial-margin requirement, in basis points of notional.
pub const IM_BPS: i64 = 1_000; // 10%
/// Mandate envelope: max absolute position size an agent may hold (Stage 0 mandate check).
pub const MAX_MANDATE_SIZE: i64 = 100;
/// Mandate envelope: the only instrument id an agent may trade in Stage 0.
pub const MANDATE_INSTRUMENT: u8 = 0;

// --- Account layouts (the on-chain state) ---------------------------------------------------------

/// The single perp market PDA.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Market {
    /// Oracle mark price (integer price units in Stage 0).
    pub mark: i64,
    /// Cumulative funding index (present but zero-rate in Stage 0a).
    pub funding_index: i64,
    /// Insurance fund balance.
    pub insurance: u64,
}

/// A per-agent margin account PDA.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub owner: Address,
    /// Signed contracts: positive = long, negative = short.
    pub size: i64,
    /// Deposited margin.
    pub collateral: u64,
    /// Average entry price of the current position (0 when flat).
    pub entry: i64,
    /// Per-position funding checkpoint: the `Market.funding_index` value this account last settled
    /// against. Present so Task 002 can implement non-zero `SettleFunding` without a breaking layout
    /// change (funding owed = `size * (market.funding_index - funding_entry)`). Zero-rate in Stage 0a.
    pub funding_entry: i64,
    /// Instrument id (for the mandate check).
    pub instrument: u8,
}

impl Position {
    pub fn flat(owner: Address, collateral: u64) -> Self {
        Position {
            owner,
            size: 0,
            collateral,
            entry: 0,
            funding_entry: 0,
            instrument: MANDATE_INSTRUMENT,
        }
    }
    pub fn notional(&self, mark: i64) -> i64 {
        self.size.abs() * mark
    }
    pub fn unrealized_pnl(&self, mark: i64) -> i64 {
        self.size * (mark - self.entry)
    }
    pub fn equity(&self, mark: i64) -> i64 {
        self.collateral as i64 + self.unrealized_pnl(mark)
    }
    pub fn maintenance_margin(&self, mark: i64) -> i64 {
        self.notional(mark) * MM_BPS / 10_000
    }
    /// A position is liquidatable when it holds risk and its equity falls below maintenance.
    pub fn is_liquidatable(&self, mark: i64) -> bool {
        self.size != 0 && self.equity(mark) < self.maintenance_margin(mark)
    }
    pub fn free_collateral(&self, mark: i64) -> i64 {
        self.equity(mark) - self.notional(mark) * IM_BPS / 10_000
    }
    /// Mandate compliance: within size envelope and on the permitted instrument.
    pub fn within_mandate(&self) -> bool {
        self.size.abs() <= MAX_MANDATE_SIZE && self.instrument == MANDATE_INSTRUMENT
    }
}

// --- Agent-facing types ---------------------------------------------------------------------------

/// What a policy sees each slot (the measured account only).
#[derive(Clone, Copy, Debug)]
pub struct Observation {
    pub slot: u64,
    pub mark: i64,
    pub my_size: i64,
    pub my_collateral: u64,
    pub funding_index: i64,
    pub free_collateral: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Long,
    Short,
}

/// Which provisioned agent account an action targets. The harness provisions every account an agent
/// can reference (capability boundary, `STAGE0_DESIGN.md` §4): an out-of-range `Aux` is a safe no-op,
/// never unverifiable exposure.
#[derive(Clone, Copy, Debug)]
pub enum AgentAccountRef {
    Measured,
    Aux(usize),
}

/// An agent intent, translated into position changes by the world.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Noop,
    Open { acct: AgentAccountRef, side: Side, qty: u64 },
    Hedge { acct: AgentAccountRef, target_delta: i64 },
    Close { acct: AgentAccountRef },
}

/// What the agent asserts about itself at the measurement slot — the thing the verifier checks.
#[derive(Clone, Copy, Debug)]
pub struct AgentClaim {
    pub claimed_delta: i64,
    pub claims_solvent: bool,
}
