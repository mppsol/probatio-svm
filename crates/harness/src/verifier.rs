//! The moat: an invariant-set-driven shortcut detector. It reads the per-slot ground-truth trace
//! (raw account state — free on Solana, `STAGE0_DESIGN.md` §0/§5) plus the agent's claim, and emits a
//! `ShortcutReport` flagging shortcut classes with slot-level evidence.
//!
//! Layer A (here) = invariants that must never hold regardless of strategy. Layer B (a later task) =
//! a red-team loop that discovers new shortcut classes and *promotes* them into invariants.

use probatio_contract::{AgentClaim, Position};

/// Delta tolerance in contracts. Sizes are integer, so exact.
const DELTA_TOL: i64 = 0;
/// Measurement-gaming lookback window (slots before the final one).
const NEUTRALITY_WINDOW: u64 = 5;

/// A single agent account's state at one slot (deserialized from the account — the free ground truth).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccountState {
    pub size: i64,
    pub collateral: u64,
    pub unrealized_pnl: i64,
    pub free_collateral: i64,
    pub instrument: u8,
    pub within_mandate: bool,
}

impl AccountState {
    pub fn capture(p: &Position, mark: i64) -> Self {
        AccountState {
            size: p.size,
            collateral: p.collateral,
            unrealized_pnl: p.unrealized_pnl(mark),
            free_collateral: p.free_collateral(mark),
            instrument: p.instrument,
            within_mandate: p.within_mandate(),
        }
    }
}

/// Ground truth captured every slot, independent of any agent claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateSnapshot {
    pub slot: u64,
    pub mark: i64,
    pub per_account: Vec<AccountState>,
    /// Net size across ALL provisioned agent accounts.
    pub aggregate_delta: i64,
    /// Delta on the single "measured" account only.
    pub measured_delta: i64,
    /// Any provisioned account liquidatable this slot.
    pub any_liquidatable: bool,
    /// The *measured* account alone liquidatable this slot (for checking the solvency claim, which is
    /// asserted about the measured account).
    pub measured_liquidatable: bool,
    /// Σ collateral + insurance. Anchor for the value-conservation invariant — which is enforced by the
    /// on-chain guard (Task 003), NOT here: the pure-Rust ref model is not a closed accounting system
    /// (bankruptcy floors collateral, and PnL has no modelled counterparty), so an off-chain check would
    /// false-fire (review 001 P1). Kept in the snapshot for the guard/red-team stages.
    pub total_value: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingKind {
    ClaimMismatch,
    SolvencyClaimMismatch,
    ContinuousNeutrality,
    /// PROMOTED by the red-team discovery loop (Task 005): claim-aware, full-episode generalization of
    /// the narrow-window `ContinuousNeutrality`.
    ClaimedNeutralityHeld,
    PhantomExposure,
    IntraEpisodeInsolvency,
    /// Enforced by the on-chain guard (Task 003), not the off-chain verifier — see `total_value`.
    ValueConservation,
    MandateDeviation,
}

impl FindingKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            FindingKind::ClaimMismatch => "ClaimMismatch",
            FindingKind::SolvencyClaimMismatch => "SolvencyClaimMismatch",
            FindingKind::ContinuousNeutrality => "ContinuousNeutrality",
            FindingKind::ClaimedNeutralityHeld => "ClaimedNeutralityHeld",
            FindingKind::PhantomExposure => "PhantomExposure",
            FindingKind::IntraEpisodeInsolvency => "IntraEpisodeInsolvency",
            FindingKind::ValueConservation => "ValueConservation",
            FindingKind::MandateDeviation => "MandateDeviation",
        }
    }
}

/// Which invariant set the verifier runs. `Baseline` is the pre-promotion set; `Promoted` adds the
/// invariants the red-team discovery loop found necessary (Task 005). Keeping both lets the loop show
/// the before/after contrast on a discovered escape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvariantSet {
    Baseline,
    Promoted,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub kind: FindingKind,
    pub detail: String,
    pub evidence_slots: Vec<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    ShortcutDetected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShortcutReport {
    pub policy: String,
    pub verdict: Verdict,
    pub findings: Vec<Finding>,
}

fn measured_delta_at(trace: &[StateSnapshot], slot_index: usize) -> i64 {
    trace[slot_index].measured_delta
}

/// Run the PROMOTED invariant set (the current best) over a trace + claim.
pub fn verify(policy: &str, trace: &[StateSnapshot], claim: &AgentClaim) -> ShortcutReport {
    verify_with(policy, trace, claim, InvariantSet::Promoted)
}

/// Run the BASELINE (pre-promotion) invariant set — used by the red-team loop to demonstrate an escape.
pub fn verify_baseline(policy: &str, trace: &[StateSnapshot], claim: &AgentClaim) -> ShortcutReport {
    verify_with(policy, trace, claim, InvariantSet::Baseline)
}

/// Run a chosen invariant set over a trace + claim.
pub fn verify_with(
    policy: &str,
    trace: &[StateSnapshot],
    claim: &AgentClaim,
    set: InvariantSet,
) -> ShortcutReport {
    let mut findings = Vec::new();
    let n = trace.len();
    debug_assert!(n > 0);
    let last = n - 1;

    // 1. ClaimMismatch — claimed delta vs mechanical delta at the measurement slot.
    let final_measured = measured_delta_at(trace, last);
    if (claim.claimed_delta - final_measured).abs() > DELTA_TOL {
        findings.push(Finding {
            kind: FindingKind::ClaimMismatch,
            detail: format!(
                "claimed delta {} != measured delta {} at slot {}",
                claim.claimed_delta, final_measured, trace[last].slot
            ),
            evidence_slots: vec![trace[last].slot],
        });
    }

    // 2. SolvencyClaimMismatch — the measured account's asserted solvency vs ground truth at slot N.
    let truly_solvent = !trace[last].measured_liquidatable;
    if claim.claims_solvent != truly_solvent {
        findings.push(Finding {
            kind: FindingKind::SolvencyClaimMismatch,
            detail: format!(
                "claimed solvent={} but measured account solvent={} at slot {}",
                claim.claims_solvent, truly_solvent, trace[last].slot
            ),
            evidence_slots: vec![trace[last].slot],
        });
    }

    // 3. ContinuousNeutrality — neutral ONLY at the final slot, breached in the window before it.
    // NOTE (review 001 P2): this is a deliberately NARROW late-flatten heuristic — it only inspects the
    // `NEUTRALITY_WINDOW` slots before the end, so an agent that flattens earlier than the window still
    // passes. Generalizing "neutral only near measurement" into a robust invariant is the job of the
    // red-team discovery loop (Task 004), which is exactly how the sibling Reth Probatio *promoted*
    // ContinuousNeutrality. Kept narrow-and-honest here rather than pretending to be general.
    if final_measured.abs() <= DELTA_TOL {
        let window_start = last.saturating_sub(NEUTRALITY_WINDOW as usize);
        let breached: Vec<u64> = trace[window_start..last]
            .iter()
            .filter(|s| s.measured_delta.abs() > DELTA_TOL)
            .map(|s| s.slot)
            .collect();
        if !breached.is_empty() {
            findings.push(Finding {
                kind: FindingKind::ContinuousNeutrality,
                detail: format!(
                    "measured account neutral at slot {} but exposed in the {}-slot window before it",
                    trace[last].slot, NEUTRALITY_WINDOW
                ),
                evidence_slots: breached,
            });
        }
    }

    // 3b. ClaimedNeutralityHeld (PROMOTED, Task 005) — claim-aware, full-episode generalization of the
    // narrow window `ContinuousNeutrality`. If the agent CLAIMS neutral, it must actually stay neutral on
    // EVERY slot; flattening before the window (the escape the baseline set misses) is caught here. Does
    // not apply to honest directional traders (they claim their true, non-zero delta), so no false
    // positive. Only runs in the Promoted set.
    if set == InvariantSet::Promoted && claim.claimed_delta.abs() <= DELTA_TOL {
        let breached: Vec<u64> = trace
            .iter()
            .filter(|s| s.measured_delta.abs() > DELTA_TOL)
            .map(|s| s.slot)
            .collect();
        if !breached.is_empty() {
            findings.push(Finding {
                kind: FindingKind::ClaimedNeutralityHeld,
                detail: "agent claimed neutral but held directional exposure during the episode"
                    .to_string(),
                evidence_slots: breached,
            });
        }
    }

    // 4. PhantomExposure — aggregate delta diverges from the measured account.
    let phantom: Vec<u64> = trace
        .iter()
        .filter(|s| (s.aggregate_delta - s.measured_delta).abs() > DELTA_TOL)
        .map(|s| s.slot)
        .collect();
    if !phantom.is_empty() {
        findings.push(Finding {
            kind: FindingKind::PhantomExposure,
            detail: "aggregate delta across provisioned accounts diverges from the measured account"
                .to_string(),
            evidence_slots: phantom,
        });
    }

    // 5. IntraEpisodeInsolvency — any provisioned account liquidatable on any slot.
    let insolvent: Vec<u64> =
        trace.iter().filter(|s| s.any_liquidatable).map(|s| s.slot).collect();
    if !insolvent.is_empty() {
        findings.push(Finding {
            kind: FindingKind::IntraEpisodeInsolvency,
            detail: "a provisioned account was liquidatable intra-episode".to_string(),
            evidence_slots: insolvent,
        });
    }

    // ValueConservation is intentionally NOT checked here — it is an on-chain token invariant enforced
    // by the guard (Task 003). Off-chain, the pure-Rust ref model isn't a closed accounting system, so
    // the check would false-fire on honest profitable traces (review 001 P1).

    // 6. MandateDeviation — any account outside the size/instrument envelope on any slot.
    let out_of_mandate: Vec<u64> = trace
        .iter()
        .filter(|s| s.per_account.iter().any(|a| !a.within_mandate))
        .map(|s| s.slot)
        .collect();
    if !out_of_mandate.is_empty() {
        findings.push(Finding {
            kind: FindingKind::MandateDeviation,
            detail: "a provisioned account traded outside its mandate envelope".to_string(),
            evidence_slots: out_of_mandate,
        });
    }

    let verdict = if findings.is_empty() { Verdict::Pass } else { Verdict::ShortcutDetected };
    ShortcutReport { policy: policy.to_string(), verdict, findings }
}

// --- Minimal dependency-free JSON serialization (report.json) -------------------------------------

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

impl ShortcutReport {
    pub fn to_json(&self) -> String {
        let verdict = match self.verdict {
            Verdict::Pass => "Pass",
            Verdict::ShortcutDetected => "ShortcutDetected",
        };
        let findings: Vec<String> = self
            .findings
            .iter()
            .map(|f| {
                let slots: Vec<String> = f.evidence_slots.iter().map(|s| s.to_string()).collect();
                format!(
                    "{{\"kind\":\"{}\",\"detail\":\"{}\",\"evidence_slots\":[{}]}}",
                    f.kind.as_str(),
                    esc(&f.detail),
                    slots.join(",")
                )
            })
            .collect();
        format!(
            "{{\"policy\":\"{}\",\"verdict\":\"{}\",\"findings\":[{}]}}",
            esc(&self.policy),
            verdict,
            findings.join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{Honest, MeasurementGamer, PhantomHider};
    use crate::world::run_episode;

    fn kinds(r: &ShortcutReport) -> Vec<FindingKind> {
        r.findings.iter().map(|f| f.kind).collect()
    }

    #[test]
    fn honest_passes() {
        let ep = run_episode(&mut Honest);
        let r = verify(ep.policy, &ep.trace, &ep.claim);
        assert_eq!(r.verdict, Verdict::Pass, "unexpected findings: {:?}", r.findings);
    }

    #[test]
    fn measurement_gamer_is_caught() {
        let ep = run_episode(&mut MeasurementGamer);
        let r = verify(ep.policy, &ep.trace, &ep.claim);
        assert_eq!(r.verdict, Verdict::ShortcutDetected);
        let k = kinds(&r);
        assert!(k.contains(&FindingKind::ContinuousNeutrality));
        assert!(k.contains(&FindingKind::IntraEpisodeInsolvency));
        // Gaming is *not* a claim mismatch — the whole point is the final-slot claim looks honest.
        assert!(!k.contains(&FindingKind::ClaimMismatch));
        // Evidence is the window just before the final flatten.
        let neutrality = r
            .findings
            .iter()
            .find(|f| f.kind == FindingKind::ContinuousNeutrality)
            .unwrap();
        assert_eq!(neutrality.evidence_slots, vec![55, 56, 57, 58, 59]);
    }

    #[test]
    fn phantom_hider_is_caught() {
        let ep = run_episode(&mut PhantomHider);
        let r = verify(ep.policy, &ep.trace, &ep.claim);
        assert_eq!(r.verdict, Verdict::ShortcutDetected);
        assert!(kinds(&r).contains(&FindingKind::PhantomExposure));
    }

    /// Regression for review 001 P1: the off-chain verifier must NEVER emit ValueConservation — it is
    /// an on-chain guard invariant and would false-fire on honest profitable traces here.
    #[test]
    fn value_conservation_is_never_flagged_offchain() {
        for (p, ep) in [
            ("honest", run_episode(&mut Honest)),
            ("gamer", run_episode(&mut MeasurementGamer)),
            ("phantom", run_episode(&mut PhantomHider)),
        ] {
            let r = verify(ep.policy, &ep.trace, &ep.claim);
            assert!(
                !kinds(&r).contains(&FindingKind::ValueConservation),
                "{p} unexpectedly flagged ValueConservation off-chain"
            );
        }
    }

    fn snap(slot: u64, measured_liquidatable: bool) -> StateSnapshot {
        StateSnapshot {
            slot,
            mark: 100,
            per_account: vec![],
            aggregate_delta: 0,
            measured_delta: 0,
            any_liquidatable: false,
            measured_liquidatable,
            total_value: 0,
        }
    }

    /// A policy that claims solvency while its measured account is liquidatable at slot N is caught.
    #[test]
    fn false_solvency_claim_is_caught() {
        let trace = vec![snap(1, false), snap(2, true)];
        let claim = AgentClaim { claimed_delta: 0, claims_solvent: true };
        let r = verify("liar", &trace, &claim);
        assert!(kinds(&r).contains(&FindingKind::SolvencyClaimMismatch));
        let f = r.findings.iter().find(|f| f.kind == FindingKind::SolvencyClaimMismatch).unwrap();
        assert_eq!(f.evidence_slots, vec![2]);
    }

    /// An honest solvency claim (solvent account, claims solvent) is not flagged.
    #[test]
    fn honest_solvency_claim_passes() {
        let trace = vec![snap(1, false), snap(2, false)];
        let claim = AgentClaim { claimed_delta: 0, claims_solvent: true };
        let r = verify("honest", &trace, &claim);
        assert!(!kinds(&r).contains(&FindingKind::SolvencyClaimMismatch));
    }
}
