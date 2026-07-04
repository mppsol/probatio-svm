APPROVE

Reviewer: CC. Implementer: Codex. Branch: task/003-pinocchio-guard @ 5a3672a.

## Verdict rationale

The second primitive works and is genuinely demonstrated: a Pinocchio guard, composed as a second
instruction after the perp `Open` in one transaction, reads the **post-mutation** `Position` and returns
`Err` on a violated invariant, so the whole transaction **reverts atomically in-block**. The atomicity is
proven the right way — the tests read the position back after the revert and assert `before == after`.
No P0 issues; no code changes required. The one P1 is a **framing/honesty** requirement (wiring (B) is
opt-in), addressed by an editorial doc note (below) and a tracked follow-up — not a code defect, and the
brief sanctioned (B) for Stage 0c.

## What was verified (positives)

- **Atomic in-block revert, proven.** `guard_reverts_out_of_mandate_open_atomically` (qty=101 ⇒
  `Custom(10)`) and `guard_reverts_self_inflicted_insolvency_atomically` (collateral=10, qty=10, mark=100
  ⇒ `Custom(11)`) both assert the `Position` account is byte-identical before and after the reverted tx.
  `guard_allows_honest_open_and_mutates_position` confirms the honest `Open` passes and mutates. This is
  the pitch "wow", and it is real.
- **Post-state visibility is correct.** The guard (2nd ix) sees the perp `Open`'s (1st ix) mutation
  within the same tx — the mandate test only passes because the guard read size=101, not the pre-state 0.
- **Single source of truth.** The guard reuses `Position::within_mandate()` and
  `Position::is_liquidatable()` from `crates/contract`, so guard and off-chain verifier share one
  definition of "bad". Exactly right.
- **CU measured:** honest guarded open=714, mandate reject=508, insolvency reject=713. Recorded.
- 22 tests green offline; `GuardInstruction` codec round-trips.

## Findings

P1 [framing — wiring (B) is opt-in / bypassable]: the guard is a *separate* instruction the harness
appends (`dispatch_guarded_action` composes `[perp, guard]`; the plain `dispatch_action` composes
`[perp]` only). An agent that omits the guard ix bypasses it entirely. So Stage 0c demonstrates the
atomic-revert **mechanism**, but is NOT tamper-proof **enforcement**. This is inherent to (B) and was
sanctioned by the brief — the requirement is only that we never overclaim. The honest pitch line is:
*"the guard atomically reverts a violating tx in-block; making it unbypassable (the perp unconditionally
CPIs the guard, or the guard owns the settlement authority) is the next step."* → **applied**: a note
added to `STAGE0_DESIGN.md` §7; CPI promotion tracked as a follow-up / Task 004 candidate. No code change
required now.

P2 [programs/guard/src/lib.rs:38-46]: the guard reads `market_acc`/`position_acc` without verifying they
are program-owned or are the accounts the perp actually touched. Safe under (B) because the harness
controls tx composition, but a real deployment (or the CPI promotion) must validate account ownership so
the guard can't be fed a spoofed benign account. Track with the CPI promotion.

P2 [crates/harness/src/world.rs:611-729]: `dispatch_guarded_action` and `measure_rejected_guarded_action`
are ~95% identical (same tx composition; one unwraps `Ok(cu)`, the other tolerates `Err` and returns
`err.meta.compute_units_consumed`). Factor the shared message-build into one helper. DRY, non-blocking.

P2 [build-sbf `sol_memcpy_` warning]: same benign SBF post-processing warning as Task 002; guard builds,
loads, and reverts correctly. Already tracked from review 002; fold into that polish item.

## Follow-ups to track (not blocking this merge)

- CPI promotion (wiring A): perp unconditionally CPIs the guard so it cannot be bypassed → real
  enforcement. Carries the P2 account-ownership validation with it. Strong Task 004 candidate alongside
  the red-team loop.
- Silence/document the `sol_memcpy_` build-sbf warning (shared with Task 002).
- DRY the two guarded-dispatch helpers.
