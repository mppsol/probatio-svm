# Task 018 — Live path hardening: real short fixture + snapshot-slot provenance

**Frame:** extends task 017's ground-truth-recovery moat and honesty card. Implemented by **CC**; **Codex
reviews** (adversarial: does the short fixture truly exercise a *distinct* layout, or just re-hit the same
offsets? is the snapshot slot trustworthy, and is the persisted source credential-safe?).

## Why

The task 017 review (`reviews/017-jupiter-live-rpc.md`) was **APPROVE** with two P2s explicitly deferred
to "before a public live-demo follow-up". Both must close before we point `certify-jupiter --live` at a
real wallet in public:

- **P2-1 — only the LONG layout is recovered from real chain bytes.** The one committed mainnet fixture is
  an open SOL *long*. The short-side test (`closed_slot_and_short_side_decode`) *manufactures* bytes at the
  same offsets, so it cannot catch a real-world short / different-custody / layout misunderstanding — it
  only proves the code round-trips its own synthetic bytes.
- **P2-3 — the live card records synthetic slot 0.** `live_slot` hard-codes `slot: 0` and the RPC request
  uses no context slot, so a point-in-time due-diligence card cannot say *which* on-chain snapshot it
  judged. A DD attestation that can't identify its snapshot is not verifiable after the fact.

## Scope

### Part A — real short Position fixture (P2-1)

- Acquire a **real mainnet `getProgramAccounts` response** for a wallet with an **open SHORT** Jupiter
  Perps position — ideally a **different custody / token** than the SOL long fixture (e.g. a BTC or ETH
  short) so it also exercises a second custody `Pubkey`. Commit as
  `crates/harness/src/testdata/jupiter_gpa_short.json`.
- Add an offline decode test (mirror of `decode_real_mainnet_position`) asserting `side == Short` and that
  the reconstructed `size_usd` / `collateral_usd` / `entry_usd` match values independently read from a
  block explorer at capture time. A regression here means the short/custody layout drifted.
- **Keep** the existing manufactured `synth_open` short assertion — it exercises the branch cheaply. The
  *real* fixture is what proves the fixed offsets are not a coincidence at those particular bytes.

**Acquisition dependency (flagged):** capturing this fixture requires a **live mainnet query** (a `curl`
`getProgramAccounts` with the `dataSize` + `POSITION_DISCRIMINATOR` memcmp filter, then find an account
whose `side@152 == 2`). Neither agent's **test** path may hit the network (AGENTS.md gate), so the capture
is a **one-time data step** done by Hiro or an agent with network access; the committed result is a static
blob. Record the capture date + the figures in a comment/const beside the test, exactly as 017 did for
the long fixture.

**Acquisition outcome (2026-07-30):** the free Alchemy endpoint rejected `getProgramAccounts` (HTTP 429,
per-second compute cap — even a single owner-scoped scan); `getAccountInfo` worked. The public
`api.mainnet-beta.solana.com` served the full side-filtered `getProgramAccounts` (withContext) — 351,855
short accounts at slot 436117349 — from which one open short on a **different custody** than the SOL long
(`5Pv3gM9…` vs `7xS2gz2…`, entry ≈ $63k ⇒ BTC-class) was trimmed to a single-account withContext blob
(`testdata/jupiter_gpa_short.json`, 816 bytes, bytes verbatim). It doubles as a real-bytes exercise of the
context-shape parse. No RPC key touches the repo.

### Part B — snapshot slot + source provenance (P2-3)

- `fetch_owner_positions` requests `"withContext": true`.
- `parse_gpa_response` handles **both** response shapes without breaking the existing bare-array fixture:
  - legacy: `result: [ …accounts… ]`
  - withContext: `result: { context: { slot: N }, value: [ …accounts… ] }`
  It returns the positions **and** an `Option<u64>` snapshot slot (a small struct or a tuple — this is a
  harness-internal signature, **not** the frozen contract, so no ADR needed; just note it in the review).
- `live_slot` takes the real chain slot instead of hard-coding `0`; it flows into the card's
  `slots[].slot`.
- Serialize **live-path provenance** into the card (jupiter-live backend only):
  - `snapshot_slot` — the `context.slot` the fetch observed.
  - `captured_at` — **Unix epoch seconds (`u64`)** from `SystemTime` in the CLI. Epoch, not RFC3339, to
    stay dependency-free (no `chrono`) and keep tests off the clock — inject a fixed value in tests.
  - `rpc_source` — **HOST ONLY**, never the full URL (it may carry an API key / token in the path or
    query). Parse out the host; if parsing fails, store `"<redacted>"`.
- Extend the offline live-card provenance test (`live_card_persists_unsolicited_dd_provenance`) to assert
  `snapshot_slot`, `captured_at`, and a host-only `rpc_source` are present, and that a key-bearing URL
  (e.g. `https://x.rpc/?api-key=SECRET`) is **not** serialized verbatim.

## Honesty / safety (non-negotiable, cf tasks 015/016/017)

- The short fixture is a **raw layout-regression blob**, never a published judgment on a trader — same
  stance as the 017 long fixture. No live FLAG verdict is committed; `gallery/jupiter-live-*.json` stays
  gitignored.
- `rpc_source` **must be host-only / credential-redacted.** A DD card outlives the console; it must not
  leak the endpoint's key. `snapshot_slot` + `captured_at` make the card self-describing about **which**
  snapshot and **when** — that is the point of a point-in-time attestation.

## Files to touch

```
crates/harness/src/jupiter.rs                     withContext dual-shape parse + return slot;
                                                  fetch withContext; live_slot(slot) arg; short decode test
crates/harness/src/transcript.rs                  snapshot_slot / captured_at / rpc_source (live path);
                                                  serialize; extend provenance test
crates/harness/src/main.rs                        thread slot + captured_at (SystemTime) + host-only
                                                  rpc_source into the live capture
crates/harness/src/testdata/jupiter_gpa_short.json  NEW — real mainnet short fixture (static blob)
reviews/018-live-fixture-and-snapshot-slot.md     Codex review verdict
```

## Acceptance criteria / gates

- `cargo test -p probatio-svm-harness` green. New tests: real short decode (distinct custody);
  `parse_gpa_response` accepts **both** shapes and yields identical positions, with the context shape also
  returning the slot; the live card carries `snapshot_slot` + `captured_at` + host-only `rpc_source`; a
  key-bearing RPC URL is **not** serialized.
- **No test hits the network** — the short fixture is static; `captured_at` is injected in tests, never
  read from the clock inside a unit test.
- `cargo build -p probatio-svm-harness` clean, no new warnings; `Cargo.lock` unchanged (still **no**
  HTTP/RPC/base64/time/date crate — use `std::time` + epoch seconds; no `chrono`).
- Determinism intact: sample cards (`jupiter-neutral` / `jupiter-drift`) are byte-identical to before —
  only the `jupiter-live` backend gains provenance fields.

## Out of scope

- Multi-slot polling / time-series live trace (v1 stays a single point-in-time snapshot).
- On-chain oracle mark from the Custody account (still `--mark` / size-weighted entry).
- A dashboard/web card for a live cert (`web/`, Codex UI lane — separate task).
