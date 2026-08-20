# Task 023 — Autonomous cross-review harness (SPEC → SPEC_REVIEW → IMPL → IMPL_REVIEW → DONE)

**Frame:** thin (CC). Process/architecture, not product code. **Spec author:** CC. **Spec reviewer:** Codex.
**Implementer:** CC. **Impl reviewer:** Codex. No contract-surface change.

**Revision 2** — rewritten after Codex spec review `reviews/023-autonomous-harness.spec.md`
(verdict CHANGES, 2×P0 + 3×P1 + 1×P2). Every finding is answered in §Response to spec review r1.

## Goal

Turn the `AGENTS.md` two-agent contract from *prose a human enforces* into a **mechanical loop the two
agents run without per-step human judgement**: an explicit task state machine, a git-verifiable
ordering rule, one board file with a parseable grammar, one gate command with a committed baseline,
and role definitions both agents can load.

A task moves `SPEC → SPEC_REVIEW → IMPL → IMPL_REVIEW → DONE` where every transition is authorized by
an artifact committed to the repo, and the **ordering and completeness** of those artifacts is
checkable from the repo alone — because the repo is the only shared memory between CC and Codex.

## What this harness proves — and what it does not (honest scope of the claim)

Spec review r1 correctly showed that "the board says the reviewer was Codex" proves nothing: both
agents commit under one git identity (`psyto <saito.hiroyuki@gmail.com>`), no signing key is available
to either, and headless `codex exec` leaves no durable out-of-repo receipt (checked: no rollout file
under `~/.codex/sessions` for an `exec` run). So the claim is split into three honest tiers, and
`docs/HARNESS.md` must carry this table verbatim:

| Tier | Property | Status |
|---|---|---|
| **Verifiable** | Ordering (spec review is a git ancestor of every implementation commit), artifact existence, commit resolvability, candidate/review commit binding, board↔git agreement, required brief sections | **Enforced** by `scripts/board-check.sh` — a violating board fails |
| **Falsifiable** | The recorded gate result at the recorded tree | **Re-runnable**: the evidence block names commit + tree hash + script hash, so a third party re-running `scripts/verify.sh` detects a forged result |
| **Asserted** | *Which intelligence* authored a file | **NOT enforced.** No trust root exists. Mitigated by: self-review makes a task **ineligible for DONE** (hard rule, checked), and every review records the reviewer tool + version so a human can spot-check |

Claiming more than this would be exactly the green-washing the product exists to catch. The harness is
an **audit trail with enforced ordering**, not an unforgeable attestation.

## Non-goals

- **Not** a CI service, scheduler, or daemon. No background process, cron, or webhook. The loop is
  driven by whichever agent is invoked; the repo carries the state.
- **Not** a replacement for `AGENTS.md`. This operationalizes it; division-of-labour and
  contract-surface rules stay there and are referenced, not forked.
- **Not** a product feature. No behaviour change to `crates/`, `programs/`, the verifier, the guard,
  or any certification output. Enforced by the protected-surface check in AC6.
- **Not** approved future work. Backlog rows are **proposals only** (state `BACKLOG`): no frame is
  assigned, no work may start from them, and entering `SPEC` requires its own brief + spec review.
- **Not** a repo-wide reformat. `cargo fmt` drift predates this task and stays an `OPEN_RISK` with its
  own backlog slot so it never contaminates a feature diff.
- **No** irreversible external action on the loop's own authority (I7).

## Threat model (what an autonomous two-agent loop gets wrong)

| # | Threat | Consequence | Control | Tier |
|---|---|---|---|---|
| T1 | **Self-review collusion** — the implementer also reviews, or "the other agent" is the same model in a fresh context | Shared blind spots pass as independent sign-off; the product's premise (a *different* intelligence catches the cheat) is faked. **Already happened once**: task 006 merged on "round-3 CC self-verification; headless Codex unavailable" (`8ed9104`) | `implementer == impl_reviewer` ⇒ state may **not** be `DONE`; the row must carry a `blocking` risk. Reviewer tool+version recorded | Asserted + hard eligibility rule |
| T2 | **Green-washing** — an agent asserts gates passed without running them | `DONE` becomes meaningless; a broken `master` looks certified | Evidence block must name candidate commit, its **tree hash**, the `verify.sh` **script hash**, and per-gate exit codes; the impl reviewer re-runs the gate independently on that exact commit | Falsifiable |
| T3 | **Contract drift** — an agent unilaterally edits the account layout / verifier / policy surface | Perp, guard and verifier disagree; all three break at once (`AGENTS.md` §"The contract") | `board-check.sh` diffs `base..candidate` against the contract path list; if any is touched, the brief must carry an `adr:` field and the impl review an `adr-ack:` line, else fail | Verifiable |
| T4 | **Hidden failure** — a sub-part is broken or unproven, and the task is closed anyway | The repo's honesty (its stated differentiator) rots; demo claims outrun evidence | `BLOCKED` / `OPEN_RISK` are first-class with a required `risks:` field and a severity class; a `blocking` risk makes `DONE` fail | Verifiable |
| T5 | **Runaway autonomy** — the loop pushes, spends a key, or submits on mainnet | Irreversible, real-money action with no human in the loop | `docs/HARNESS.md` §Forbidden actions, restated in every role definition. **Honest limit:** git cannot prove an external action did *not* happen; the real control is that no agent definition grants the capability and `--send`/push require a human-issued instruction naming the target | Asserted (documented, not provable) |
| T6 | **Board drift** — `STATUS.md` disagrees with git | The single source of progress lies; dependencies get scheduled off stale state | Every commit field must resolve (`git cat-file -e`); every path field must exist | Verifiable |
| T7 | **Stale evidence** — docs keep an old number after the code moves | Already happened twice here (Findings 1 & 3) | `verify.sh` compares the test count asserted in `README.md` against the **live** count and fails on mismatch | Verifiable |
| T8 | **Artifact provenance / equivocation (TOCTOU)** — the reviewer is shown revision A while the board certifies revision B; or a brief/review is edited after approval | An APPROVE is recycled onto different code; review becomes decorative | Review files must record `reviewed-commit:`; `board-check.sh` requires `spec_review_commit` to be an ancestor of `candidate_commit`, requires the impl review's `reviewed-commit` to **equal** `candidate_commit`, and rejects a spec-review commit that also touches code | Verifiable |

*(T8 was raised by spec review r1 as the missing threat.)*

## Invariants

`scripts/board-check.sh` mechanically enforces I1, I2, I4, I5, I6, I8. I3 is falsifiable-by-re-run.
I7 is documented-only — stated as such rather than advertised as enforced.

- **I1 — Separation of duties.** `spec_author ≠ spec_reviewer` and `implementer ≠ impl_reviewer`.
  A row where they are equal is **ineligible for `DONE`** and must carry a `blocking` risk.
- **I2 — Spec-before-code ordering (git ancestry).** `spec_review_commit` must be a strict ancestor of
  `candidate_commit`, and the spec-review commit itself must touch only `reviews/` and `docs/`. A
  single commit adding both a review and code fails.
- **I3 — Evidence gate.** `DONE` requires a `verify:` field of the form `PASS @ <candidate_commit>`
  and an evidence block in the impl review naming commit, tree hash, script hash, and per-gate exits.
- **I4 — Traceability.** Every path field resolves to an existing file; every commit field resolves in
  git; a `DONE` row has all of brief, spec review, impl review, base, candidate.
- **I5 — Contract integrity.** If `base..candidate` touches a contract-surface path, the brief carries
  `adr:` and the impl review an `adr-ack:` line.
- **I6 — Honest state.** A row with any `blocking` risk, or state `BLOCKED`, may not be `DONE`.
  `BLOCKED` requires a non-empty `blocked_by:`.
- **I7 — Reversibility.** The loop performs no irreversible external action on its own authority.
  *(Documented; not repo-checkable — see the tier table.)*
- **I8 — Brief completeness.** A brief for a task past `SPEC` contains all required sections
  (Goal, Non-goals, Threat model, Invariants, Acceptance criteria, Reproduce), checked by heading.

## Board record grammar (normative)

`STATUS.md` holds one block per task. The grammar is deliberately line-oriented so a shell parser is
robust — no pipe-escaping, no multi-line values:

```
### NNN — <title>
state: BACKLOG | SPEC | SPEC_REVIEW | IMPL | IMPL_REVIEW | DONE | BLOCKED
frame: thin | thick | -
brief: <repo-relative path> | -
spec_author: <actor> | -
spec_reviewer: <actor> | -
spec_review: <repo-relative path> | -
spec_review_commit: <hex sha> | -
implementer: <actor> | -
impl_reviewer: <actor> | -
impl_review: <repo-relative path> | -
base_commit: <hex sha> | -
candidate_commit: <hex sha> | -
verify: PASS @ <hex sha> | FAIL @ <hex sha> | -
depends_on: <NNN>[, <NNN>...] | -
risks: <class>:<slug> [, ...] | -
blocked_by: <free text> | -
```

- `NNN` is 3 digits; a block ends at the next `### ` or end of file.
- Keys are lowercase snake_case, one per line, `key: value`, value trimmed, `-` means empty.
- Actors are `CC` | `Codex` | `human` (extend in `docs/HARNESS.md`, not ad hoc).
- Risk classes: `blocking` (bars `DONE`) | `open` (recorded, does not bar `DONE`).
- `BACKLOG` rows are proposals: only `state`, title, a one-line scope, `depends_on` are required; all
  other fields must be `-`. `board-check.sh` applies no commit/link requirements to them.

Required fields per state are tabulated in `docs/HARNESS.md`; `board-check.sh` is the authority.

## Gate definition (normative) — `scripts/verify.sh`

Exact command list, all offline, no network, no key material, no reviewer dependency:

| Gate | Command | Pass condition |
|---|---|---|
| `build` | `cargo build --offline --workspace` | exit 0 |
| `test` | `cargo test --offline` | exit 0; live total extracted |
| `sbf-perp` | `cargo build-sbf --offline --manifest-path programs/perp/Cargo.toml --features bpf-entrypoint --sbf-out-dir target/deploy` | exit 0 |
| `sbf-guard` | same for `programs/guard/Cargo.toml` | exit 0 |
| `episode-ref` | `cargo run --offline -q -p probatio-svm-harness -- --backend ref` | exit 0 |
| `episode-svm` | `cargo run --offline -q -p probatio-svm-harness -- --backend svm` | exit 0 |
| `certify-sample` | `cargo run --offline -q -p probatio-svm-harness -- certify-jupiter --sample` | exit 0 |
| `clippy` | `cargo clippy --offline --workspace --all-targets` | **no lint name absent from** `scripts/verify-baseline.txt` |
| `fmt-drift` | `cargo fmt --all --check` | hunk count **≤** the baseline count (non-regression, advisory-labelled) |
| `evidence` | README asserted test count vs live count | equal (T7) |

- Baseline lives at `scripts/verify-baseline.txt` (committed, human-readable: allowed clippy lint
  names + the fmt hunk count). A new diagnostic fails the gate; the baseline may only be *lowered*
  without a brief.
- `codex.sh` reachability is **not** a gate (it is environmental) — this removes the r1 contradiction
  between AC3 and the gate list.
- Full logs to `target/verify/<commit>/` (gitignored). Stdout ends with the pasteable evidence block:
  commit, tree hash, `verify.sh` sha256, per-gate exit codes, live test count.

**An unavailable reviewer is `BLOCKED`, not a permitted silent fallback.** A self-review may be
*recorded* (labelled, with a `blocking:self-review` risk) but the task cannot reach `DONE`.

## Findings this task must fix (all three independently confirmed by spec review r1)

1. **`AGENTS.md` Codex path is dead.** `/Applications/Codex.app/Contents/Resources/codex` does not
   exist; the CLI is `/Applications/ChatGPT.app/Contents/Resources/codex` (`codex-cli 0.148.0-alpha.9`).
   An autonomous CC following `AGENTS.md` cannot reach its reviewer → every review silently degrades
   to self-review (T1).
2. **`AGENTS.md` build gate is wrong as written.** Bare `cargo build-sbf` at the workspace root
   **fails** (`getrandom` has no SBF target, pulled in via the harness crate). It works only
   per-program with `--features bpf-entrypoint`, as `crates/harness/src/world.rs:384-443` already does.
3. **`README.md` test count is stale** — claims 87, live is 91 (7 contract + 1 guard + 1 perp + 77
   harness lib + 2 harness bin + 3 reexec-spec). Threat T7 in the file carrying the project's
   credibility; fixed *and* put under a permanent gate.

## Deliverables

- `docs/HARNESS.md` — canonical, **agent-neutral** protocol: state machine + transition table, the
  three-tier claim table verbatim, role definitions (the single source both CC and Codex load),
  required fields per state, contract surface, forbidden actions, how a `BLOCKED`/`OPEN_RISK` is recorded.
- `STATUS.md` (repo root) — the board, in the grammar above; task 023 plus `BACKLOG` proposals.
- `.claude/agents/probatio-{spec-author,spec-reviewer,implementer,impl-reviewer,evidence-curator}.md`
  — thin CC-facing wrappers that **reference `docs/HARNESS.md` §Roles** rather than fork it.
- `scripts/codex-review.sh` — builds the Codex reviewer prompt from the *same* `docs/HARNESS.md` role
  text and invokes the CLI, so Codex is a first-class role holder, not just a resolved binary path.
- `docs/templates/{task-brief,spec-review,impl-review}.md` — required sections; enforced by I8, not by
  the template's mere existence.
- `scripts/verify.sh` + `scripts/verify-baseline.txt` — the gate above.
- `scripts/codex.sh` — resolves the Codex CLI across known locations; fails loudly (never silently
  degrades to self-review).
- `scripts/board-check.sh` + `scripts/fixtures/board/` — enforces I1/I2/I4/I5/I6/I8, with adversarial
  fixtures.
- `AGENTS.md` — corrected Codex path and SBF gate + pointer to `docs/HARNESS.md`. Division-of-labour
  and contract sections **not** rewritten.
- `README.md` — test count corrected (Finding 3). No other prose touched.

## Acceptance criteria

1. `bash scripts/verify.sh` exits 0 on this branch and ends with an evidence block naming: candidate
   commit, tree hash, `verify.sh` sha256, every gate with its exit code, and the **live** test count.
2. `bash scripts/board-check.sh` exits 0 on `STATUS.md`, and `--selftest` exits 0 by demonstrating that
   each adversarial fixture is **rejected** with the violated invariant named. Fixtures must include,
   at minimum: (a) `impl_reviewer == implementer` on a `DONE` row [I1]; (b) code committed before the
   spec review — spec-review commit not an ancestor of candidate [I2]; (c) one commit containing both
   the spec review and code [I2]; (d) an impl review whose `reviewed-commit` ≠ `candidate_commit` [T8];
   (e) a missing artifact path and an unresolvable commit [I4]; (f) `DONE` with a `blocking` risk [I6];
   (g) a contract-surface edit with no `adr:` field [I5]; (h) a brief missing a required section [I8].
3. `bash scripts/codex.sh --version` prints a Codex version, or exits non-zero with an explicit
   "reviewer unavailable → task is BLOCKED, self-review cannot reach DONE" message. No silent success.
4. Each of the five agent definitions states its phase, entry condition, exit artifact, who it may
   **not** be, and the forbidden-action list — and defers to `docs/HARNESS.md` as canonical.
5. `STATUS.md` parses under `board-check.sh`; every `BACKLOG` row is marked a proposal with empty
   role/commit fields, so no unreviewed planning is presented as approved work.
6. **Regression / protected surface:** `git diff --name-only master...HEAD` intersected with the
   protected list (`crates/**`, `programs/**`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`,
   `vendor/**`, `.github/**`) is **empty**; `cargo test --offline` passes with the live total reported
   (not hard-coded), and both per-program SBF builds exit 0.
7. `docs/HARNESS.md` §Forbidden actions explicitly names: force-push, history rewrite, committing key
   material, `attest/send.mjs --send`, mainnet writes, and pushing without a human-named remote+branch.
8. `docs/HARNESS.md` carries the three-tier claim table verbatim, so the harness never advertises
   unforgeable attestation.

## Reproduce

```bash
bash scripts/verify.sh                  # the one gate; evidence block on stdout
bash scripts/board-check.sh             # board invariants over STATUS.md
bash scripts/board-check.sh --selftest  # every adversarial fixture must be REJECTED
bash scripts/codex.sh --version         # reviewer reachability (not a gate)

# protected surface untouched by this task
git diff --name-only master...HEAD | grep -E '^(crates/|programs/|Cargo\.(toml|lock)|rust-toolchain\.toml|vendor/|\.github/)' ; echo "exit=$? (1 = clean)"
```

## Response to spec review r1

| Finding | Response |
|---|---|
| P0 role labels cannot establish independence | **Accepted, claim narrowed.** §"What this harness proves" splits Verifiable / Falsifiable / Asserted. Self-review is now **ineligible for `DONE`** (I1) and an unavailable reviewer is `BLOCKED`, resolving the I1↔I6 contradiction |
| P0 no mechanically checkable ordering | **Accepted.** I2/T8 now use git ancestry: spec-review commit is a strict ancestor of candidate, spec-review commit may not touch code, impl review's `reviewed-commit` must equal `candidate_commit`. Fixtures (b)(c)(d) in AC2 |
| P1 guarantees are only review conventions | **Accepted.** Each invariant and threat is tagged with its tier; I5 contract detection and the ADR-ack parse move into `board-check.sh`; I7 is explicitly labelled documented-only |
| P1 `verify.sh` gate vs non-regression | **Accepted.** Exact command table, offline policy, committed `verify-baseline.txt`, fail-on-new-diagnostic; `codex.sh` removed from the gate set |
| P1 grammar underspecified + scope creep into 024–029 | **Accepted.** Normative grammar section added; `BACKLOG` rows demoted to proposals with empty fields (AC5). `docs/HARNESS.md` becomes the agent-neutral role source and `scripts/codex-review.sh` gives Codex a real entrypoint |
| P2 snapshot evidence / protected surface | **Accepted.** Test count is reported live and cross-checked against README (a permanent T7 gate); protected surface is an explicit path list, not two directories |

## Out of scope / notes

- Repo-wide `cargo fmt` (166 hunks / 16 files) → `BACKLOG` proposal, `open` risk until then.
- Pre-existing clippy lints (`manual_is_multiple_of`, `items_after_test_module`) form the committed
  baseline; the gate is non-regression, not zero.
- A remote exists (`origin https://github.com/mppsol/probatio-svm.git`, `master` in sync at `7ab85b4`)
  but the loop **pushes nothing on its own authority** (I7). The remote org is `mppsol` while
  `AGENTS.md` §Git hygiene says `gh auth switch -u psyto`; that mismatch is recorded as an `open` risk
  and is a further reason push stays human-initiated.
</content>
