# Task 022 — Attestation send (Path A): giveFeedback to the Solana Reputation Registry

**Frame:** CC-authored scaffold (exploratory: unverified external SDK + keys + network). Not a
contract-surface change. Follows task 021 (which produces the receipt + `FeedbackCall` offline).

## Why

Task 021 emits, offline, a re-exec `FeedbackCall` (`{agent, value, tag, feedback_uri}`). This task is the
thin bridge that actually writes it on-chain via the **live, permissionless** Solana Agent Registry
Reputation Registry — `giveFeedback(agentAsset, { value, tag1:'re-exec', feedbackUri })`
(program `8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`, SDK npm `8004-solana`).

## Design (safety-first)

- `attest/send.mjs` — Node ESM, **dry-run by default**: reads the `FeedbackCall` JSON, prints the exact
  planned call, and **sends nothing**; needs no SDK, no keypair, no network. Runnable immediately
  (`node attest/send.mjs --call call.json`).
- **`--send`** is the only path that submits: requires `--keypair <path>` + `--rpc` (default devnet), and
  **lazily imports** `@solana/web3.js` + `8004-solana` only then. Refuses to send if `feedback_uri` is a
  placeholder (`ipfs://pending`) — the receipt must be pinned first.
- No keys in the repo. The SDK constructor/method is annotated **"verify against the installed version
  before --send"** — the API shape is from `8004-solana` docs, not a verified install.

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
