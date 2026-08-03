# Re-review 022c — attestation dependency resolution

**Branch:** `task/022c-attestation-deps` (`bb2f9cd`, fixes `6925dd1`, `bd8e4b0`) · **Reviewer:** Codex · **Verdict: CHANGES**

## P2 — Task 022 still says the committed lockfile is not committed

`attest/README.md` is now correct: `node_modules` is ignored and `package-lock.json` is committed to
pin the `uuid@9.0.1` override.  But the Design section in
`docs/tasks/022-attestation-send.md` still says "node_modules/lockfile not committed."  That remains
a contradictory operator-facing statement about the resolution's reproducibility.  Make it match the
README: `node_modules` gitignored; `package-lock.json` committed and pins the working resolution.

## Verified

- The stale `runtime BLOCKED` heading is gone.  The source header, README, and Task 022 brief all
  correctly describe `--send` as a real funded on-chain write and default invocation as a no-send
  dry-run.
- `import('8004-solana')` succeeded on Node 18; `SolanaSDK` and
  `SolanaSDK.prototype.giveFeedback` resolved as functions.
- `npm ls --all` shows `rpc-websockets@9.3.9` using `uuid@9.0.1 overridden`/`deduped`; the committed
  version-3 lockfile pins `uuid@9.0.1`, `@solana/web3.js@1.98.4`, and `8004-solana@0.8.3`.
- Default `send.mjs --call -` printed only its dry-run plan.  No real `--send` was attempted.  Existing
  keypair, placeholder-URI, strict FeedbackCall, fail-safe import, and runtime-SDK-surface gates remain.
- No key material or `node_modules` is tracked.
