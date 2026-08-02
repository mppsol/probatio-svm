# attest/ — Path A attestation preparer

Takes the offline `FeedbackCall` from `certify-jupiter --attest` (task 021) and prepares the
`giveFeedback` call for the **live, permissionless** Solana Agent Registry **Reputation Registry**.

**Prepare-only. It does NOT submit** — no SDK import, no keypair read, no network. Enabling the real
on-chain submit is **task 022b**, done only after pinning + verifying the `8004-solana` SDK against a
fixed version (Codex review 022). Zero runtime dependencies; runs on Node 18 as-is.

## Flow

```bash
# 1. Produce the receipt + FeedbackCall (offline, Rust), pointing at where you'll pin the receipt:
cargo run -p probatio-svm-harness -- certify-jupiter --attest <AGENT_ASSET_BASE58> \
  --feedback-uri ipfs://<PINNED_RECEIPT_CID> --sample
#    → prints the receipt JSON, then the FeedbackCall JSON. Save the FeedbackCall line to call.json.

# 2. Prepare + inspect the exact planned giveFeedback (no network, no keys):
node attest/send.mjs --call call.json
#    (or pipe: `... | node attest/send.mjs --call -`)

# 3. Strict-validate the call as send-ready (still submits nothing; live path disabled until 022b):
node attest/send.mjs --call call.json --feedback-uri ipfs://<CID> --send --keypair <PATH>
```

## Safety / honesty

- **No live send here.** `--send` strict-validates and prints the ready call, then refuses — nothing is
  submitted. This removes the risk of an unverified SDK mis-signing with a funded keypair.
- **Bound to Task 021's `FeedbackCall`:** rejects a bad base58 agent, a `value` outside `0..=100`, a tag
  other than `"re-exec"`, and a placeholder `feedback_uri` — never signs arbitrary JSON.
- **Strict args:** every value-taking flag requires a following value; duplicates and empty `--feedback-uri`
  are errors — a malformed command fails instead of silently falling back.
- **No keys in the repo.** Program (mainnet): `8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`.

## `--send` status (task 022b)

`--send` is **wired to the verified `8004-solana@0.8.3` API** and fails safe:
`new SolanaSDK({ cluster, rpcUrl, signer }).giveFeedback(assetPubkey, { value, score, tag1, feedbackUri })`,
mapping PASS=100/FLAG=0 to the SDK's direct 0–100 `score`. A runtime guard rejects an unexpected SDK
surface, and the import is in a try/catch so a broken/absent SDK errors cleanly — never a partial send.

**Known blocker:** the SDK does not load in the tested environments (Node 18/20/22) because
`@solana/web3.js@1.98.4` pulls a `rpc-websockets` whose nested `uuid` (ESM) is `require()`d from CJS. So
the real send is not yet runnable here. To finish: resolve that dep load (an `overrides`/resolution
giving one CJS-consistent `uuid`, a compatible web3.js pin, or `bun`/`pnpm`), `npm install`, smoke-test
`import('8004-solana')`, then run one `--send` with a **funded devnet keypair** against a **registered
agent** and record the signature. `node_modules`/lockfile are not committed (the tree does not load).

The Validation Registry is archived, so this stays on the **Reputation** path; when it ships, swap to the
`validationResponse`-shaped call (same `value`/`uri` semantics).
