<!-- KILL-GATE-BANNER -->
# ⛔ PHASE: BETWEEN HYPOTHESES — H4 is `KILLED`; nothing is live

**H4 (Upgrade Behavior Sentinel) is closed — `KILLED` at G0 on 2026-08-22, `KILL-2` + `KILL-3`**:
[`docs/decisions/H4-sentinel-kill.md`](docs/decisions/H4-sentinel-kill.md), gate
[`docs/H4-GATE.md`](docs/H4-GATE.md), review [`reviews/H4-G0-sentinel.md`](reviews/H4-G0-sentinel.md).
The measurement ran; all four cases returned `unknown`, so the action was `RE-VERIFY` — no more
actionable than the code-hash difference already was.

**All four gates are closed and their verdicts are not to be rewritten, revived, or worked around:**

- `docs/GATE.md` — H1, **`KILLED`** ([`docs/decisions/P1-real-target.md`](docs/decisions/P1-real-target.md)).
- `docs/H2-GATE.md` — H2, **`FROZEN UNEXECUTED`** (founder, 2026-08-21). No H2 measurement was ever run.
- `docs/H3-GATE.md` — H3, **`KILLED` at the design gate** (2026-08-21). What died is the fixed,
  protocol-independent capability schema. No H3 gate item was ever executed.
- `docs/H4-GATE.md` — H4, **`KILLED` at G0** (2026-08-22), on its own pre-registered numbers.

**No hypothesis is currently live.** Until the founder rules a new one open with its own committed
G0, the only permitted work is recording verdicts. No implementation, no UI, no token, no deploy, no
second operation, no second protocol. No artifact from H1–H4 — verifier, attestation, `MandateSpec`,
guard, gallery, `attest/`, the agent framing, the H4 sentinel harness — may be reused as a new
hypothesis's **foundation**; reusing one because it exists is 延命, not scope. The sibling repo
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
