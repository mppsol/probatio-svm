<!-- KILL-GATE-BANNER -->
# ⛔ PHASE: NO HYPOTHESIS IS LIVE — all five are closed

**H5 (Agent Release Tests for Solana) is `CLOSED`** (founder, 2026-08-22):
[`docs/decisions/H5-closed.md`](docs/decisions/H5-closed.md), gate [`docs/H5-GATE.md`](docs/H5-GATE.md).

> **`CLOSED — all authorised candidates failed at the design gate; H5 remains unproven.`**

Four authorised candidates, four **design-gate** failures, **zero measurements and zero lines of
product code**. `C3` — the last and strongest — died because its three "meaningful" transactions were
**件数合わせ**, and because its failure *is* a duplicate spend, which **a cumulative spend cap set at
the agent's own intent total catches by definition**: it **was not differentiable from a runtime
wallet policy at all**. **H5 is `unproven`, NOT `refuted` — nothing says sequence-only failures do not
exist on Solana; that was never tested.**

**All five gates are closed. No verdict is rewritten, revived, or worked around:**

- `docs/GATE.md` — H1, **`KILLED`** ([`decisions/P1-real-target.md`](docs/decisions/P1-real-target.md)).
- `docs/H2-GATE.md` — H2, **`FROZEN UNEXECUTED`**. No H2 measurement was ever run.
- `docs/H3-GATE.md` — H3, **`KILLED` at the design gate**. No H3 gate item was ever executed.
- `docs/H4-GATE.md` — H4, **`KILLED` at G0** (`KILL-2` + `KILL-3`), on its own pre-registered numbers.
- `docs/H5-GATE.md` — H5, **`CLOSED`** ([`decisions/H5-closed.md`](docs/decisions/H5-closed.md)).

**Nothing is authorised right now.** Not a new fixture, not a candidate search, not implementation,
measurement, UI, token, or deploy. **Reopening anything requires a founder ruling**, and reopening the
H5 line additionally requires a **new, independent hypothesis** — not a repair or rewording, and not
resting on H5's §8.1, `C1`, `C2` or `C3`, all finished — plus a **new pre-registration** that names the
comparator **the hypothesis itself names, at full strength**. Choosing a weaker comparator is the
specific mistake that ended H5.

No artifact from H1–H5 — verifier, attestation, `MandateSpec`, guard, gallery, `attest/`, the agent
framing, the H4 sentinel harness, the H5 fixtures — may be reused as a new hypothesis's **foundation**;
reusing one because it exists is 延命, not scope. **Reusing mechanics as fixtures is fine; reusing a
conclusion as evidence is not.** The sibling repo `../solvo` is a **read-only** reference and is never
edited from here.

- A verdict is a **number or a reproducible experiment** — never an assessment or a plan.
- **Not-proven is a KILL**, not a pending. Adding a hypothesis to stay alive is forbidden.
- **At most two agents**: Claude for spec/evidence/task progression, Codex for implementation *or*
  independent review — **never both at once**, and no model reviews its own output.
- **On reaching the gate, STOP.** `GO` ends this phase; it does not start the next one. Update
  `STATUS.md`, commit the evidence, push, and hand back to the founder.
- Forbidden without exception: real funds, mainnet deploy, force push, adding secrets.

<!-- /KILL-GATE-BANNER -->

# AGENTS.md — Probatio SVM two-agent operating contract

Probatio SVM is built by **two agents that cross-review each other**: Claude Code (CC) and Codex.
The repo — not chat memory — is the only shared memory. **Every handoff is a committed artifact**
(task brief, code, review file). This mirrors the proven loop in the sibling `../probatio` (Reth) repo.

## Division of labour (by task *frame*, not "implement vs review")

The split axis is how much **frame** a task has — goal clarity, convergence condition, the contour of
the right answer.

- **Frame thin** (exploration: vague goal, unknown state, trial-and-error) → **CC**. Manufactures the
  contour while progressing. Owns: architecture, ADRs, task briefs, the shared account-layout contract,
  the pure-Rust reference model, and the final "is this explainable / safe to operate?" pass.
- **Frame thick** (convergence: clear diff, fixed perspective, a converging answer) → **Codex**. Fast
  and sharp. Owns: the Pinocchio programs (perp + guard) to spec, the LiteSVM driver, tooling, and
  **adversarial audits/reviews**.

**Product-specific:** the moat is a verifier/guard that catches what *a different intelligence* does.
So Codex is also the natural **independent red-teamer** against CC's invariants — a genuinely different
model trying to beat the verifier is worth more than CC red-teaming itself (`STAGE0_DESIGN.md` §8 —
historical, but the cross-review rule it motivates still stands and earned its keep across H3–H5).

**Cross-pass rule:** whoever implemented a change is NOT its reviewer. The other agent reviews (same
type shares blind spots). A change merges only after a review by the other agent.

## Workflow (brief → branch → review → merge)

1. **Brief.** CC writes a task brief in `docs/tasks/NNN-slug.md` (goal, scope, acceptance criteria,
   out-of-scope, files to touch). A brief is "frame" — make it thick before handing a task to Codex.
2. **Branch.** Always branch from `master`: `task/NNN-slug` (or `claude/...` / `codex/...`). One task
   per branch. Never pile unrelated work onto someone else's branch.
3. **Implement.** The assigned agent implements ON that branch and commits small and often.
4. **Review.** The OTHER agent reviews the branch diff and writes `reviews/NNN-slug.md` (verdict:
   APPROVE / CHANGES, prioritized P0/P1/P2 findings). Be specific and adversarial: missing tests,
   leaky/misnamed abstractions, untested error paths, correctness, CU regressions. Iterate until APPROVE.
5. **Merge.** Only an APPROVED branch merges to `master`. **No agent merges its own un-reviewed work.**

**Review surface — GitHub PRs preferred** once a remote exists. Push branches and review with
`gh pr create` / `gh pr diff` / inline comments. Local fallback: branch + commit + exchange
`reviews/NNN-slug.md` files.

## The contract (neither agent changes these unilaterally)

The cross-module API is the contract — changing it needs a brief/ADR both agents see. In this repo the
contract is **doubly load-bearing**: the same account layout is read by the perp program, the guard
program, AND the off-chain verifier, so a drift breaks all three.

- `crates/contract/` — `Market` / `Position` account layouts + (de)serialization, `Observation`,
  `Action`, `AgentAccountRef`, `AgentClaim`. **Single source of truth** shared by programs + harness.
- `crates/harness/src/verifier.rs` — `StateSnapshot`, `Invariant`, `Verdict`, `Finding`, `FindingKind`,
  `ShortcutReport`.
- `crates/harness/src/policy.rs` — the `Policy` trait.

## Gates (must hold before review is requested)

- `cargo build` clean, no new warnings; on-chain crates also build under `cargo build-sbf`.
- `cargo test` green. New branching logic ships with tests; live-API code must stay testable offline
  (no test may hit the network). Episode traces must be **deterministic** (same seed ⇒ byte-identical).
- No secrets committed. `ANTHROPIC_API_KEY` lives in the environment, never in the repo.

## Git hygiene

- Branch from `master`; rebase onto `master` (not merge commits) to stay current.
- Commit as `psyto <saito.hiroyuki@gmail.com>`.
- Repo is intended **public** (MIT+Apache). Keep the private [[solinv]] catalog OUT of this tree — only
  the small published invariant set lives here. Before any `gh` push: `gh auth switch -u psyto`
  (r3saito is the wrong account → 404).

## Running Codex (repo at /Users/hiroyusai/src/probatio-svm; codex not on PATH)

```bash
# Path corrected 2026-08-22: the binary ships inside ChatGPT.app, not a standalone Codex.app.
CODEX=/Applications/ChatGPT.app/Contents/Resources/codex
# Review (read-only):
"$CODEX" exec -C /Users/hiroyusai/src/probatio-svm -s read-only "<review prompt: branch + reviews/NNN file>"
# Implement on a branch (writes):
"$CODEX" exec -C /Users/hiroyusai/src/probatio-svm -s workspace-write -o /tmp/codex-out.md "<task brief ref>"
```

## Project context

Probatio SVM is now a **falsification record**: five hypotheses about verifying autonomous agents on
Solana, run through kill gates, **all five closed, none survived**. The description this section used
to carry — *"stress-tests autonomous Solana-DeFi agents and ships a guard that reverts cheating
transactions in-block"* — was **H1's product claim**, and H1 is `KILLED`.

The code is real and its tests pass; what was refuted is the claim it was worth building for the
reason given. Layout: `crates/contract` (shared account layout), `crates/harness` (episode driver +
`verifier.rs` + `policy.rs` + `world.rs` reference model), `programs/perp` + `programs/guard`
(Pinocchio), `crates/h4-sentinel` (H4's evidence tooling, separate workspace). **These are refutation
assets and fixtures, not a foundation** — reusing mechanics is fine, reusing a conclusion as evidence
is not. See
`STATUS.md` for the state of record and `README.md` for what this repo now is. **There is no staged
roadmap**: `STAGE0_DESIGN.md` is H1's design document and H1 is `KILLED`, so its roadmap and
positioning are refuted and must not be cited as a plan.
