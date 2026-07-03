# Task 002 — Pinocchio perp program + LiteSVM driver (Stage 0b)

**Owner:** Codex (frame-thick: tightly-specified on-chain program to the Task 001 contract).
**Reviewer:** CC.
**Branch:** `task/002-pinocchio-perp-litesvm`.
**Depends on:** Task 001 merged (the `crates/contract` account layout is the spec).

## Goal

Replace the pure-Rust reference model with a **real Pinocchio perp program** driven through `LiteSVM`,
producing the **same episode traces** the reference model produced — proving the harness runs against a
real Solana program, not a mock.

## Scope (in)

- `programs/perp` (Pinocchio): instructions `Deposit`, `Open{side,qty}`, `Hedge{target_delta}`,
  `Close`, `CrankOracle{mark}` (harness-only authority), `SettleFunding`. Account layout is imported
  from `crates/contract` — do NOT redefine it. Builds under `cargo build-sbf`.
- `crates/harness/src/world.rs`: add a `LiteSvmWorld` behind the same driver interface as the ref model;
  set `Clock` sysvar per slot for determinism; read `Market`/`Position` accounts each slot for the
  snapshot (`StateSnapshot` unchanged).
- Keep the pure-Rust ref model as a selectable backend (regression oracle: both must yield identical
  verdicts on the same policies).

## Acceptance criteria

- `cargo run --features litesvm` (or a `--backend svm` flag) plays the same 3 policies with the **same
  verdicts and evidence_slots** as the ref model.
- A test asserts ref-model trace == LiteSVM trace for the honest policy (or documents the exact,
  intentional deltas).
- `cargo build-sbf` clean for `programs/perp`. `cargo test` green offline.

## Out of scope

- The guard program (Task 003). LLM agent, red-team loop (later).

## Notes

- Determinism: no wallclock — the driver sets `Clock.slot`/`unix_timestamp` explicitly each step.
- CU: record the CU cost of `Open`/`SettleFunding` in the PR description (feeds the Pinocchio low-CU
  pitch; see `STAGE0_DESIGN.md` §7 and [[project_pinocchio_solinv_wedge]]).
