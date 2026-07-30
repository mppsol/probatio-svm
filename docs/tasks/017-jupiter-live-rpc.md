# Task 017 — Live Jupiter Perps ingestion (getProgramAccounts → certify)

**Frame:** ground-truth recovery (the moat) + a thin curl fetch. Implemented by CC; **Codex reviews**
(adversarial: offset correctness, honesty framing, offline-build cleanliness).

## Why

Tasks 010/011 certify Jupiter agents from a **hand-supplied trace file** (`certify-jupiter <trace.json>`)
or synthetic `--sample`s. The "certify a REAL venue's agent" win-path was only half-closed: nothing read
live chain state. This task adds `certify-jupiter --live <owner>` — fetch an owner's actual Jupiter
positions on-chain, reconstruct net exposure independently, and certify the current snapshot against a
delta-neutral mandate. Ground truth is the account bytes, recovered by us, not asserted by anyone.

## What shipped

Pure, offline-tested (the moat):
- `decode_position(&[u8]) -> Option<JupPosition>` — fixed Borsh offsets, **empirically validated against
  mainnet 2026-07-30**: `side@152` (1=Long/2=Short), `price@153`, `sizeUsd@161`, `collateralUsd@169`
  (all u64 atomic 1e6; account `space == 216`). Closed slots (`sizeUsd==0`, Jupiter pre-allocates ≤9
  slots) and non-Long/Short sides are filtered to `None`.
- `base64_decode` — dependency-free (same discipline as the curl-not-a-crate HTTP path in `llm.rs`).
- `parse_gpa_response(json) -> Result<Vec<JupPosition>>` — extracts `result[].account.data[0]`, decodes,
  surfaces an RPC `error` object instead of returning an empty set silently.
- Tests decode a **real committed mainnet gPA response** (`testdata/jupiter_gpa_owner.json`, owner
  AhUvhrHH…, one open SOL long → side=Long, entry=$143, size=$16, collateral=$15). A regression here
  means the on-chain layout shifted and every live cert would be silently wrong.

Network (not unit-tested; mirrors `CurlClaude`):
- `fetch_owner_positions(rpc_url, owner)` — shells `curl` for `getProgramAccounts` with a
  `dataSize:216` + owner `memcmp@8` filter (returns only that owner's ≤9 position accounts).

CLI: `certify-jupiter --live <owner_pubkey> [--rpc <url>] [--mark <usd>]`
- Default RPC = `$PROBATIO_RPC_URL` else public mainnet.
- Single live snapshot → 1-slot trace → certify vs `NEUTRAL_MM`.
- **Net signed notional is USD-denominated ⇒ mark-independent**, so the delta-neutrality verdict (the
  moat) needs no oracle. `--mark` feeds only the secondary liquidation/equity model; absent, it defaults
  to the size-weighted average entry (documented approximation; liquidation flag advisory, delta verdict
  unaffected).
- Multi-token owners sum naturally (all `sizeUsd` are USD), removing the old single-token SOL limit.

## Honesty (non-negotiable, cf tasks 015/016)

A live card certifies an **unsolicited due-diligence** check against a mandate **we declare**, not one
the operator claimed to us. A FLAG means "these live positions do not satisfy a delta-neutral mandate",
never "the operator lied." The console banner and card state this. Live cards name a real wallet and
capture point-in-time on-chain state → **`gallery/jupiter-live-*.json` is gitignored, never committed.**
The committed `testdata/` fixture is a raw layout-regression blob, not a published judgment on a trader.

## Verified

- `cargo test -p probatio-svm-harness` green (61 lib + 2 bin; +5 new: base64, real-decode, closed/short,
  rpc-error, mark-independence).
- Live smoke, both directions: near-neutral wallet → PASS; a real $127k directional long →
  ShortcutDetected (ClaimMismatch + ClaimTracksExposure + MandateDeviation).
- Offline-build invariant intact: no HTTP/RPC/base64 crate added.

## Follow-ups (not in scope)

- Multi-slot live trace via polling (v1 is a single point-in-time snapshot).
- On-chain oracle mark from the Custody account (v1 uses `--mark`/size-weighted entry).
- Dashboard card for a live cert (web/) — separate UI task, Codex lane.
