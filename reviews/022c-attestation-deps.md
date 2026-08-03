# Re-review 022c — attestation dependency resolution

**Branch:** `task/022c-attestation-deps` (`bb2f9cd`, fix `6925dd1`) · **Reviewer:** Codex · **Verdict: CHANGES**

## P2 — README incorrectly says the lockfile is not committed

The stale `runtime BLOCKED` heading is fixed, and the Task 022 brief now accurately says that SDK loading
is resolved and only the deliberate, funded `--send` remains.  However, `attest/README.md` Safety still
says "node_modules/lockfile are not committed."  `attest/package-lock.json` is now deliberately
committed to pin `uuid@9.0.1`, and the README's later resolved-SDK section says so.  Correct the Safety
bullet to distinguish the ignored `node_modules` from the committed lockfile.  Otherwise an operator can
mistake the reproducible, pinned dependency resolution for an uncommitted local state.

## Verified

- No remaining heading or current operator instruction in `attest/README.md`, `attest/send.mjs`, or
  `docs/tasks/022-attestation-send.md` says SDK loading is blocked, `--send` does not submit, or that it
  is a no-op.  The source header and both documents clearly label `--send` a real funded on-chain write.
- `import('8004-solana')` succeeded on Node 18; `SolanaSDK` and
  `SolanaSDK.prototype.giveFeedback` resolved as functions.
- `npm ls --all` shows `rpc-websockets@9.3.9` using `uuid@9.0.1 overridden`/`deduped`; the committed
  lockfile is version 3 and pins `uuid@9.0.1`, `@solana/web3.js@1.98.4`, and `8004-solana@0.8.3`.
- Default `send.mjs --call -` printed only its dry-run plan.  No real `--send` was attempted.  Existing
  keypair, placeholder-URI, strict FeedbackCall, fail-safe import, and runtime-SDK-surface gates remain.
- No key material or `node_modules` is tracked.
