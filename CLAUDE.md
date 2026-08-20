<!-- KILL-GATE-BANNER -->
# ⛔ PHASE: KILL GATE ONLY — H2

**Read [`docs/H2-GATE.md`](docs/H2-GATE.md) before doing anything in this repo.** It is binding and it
overrides any older plan, roadmap or task list you find here — including `docs/GATE.md`, which is H1's
gate and is **closed**: H1 is `KILLED` and its verdict is not to be rewritten.

**H2 is at G0 (pre-registration). No code is written in this phase**, and no H1 artifact — verifier,
attestation, `MandateSpec`, guard, gallery, the agent framing — may be reused as H2's foundation.
Reusing one because it exists is 延命, not scope.

The only work permitted right now is producing evidence for this project's kill gate. No
generalisation, no large UI, no peripheral features, no production deployment, until the gate
returns `GO`.

- A verdict is a **number or a reproducible experiment** — never an assessment or a plan.
- **Not-proven is a KILL**, not a pending. Adding a hypothesis to stay alive is forbidden.
- **At most two agents**: Claude for spec/evidence/task progression, Codex for implementation *or*
  independent review — **never both at once**, and no model reviews its own output.
- **On reaching the gate, STOP.** `GO` ends this phase; it does not start the next one. Update
  `STATUS.md`, commit the evidence, push, and hand back to the founder.
- Forbidden without exception: real funds, mainnet deploy, force push, adding secrets.

<!-- /KILL-GATE-BANNER -->

# CLAUDE.md

This repo is shared by **Claude Code (CC) and Codex**, which cannot see each other's chat. The repo is
the only shared memory.

**Collaboration model: cross-review, split by task *frame*.** Frame-thin/exploratory work (architecture,
ADRs, task briefs, the shared account-layout contract, the pure-Rust reference model, the "explainable /
safe to operate?" pass) is CC's; frame-thick/converging work (Pinocchio programs to spec, LiteSVM
driver, tooling, adversarial audits) is Codex's. Whoever implements a change does **not** review it — the
other agent does.

- **Operating contract (read first):** [`AGENTS.md`](./AGENTS.md) — the brief → branch → review → merge
  loop, the contract surface neither agent changes alone, and how to invoke Codex.
- **Task briefs:** [`docs/tasks/`](./docs/tasks/). **Reviews:** [`reviews/`](./reviews/).
- **What/why + roadmap:** [`README.md`](./README.md), [`STAGE0_DESIGN.md`](./STAGE0_DESIGN.md).

Keep `cargo test` green and episode traces deterministic. Work on a branch, commit, and have the other
agent review before merge. Commit as `psyto <saito.hiroyuki@gmail.com>`. Sibling repo `../probatio` is
the Reth/revm proving ground this one mirrors.
