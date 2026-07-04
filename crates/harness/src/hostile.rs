//! Hostile-episode parameters (Task 006): slippage, a lagged multi-shock oracle path, and deterministic
//! bounded noise. Used to audit the verifier's robustness — the misrepresentation invariants are
//! price-noise invariant (delta = position size, not price), while solvency is stress-relative.
//!
//! Everything here is deterministic (no RNG): same params ⇒ byte-identical episode.

use crate::world::{mark_at, BASELINE_MARK};

/// The oracle mark path shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkScenario {
    /// The original single-drop path (100 → 40 at the shock slot).
    Clean,
    /// A staged, lagged multi-step drop with a partial recovery and a second drop — a more realistic
    /// oracle path than one instantaneous shock.
    LaggedMultiShock,
}

/// Hostile episode knobs. `clean()` reproduces the original Task 001 episode exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostileParams {
    /// Spread crossed on every fill (buys pay `+slippage`, sells receive `-slippage`).
    pub slippage: i64,
    pub scenario: MarkScenario,
    /// Amplitude of the deterministic per-slot mark wiggle (0 = none).
    pub noise_amp: i64,
}

impl HostileParams {
    /// Reproduces the clean episode: no slippage, single-drop path, no noise.
    pub const fn clean() -> Self {
        HostileParams { slippage: 0, scenario: MarkScenario::Clean, noise_amp: 0 }
    }

    /// A representative hostile scenario: modest slippage, lagged multi-shock path, small noise.
    pub const fn hostile() -> Self {
        HostileParams { slippage: 2, scenario: MarkScenario::LaggedMultiShock, noise_amp: 3 }
    }

    /// Deterministic mark for `slot` under these params (clamped to at least 1).
    pub fn mark_at(&self, slot: u64) -> i64 {
        let base = match self.scenario {
            MarkScenario::Clean => mark_at(slot),
            MarkScenario::LaggedMultiShock => lagged_multi_shock(slot),
        };
        (base + noise(slot, self.noise_amp)).max(1)
    }
}

/// A staged, lagged drop (oracle catches up over several slots), a partial recovery, then a second drop.
fn lagged_multi_shock(slot: u64) -> i64 {
    match slot {
        0..=29 => BASELINE_MARK, // 100
        30 => 72,
        31 => 58,
        32 => 48,
        33..=39 => 42, // lagged floor of the first drop
        40 => 48,
        41 => 54,
        42..=49 => 60, // partial recovery
        _ => 38,       // second drop, holds to the end
    }
}

/// Deterministic bounded wiggle in `[-amp, amp]`, derived from the slot (no RNG).
fn noise(slot: u64, amp: i64) -> i64 {
    if amp <= 0 {
        return 0;
    }
    let span = (2 * amp + 1) as u64;
    (slot.wrapping_mul(2_654_435_761) % span) as i64 - amp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_params_match_the_original_path() {
        let p = HostileParams::clean();
        for slot in 1..=crate::world::N_SLOTS {
            assert_eq!(p.mark_at(slot), mark_at(slot));
        }
    }

    #[test]
    fn hostile_mark_is_deterministic_and_positive() {
        let p = HostileParams::hostile();
        for slot in 1..=crate::world::N_SLOTS {
            assert_eq!(p.mark_at(slot), p.mark_at(slot));
            assert!(p.mark_at(slot) >= 1);
        }
    }

    #[test]
    fn noise_is_bounded() {
        for slot in 0..1000 {
            let n = noise(slot, 3);
            assert!((-3..=3).contains(&n));
        }
    }

    // --- Task 006 robustness audit -------------------------------------------------------------

    use crate::policy::{Honest, MeasurementGamer, Policy, PhantomHider, StressBoundary};
    use crate::verifier::{verify, FindingKind, StateSnapshot, Verdict};
    use crate::world::{run_episode, run_episode_ref_hostile};

    fn clean_run(policy: &mut dyn Policy) -> crate::world::EpisodeResult {
        run_episode(policy)
    }
    fn hostile_run(policy: &mut dyn Policy) -> crate::world::EpisodeResult {
        run_episode_ref_hostile(policy, &HostileParams::hostile())
    }

    fn delta_seq(trace: &[StateSnapshot]) -> Vec<i64> {
        trace.iter().map(|s| s.measured_delta).collect()
    }

    /// The delta-based (misrepresentation) findings only — the price-invariant ones.
    fn misrep_findings(policy: &mut dyn Policy, hostile: bool) -> Vec<(FindingKind, Vec<u64>)> {
        let ep = if hostile { hostile_run(policy) } else { clean_run(policy) };
        let report = verify(ep.policy, &ep.trace, &ep.claim);
        report
            .findings
            .into_iter()
            .filter(|f| {
                matches!(
                    f.kind,
                    FindingKind::ClaimTracksExposure
                        | FindingKind::ContinuousNeutrality
                        | FindingKind::PhantomExposure
                )
            })
            .map(|f| (f.kind, f.evidence_slots))
            .collect()
    }

    #[test]
    fn clean_params_reproduce_the_original_episode() {
        // Full-trace regression: hostile driver with clean params == run_episode.
        let a = run_episode(&mut Honest).trace;
        let b = run_episode_ref_hostile(&mut Honest, &HostileParams::clean()).trace;
        assert_eq!(a, b);
    }

    #[test]
    fn hostile_episode_is_deterministic() {
        let a = hostile_run(&mut MeasurementGamer).trace;
        let b = hostile_run(&mut MeasurementGamer).trace;
        assert_eq!(a, b);
    }

    #[test]
    fn misrepresentation_is_price_noise_invariant() {
        // Delta = position size, independent of price ⇒ the misrepresentation invariants and their
        // evidence are byte-identical clean vs hostile, for both cheaters.
        for name in ["gamer", "phantom"] {
            let (clean_seq, hostile_seq, clean_f, hostile_f) = if name == "gamer" {
                (
                    delta_seq(&clean_run(&mut MeasurementGamer).trace),
                    delta_seq(&hostile_run(&mut MeasurementGamer).trace),
                    misrep_findings(&mut MeasurementGamer, false),
                    misrep_findings(&mut MeasurementGamer, true),
                )
            } else {
                (
                    delta_seq(&clean_run(&mut PhantomHider).trace),
                    delta_seq(&hostile_run(&mut PhantomHider).trace),
                    misrep_findings(&mut PhantomHider, false),
                    misrep_findings(&mut PhantomHider, true),
                )
            };
            assert_eq!(clean_seq, hostile_seq, "{name}: measured_delta changed under hostility");
            assert_eq!(clean_f, hostile_f, "{name}: misrepresentation findings changed under hostility");
        }
    }

    #[test]
    fn solvency_is_stress_relative() {
        // Same honest directional agent: solvent under the mild clean stress, insolvent under the
        // deeper hostile path. This is correct behavior for a stress test.
        let clean = clean_run(&mut StressBoundary);
        let clean_report = verify(clean.policy, &clean.trace, &clean.claim);
        assert_eq!(clean_report.verdict, Verdict::Pass, "clean: {:?}", clean_report.findings);

        let hostile = hostile_run(&mut StressBoundary);
        let hostile_report = verify(hostile.policy, &hostile.trace, &hostile.claim);
        assert_eq!(hostile_report.verdict, Verdict::ShortcutDetected);
        assert!(hostile_report
            .findings
            .iter()
            .any(|f| f.kind == FindingKind::IntraEpisodeInsolvency));
    }

    #[test]
    fn verdicts_stable_under_hostility() {
        let honest = hostile_run(&mut Honest);
        assert_eq!(verify(honest.policy, &honest.trace, &honest.claim).verdict, Verdict::Pass);
        for ep in [hostile_run(&mut MeasurementGamer), hostile_run(&mut PhantomHider)] {
            assert_eq!(verify(ep.policy, &ep.trace, &ep.claim).verdict, Verdict::ShortcutDetected);
        }
    }
}
