# Task 003 — Pinocchio guard program: revert a cheating tx (Stage 0c)

**Owner:** Codex (frame-thick).
**Reviewer:** CC.
**Branch:** `task/003-pinocchio-guard`.
**Depends on:** Task 002 merged.

## Goal

Ship the **second primitive** — a Pinocchio CPI-guard that reverts a violating transaction *in-block*.
This is the pitch-video "wow": prevention, not just post-hoc detection.

## Scope (in)

- `programs/guard` (Pinocchio): reads `Market`/`Position` (from `crates/contract`) and returns `Err`
  (aborting the tx) when a runtime invariant is violated. Stage 0c enforces **two**:
  `IntraEpisodeInsolvency` (post-state solvency of the acting account) and `MandateDeviation`
  (instrument / size envelope).
- Wire the perp settlement path to CPI into the guard (or have the guard wrap the ix) so a violating
  `Open`/`Close` actually fails on-chain.
- Harness test: cheater #1's flatten-and-lie tx and an out-of-mandate `Open` both revert; honest path
  unaffected.

## Acceptance criteria

- A `LiteSVM` test asserts the guard returns `Err` (tx reverts) for at least one insolvency case and one
  mandate case, and `Ok` for the honest path.
- `cargo build-sbf` clean. `cargo test` green.
- Guard CU overhead per check recorded in the PR description (target: low hundreds of CU).

## Out of scope

- Compiling the *full* invariant set into the guard (Stage 0 does 2). Red-team promotion loop = Task 004.
