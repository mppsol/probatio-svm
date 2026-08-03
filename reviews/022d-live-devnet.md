# Review 022d — live devnet attestation

**Branch:** `task/022d-live-devnet` (`805ea28`) · **Reviewer:** Codex · **Verdict: CHANGES**

## P2 — Task 022's Out-of-scope list still says the completed devnet submission is pending

`docs/tasks/022-attestation-send.md` now has a correct Task 022d section recording the registration and
finalized `giveFeedback` transaction.  Its final Out-of-scope bullet still says a verified end-to-end
devnet submission "needs Hiro's funded keypair and one manual `--send`" and asks to record the signature.
That is the stale pre-022d state and contradicts the new on-chain record.  Remove it or replace it with a
future/prod follow-up; otherwise the task brief simultaneously says the devnet proof is both done and
pending.

## Verified

- `send.mjs` now obtains a signature from either SDK result shape and calls `die(..., 1)` when it is empty
  or missing, before any `submitted giveFeedback` output.  The default path still exits before imports,
  key reads, RPC use, or submission; a dry-run printed its plan only.
- The existing `--send` gates remain: keypair, non-placeholder URI, exact-32-byte base58 agent, integer
  `value` in `0..=100`, `re-exec` tag, caught dynamic import, and the runtime SDK-surface guard.
- `attest/register.mjs` is clearly marked as a one-off **devnet** writer, imports no secret from source,
  prints only public signer/asset/signature data, and has no committed key material.  It is separate from
  the safe-by-default sender and intentionally performs registration when invoked.
- Read-only devnet RPC checks confirm both documented transactions finalized successfully: registration
  created agent `A3DrRkqJoismmVutuCiWvNcCsviZTwemzecTgsAAjTYX` with owner
  `AmSYugrtHAEZi3TDj3HP7qbjY1hw6uv1df1oFDMxKeb1`; the feedback transaction succeeded with independent
  client `8TNv14eKhoJctkhpo5aAd7t5ySqqBaJXSfFTyYsNArBT`, `score=100`, and tag `re-exec`.
- README and the new Task 022d section accurately state the self-feedback restriction, devnet proof,
  and the need for a production receipt URI.  No keys or `node_modules` are tracked; the lockfile is
  committed as intended.
