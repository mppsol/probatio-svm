# Codex review payload — H5 G0 candidate search (branch A: propose `KILLED`)

**Not yet run.** Prepared by CC per the founder ruling of 2026-08-22 ("独立Codexレビュー用のpayloadを
作成してからpushしてください"). Invoke exactly as below, read-only, one round. **The founder decides
whether it runs** — at most two agents, and no model reviews its own output.

```sh
/Applications/ChatGPT.app/Contents/Resources/codex exec \
  -C /Users/hiroyusai/src/probatio-svm -s read-only \
  "$(cat reviews/H5-G0-candidates.codex-prompt.md)" < /dev/null
```

Cross-review is clean: **CC wrote every line under review.** Codex wrote `crates/h4-sentinel`, whose
committed evidence is cited as input — it should say where it leans on that.

---

You are the INDEPENDENT REVIEWER for the H5 Gate 0 **candidate search** in
/Users/hiroyusai/src/probatio-svm (branch `h5/agent-release-tests-g0`). Read-only: write nothing, run
no measurement or harness, no network, do not touch `/Users/hiroyusai/src/solvo`.

Read as binding: `docs/H5-CANDIDATES.md` (the object of this review), `docs/H5-GATE.md` (§8.1 is a
`REJECTED` candidate; §8.2 points here), `STATUS.md`, `reviews/H5-G0-agent-release-tests.md` (the
review that killed §8.1), `docs/decisions/H4-sentinel-kill.md` and `evidence/h4-sentinel.json`,
`fixtures/h4/` (accounts + `MANIFEST.json`).

## Context

H5: an agent that moves capital on Solana should be regression-tested before release against
adversarial scenarios spanning real BPF, real cloned state and **multiple transactions**, and such a
test finds failures **a single `simulateTransaction` or a runtime wallet policy cannot find**. Only
that second half is on trial.

Four hypotheses are already dead in this repo, and H5's own first candidate died to a single P0: the
failure was fully witnessed inside one transaction's post-state, so the multi-transaction framing did
no work. The founder then authorised a **design investigation only** — enumerate at most three new
candidates, kill at design time any whose failure is single-transaction-observable, and then take
branch **A** (all dead → propose H5 `KILLED`, no kill commit without a ruling) or **B** (exactly one
survives → freeze it completely, await a ruling).

CC took **branch A**: `C1` and `C2` marked `KILL-1`, `C3` marked `KILL-2`, none implemented, none
measured. **Your job is not to help H5 survive, and not to rubber-stamp its death.** Both errors are
real: a wrong kill discards a live hypothesis, and a soft kill wastes a build.

## Answer each, with an explicit verdict

1. **Is the fixture bound in §2 correct?** CC claims the committed obligation has **exactly one**
   deposit (USDC, 2,248,785,777) and **exactly one** borrow (SOL), and derives from that: no second
   capital leg, therefore no ordering-dependent failure. **Re-derive the deposits and borrows
   yourself from `fixtures/h4/accounts/obligation.json`** (deposits base 96, 8 × 136 B; borrows base
   1208, 5 × 200 B) and say whether the layout assumption and the conclusion hold. If the layout is
   wrong, the whole investigation is wrong.
2. **Is the second bound correct** — that klend's clamping reaches the solvency limit in a *single*
   transaction (H4 case E: `u64::MAX` → 486,657,686 of 2,248,785,777), so a cumulative multi-step walk
   has nowhere further to go? Could a sequence of *smaller* withdrawals reach a state a single
   maximal withdrawal cannot? If yes, `C1`–`C3` were killed too early.
3. **`C1` — is the `KILL-1` right?** CC concedes `T4`'s bytes are unconstructible in advance (a real
   structural barrier) but kills it anyway because the failure is still witnessed in `T4`'s own
   post-state. Is that the correct place to draw the line, or does "the transaction cannot be built
   without the episode" actually satisfy condition 2 and make `C1` a survivor?
4. **`C2` — is the `KILL-1` right?** CC kills it because `A₂` is fixed at `T1`, so `T4` is
   constructible and one simulation at submission time determines the failure. Does the *pre-release
   CI* framing change that — i.e. is "a single `simulateTransaction`" fairly read as something a CI
   run can perform at all, given the RPC cannot advance chain state?
5. **`C3` — is the `KILL-2` right?** CC kills it because a lost confirmation / RPC timeout is a
   client-side event that real BPF on cloned state cannot produce. Is that correct, or is a dropped
   confirmation legitimately part of the system under test for an agent release test?
6. **The decisive question, stated plainly.** If "a single `simulateTransaction`" may be **chained**
   across an episode, it becomes the same machine as H5 and **no** candidate can ever satisfy
   condition 2 — H5 is dead in general, not per-candidate. If it may **not** be chained (the RPC
   cannot advance mainnet state; chaining requires exactly the local-fork machinery H5 is), then the
   distinction is real. **Which is it?** The §8.1 review already granted the baseline "post-`T1`
   state", i.e. one step of chaining. Was that grant correct, and does it settle H5?
7. **Was the search wide enough for "at most three"?** Name any candidate shape CC failed to consider
   that the committed fixture could express — in particular under the founder's named shapes
   ("success response but funds unusable", "wrong retry after revert", "price halt / stale state /
   partial fill"). If a fourth shape exists on this fixture, branch A is premature.
8. **Is branch A correctly selected**, and is the two-part resume condition in §7 tight enough that a
   future agent cannot loosen it? Is anything in §6–§7 an argument for keeping H5 alive dressed as a
   record (延命)?
9. **Anything asserted in `docs/H5-CANDIDATES.md` that is not supported** by committed bytes or by
   H4's frozen evidence — especially any number, and especially the dollar values, which are derived
   from `_sf` fixed-point fields and could be misscaled.
10. **The one change** that would most improve this record, whether H5 lives or dies.

## Output

Print (do not write files) a Markdown review to be saved verbatim as
`reviews/H5-G0-candidates.md`:

- **Verdict line**: `CONCUR — H5 KILLED` / `CHANGES` (the record is wrong or incomplete; list what) /
  `DISSENT — a candidate survives` (name it and say exactly which condition it clears).
- P0 / P1 / P2 findings with file:line. A **P0** is: a candidate killed on a wrong argument, a
  factual error in the fixture derivation, or a missed candidate shape the fixture supports.
- A direct answer to each of the ten questions.
- Your independently re-derived deposits/borrows numbers, with the command used.
- What you could not verify read-only.

If the honest answer is that CC killed a live candidate, say so — that is the expensive mistake here.
