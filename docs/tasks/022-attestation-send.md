# Task 022 — Attestation send (Path A): giveFeedback to the Solana Reputation Registry

**Frame:** CC-authored scaffold (exploratory: unverified external SDK + keys + network). Not a
contract-surface change. Follows task 021 (which produces the receipt + `FeedbackCall` offline).

## Why

Task 021 emits, offline, a re-exec `FeedbackCall` (`{agent, value, tag, feedback_uri}`). This task is the
thin bridge that actually writes it on-chain via the **live, permissionless** Solana Agent Registry
Reputation Registry — `giveFeedback(agentAsset, { value, tag1:'re-exec', feedbackUri })`
(program `8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`, SDK npm `8004-solana`).

## Design (safety-first) — two modes

- **Default (no `--send`) — DRY RUN:** `attest/send.mjs` reads the `FeedbackCall` JSON, strict-validates
  it, and prints the exact planned `giveFeedback` call. It submits nothing, imports no SDK, reads no
  keypair, makes no network call — zero runtime deps. Runnable immediately (`node attest/send.mjs --call
  call.json`).
- **`--send` — REAL ON-CHAIN SUBMISSION** (not a no-op): with the SDK installed and a funded keypair it
  signs and submits `giveFeedback`. Gated by a required `--keypair`, a non-placeholder `feedback_uri`,
  strict `FeedbackCall` validation (base58 agent decoding to 32 bytes, `value` 0..=100, tag `re-exec`), a
  runtime guard rejecting an unexpected `8004-solana` surface, and a try/catch import that fails safe
  (clean error, never a partial send). Gated, **not disabled** — treat every `--send` as spending the key.
- Strict arg parsing: every value-flag needs a following value; duplicates and empty `--feedback-uri` are
  errors. No keys in the repo; `node_modules` is gitignored and `package-lock.json` is committed (it pins
  the `uuid@9.0.1` override that fixes the SDK load). (History: `--send` was a refuse-only stub after
  review 022 while the SDK was unverified; task 022b verified `8004-solana@0.8.3` and wired the real
  submit; task 022c resolved the SDK load — see below.)

## Task 022b/022c — real submit: API verified + wired; SDK load resolved; only the funded send remains

**Done:** the `8004-solana@0.8.3` API was verified from its shipped types and `--send` is wired to it
(with a runtime surface guard + fail-safe import):
- `new SolanaSDK({ cluster, rpcUrl, signer: Keypair })` (`dist/core/sdk-solana.d.ts`).
- `giveFeedback(asset: PublicKey, params: GiveFeedbackParams, options?)` where `GiveFeedbackParams` has
  `value` (string|number|bigint, required), **`score?` — a direct 0–100 integer (perfect for PASS=100 /
  FLAG=0)**, `tag1?`, `feedbackUri?` (≤250 bytes). We map: `score = value = 0|100`, `tag1 = "re-exec"`,
  `feedbackUri = <pinned receipt>`.
- Versions pinned in `package.json` (`8004-solana@0.8.3`, `@solana/web3.js@1.98.4`).

**SDK load — RESOLVED.** `@solana/web3.js@1.98.4` pulled `rpc-websockets@9.3.9`, which nests `uuid@14.0.1`
(ESM) and `require()`s it from CJS → import crash (`require() of ES Module …/uuid/dist-node/index.js not
supported`). Root cause: the nested `uuid` was ESM-only. Fix: `"overrides": { "uuid": "9.0.1" }` (a
CJS-consistent `uuid`, deduped across the whole tree), committed with `package-lock.json`. Verified on
Node 18: `import('8004-solana')` loads and `SolanaSDK`/`giveFeedback` resolve as functions — no send.
`--send` also still fails safe (import in try/catch; surface guard; placeholder/keypair gates).

## Task 022d — DONE on-chain (Solana devnet)

Path A is proven end-to-end on devnet (`attest/register.mjs` to create a target agent, then `send.mjs
--send`):

- **Registered agent** (asset): `A3DrRkqJoismmVutuCiWvNcCsviZTwemzecTgsAAjTYX`
  ([register tx](https://explorer.solana.com/tx/UJXoWFqnHwquGZNs9seqcudJcrKD1XTmjFFNQzXSwtP4uEJJueVfmtTFhBURynAYyft5yBwpDkK4ZJLDhbTvjkw?cluster=devnet), owner `AmSYugrt…`).
- **giveFeedback attestation** (score=100 / PASS, tag `re-exec`), by an **independent validator** keypair
  `8TNv14eKhoJctkhpo5aAd7t5ySqqBaJXSfFTyYsNArBT` (≠ the agent owner — the registry enforces
  `SelfFeedbackNotAllowed`, which we hit first with the owner key, confirming the rule): **Finalized**,
  tx `5CQsPC2HGNz8yK8LGDZquYcbhxHDh7kyNJ8j7KYEGmPRZkrqhVkSvc7fJgbVKmja86NQ6uPubnokh9fDeBk97Lqn`
  ([explorer](https://explorer.solana.com/tx/5CQsPC2HGNz8yK8LGDZquYcbhxHDh7kyNJ8j7KYEGmPRZkrqhVkSvc7fJgbVKmja86NQ6uPubnokh9fDeBk97Lqn?cluster=devnet)).

Finding: the attester must differ from the agent owner (self-feedback is rejected on-chain) — so
Probatio's validator identity is naturally a separate keypair from whoever owns the agent. `send.mjs` was
hardened to treat an empty/missing signature as a failure (never print "submitted" on a rejected tx).
The receipt `feedback_uri` here is an unpinned placeholder URL; a production attestation should pin the
`receipt_json` (task 021 output) at that URI so third parties can fetch and re-run the verdict.
`node_modules` stays gitignored; `package-lock.json` IS committed (pins the working uuid override).

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
- (Done in task 022d: the verified end-to-end devnet submission — see the "DONE on-chain" section above.)
- Mainnet submission + pinning the real `receipt_json` at `feedback_uri` (devnet used a placeholder URL).
