# Review 022c — attestation dependency resolution

**Branch:** `task/022c-attestation-deps` (`bb2f9cd`) · **Reviewer:** Codex · **Verdict: CHANGES**

## P2 — Task 022b heading still says the now-resolved runtime is BLOCKED

`docs/tasks/022-attestation-send.md` has an updated body that accurately says the UUID resolution is
resolved and that only the deliberate, funded `--send` remains.  Its section heading, however, still
reads: `Task 022b — real submit: API verified + wired; runtime BLOCKED on a web3.js dep bug`.

That is the exact stale blocker terminology the task says must not remain.  Rename the heading to state
that the SDK load is resolved (and retain the explicit real-send warning).  This is documentation-only,
but it should be corrected so an operator does not infer the import failure remains a safety boundary.

## Verified

- The committed lockfile pins the override.  On Node `v18.16.1`, `npm ls --all` resolves
  `rpc-websockets@9.3.9` and its UUID dependency to `uuid@9.0.1 overridden`/`deduped`.
- A no-send import probe resolved `SolanaSDK` and `SolanaSDK.prototype.giveFeedback` as functions.
- Default `send.mjs --call -` remained a dry-run: it printed the planned call and did not submit,
  import the SDK, or read a keypair.  No real `--send` was attempted.
- The README correctly says the blocker is resolved and warns that `--send` is a real funded write;
  `node_modules` remains ignored and `package-lock.json` is intentionally committed to pin the working
  resolution.
