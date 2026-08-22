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
- **State of record:** [`STATUS.md`](./STATUS.md) — all five hypotheses and their verdicts.
- **What this repo is now:** [`README.md`](./README.md) — a falsification record. **There is no
  roadmap.** [`STAGE0_DESIGN.md`](./STAGE0_DESIGN.md) is **H1's design doc and is HISTORICAL**; its
  positioning and "next stages" are refuted and must not be cited as a plan.

Keep `cargo test` green and episode traces deterministic. Work on a branch, commit, and have the other
agent review before merge. Commit as `psyto <saito.hiroyuki@gmail.com>`. Sibling repo `../probatio` is
the Reth/revm proving ground this one mirrors.
