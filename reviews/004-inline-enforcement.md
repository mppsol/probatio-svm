APPROVE

Reviewer: CC. Implementer: Codex. Branch: task/004-inline-enforcement @ ca27bec.

## Verdict rationale

This closes the external critique's central flaw. The invariant check is now **inline in the perp's
mutating instructions**, so a transaction that omits the guard — the exact bypass the critique described —
still reverts. Because `Position` accounts are owned by the perp program, and only the owning program can
mutate account data, there is **no path to change a `Position` that skips the check**. That is real,
unbypassable enforcement, achieved without CPI and at low CU. No P0/P1 issues.

## What was verified (positives)

- **Bypass is closed — proven.** `inline_enforcement_blocks_out_of_mandate_open_without_guard_ix`
  (qty=101 ⇒ `Custom(10)`) and `inline_enforcement_blocks_self_inflicted_insolvency_without_guard_ix`
  (collateral=10 ⇒ `Custom(11)`) send the perp `Open` **alone via `dispatch_action` (no guard ix)** and
  assert the tx reverts with `Position` byte-identical (`before == after`).
  `inline_enforcement_allows_honest_open_without_guard_ix` confirms the honest solo open still mutates.
- **Single source of truth.** `check_position(&Market, &Position) -> Result<(), EnforcementError>` +
  shared `EnforcementError` (codes 10/11) live in `crates/contract`; the perp (inline, post-`trade`,
  pre-`write`) and the standalone guard both call it. The old `GuardError` duplicate was removed.
- **Correct placement.** Enforcement runs on the post-state of `Open`/`Hedge`/`Close`, after `trade`
  mutates and before `write_position` persists — a violating result never lands.
- **Episodes unaffected.** honest/gamer/phantom all open at mark 100 with a non-liquidatable, in-mandate
  post-state (the shock arrives via `CrankOracle`, not an agent tx), so none trip the inline check; the
  gamer's slot-60 `Close` flattens to size 0 (solvent) and passes. Trace/verdict parity across backends
  holds — 25 tests green (the 22 prior + 3 bypass tests).
- **CU recorded:** inline `Open`=583 (+235 vs Task 002's 348), `Hedge`=758, `SettleFunding`=356 — far
  under the 200k/ix budget, and lighter than the retained same-tx guard honest open (941). The
  enforcement path is essentially free at Solana scale; the critique's CU worry is quantified and small.

## Findings

P2 [programs/perp — enforcement semantics on risk-reducing `Hedge`]: the inline rule is "post-state must
not be liquidatable" applied to *every* mutation. `Open` (correct to block) and `Close` (always flattens
to size 0 ⇒ always passes, so exit is never blocked) are fine. But a partial `Hedge` that *reduces* an
already-underwater position while leaving it still liquidatable would be **blocked even though it lowers
risk**. None of the Stage-0 policies `Hedge`, so no current impact. When agents/red-team (Task 005) use
`Hedge`, switch the rule from "post-state absolutely solvent" to "does not cross healthy→liquidatable /
does not increase risk," so de-risking actions are never rejected. Track; not a blocker.

P2 [sol_memcpy_ build-sbf warning]: unchanged from Task 002/003; benign, functionality intact. Folded
into the existing polish item.

P3 [nit]: `map_enforcement_error` is defined identically in both `programs/perp` and `programs/guard`.
Harmless (separate program crates), not worth a shared crate. Leave as-is.

## Follow-through (CC, now)

- Reframe `README.md`: lead with the Verifier (unbypassable offline eval) and state that the perp
  **inline-enforces** the invariants (unbypassable, since Position is perp-owned); relabel the standalone
  guard as the *composable primitive for wrapping third-party accounts*. Update CU/test numbers. Add the
  eval-vs-monitor note (offline proving ground, not a realtime mainnet monitor). — done in the follow-up
  commit.
- Next: Task 005 red-team discovery loop (the critique's real remaining point — hand-authored policies
  prove mechanism, not coverage).
