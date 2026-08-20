# Task 023 — Autonomous cross-review harness (SPEC → SPEC_REVIEW → IMPL → IMPL_REVIEW → DONE)

**Frame:** thin (CC). Process/architecture, not product code. **Spec author:** CC. **Spec reviewer:** Codex.
**Implementer:** CC. **Impl reviewer:** Codex. No contract-surface change.

**Revision 3** — answers Codex spec review round 2 (`reviews/023-autonomous-harness.spec.md`, verdict
CHANGES: one surviving P0 — git ancestry admits a merge-parent bypass and a post-candidate-append
bypass). See §Response to spec review.

## Goal

Turn the `AGENTS.md` two-agent contract from *prose a human enforces* into a **mechanical loop the two
agents run without per-step human judgement**: an explicit task state machine, a git-checkable close
predicate, one board file with a normative grammar, one gate command with a committed baseline, and
role definitions both agents load from one agent-neutral source.

A task moves `SPEC → SPEC_REVIEW → IMPL → IMPL_REVIEW → DONE` where every transition is authorized by
an artifact committed to the repo, and the **ordering, binding and completeness** of those artifacts is
checkable from the repo alone — because the repo is the only shared memory between CC and Codex.

## What this harness proves — and what it does not (honest scope of the claim)

Spec review r1 showed that "the board says the reviewer was Codex" proves nothing: both agents commit
under one git identity (`psyto <saito.hiroyuki@gmail.com>`), no signing key is available to either, and
headless `codex exec` leaves no durable out-of-repo receipt (checked: no rollout file appears under
`~/.codex/sessions` for an `exec` run). The claim is therefore split into three tiers, carried verbatim
in `docs/HARNESS.md`:

| Tier | Property | Status |
|---|---|---|
| **Verifiable** | The close predicate (§Close predicate): ordering, no pre-review code via any parent, no post-candidate code, artifact↔commit blob binding, evidence presence and binding, artifact existence, commit resolvability, required brief sections, declared-role separation | **Enforced** by `scripts/board-check.sh` — a violating board fails |
| **Falsifiable** | The *result* the recorded gate claims at the recorded tree | **Re-runnable**: the evidence block names commit + tree hash + script hash, so re-running `scripts/verify.sh` at that commit detects a forged result |
| **Asserted** | *Which intelligence* authored a file | **NOT enforced. No trust root exists.** A self-review labelled `impl_reviewer: Codex` passes the checker. What is enforced is only that a row **declaring** the same actor as implementer and impl reviewer is ineligible for `DONE`. Every review records the reviewer tool + version for human spot-check |

Claiming more than this would be exactly the green-washing the product exists to catch. The harness is
an **audit trail with an enforced close predicate**, not an unforgeable attestation.

## Non-goals

- **Not** a CI service, scheduler, or daemon. No background process, cron, or webhook. The loop is
  driven by whichever agent is invoked; the repo carries the state.
- **Not** a replacement for `AGENTS.md`. This operationalizes it; division-of-labour and
  contract-surface rules stay there and are referenced, not forked.
- **Not** a product feature. No behaviour change to `crates/`, `programs/`, the verifier, the guard,
  or any certification output — enforced by the protected-surface check in AC7.
- **Not** approved future work. Backlog rows are **proposals** (state `BACKLOG`): no frame assigned, no
  work may start from them; entering `SPEC` requires their own brief + spec review.
- **Not** a repo-wide reformat. `cargo fmt` drift predates this task and stays an `open` risk with its
  own backlog proposal so it never contaminates a feature diff.
- **No** irreversible external action on the loop's own authority (I7).

## Threat model

| # | Threat | Consequence | Control | Tier |
|---|---|---|---|---|
| T1 | **Self-review collusion** — the implementer also reviews, or "the other agent" is the same model in a fresh context | Shared blind spots pass as independent sign-off; the product's premise (a *different* intelligence catches the cheat) is faked. **Already happened once**: task 006 merged on "round-3 CC self-verification; headless Codex unavailable" (`8ed9104`) | A row **declaring** `implementer == impl_reviewer` may not be `DONE` and must carry a `blocking` risk. Reviewer tool+version recorded. Undeclared self-review is *not* detectable — stated, not hidden | Asserted + declared-field rule |
| T2 | **Green-washing** — gates asserted, never run | `DONE` becomes meaningless; broken `master` looks certified | Evidence block binding (E1–E4 below) is parser-enforced; the claimed result is falsifiable by re-run; the impl reviewer re-runs the gate on the exact candidate | Verifiable binding + Falsifiable result |
| T3 | **Contract drift** — unilateral edit of the account layout / verifier / policy surface | Perp, guard and verifier disagree; all three break at once (`AGENTS.md` §"The contract") | `board-check.sh` diffs `base..candidate` against the enumerated contract surface; if touched, the brief must carry `adr:` **and** the impl review an `adr-ack:` line — both halves checked | Verifiable |
| T4 | **Hidden failure** — a broken or unproven part, task closed anyway | The repo's honesty (its differentiator) rots; demo claims outrun evidence | `risks:` with severity class; any `blocking` risk bars `DONE`; `BLOCKED` requires `blocked_by:` | Verifiable |
| T5 | **Runaway autonomy** — push, key spend, mainnet submit | Irreversible, real-money action with no human in the loop | `docs/HARNESS.md` §Forbidden actions, restated in every role definition. **Honest limit:** git cannot prove an external action did *not* happen; the control is that no role definition grants the capability and `--send`/push require a human instruction naming the target | Asserted (documented, not provable) |
| T6 | **Board drift** — `STATUS.md` disagrees with git | The single progress source lies; dependencies scheduled off stale state | Every commit field must resolve; every path field must exist; the candidate must be an ancestor of the checked ref | Verifiable |
| T7 | **Stale evidence** — docs keep an old number after the code moves | Already happened twice here (Findings 1 & 3) | `verify.sh` compares the test count asserted in `README.md` against the **live** count and fails on mismatch | Verifiable |
| T8 | **Provenance / equivocation (TOCTOU)** — reviewer shown revision A, board certifies revision B; or brief/review edited after approval; or code slipped in via a merge parent or appended after the candidate | An APPROVE is recycled onto different code; review becomes decorative | The **close predicate** C1–C7 below, including all-parent code-origin checks and blob binding | Verifiable |

## Close predicate (normative)

Let `B = base_commit`, `R = spec_review_commit`, `C = candidate_commit`, `H` = the ref `board-check.sh`
is run against (default `HEAD`). Let **artifact paths** = `STATUS.md`, `docs/**`, `reviews/**`; every
other path is **code** for this predicate's purpose.

For a row in state `DONE` (and, where the field exists, at earlier states):

- **C1 — base before review.** `B` is a strict ancestor of `R`.
- **C2 — review before candidate.** `R` is a strict ancestor of `C`.
- **C3 — no merges in the task range.** `git rev-list --merges B..C` is empty. (One task, one linear
  branch; this is what makes C4 exhaustive rather than diff-suppressed on merge commits.)
- **C4 — no pre-review code via any parent.** For every commit `X` in `git rev-list B..C`: if
  `git diff-tree --no-commit-id --name-only -r X` names any **code** path, then `R` must be an ancestor
  of `X`. *This is what rejects the merge-side-parent DAG that r2 found — `C` on the side branch is in
  `rev-list B..M` and is not a descendant of `R`.*
- **C5 — candidate is on the closing ref, and nothing but artifacts follows it.** `C` is an ancestor of
  (or equal to) `H`, and every commit in `git rev-list C..H` touches **artifact paths only**. *This is
  what rejects appending unreviewed code after the recorded candidate.*
- **C6 — artifact↔commit blob binding.** The blob of each recorded artifact at `H` must equal its blob
  at the commit that recorded it:
  `git rev-parse <brief_commit>:<brief> == git rev-parse H:<brief>`,
  `git rev-parse <spec_review_commit>:<spec_review> == git rev-parse H:<spec_review>`,
  `git rev-parse <impl_review_commit>:<impl_review> == git rev-parse H:<impl_review>`.
  Additionally `brief_commit` must be an ancestor of `R` (the reviewer saw that brief) and
  `impl_review_commit` must lie in `C..H`.
- **C7 — evidence binding (I3, parser-enforced).** The impl review must contain a fenced evidence block
  with **E1** `reviewed-commit: <sha>` equal to `C`; **E2** `tree: <sha>` equal to `git rev-parse C^{tree}`;
  **E3** `verify-script-sha256: <hex>` equal to the sha256 of `scripts/verify.sh` **as of `C`**
  (`git show C:scripts/verify.sh | shasum -a 256`); **E4** one `gate <name> exit=<n>` line per gate in
  the §Gate table with `n == 0`. Absent, malformed, or mismatched ⇒ reject.

`git rev-list`, `git merge-base --is-ancestor`, `git diff-tree`, `git rev-parse <commit>:<path>` are
sufficient to implement all of C1–C7 with no extra state.

**Honest residual (must be stated in `docs/HARNESS.md`):** C1–C7 bind *what was reviewed* to *what is
closed*. They do not prove *who* reviewed it (Asserted tier), and an actor with write access to both
the board and history can rewrite the branch wholesale — the harness detects inconsistency, not a
determined rewrite by the only writer. The rewrite case is bounded by the forbidden-actions rule
(no history rewrite, no force-push) which is documented, not enforced.

## Invariants

`board-check.sh` enforces I1, I2 (= C1–C5), I3 (= C7), I4, I5, I6, I8, and C6. I7 is documented-only
and labelled as such.

- **I1 — Declared separation of duties.** `spec_author ≠ spec_reviewer` and
  `implementer ≠ impl_reviewer` **as declared**. Equality ⇒ ineligible for `DONE` + `blocking` risk.
- **I2 — Ordering.** The close predicate C1–C5 holds.
- **I3 — Evidence.** C7 holds, and `verify:` reads `PASS @ <candidate_commit>`.
- **I4 — Traceability.** Every path field exists at `H`; every commit field resolves; a `DONE` row has
  brief, brief_commit, spec_review, spec_review_commit, impl_review, impl_review_commit, base, candidate.
- **I5 — Contract integrity.** If `base..candidate` touches the contract surface (enumerated below),
  the brief carries `adr:` **and** the impl review carries `adr-ack:` naming the same ADR. Both halves
  are checked.
- **I6 — Honest state.** Any `blocking` risk, or state `BLOCKED`, bars `DONE`. `BLOCKED` requires a
  non-empty `blocked_by:`.
- **I7 — Reversibility.** No irreversible external action on the loop's own authority.
  *(Documented; not repo-checkable — see the tier table.)*
- **I8 — Brief completeness.** A brief for a task past `SPEC` contains all required headings: Goal,
  Non-goals, Threat model, Invariants, Acceptance criteria, Reproduce.

**Contract surface (enumerated, per `AGENTS.md` §"The contract"):** `crates/contract/**`,
`crates/reexec-spec/**`, `crates/harness/src/verifier.rs`, `crates/harness/src/policy.rs`.
The Pinocchio programs (`programs/**`) are **not** contract surface — they *consume* it — but they are
protected surface for AC7's regression check. `docs/HARNESS.md` repeats this list; `board-check.sh`
reads it from a single constant.

## Board record grammar (normative)

`STATUS.md` holds one block per task, line-oriented so a shell parser is robust — no pipe escaping,
no multi-line values:

```
### NNN — <title>
state: BACKLOG | SPEC | SPEC_REVIEW | IMPL | IMPL_REVIEW | DONE | BLOCKED
scope: <one line, required for every state>
frame: thin | thick | -
brief: <repo-relative path> | -
brief_commit: <hex sha> | -
spec_author: <actor> | -
spec_reviewer: <actor> | -
spec_review: <repo-relative path> | -
spec_review_commit: <hex sha> | -
implementer: <actor> | -
impl_reviewer: <actor> | -
impl_review: <repo-relative path> | -
impl_review_commit: <hex sha> | -
base_commit: <hex sha> | -
candidate_commit: <hex sha> | -
verify: PASS @ <hex sha> | FAIL @ <hex sha> | -
depends_on: <NNN>[, <NNN>...] | -
risks: <class>:<slug>[, ...] | -
blocked_by: <free text> | -
```

- `NNN` is 3 digits; a block ends at the next `### ` or EOF.
- Keys are lowercase snake_case, exactly one `key: value` per line, value trimmed, `-` = empty.
  **An unknown key is a hard parse error** (so grammar drift cannot pass silently).
- Actors: `CC` | `Codex` | `human`. Extend in `docs/HARNESS.md`, never ad hoc.
- Risk classes: `blocking` (bars `DONE`) | `open` (recorded, does not bar `DONE`).
- `BACKLOG` rows are proposals: `state`, `scope`, `depends_on` may be set; **all other fields must be
  `-`**, and no commit/link/predicate requirement applies to them.

Required fields per state are tabulated in `docs/HARNESS.md`; `board-check.sh` is the authority.

## Gate definition (normative) — `scripts/verify.sh`

All offline, no network, no key material, no reviewer dependency:

| Gate | Command | Pass condition |
|---|---|---|
| `build` | `cargo build --offline --workspace` | exit 0 |
| `test` | `cargo test --offline` | exit 0; live total extracted |
| `sbf-perp` | `cargo build-sbf --offline --manifest-path programs/perp/Cargo.toml --features bpf-entrypoint --sbf-out-dir target/deploy` | exit 0 |
| `sbf-guard` | same for `programs/guard/Cargo.toml` | exit 0 |
| `episode-ref` | `cargo run --offline -q -p probatio-svm-harness -- --backend ref` | exit 0 |
| `episode-svm` | `cargo run --offline -q -p probatio-svm-harness -- --backend svm` | exit 0 |
| `certify-sample` | `cargo run --offline -q -p probatio-svm-harness -- certify-jupiter --sample` | exit 0 |
| `clippy` | `cargo clippy --offline --workspace --all-targets` | every observed lint name ∈ baseline allow-list |
| `fmt-drift` | `cargo fmt --all --check` | observed hunk count ≤ baseline `fmt_hunks_max` |
| `evidence` | README asserted test count vs live count | equal (T7) |

**Baseline file grammar** — `scripts/verify-baseline.txt`, committed:

```
# comment lines and blank lines ignored
clippy_allow: clippy::manual_is_multiple_of
clippy_allow: clippy::items_after_test_module
fmt_hunks_max: <integer>
```

Comparison predicate: **fail** if any observed clippy lint name is not in the `clippy_allow` set;
**fail** if observed fmt hunks > `fmt_hunks_max`. The comparator is a separate pure function
(`scripts/lib/baseline-cmp.sh`) so it can be tested against synthetic inputs without a compile
(AC4). Raising `fmt_hunks_max` or adding a `clippy_allow` entry is a **contract-visible change**: it
must be justified in the task brief of whichever task does it — stated as a review duty, since a
script cannot judge justification.

`codex.sh` reachability is **not** a gate (it is environmental). Logs to `target/verify/<commit>/`
(gitignored). Stdout ends with the pasteable evidence block (E1–E4 plus the live test count).

**An unavailable reviewer is `BLOCKED`, never a silent fallback.** A self-review may be *recorded*
(labelled, with a `blocking:self-review` risk) but the task cannot reach `DONE`.

## Findings this task must fix (all three independently confirmed by spec review r1)

1. **`AGENTS.md` Codex path is dead.** `/Applications/Codex.app/Contents/Resources/codex` does not
   exist; the CLI is `/Applications/ChatGPT.app/Contents/Resources/codex` (`codex-cli 0.148.0-alpha.9`).
   An autonomous CC following `AGENTS.md` cannot reach its reviewer → every review silently degrades
   to self-review (T1).
2. **`AGENTS.md` build gate is wrong as written.** Bare `cargo build-sbf` at the workspace root
   **fails** (`getrandom` has no SBF target, pulled in via the harness crate). It works only
   per-program with `--features bpf-entrypoint`, as `crates/harness/src/world.rs:384-443` already does.
3. **`README.md` test count is stale** — claims 87, live is 91. Threat T7 in the file carrying the
   project's credibility; fixed *and* put under a permanent gate.

## Deliverables

- `docs/HARNESS.md` — canonical, **agent-neutral** protocol: state machine + transition table, the
  three-tier claim table and the honest residual verbatim, the close predicate, role definitions (the
  single source both CC and Codex load), required fields per state, contract surface, forbidden
  actions, how `BLOCKED`/`OPEN_RISK` is recorded.
- `STATUS.md` — the board in the grammar above: task 023 plus `BACKLOG` proposals.
- `.claude/agents/probatio-{spec-author,spec-reviewer,implementer,impl-reviewer,evidence-curator}.md`
  — thin CC-facing wrappers that **reference `docs/HARNESS.md` §Roles** rather than fork it.
- `scripts/codex-review.sh` — builds the Codex reviewer prompt from the *same* `docs/HARNESS.md` role
  text and invokes the CLI; `--print-prompt` emits it without invoking anything.
- `docs/templates/{task-brief,spec-review,impl-review}.md` — required sections (enforced by I8/C7,
  not by the template's existence).
- `scripts/verify.sh`, `scripts/lib/baseline-cmp.sh`, `scripts/verify-baseline.txt`.
- `scripts/codex.sh` — resolves the Codex CLI across known locations; fails loudly.
- `scripts/board-check.sh` — enforces I1–I6, I8, C1–C7, with an adversarial `--selftest` that builds
  throwaway git repositories under `target/board-selftest/` to construct the required DAGs.
- `AGENTS.md` — corrected Codex path and SBF gate + pointer to `docs/HARNESS.md`. Division-of-labour
  and contract sections **not** rewritten.
- `README.md` — test count corrected (Finding 3). No other prose touched.

## Acceptance criteria

1. `bash scripts/verify.sh` exits 0 on this branch and ends with an evidence block containing E1–E4
   and the live test count.
2. `bash scripts/board-check.sh` exits 0 on `STATUS.md` at `HEAD`.
3. `bash scripts/board-check.sh --selftest` exits 0 by demonstrating that **every** fixture below is
   rejected, each naming the violated invariant. Fixtures (a)–(n):
   (a) `impl_reviewer == implementer` on a `DONE` row [I1];
   (b) spec-review commit not an ancestor of candidate [C2];
   (c) one commit containing both the spec review and code [C4];
   (d) **merge-side-parent DAG** — code committed on a side branch before the review, merged into the
   candidate [C3/C4]; (e) **code appended after the candidate** on the closing ref [C5];
   (f) brief blob changed after the spec review approved it [C6];
   (g) spec-review blob changed after it was recorded [C6];
   (h) missing artifact path / unresolvable commit [I4];
   (i) `DONE` with a `blocking` risk [I6];
   (j) contract-surface edit with no `adr:` in the brief [I5];
   (k) contract-surface edit with `adr:` but **no** `adr-ack:` in the impl review [I5];
   (l) impl review with **absent or malformed** evidence block [C7];
   (m) evidence `reviewed-commit` ≠ candidate, and evidence `tree`/`verify-script-sha256` mismatched [C7];
   (n) brief missing a required heading [I8], and a board row with an unknown key [grammar].
4. `bash scripts/verify.sh --selftest` exits 0 by demonstrating the baseline comparator **rejects**
   (i) a clippy lint name absent from the allow-list and (ii) an fmt hunk count above
   `fmt_hunks_max`, and **accepts** the recorded baseline — using synthetic inputs, no compile.
5. `bash scripts/codex.sh --version` prints a Codex version, or exits non-zero with an explicit
   "reviewer unavailable → task is BLOCKED; a self-review cannot reach DONE" message. No silent success.
6. `bash scripts/codex-review.sh --print-prompt 023` exits 0, invokes no CLI, and emits text containing
   the canonical role sentinel line from `docs/HARNESS.md` (proving the prompt is loaded from the
   single source, not forked).
7. **Regression / protected surface:** `git diff --name-only master...HEAD` intersected with
   (`crates/**`, `programs/**`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `vendor/**`,
   `.github/**`) is **empty**; `cargo test --offline` passes with the live total reported (not
   hard-coded); both per-program SBF builds exit 0.
8. Each of the five agent definitions states its phase, entry condition, exit artifact, who it may
   **not** be, and the forbidden-action list — and defers to `docs/HARNESS.md` as canonical.
9. `STATUS.md` parses; every `BACKLOG` row is a proposal with `-` in all role/commit fields.
10. `docs/HARNESS.md` §Forbidden actions explicitly names: force-push, history rewrite, committing key
    material, `attest/send.mjs --send`, mainnet writes, and pushing without a human-named remote+branch.
11. `docs/HARNESS.md` carries the three-tier claim table **and** the close-predicate honest residual
    verbatim, so the harness never advertises unforgeable attestation.

## Reproduce

```bash
bash scripts/verify.sh                  # the one gate; evidence block on stdout
bash scripts/verify.sh --selftest       # baseline comparator negative tests
bash scripts/board-check.sh             # close predicate + invariants over STATUS.md
bash scripts/board-check.sh --selftest  # every adversarial fixture must be REJECTED
bash scripts/codex.sh --version         # reviewer reachability (not a gate)
bash scripts/codex-review.sh --print-prompt 023 | head -20

# protected surface untouched by this task (expect no output)
git diff --name-only master...HEAD | grep -E '^(crates/|programs/|Cargo\.(toml|lock)|rust-toolchain\.toml|vendor/|\.github/)'
```

## Response to spec review

### Round 2

| Finding | Response |
|---|---|
| P0 ordering: merge-side-parent bypass; post-candidate append; no brief/review blob binding | **Accepted — predicate replaced.** §Close predicate C1–C7: C3 rejects merges in `B..C`, C4 requires *every* code-touching commit in the range to descend from `R` (killing the side-parent DAG), C5 forbids non-artifact commits in `C..H`, C6 binds brief/spec-review/impl-review blobs to their recording commits. Fixtures (d)(e)(f)(g) added |
| P0 wording: "self-review ineligible" overclaims detection | **Accepted.** Reworded throughout to "a row **declaring** the same implementer and reviewer"; the tier table now states plainly that a mislabelled self-review passes |
| I3 asserted but not parser-enforced | **Accepted.** C7 makes evidence presence/binding (E1–E4) parser-enforced; only the *result* stays Falsifiable. Fixtures (l)(m) |
| Baseline policy has no mechanical predicate; no negative test | **Accepted.** Baseline file grammar + comparison predicate specified; comparator extracted to `scripts/lib/baseline-cmp.sh`; AC4 requires negative tests for a new lint and an increased hunk count |
| `codex-review.sh` never exercised by an AC | **Accepted.** AC6 requires `--print-prompt` to emit the canonical role sentinel without invoking the CLI |
| `scope:` key missing from the grammar | **Accepted.** `scope:` added and required for every state; unknown keys are now a hard parse error |
| Contract surface not enumerated; `adr-ack` half untested | **Accepted.** Surface enumerated (contract + reexec-spec + verifier.rs + policy.rs; programs are protected-but-not-contract); fixtures (j)(k) cover both halves |

### Round 1

| Finding | Response |
|---|---|
| P0 role labels cannot establish independence | Accepted in r2; claim split into Verifiable / Falsifiable / Asserted tiers |
| P0 no mechanically checkable ordering | Accepted in r2, **completed in r3** by the close predicate |
| P1 guarantees are only review conventions | Accepted; every invariant and threat carries its tier; I5 and I3 moved into the checker; I7 labelled documented-only |
| P1 `verify.sh` gate vs non-regression | Accepted; exact command table, committed baseline, `codex.sh` removed from the gate set |
| P1 grammar underspecified + scope creep into 024–029 | Accepted; normative grammar; `BACKLOG` demoted to proposals; `codex-review.sh` gives Codex a first-class entrypoint |
| P2 snapshot evidence / protected surface | Accepted; live test count cross-checked against README; protected surface is an explicit path list |

## Out of scope / notes

- Repo-wide `cargo fmt` (166 hunks / 16 files) → `BACKLOG` proposal, `open` risk until then.
- Pre-existing clippy lints (`manual_is_multiple_of`, `items_after_test_module`) form the committed
  baseline; the gate is non-regression, not zero.
- A remote exists (`origin https://github.com/mppsol/probatio-svm.git`, `master` in sync at `7ab85b4`)
  but the loop **pushes nothing on its own authority** (I7). The remote org is `mppsol` while
  `AGENTS.md` §Git hygiene says `gh auth switch -u psyto`; recorded as an `open` risk and a further
  reason push stays human-initiated.
</content>
