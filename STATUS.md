# STATUS — Probatio SVM kill gate

Phase: **KILL GATE ONLY** (`docs/GATE.md` is binding).
One item per run. A verdict is a number or a reproducible experiment.

| # | Item | Status | Evidence | Verdict date |
|---|---|---|---|---|
| **P1** | A real target — an actual on-chain agent or agent vault | **KILLED** | [`docs/decisions/P1-real-target.md`](docs/decisions/P1-real-target.md) + Codex [r1](reviews/P1-real-target.md) → [r2 `APPROVE`](reviews/P1-real-target-r2.md) | 2026-08-20 |
| P2 | Reproducible certification | not reached | — | — |
| P3 | The constraint binds | not reached | — | — |
| P4 | Differentiation vs an existing sandbox/fuzzer | not reached | — | — |

## P1 — KILLED (2026-08-20)

Reproduce: `python3 docs/decisions/repro/p1_real_target.py` (~60s, stdlib only).
Independently reviewed by Codex: [r1 `CHANGES`](reviews/P1-real-target.md) (one P0, fixed) →
[r2 **`APPROVE`**](reviews/P1-real-target-r2.md).

At mainnet slot 440479436, of the **1,767** distinct agent identities carried by the **1,471**
`AgentAccount` records in the Solana Agent Registry (`8oo4dC4Jv…`) — `creator`, `owner`, `asset`,
`agent_wallet`, `parent_asset`, decoded against the registry's published Borsh schema — **6** have ever
held a Jupiter Perps position and **0** have an open one. The set of registered on-chain agents this
harness can certify is empty. Of the **4,708** addresses that do hold open Jupiter positions,
**4,366 (96.4%, $62.2M)** are plain System-Program wallets indistinguishable from human traders;
**337** are rent-collected accounts of which **0 are off-curve**, so none is a vault PDA; and only
**5 ($32,423 = 0.05%)** are program-controlled, none identifiable as an agent vault.

The repo's headline live PASS (`gallery/jupiter-live-AhUvhrHH.json`) was recomputed from the
definition of `delta_units`: PASS ⟺ `|net notional| < $50`, and that wallet's whole position is **$16**
— it passes by being too small to measure. Across the live population **98.4% of the PASS class is
sub-$50 dust**; 83.16% FLAG.

Not `BLOCKED`: no external dependency with a stated action on a stated date. Not rescued by a new
ingestion adapter — `docs/GATE.md` treats *"it would work if we also had X"* as a KILL, not a new scope.

**The gate stops here. Per `docs/GATE.md`, the founder decides what happens next.**
