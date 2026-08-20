# Task 023 — Autonomous cross-review harness (SPEC → SPEC_REVIEW → IMPL → IMPL_REVIEW → DONE)

**Frame:** thin (CC). Process/architecture, not product code. **Spec author:** CC. **Spec reviewer:** Codex.
**Implementer:** CC. **Impl reviewer:** Codex. No contract-surface change.

## Goal

Turn the `AGENTS.md` two-agent contract from *prose a human enforces* into a **mechanical loop the two
agents run without per-step human judgement**: an explicit task state machine, machine-checkable role
separation, one board file, one gate command, and reusable agent definitions in `.claude/agents/`.

The loop must move a task through `SPEC → SPEC_REVIEW → IMPL → IMPL_REVIEW → DONE` where every
transition is (a) authorized by an artifact committed to the repo and (b) checkable after the fact by
reading the repo alone — because the repo is the only shared memory between CC and Codex.

## Non-goals

- **Not** a CI service, scheduler, or daemon. No background process, no cron, no webhook. The loop is
  driven by whichever agent is invoked; the repo carries the state.
- **Not** a replacement for `AGENTS.md`. This task *operationalizes* it; the division-of-labour and
  contract-surface rules stay where they are and are referenced, not forked.
- **Not** a product feature. Zero changes to `crates/`, `programs/`, verifier behaviour, or any
  certification output. The one exception below is an evidence-truth fix, called out explicitly.
- **Not** a repo-wide reformat. `cargo fmt` drift predates this task (166 hunks / 16 files) and stays
  an `OPEN_RISK` with its own backlog slot, so it never contaminates a feature diff.
- **No** autonomous irreversible action: no `push` without an explicit human instruction naming the
  remote and branch, no force-push, no history rewrite, no key material, no mainnet or real-fund write.

## Threat model (what an autonomous two-agent loop gets wrong)

| # | Threat | Consequence | Mitigation this task ships |
|---|---|---|---|
| T1 | **Self-review collusion** — the implementer also reviews, or "the other agent" is the same model in a fresh context | Shared blind spots pass as independent sign-off; the product's whole premise (a *different* intelligence catches the cheat) is faked. **Already happened once**: task 006 merged on "round-3 CC self-verification; headless Codex unavailable" (`8ed9104`) | Board records `author→reviewer` per phase; `board-check.sh` fails when they are equal. A CC-subagent / self review is legal only when labelled `(fallback: Codex unavailable)` in the review file **and** carried as an `OPEN_RISK` row, so weak independence is visible, never silent |
| T2 | **Green-washing** — an agent asserts "tests pass" without running them | `DONE` becomes meaningless; a broken `master` looks certified | `DONE` requires a `scripts/verify.sh` summary block, pasted into the review file, naming the commit and each gate's exit code |
| T3 | **Contract drift** — an agent unilaterally edits the account layout / verifier / policy surface | Perp, guard and verifier disagree; all three break at once (`AGENTS.md` §"The contract") | Contract-surface paths listed in `docs/HARNESS.md`; touching one requires an ADR named in the brief and an explicit reviewer acknowledgement line |
| T4 | **Hidden failure** — a sub-part is broken or unproven, and the task is closed anyway | The repo's honesty (its stated differentiator) rots; a demo claim outruns the evidence | `BLOCKED` / `OPEN_RISK` are first-class board states with required fields; `DONE` forbids an unresolved blocker on the same row |
| T5 | **Runaway autonomy** — the loop pushes, spends a key, or submits on mainnet because a brief implied it | Irreversible, real-money action taken with no human in the loop | `docs/HARNESS.md` §Forbidden actions, restated in every agent definition; `attest/send.mjs --send` is explicitly outside every agent's authority |
| T6 | **Board drift** — `STATUS.md` disagrees with git | The single source of progress lies; dependencies get scheduled off stale state | Every non-backlog row carries a commit hash; `board-check.sh` verifies each hash resolves and each linked artifact exists |
| T7 | **Stale evidence** — docs keep an old number after the code moves | Already happened twice in this repo (see Findings) | `verify.sh` prints the live test count; the evidence role owns re-stamping README/STATUS on every `DONE` |

## Invariants

Process invariants. `scripts/board-check.sh` mechanically enforces I1, I2, I4, I6; the rest are review duties.

- **I1 — Separation of duties.** For every task, `spec_author ≠ spec_reviewer` **and**
  `implementer ≠ impl_reviewer`.
- **I2 — Spec gate.** No implementation commit lands on `task/NNN-*` before that task's spec review
  exists with verdict `APPROVE`.
- **I3 — Evidence gate.** A row is `DONE` only if a `verify.sh` summary at the recorded commit shows
  every required gate exiting 0.
- **I4 — Traceability.** Every `DONE` row links brief, spec review, impl review, and a resolvable
  commit hash.
- **I5 — Contract integrity.** Contract-surface files change only via a brief that names an ADR and a
  review that acknowledges it.
- **I6 — Honest state.** A known failure or unknown is recorded as `BLOCKED` or `OPEN_RISK` on the
  board, never dropped. `DONE` and an open `BLOCKED` cannot coexist on one row.
- **I7 — Reversibility.** The loop performs no irreversible external action on its own authority.

## Findings this task must fix (discovered while writing the spec)

These are live defects in the *existing* operating instructions — an autonomous agent following them
today fails or lies. They are in scope precisely because the harness depends on them:

1. **`AGENTS.md` Codex path is dead.** `/Applications/Codex.app/Contents/Resources/codex` does not
   exist; the CLI is at `/Applications/ChatGPT.app/Contents/Resources/codex` (`codex-cli 0.148.0-alpha.9`).
   An autonomous CC following `AGENTS.md` cannot reach its reviewer at all → every review silently
   degrades to self-review (T1).
2. **`AGENTS.md` build gate is wrong as written.** Bare `cargo build-sbf` at the workspace root
   **fails** (`getrandom` has no SBF target — it is pulled in by the harness crate). The build only
   works per-program with `--features bpf-entrypoint`, as `world.rs:384-443` already does. An agent
   running the documented gate sees a red build on a green repo.
3. **`README.md` test count is stale** — claims 87, actual is 91 (7 contract + 1 guard + 1 perp +
   77 harness lib + 2 harness bin + 3 reexec-spec). Small, but it is exactly threat T7 in the file
   that carries the project's credibility.

## Deliverables

- `docs/HARNESS.md` — the state machine, transition gates, role table, forbidden actions, contract
  surface, and how a `BLOCKED`/`OPEN_RISK` is recorded.
- `STATUS.md` (repo root) — the single board: one machine-checkable row per task + open-risk register.
- `.claude/agents/probatio-{spec-author,spec-reviewer,implementer,impl-reviewer,evidence-curator}.md`
  — five role definitions, each carrying its phase's entry/exit conditions and the forbidden-action list.
- `docs/templates/{task-brief,spec-review,impl-review}.md` — the required sections, so a brief can
  never ship without goal / non-goals / threat model / invariants / acceptance criteria / reproduce.
- `scripts/verify.sh` — the one gate command. Runs build, tests, both per-program SBF builds, the
  reproduce commands, and clippy/fmt drift; prints a pasteable summary with per-gate exit codes; full
  logs under `target/verify/` (already gitignored).
- `scripts/codex.sh` — resolves the Codex CLI across known install locations and fails loudly with a
  fallback instruction instead of silently degrading to self-review (fixes Finding 1 operationally).
- `scripts/board-check.sh` — enforces I1/I2/I4/I6 against `STATUS.md` + git.
- `AGENTS.md` — corrected Codex path and SBF gate; a pointer to `docs/HARNESS.md`. The
  division-of-labour and contract sections are **not** rewritten.
- `README.md` — test count corrected to 91 (Finding 3). No other prose touched.
- Backlog: tasks 024–029 seeded on the board with dependencies and frame assignment.

## Acceptance criteria

1. `bash scripts/verify.sh` exits 0 on this branch and prints a summary block naming each gate, its
   exit code, the live test count, and the commit under test.
2. `bash scripts/board-check.sh` exits 0 and, when fed a row that violates I1 (author == reviewer),
   I4 (missing link or unresolvable commit), or I6 (`DONE` + `BLOCKED`), exits non-zero naming the row
   and the invariant. Demonstrated by the script's own `--selftest` over synthetic fixture rows —
   the "attack case", not just the happy path.
3. `bash scripts/codex.sh --version` prints a Codex version, or exits non-zero with an explicit
   "reviewer unavailable → label the review as fallback" message. No silent success either way.
4. Every one of the five agent definitions states: its phase, its entry condition, its exit artifact,
   who it may **not** be (the separation rule), and the forbidden-action list.
5. `STATUS.md` contains task 023 plus the 024–029 backlog, each row parseable by `board-check.sh`.
6. **Regression:** `cargo test --offline` still reports 91 passing tests and both per-program SBF
   builds still exit 0 — this task changes no Rust and must prove it changed no Rust
   (`git diff --stat master -- crates programs` is empty).
7. `docs/HARNESS.md` §Forbidden actions explicitly names: force-push, history rewrite, committing key
   material, `attest/send.mjs --send`, mainnet writes, and pushing without a human-named remote+branch.

## Reproduce

```bash
# gate (the one command; ~90s cold, seconds warm)
bash scripts/verify.sh

# board invariants, including the violation cases
bash scripts/board-check.sh
bash scripts/board-check.sh --selftest

# reviewer reachability
bash scripts/codex.sh --version

# regression: this task touched no product code
git diff --stat master -- crates programs   # must be empty
cargo test --offline 2>&1 | grep "test result"
```

## Out of scope / notes

- Repo-wide `cargo fmt` (166 hunks / 16 files) → backlog 029, `OPEN_RISK` on the board until then.
- The two pre-existing clippy warnings (`manual_is_multiple_of`, `items_after_test_module`) are
  recorded as the clippy baseline; the gate is **non-regression**, not zero.
- GitHub PR review surface (`gh pr create`) stays optional. A remote exists
  (`origin https://github.com/mppsol/probatio-svm.git`, `master` in sync at `7ab85b4`), but the loop
  runs on the local branch + `reviews/NNN-*.md` surface `AGENTS.md` already allows, and **pushes
  nothing on its own authority** (I7). Note the remote org is `mppsol` while `AGENTS.md` §Git hygiene
  says `gh auth switch -u psyto` before any push — that mismatch is why push stays human-initiated.
</content>
