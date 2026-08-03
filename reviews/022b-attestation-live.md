# Re-review 022b — live attestation sender

**Branch:** `task/022b-attestation-live` (`38fbe5c`, fix `cd053d3`) · **Reviewer:** Codex · **Verdict: APPROVE**

The P1 stale-documentation finding is resolved.  `attest/send.mjs` now labels its two modes accurately:
the default is a dependency-free dry-run, while `--send` is a real funded, on-chain `giveFeedback`
submission.  The README and Task 022 brief say the same thing, including an explicit warning not to
treat the current dependency-load blocker as a safety boundary.  No remaining operator-facing document
describes `--send` as prepare-only, refusing, disabled, or a no-op.

Safety gates remain intact:

- Default invocation exits before SDK import, keypair read, RPC activity, or submission.  A dry-run
  printed its plan only.
- `--send` still requires a keypair and a non-placeholder URI, then applies the exact-32-byte base58
  agent check, integer `value` in `0..=100`, and `re-exec` tag check before any imports.
- The dynamic SDK imports are caught; the runtime `SolanaSDK.prototype.giveFeedback` surface guard runs
  before parsing the keypair or constructing the sender.  A placeholder URI was rejected before that
  path.
- No key material, `node_modules`, or lockfile is tracked; `attest/.gitignore` covers `node_modules/`.

The documented `@solana/web3.js`/`rpc-websockets` import blocker is still a clean, fail-safe runtime
failure here, not an alternative execution path.  No real send was attempted during this review.
