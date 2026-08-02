//! Probatio SVM harness (Stage 0a): a pure-Rust reference model of the perp world, scripted policies,
//! and the invariant-set-driven verifier (the moat). Task 002 swaps `world` for a real Pinocchio
//! program driven through LiteSVM behind the same `contract` account layout.

pub mod agent;
pub mod attest;
pub mod hostile;
pub mod jupiter;
pub mod llm;
pub mod mandate;
pub mod policy;
pub mod redteam;
pub mod transcript;
pub mod verifier;
pub mod world;

pub use agent::{ClaudeAgent, Decider, Mandate, ScriptedDecider, NEUTRAL_MM};
pub use attest::{attest, report_hash, Attestation, FeedbackCall, Reproduce};
pub use transcript::Transcript;
pub use hostile::{HostileParams, MarkScenario};
pub use jupiter::{
    jupiter_to_snapshots, jupiter_to_snapshots_with_mandate, sample_drift, sample_neutral,
    JupPosition, JupSide, JupSlot,
};
pub use llm::{parse_submit_action, CurlClaude, LlmError};
pub use mandate::spec_hash;
pub use redteam::{demonstrate, discover, Demo, Escape};
pub use verifier::{
    verify, verify_baseline, verify_with, Finding, FindingKind, InvariantSet, ShortcutReport,
    StateSnapshot, Verdict,
};
pub use world::{run_episode_ref_hostile, run_episode_ref_hostile_with_mandate};
pub use world::{
    measure_guard_compute_units, measure_honest_compute_units, run_episode,
    run_episode_with_backend, Backend, ComputeUnitReport, EpisodeResult, GuardComputeUnitReport,
    run_episode_with_mandate, N_SLOTS, SHOCK_SLOT,
};
