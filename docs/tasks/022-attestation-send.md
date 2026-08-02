# Task 022 — Attestation send (Path A): giveFeedback to the Solana Reputation Registry

**Frame:** CC-authored scaffold (exploratory: unverified external SDK + keys + network). Not a
contract-surface change. Follows task 021 (which produces the receipt + `FeedbackCall` offline).

## Why

Task 021 emits, offline, a re-exec `FeedbackCall` (`{agent, value, tag, feedback_uri}`). This task is the
thin bridge that actually writes it on-chain via the **live, permissionless** Solana Agent Registry
Reputation Registry — `giveFeedback(agentAsset, { value, tag1:'re-exec', feedbackUri })`
(program `8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`, SDK npm `8004-solana`).

## Design (safety-first) — resolved to PREPARE-ONLY after Codex review 022

- `attest/send.mjs` — Node ESM, **prepare-only**: reads the `FeedbackCall` JSON, strict-validates it, and
  prints the exact planned `giveFeedback` call. **It submits nothing** — no SDK import, no keypair read,
  no network, zero runtime deps. Runnable immediately (`node attest/send.mjs --call call.json`).
- **`--send`** does *strict* validation and prints the send-ready call, then **refuses** ("live submission
  disabled, task 022b"). This removes the Codex-flagged risk of an unverified wildcard SDK mis-signing
  with a funded keypair.
- Strict arg parsing (every value-flag needs a value; duplicates/empty rejected); the call is bound to
  Task 021's shape (base58 agent, `value` 0..=100, tag `re-exec`, non-placeholder URI) — never arbitrary
  JSON. No keys in the repo.

## Task 022b — real submit: API verified + wired; runtime BLOCKED on a web3.js dep bug

**Done:** the `8004-solana@0.8.3` API was verified from its shipped types and `--send` is wired to it
(with a runtime surface guard + fail-safe import):
- `new SolanaSDK({ cluster, rpcUrl, signer: Keypair })` (`dist/core/sdk-solana.d.ts`).
- `giveFeedback(asset: PublicKey, params: GiveFeedbackParams, options?)` where `GiveFeedbackParams` has
  `value` (string|number|bigint, required), **`score?` — a direct 0–100 integer (perfect for PASS=100 /
  FLAG=0)**, `tag1?`, `feedbackUri?` (≤250 bytes). We map: `score = value = 0|100`, `tag1 = "re-exec"`,
  `feedbackUri = <pinned receipt>`.
- Versions pinned in `package.json` (`8004-solana@0.8.3`, `@solana/web3.js@1.98.4`).

**BLOCKER (not yet resolved):** the SDK will not load in this environment — `@solana/web3.js@1.98.4`
pulls `rpc-websockets` whose nested `uuid` is ESM but is `require()`d from CJS
(`require() of ES Module rpc-websockets/node_modules/uuid/dist-node/index.js not supported`). Reproduced
on Node 18/20/22; a `uuid` override did not dedupe it; `bun` not available here. So the import — and thus
any real send — cannot run/smoke-test in this environment. `--send` fails **safe** (the import is in a
try/catch → clean error, never a partial send).

**To finish (in the send environment, with Hiro):** resolve the web3.js dep load (e.g. an `overrides`/
resolution that gives `rpc-websockets` a single CJS-consistent `uuid`, a compatible `@solana/web3.js`
pin, or a runtime like `bun`/`pnpm` that resolves it), `npm install`, smoke-test `import('8004-solana')`
(no send), then run one real `--send` with a **funded devnet keypair** against a **registered agent**, and
record the on-chain signature here. `node_modules`/lockfile are intentionally not committed (the tree
above does not load).

## Flow

```
certify-jupiter --attest <asset> --feedback-uri <pinned-uri>   # (task 021) prints receipt + FeedbackCall
  → save the FeedbackCall line as call.json
  → node attest/send.mjs --call call.json                       # DRY RUN: prints the planned giveFeedback
  → node attest/send.mjs --call call.json --send --keypair kp.json [--rpc <devnet>]   # actually submits
```

## Out of scope / notes

- Automatic IPFS pinning of the receipt (bring your own pinned `feedback_uri` for now).
- ERC-8004 `validationResponse` EVM emitter (Reckn/EVM lane).
- A verified end-to-end devnet submission — that needs Hiro's funded keypair and one manual `--send`;
  do it together, then record the resulting on-chain signature here.
