# attest/ — Path A on-chain attestation sender

Bridges the offline receipt from `certify-jupiter --attest` (task 021) to the **live, permissionless**
Solana Agent Registry **Reputation Registry** (`giveFeedback`). **Dry-run by default — it sends nothing
and needs no SDK, keypair, or network.**

## Flow

```bash
# 1. Produce the receipt + FeedbackCall (offline, Rust), pointing at where you'll pin the receipt:
cargo run -p probatio-svm-harness -- certify-jupiter --attest <AGENT_ASSET_BASE58> \
  --feedback-uri ipfs://<PINNED_RECEIPT_CID> --sample
#    → prints the receipt JSON, then the FeedbackCall JSON. Save the FeedbackCall line to call.json.

# 2. DRY RUN (default): see exactly what would be submitted — no network, no keys:
node attest/send.mjs --call call.json
#    (or pipe: `... | node attest/send.mjs --call -`)

# 3. SUBMIT (only when you mean it): needs a funded devnet keypair + the deps installed:
cd attest && npm install
node send.mjs --call ../call.json --feedback-uri ipfs://<PINNED_RECEIPT_CID> \
  --send --keypair <PATH_TO_KEYPAIR_JSON> --rpc https://api.devnet.solana.com
```

## Safety / honesty

- **Dry-run is the default.** `--send` is the only path that touches the network or a key.
- **No keys in the repo.** Pass `--keypair`; it is read at run time only.
- **Pin first.** `--send` refuses a placeholder `feedback_uri` (`ipfs://pending`) — the receipt content
  must be pinned (IPFS/HTTPS) so third parties can fetch and **re-run** the verdict.
- **Verify the SDK.** The `8004-solana` `giveFeedback` shape here is from the package docs, not a
  verified install; confirm the constructor/method against the version you `npm install` before trusting
  a real send. Program (mainnet): `8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`.
- The Validation Registry is archived; this uses the **Reputation** path (Path A). When Validation ships,
  swap `giveFeedback` for the `validationResponse`-shaped call (same `value`/`uri` semantics).
