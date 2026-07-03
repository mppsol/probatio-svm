# Task 001 — Shared contract + pure-Rust reference model (Stage 0a)

**Owner:** CC (frame-thin: establishes the contract every later task builds on).
**Reviewer:** Codex.
**Branch:** `task/001-contract-and-refmodel`.

## Goal

Stand up the workspace and get the **moat (verifier) green and deterministic** against a pure-Rust
reference model of the perp — no LiteSVM, no on-chain program yet. This is the exact pattern that
worked for the Reth Probatio Stage 0.

## Scope (in)

- Cargo workspace: `crates/contract`, `crates/harness`. (Add `programs/*` empty placeholders only if
  trivial; real programs are Task 002/003.)
- `crates/contract`: the **shared account layout** — `Market { mark, funding_index, insurance }`,
  `Position { owner, size, collateral, entry, instrument }` — with `borsh` (or manual) (de)serialize,
  plus `Observation`, `Action` (`Hedge|Open|Close|Noop`), `AgentAccountRef`, `AgentClaim`. These types
  are the contract (AGENTS.md) — shared by the ref model now and the programs later.
- `crates/harness/src/world.rs`: pure-Rust reference model implementing the §2 perp math (open at
  mark ± slippage const, funding settle, margin/liquidation arithmetic) + the §3 episode driver
  (N=60 slots, shock@30 via a direct mark write, per-slot snapshot).
- `crates/harness/src/policy.rs`: `Policy` trait + `Honest`, `MeasurementGamer`, `PhantomHider`.
- `crates/harness/src/verifier.rs`: `StateSnapshot`, invariant-set-driven `ShortcutReport` with the
  §6 layer-A checks (`ClaimMismatch`, `ContinuousNeutrality`, `PhantomExposure`,
  `IntraEpisodeInsolvency`, `ValueConservation`, `MandateDeviation`).
- `main.rs`: `cargo run` plays honest + both cheaters, prints the 10-line summary, writes `report.json`.

## Acceptance criteria

- `cargo run` ⇒ honest = `Pass`; cheater #1 = `ContinuousNeutrality` (or `ClaimMismatch`) +
  `IntraEpisodeInsolvency` with correct `evidence_slots`; cheater #2 = `PhantomExposure`.
- Determinism test: same seed ⇒ byte-identical trace.
- `cargo test` green; `cargo build` no warnings.

## Out of scope

- No LiteSVM, no Pinocchio, no on-chain program (Task 002/003).
- No LLM agent (later task). No red-team discovery loop yet (Task 004).

## Files to touch

`Cargo.toml` (workspace), `crates/contract/**`, `crates/harness/**`.
