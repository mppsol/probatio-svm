//! Probatio SVM harness (Stage 0a): a pure-Rust reference model of the perp world, scripted policies,
//! and the invariant-set-driven verifier (the moat). Task 002 swaps `world` for a real Pinocchio
//! program driven through LiteSVM behind the same `contract` account layout.

pub mod policy;
pub mod verifier;
pub mod world;

pub use verifier::{verify, Finding, FindingKind, ShortcutReport, StateSnapshot, Verdict};
pub use world::{
    measure_guard_compute_units, measure_honest_compute_units, run_episode,
    run_episode_with_backend, Backend, ComputeUnitReport, EpisodeResult, GuardComputeUnitReport,
    N_SLOTS, SHOCK_SLOT,
};
