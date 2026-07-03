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
