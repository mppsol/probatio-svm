<!-- KILL-GATE-BANNER -->
# ⛔ NO HYPOTHESIS IS LIVE — every gate in this repo is closed

**Read [`STATUS.md`](STATUS.md) before doing anything here.** All three gates are closed, and their
verdicts are **not to be rewritten, revived, or worked around**:

- `docs/GATE.md` — H1, **`KILLED`** ([`docs/decisions/P1-real-target.md`](docs/decisions/P1-real-target.md)).
- `docs/H2-GATE.md` — H2, **`FROZEN UNEXECUTED`** (founder, 2026-08-21). No H2 measurement was ever
  run; resuming it needs a founder ruling and its own re-frozen G0.
- `docs/H3-GATE.md` — H3, **`KILLED` at the design gate** (2026-08-21),
  [`docs/decisions/H3-design-gate-kill.md`](docs/decisions/H3-design-gate-kill.md). What died is the
  fixed, protocol-independent capability schema. No H3 gate item was ever executed.

**Nothing may be built until a founder ruling opens a new gate**, and a new gate starts at its own G0
with its own pre-registration. Do not start a successor hypothesis because one is sketched in a
deferral note — a verdict ends a phase, it does not start the next one. No H1 artifact — verifier,
attestation, `MandateSpec`, guard, gallery, `attest/`, the agent framing — may be reused as a new
hypothesis's foundation; reusing one because it exists is 延命, not scope. The sibling repo
`../solvo` is a **read-only** reference and is never edited from here.

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
