# Task 004 — Unbypassable enforcement: inline the invariant check into the perp (Stage 1)

**Owner:** Codex (frame-thick).
**Reviewer:** CC.
**Branch:** `task/004-inline-enforcement`.
**Depends on:** Task 003 merged.
**Motivation:** external critique (2026-07-04) — the wiring-(B) guard is opt-in and therefore prevents
nothing; an agent that omits the guard instruction bypasses it. A security product cannot ship that as
"prevention." This task makes the check **physically impossible to bypass**.

## The key insight

`Position` accounts are **owned by the perp program**. On Solana only the owning program can mutate an
account's data, so **no path exists to change a `Position` except through the perp program.** Therefore
enforcing the invariant *inside* the perp's mutating instructions is unbypassable — no CPI required, near
-zero extra CU. The standalone `programs/guard` stays, but for a *different* job (wrapping third-party
programs whose accounts the perp does not own).

## Scope (in)

- **Inline post-state enforcement in `programs/perp`.** At the end of `Open`, `Hedge`, `Close` (any
  instruction that mutates a `Position`), before persisting, assert the post-state invariants using the
  SHARED contract predicates: `position.within_mandate()` and `!position.is_liquidatable(market.mark)`.
  On violation return `Err(Custom(..))` so the whole tx reverts — same error codes as the guard
  (`MandateDeviation`, `SelfInflictedInsolvency`) for consistency. Reuse the guard's check function; do
  not duplicate the logic.
- **Prove unbypassability.** Add a LiteSVM test that sends the perp `Open` **alone** (NO guard ix, the
  exact bypass the critique describes) for both the mandate case (qty=101) and the self-inflicted
  insolvency case (collateral=10) and asserts the tx **still reverts** and the `Position` is unchanged
  (`before == after`). This is the test that closes the critique.
- **Keep the standalone guard** (`programs/guard`) and its same-tx path as the "wrap a program you don't
  own" primitive; relabel it in docs as *composable guard for third-party accounts*, not the perp's own
  enforcement.
- **CU delta.** Record the per-instruction CU before/after inlining (expected: small — two arithmetic
  checks). Confirm it stays far under the 200k-per-ix budget.

## Acceptance criteria

- Perp `Open`/`Hedge` reverts an out-of-mandate and a self-inflicted-insolvency mutation **with no guard
  instruction in the transaction**; `Position` byte-identical after the revert.
- Honest `Open` still succeeds and mutates.
- All existing 22 tests still green (the scripted honest/gamer/phantom episodes must be unaffected —
  none of them make an *opening* mutation that is itself out-of-mandate or immediately liquidatable, so
  behavior/trace parity must hold; if any episode changes, STOP and flag it).
- `cargo build-sbf` clean; `cargo test --offline` green; CU delta recorded in the PR description.

## Out of scope

- CPI promotion of the standalone guard for third-party programs (separate later task).
- Red-team discovery loop (Task 005) — the critique's point #5 (hand-authored policies prove mechanism,
  not coverage). That is the next task after this one.

## Honesty follow-through (CC, after merge)

- Reframe `README.md` to lead with the Verifier (unbypassable offline eval) and describe enforcement as
  *inline in the perp* (now real), with the standalone guard as the third-party-wrapping primitive.
- Add an eval-vs-monitor note: Probatio is a pre-deployment proving ground (offline replay), not a
  realtime mainnet monitor — so verifier "latency under 400ms slots / MEV" is out of frame; episode
  realism (adding slippage / oracle-lag / adversarial ordering) is the real robustness gap, tracked for
  a later "hostile episode" task.
