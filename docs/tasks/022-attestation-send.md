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

## Task 022b — enable the real submit (deferred)

Pin an audited `8004-solana` version + commit its lockfile; verify the exact `giveFeedback`
export/signature in a non-sending fixture; reject an unexpected SDK surface before building a tx; then
wire the lazy `@solana/web3.js` + SDK submit behind `--send`. Do the first real devnet `--send` together
with Hiro's funded keypair, and record the resulting on-chain signature.

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
