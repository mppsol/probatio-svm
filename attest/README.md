# attest/ — Path A attestation sender

Takes the offline `FeedbackCall` from `certify-jupiter --attest` (task 021) and writes the re-exec
verdict to the **live, permissionless** Solana Agent Registry **Reputation Registry** via `giveFeedback`.

Two modes:

- **Default (no `--send`) — DRY RUN:** prints the exact planned `giveFeedback` and validates it. Imports
  no SDK, reads no keypair, makes no network call. Zero runtime deps; runs on Node 18 as-is.
- **`--send` — REAL ON-CHAIN SUBMISSION.** This is **not** a no-op: with the SDK deps installed and a
  funded keypair it signs and submits a transaction. Gated, but a live, funded action — use deliberately.

## Flow

```bash
# 1. Produce the receipt + FeedbackCall (offline, Rust), pointing at where you'll pin the receipt:
cargo run -p probatio-svm-harness -- certify-jupiter --attest <AGENT_ASSET_BASE58> \
  --feedback-uri ipfs://<PINNED_RECEIPT_CID> --sample
#    → prints the receipt JSON, then the FeedbackCall JSON. Save the FeedbackCall line to call.json.

# 2. DRY RUN — inspect + validate the planned giveFeedback (no deps, no keys, no network):
node attest/send.mjs --call call.json
#    (or pipe: `... | node attest/send.mjs --call -`)

# 3. REAL SUBMIT — signs and sends on-chain (needs deps + a FUNDED keypair):
cd attest && npm install
node send.mjs --call ../call.json --feedback-uri ipfs://<CID> --send --keypair <PATH> --rpc <DEVNET_RPC>
```

## Safety

- **Only `--send` touches a key or the network.** The default path sends nothing and imports nothing.
- **`--send` is a real, funded on-chain write** — gated by: a required `--keypair`, a non-placeholder
  `feedback_uri`, strict `FeedbackCall` validation (base58 agent that decodes to **32 bytes**, `value`
  `0..=100`, tag exactly `"re-exec"`), a runtime guard that rejects an unexpected `8004-solana` surface,
  and a try/catch import so a broken/absent SDK errors cleanly instead of partial-sending. It is gated,
  **not disabled** — treat every `--send` as spending from the keypair.
- **Strict args:** every value-taking flag requires a following value; duplicates and an empty
  `--feedback-uri` are errors — a malformed command fails instead of silently falling back.
- **No keys in the repo;** `node_modules`/lockfile are not committed. Program (mainnet):
  `8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`.

## `--send` status (task 022b)

`--send` is wired to the verified `8004-solana@0.8.3` API:
`new SolanaSDK({ cluster, rpcUrl, signer }).giveFeedback(assetPubkey, { value, score, tag1, feedbackUri })`,
mapping PASS=100/FLAG=0 to the SDK's direct 0–100 `score`. Versions are pinned in `package.json`.

**Known blocker:** the SDK does not load in the tested environments (Node 18/20/22) because
`@solana/web3.js@1.98.4` pulls a `rpc-websockets` whose nested `uuid` (ESM) is `require()`d from CJS, so
`--send` currently errors at import here. **Do not treat that as a safety net** — the code path is a real
send. To finish: resolve the dep load (an `overrides`/resolution giving one CJS-consistent `uuid`, a
compatible web3.js pin, or `bun`/`pnpm`), `npm install`, smoke-test `import('8004-solana')` (no send),
then run one `--send` with a **funded devnet keypair** against a **registered agent** and record the
signature.

The Validation Registry is archived, so this stays on the **Reputation** path; when it ships, swap to the
`validationResponse`-shaped call (same `value`/`uri` semantics).
