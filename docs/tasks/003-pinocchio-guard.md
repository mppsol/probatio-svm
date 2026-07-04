# Task 003 — Pinocchio guard program: revert a cheating tx (Stage 0c)

**Owner:** Codex (frame-thick).
**Reviewer:** CC.
**Branch:** `task/003-pinocchio-guard`.
**Depends on:** Task 002 merged.

## Goal

Ship the **second primitive** — a Pinocchio guard that reverts a violating transaction *in-block*.
This is the pitch-video "wow": prevention, not just post-hoc detection.

## Key design point (read first — corrects the naive framing)

A transaction guard can only block **bad actions** (a tx that itself moves the account into a forbidden
state). It CANNOT block **passive bad states** driven by something other than the agent's tx — e.g. the
oracle shock (`CrankOracle`) that makes cheater #1 liquidatable at slot 30 is not a tx the agent sent, so
no guard can revert it. That passive insolvency is the **off-chain verifier's** job. This is the
complementary two-primitive story, and the pitch should say it plainly:

- **Guard (on-chain):** blocks *actions* that would violate an invariant *right now* (in-block).
- **Verifier (off-chain):** catches *states/behaviors* that emerge over the episode (measurement gaming,
  phantom exposure, passive oracle-driven insolvency).

So the guard's demonstrable blocks in this world are tx-level and self-inflicted:

1. **MandateDeviation** — an `Open`/`Hedge` whose *resulting* position leaves the size/instrument
   envelope (e.g. `qty > MAX_MANDATE_SIZE`).
2. **SelfInflictedInsolvency** — an `Open`/`Hedge` whose *resulting* position is already liquidatable at
   the current mark (e.g. open size 10 with collateral 10 at mark 100 ⇒ equity 10 < maintenance 50).

Do NOT try to make cheater #1's slot-60 `Close` revert — it flattens to solvent, correctly passes.

## Scope (in)

- `programs/guard` (Pinocchio): reads `Market`/`Position` (from `crates/contract`) and returns `Err`
  (aborting the tx) when a post-state invariant is violated. Stage 0c enforces the **two** tx-level
  invariants above. Reuse `Position::within_mandate()` and `Position::is_liquidatable()` from the
  contract so the guard and the off-chain verifier share one definition.
- Enforcement wiring — pick the cleaner of:
  - **(A) CPI:** perp `Open`/`Hedge` CPIs into the guard with the post-mutation account; guard `Err`
    ⇒ whole tx reverts. Most faithful to the "vault calls the guard" product framing.
  - **(B) Same-tx guard ix:** harness composes `[perp Open, guard Check]` in one transaction; guard
    reads post-state and `Err` ⇒ atomic revert of both. Simpler, still a real in-block block.
  State the choice and why in the PR description.
- Harness: add a `guarded` execution path in `world.rs` (svm backend) and a policy/scenario that
  attempts (1) an out-of-mandate `Open` and (2) a thin-collateral insolvency-inducing `Open`.

## Acceptance criteria

- A `LiteSVM` test asserts the guarded path returns `Err` (tx reverts, and the position account is
  UNCHANGED after the revert — prove atomicity by reading it back) for one mandate case and one
  self-inflicted-insolvency case, and `Ok` for the honest `Open`.
- `cargo build-sbf` clean for `programs/guard`. `cargo test --offline` green (all prior 18 + new).
- Guard CU overhead per check recorded in the PR description (target: low hundreds of CU).

## Out of scope

- Compiling the *full* invariant set into the guard (Stage 0 does 2). Red-team promotion loop = Task 004.
- Blocking passive oracle-driven insolvency (that's the off-chain verifier, by design).
