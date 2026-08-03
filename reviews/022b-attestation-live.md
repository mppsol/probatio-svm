# Review 022b — live attestation sender

**Branch:** `task/022b-attestation-live` (`38fbe5c`) · **Reviewer:** Codex · **Verdict: CHANGES**

## P1 — The operator documentation still says `--send` cannot submit

Task 022b changes `attest/send.mjs` so `--send` imports the SDK, reads the supplied keypair, and invokes
`sdk.giveFeedback(...)`.  But the README's opening, Flow step 3, and **Safety / honesty** section still
say this is prepare-only, that there is no SDK/keypair/network access, and that `--send` only prints a
call then refuses.  The Task 022 brief's original Design section says the same thing.  These claims now
contradict the executable path; only the later 022b status sections describe the runtime blocker.

This is a security/operationally material false assurance: once the documented dependency blocker is
resolved, an operator following those earlier sections could provide a funded keypair believing no
transaction can be submitted.  Update or remove every prepare-only/refusal claim, make the default
dry-run guarantee explicitly conditional on omitting `--send`, and retain the blocker and manual-devnet
completion guidance in one unambiguous place.

## Verified in this review

- The default path exits before either dynamic import, keypair read, RPC operation, or submission; it
  printed only the planned call in a dry-run.
- On `--send`, validation occurs before imports: base58 decodes to exactly 32 bytes, value is an integer
  in `0..=100`, tag is `re-exec`, the URI cannot be the placeholder, and `--keypair` is required.  An
  invalid value was rejected before the dependency-import path.
- With a valid call and an intentionally nonexistent keypair path, the absent/broken SDK dependency was
  caught by the import `try`/`catch`; the key path was not opened and no transaction was attempted.
- The pinned `8004-solana@0.8.3` public API documents `SolanaSDK` construction with `cluster`, `rpcUrl`,
  and `signer`, and `giveFeedback(targetAgent, { value, score, tag1, feedbackUri })`; the direct 0--100
  score mapping is consistent with that surface.
- The runtime `SolanaSDK.prototype.giveFeedback` guard is present before key parsing and submit.  No
  `node_modules`, lockfile, or key material is tracked; `attest/.gitignore` covers `node_modules/`.

The SDK import blocker is documented honestly in the new 022b sections, including a no-send import
smoke test and a funded-devnet manual completion path.  Resolve P1's contradictory older text before
this branch is treated as safe to operate.
