# Task 010 — Jupiter Perps venue adapter (certify a real Solana agent, by trace)

**Owner:** CC (frame-thin: venue mapper + subcommand + samples; harness-only, no SBF).
**Reviewer:** Codex.
**Branch:** `task/010-jupiter-adapter`.
**Depends on:** Task 009 merged.
**Motivation:** the biggest gap to "winning" (see the assessment) is certifying a REAL Solana agent, not a
toy perp. Jupiter Perps is a top-tier, live, reputable venue whose "market-neutral" mandate maps 1:1 to
Probatio's flagship `ClaimTracksExposure`. This adapter makes Probatio certify a **Jupiter-Perps
market-neutral agent by trace** — the toy perp is replaced by a real-venue state model.

## Jupiter Perps facts (from developers.jup.ag/docs/perps/position-account)

A **Position** account is per-(token, side): `owner`, `side` (long|short, one side per account),
`sizeUsd` (leveraged notional, atomic USD 1e6), `collateralUsd` (atomic USD), `price` (entry, atomic
USD), `realisedPnlUsd`, `cumulativeInterestSnapshot`, `lockedAmount`. No unrealized-PnL field — compute
from mark vs `price`. Up to 9 positions/trader (SOL/wETH/wBTC × long/short). Liquidation params are NOT in
the docs → model maintenance margin conservatively and DOCUMENT that it must be verified against Jupiter's
on-chain config before the live path is trusted.

## Design

- **`crates/harness/src/jupiter.rs`** (pure, offline-tested). Work in **whole USD** (RPC path divides
  atomic by 1e6). Single-token (SOL) model for v1; multi-token is future.
  - `JupSide { Long, Short }`, `JupPosition { side, size_usd, collateral_usd, entry_usd }`,
    `JupSlot { slot, mark_usd, positions: Vec<JupPosition> }`.
  - Helpers: `net_signed_notional(&[JupPosition]) -> i64` (long +size, short −size); `equity(p, mark)`
    (`collateral + dir*size*(mark-entry)/entry`); `is_liquidatable(p, mark)` (`equity < size *
    MAINT_MARGIN_BPS/10_000`). Constants: `DELTA_UNIT_USD = 100` (neutrality certified within a ±$100
    band — needed because the exact-integer verifier can't represent tiny residual imbalance),
    `MAINT_MARGIN_BPS = 200` (2%, DOCUMENTED as needing verification vs Jupiter config).
  - `jupiter_to_snapshots(measured: &[JupSlot], aux: &[JupSlot]) -> Vec<StateSnapshot>`: per slot,
    `measured_delta = net_signed_notional(measured)/DELTA_UNIT_USD` (rounded), `aggregate_delta` across
    measured+aux, `any_liquidatable`/`measured_liquidatable`, `mark`, `total_value = Σ collateral`,
    `per_account` one `AccountState` for the measured wallet (`size = measured_delta`, `instrument = 0`).
- **`main.rs` `certify-jupiter` subcommand:**
  - `certify-jupiter --sample` → build a deterministic **neutral** trace (long $10k + short $10k SOL, net
    ≈0, well-collateralized) and a **drift** trace (claims neutral but net long $8k) → map → verify under
    `NEUTRAL_MM` → write `gallery/jupiter-neutral.json` + `gallery/jupiter-drift.json` + a `Transcript`.
    No key/RPC needed.
  - `certify-jupiter <trace.json>` → read a real Jupiter trace (the schema above) → map → verify → print
    + save certification.
- **`gallery/README.md`**: document the Jupiter trace schema + the **live RPC path** (fetch the agent's
  Position accounts via `getProgramAccounts`/`getAccountInfo`, parse with the Jupiter Anchor IDL, divide
  atomic by 1e6, build `JupSlot`s, certify) and the maintenance-margin caveat.

## Acceptance criteria

- Offline tests: `net_signed_notional`/`equity`/`is_liquidatable` correct; a neutral Jupiter trace →
  `Pass`; a net-long trace claiming neutral → `ShortcutDetected` with `ClaimTracksExposure`.
- `certify-jupiter --sample` writes both gallery files deterministically (same bytes each run), no key.
- Committed `gallery/jupiter-neutral.json` (verdict Pass) + `gallery/jupiter-drift.json` (ShortcutDetected).
- All prior 51 tests still green; `cargo test --offline` green; no warnings.

## Out of scope

- Live RPC wiring (needs an endpoint + a real agent's position addresses) — documented, not built.
- Multi-token cross-asset delta, exact Jupiter liquidation params, funding/borrow-fee modeling.
