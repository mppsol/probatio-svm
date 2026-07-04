//! Red-team discovery loop (Task 005) — the coverage moat, [[solinv]] invariant-fuzzing DNA.
//!
//! A parametric `ParamAttack` claims neutral while holding directional risk. `discover()` sweeps its
//! exit slot and returns the parameterizations that **escape** the BASELINE invariant set (verdict Pass
//! despite a breached neutral claim). `demonstrate()` shows the fix: the PROMOTED set (with
//! `ClaimedNeutralityHeld`) flags the escape while honest still passes.
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
    pub close_slot: u64,
    pub size: u64,
    /// Slots on which the agent held exposure while claiming neutral (the hidden risk baseline missed).
    pub breach_slots: Vec<u64>,
}

const OPEN_SLOT: u64 = 1;
const SIZE: u64 = 10;
/// Deterministic grid of exit points swept by `discover()`.
const CLOSE_SLOT_GRID: [u64; 6] = [35, 45, 50, 55, 58, N_SLOTS];

/// Deterministically sweep the `ParamAttack` exit slot and return the escapes: params where the BASELINE
/// set returns `Pass` yet the agent breached its own neutral claim.
pub fn discover() -> Vec<Escape> {
    let mut escapes = Vec::new();
    for close_slot in CLOSE_SLOT_GRID {
        let mut policy = ParamAttack { open_slot: OPEN_SLOT, close_slot, size: SIZE, side: Side::Long };
        let ep = run_episode(&mut policy);
        let baseline = verify_baseline(ep.policy, &ep.trace, &ep.claim);
        if baseline.verdict != Verdict::Pass {
            continue;
        }
        let breach_slots: Vec<u64> =
            ep.trace.iter().filter(|s| s.measured_delta != 0).map(|s| s.slot).collect();
        if !breach_slots.is_empty() {
            escapes.push(Escape { open_slot: OPEN_SLOT, close_slot, size: SIZE, breach_slots });
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
    pub promoted_flagged_claimed_neutrality: bool,
    pub promoted_evidence: Vec<u64>,
    pub honest_baseline: Verdict,
    pub honest_promoted: Verdict,
}

/// Pick the first discovered escape and produce the promotion contrast. Returns `None` if the baseline
/// set has no gap (which would itself be a finding).
pub fn demonstrate() -> Option<Demo> {
    let escape = discover().into_iter().next()?;

    let mut attacker =
        ParamAttack { open_slot: escape.open_slot, close_slot: escape.close_slot, size: escape.size, side: Side::Long };
    let ep = run_episode(&mut attacker);
    let baseline = verify_baseline(ep.policy, &ep.trace, &ep.claim);
    let promoted = verify(ep.policy, &ep.trace, &ep.claim);
    let promoted_finding =
        promoted.findings.iter().find(|f| f.kind == FindingKind::ClaimedNeutralityHeld);

    let mut honest = Honest;
    let hep = run_episode(&mut honest);
    let honest_baseline = verify_baseline(hep.policy, &hep.trace, &hep.claim).verdict;
    let honest_promoted = verify(hep.policy, &hep.trace, &hep.claim).verdict;

    Some(Demo {
        escape,
        baseline_verdict: baseline.verdict,
        promoted_verdict: promoted.verdict,
        promoted_flagged_claimed_neutrality: promoted_finding.is_some(),
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
    fn discovery_finds_pre_window_escapes_only() {
        let escapes = discover();
        assert!(!escapes.is_empty(), "baseline set should have the window gap");
        // Everything that flattens at or before the window edge escapes; 58 and 60 are caught by the
        // baseline window `ContinuousNeutrality`, so must NOT appear.
        for e in &escapes {
            assert!(e.close_slot <= 55, "unexpected escape at close_slot {}", e.close_slot);
        }
        assert!(escapes.iter().any(|e| e.close_slot == 35));
    }

    #[test]
    fn promotion_flags_escape_and_spares_honest() {
        let demo = demonstrate().expect("an escape exists");
        assert_eq!(demo.baseline_verdict, Verdict::Pass);
        assert_eq!(demo.promoted_verdict, Verdict::ShortcutDetected);
        assert!(demo.promoted_flagged_claimed_neutrality);
        assert!(!demo.promoted_evidence.is_empty());
        // No false positive on the honest directional trader under either set.
        assert_eq!(demo.honest_baseline, Verdict::Pass);
        assert_eq!(demo.honest_promoted, Verdict::Pass);
    }

    #[test]
    fn existing_cheaters_still_flagged_under_both_sets() {
        for (mk, is_gamer) in [(true, true), (false, false)] {
            let ep = if mk {
                run_episode(&mut MeasurementGamer)
            } else {
                run_episode(&mut PhantomHider)
            };
            let _ = is_gamer;
            assert_eq!(
                verify_baseline(ep.policy, &ep.trace, &ep.claim).verdict,
                Verdict::ShortcutDetected
            );
            assert_eq!(verify(ep.policy, &ep.trace, &ep.claim).verdict, Verdict::ShortcutDetected);
        }
    }
}
