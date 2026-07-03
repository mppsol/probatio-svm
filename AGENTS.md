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
model trying to beat the verifier is worth more than CC red-teaming itself (`STAGE0_DESIGN.md` §8).

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
CODEX=/Applications/Codex.app/Contents/Resources/codex
# Review (read-only):
"$CODEX" exec -C /Users/hiroyusai/src/probatio-svm -s read-only "<review prompt: branch + reviews/NNN file>"
# Implement on a branch (writes):
"$CODEX" exec -C /Users/hiroyusai/src/probatio-svm -s workspace-write -o /tmp/codex-out.md "<task brief ref>"
```

## Project context

Probatio SVM stress-tests autonomous Solana-DeFi agents on a real SVM episode and ships a Pinocchio
guard that reverts cheating transactions in-block. Layout (target): `crates/contract` (shared account
layout), `crates/harness` (episode driver + `verifier.rs` moat + `policy.rs` + `world.rs` reference
model), `programs/perp` + `programs/guard` (Pinocchio), optional `crates/agent` (Claude). See
`README.md` and `STAGE0_DESIGN.md` for the staged roadmap.
