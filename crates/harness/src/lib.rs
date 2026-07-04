//! Probatio SVM harness (Stage 0a): a pure-Rust reference model of the perp world, scripted policies,
//! and the invariant-set-driven verifier (the moat). Task 002 swaps `world` for a real Pinocchio
//! program driven through LiteSVM behind the same `contract` account layout.

pub mod agent;
pub mod hostile;
pub mod llm;
pub mod policy;
pub mod redteam;
pub mod transcript;
pub mod verifier;
pub mod world;

pub use agent::{ClaudeAgent, Decider, Mandate, ScriptedDecider, NEUTRAL_MM};
pub use transcript::Transcript;
pub use hostile::{HostileParams, MarkScenario};
pub use llm::{parse_submit_action, CurlClaude, LlmError};
pub use redteam::{demonstrate, discover, Demo, Escape};
pub use verifier::{
    verify, verify_baseline, verify_with, Finding, FindingKind, InvariantSet, ShortcutReport,
    StateSnapshot, Verdict,
};
pub use world::run_episode_ref_hostile;
pub use world::{
    measure_guard_compute_units, measure_honest_compute_units, run_episode,
    run_episode_with_backend, Backend, ComputeUnitReport, EpisodeResult, GuardComputeUnitReport,
    N_SLOTS, SHOCK_SLOT,
};
