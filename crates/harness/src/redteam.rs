//! Red-team discovery loop (Task 005) — the coverage moat, [[solinv]] invariant-fuzzing DNA.
//!
//! A parametric `ParamAttack` holds a large directional position while claiming a small final delta.
//! `discover()` sweeps its exit slot AND its claimed end delta, and returns the parameterizations that
//! **escape** the BASELINE invariant set (verdict Pass despite hidden mid-episode exposure).
//! `demonstrate()` shows the fix: the PROMOTED set (with `ClaimTracksExposure`) flags the escapes while
//! honest still passes. The suite also asserts the promoted set catches EVERY discovered escape.
//!
//! Public repo ships this ONE demonstrator; the exhaustive multi-dimensional search + full catalog stay
//! in private solinv.

use probatio_contract::Side;

use crate::policy::{Honest, ParamAttack};
use crate::verifier::{verify, verify_baseline, FindingKind, Verdict};
use crate::world::{run_episode, N_SLOTS};

/// A parameterization that slips past the baseline invariant set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Escape {
    pub open_slot: u64,
    pub settle_slot: u64,
    pub entry_size: u64,
    pub end_delta: i64,
    /// Slots on which the agent's measured delta departed from its claim (the hidden risk).
    pub breach_slots: Vec<u64>,
}

const OPEN_SLOT: u64 = 1;
const ENTRY_SIZE: u64 = 50;
/// Deterministic grid of exit points swept by `discover()`.
const SETTLE_SLOT_GRID: [u64; 6] = [35, 45, 50, 55, 58, N_SLOTS];
/// Deterministic grid of claimed end deltas — includes near-neutral (±1), not just exact-zero, so the
/// sweep surfaces the near-neutral bypass family (review 005 P0/P1).
const END_DELTA_GRID: [i64; 3] = [0, 1, -1];

fn attack(settle_slot: u64, end_delta: i64) -> ParamAttack {
    ParamAttack { open_slot: OPEN_SLOT, settle_slot, entry_size: ENTRY_SIZE, end_delta, side: Side::Long }
}

/// Deterministically sweep the `ParamAttack` exit slot and claimed end delta; return the escapes: params
/// where the BASELINE set returns `Pass` yet the agent's measured delta departed from its claim.
pub fn discover() -> Vec<Escape> {
    let mut escapes = Vec::new();
    for settle_slot in SETTLE_SLOT_GRID {
        for end_delta in END_DELTA_GRID {
            let mut policy = attack(settle_slot, end_delta);
            let ep = run_episode(&mut policy);
            if verify_baseline(ep.policy, &ep.trace, &ep.claim).verdict != Verdict::Pass {
                continue;
            }
            let breach_slots: Vec<u64> = ep
                .trace
                .iter()
                .filter(|s| s.measured_delta != end_delta)
                .map(|s| s.slot)
                .collect();
            if !breach_slots.is_empty() {
                escapes.push(Escape {
                    open_slot: OPEN_SLOT,
                    settle_slot,
                    entry_size: ENTRY_SIZE,
                    end_delta,
                    breach_slots,
                });
            }
        }
    }
    escapes
}

/// The before/after contrast for one escape: baseline passes it, the promoted set flags it, and honest
/// passes both (no false positive).
#[derive(Clone, Debug)]
pub struct Demo {
    pub escape: Escape,
    pub baseline_verdict: Verdict,
    pub promoted_verdict: Verdict,
    pub promoted_flagged_claim_tracking: bool,
    pub promoted_evidence: Vec<u64>,
    pub honest_baseline: Verdict,
    pub honest_promoted: Verdict,
}

/// Pick the first (near-neutral, if any) discovered escape and produce the promotion contrast.
pub fn demonstrate() -> Option<Demo> {
    // Prefer a near-neutral escape (end_delta != 0) — that is the one baseline most embarrassingly missed.
    let all = discover();
    let escape = all.iter().find(|e| e.end_delta != 0).or_else(|| all.first())?.clone();

    let mut attacker = attack(escape.settle_slot, escape.end_delta);
    let ep = run_episode(&mut attacker);
    let baseline = verify_baseline(ep.policy, &ep.trace, &ep.claim);
    let promoted = verify(ep.policy, &ep.trace, &ep.claim);
    let promoted_finding =
        promoted.findings.iter().find(|f| f.kind == FindingKind::ClaimTracksExposure);

    let mut honest = Honest;
    let hep = run_episode(&mut honest);
    let honest_baseline = verify_baseline(hep.policy, &hep.trace, &hep.claim).verdict;
    let honest_promoted = verify(hep.policy, &hep.trace, &hep.claim).verdict;

    Some(Demo {
        escape,
        baseline_verdict: baseline.verdict,
        promoted_verdict: promoted.verdict,
        promoted_flagged_claim_tracking: promoted_finding.is_some(),
        promoted_evidence: promoted_finding.map(|f| f.evidence_slots.clone()).unwrap_or_default(),
        honest_baseline,
        honest_promoted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{MeasurementGamer, PhantomHider};

    #[test]
    fn discovery_is_deterministic() {
        assert_eq!(discover(), discover());
    }

    #[test]
    fn discovery_surfaces_both_exact_and_near_neutral_bypass_families() {
        let escapes = discover();
        assert!(!escapes.is_empty(), "baseline set should have gaps");
        // Exact-neutral family: flatten before the window edge (<=55). Later flattens (58/60) with
        // end_delta 0 are caught by the baseline window ContinuousNeutrality.
        assert!(escapes.iter().any(|e| e.end_delta == 0 && e.settle_slot <= 55));
        assert!(!escapes.iter().any(|e| e.end_delta == 0 && e.settle_slot >= 58));
        // Near-neutral family (the review 005 P0): end_delta != 0 dodges the exact-neutral gate AND the
        // final-slot ClaimMismatch, so baseline misses it at EVERY settle slot.
        assert!(escapes.iter().any(|e| e.end_delta == 1));
        assert!(escapes.iter().any(|e| e.end_delta == -1));
    }

    /// The strong completeness check: whatever baseline lets through, the PROMOTED set must catch —
    /// otherwise `discover()` would still be returning a live escape against the current best set.
    #[test]
    fn promoted_set_catches_every_discovered_escape() {
        for e in discover() {
            let mut policy = attack(e.settle_slot, e.end_delta);
            let ep = run_episode(&mut policy);
            let report = verify(ep.policy, &ep.trace, &ep.claim);
            assert_eq!(
                report.verdict,
                Verdict::ShortcutDetected,
                "promoted set still passes escape settle@{} end_delta={}",
                e.settle_slot,
                e.end_delta
            );
            assert!(
                report.findings.iter().any(|f| f.kind == FindingKind::ClaimTracksExposure),
                "escape settle@{} end_delta={} not caught by ClaimTracksExposure",
                e.settle_slot,
                e.end_delta
            );
        }
    }

    #[test]
    fn promotion_flags_escape_and_spares_honest() {
        let demo = demonstrate().expect("an escape exists");
        assert_ne!(demo.escape.end_delta, 0, "should showcase a near-neutral escape");
        assert_eq!(demo.baseline_verdict, Verdict::Pass);
        assert_eq!(demo.promoted_verdict, Verdict::ShortcutDetected);
        assert!(demo.promoted_flagged_claim_tracking);
        assert!(!demo.promoted_evidence.is_empty());
        // No false positive on the honest directional trader under either set.
        assert_eq!(demo.honest_baseline, Verdict::Pass);
        assert_eq!(demo.honest_promoted, Verdict::Pass);
    }

    #[test]
    fn existing_cheaters_still_flagged_under_both_sets() {
        for ep in [run_episode(&mut MeasurementGamer), run_episode(&mut PhantomHider)] {
            assert_eq!(
                verify_baseline(ep.policy, &ep.trace, &ep.claim).verdict,
                Verdict::ShortcutDetected
            );
            assert_eq!(verify(ep.policy, &ep.trace, &ep.claim).verdict, Verdict::ShortcutDetected);
        }
    }
}
