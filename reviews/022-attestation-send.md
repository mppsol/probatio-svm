# Review 022 — Path A attestation sender

**Branch:** `task/022-attestation-send` (`2782252`) · **Reviewer:** Codex · **Verdict: CHANGES**

The dry-run boundary is good: the default module import is only `node:fs`; it does not read a keypair,
import either SDK, or make an RPC call.  `--send` also checks both a keypair and the exact
`ipfs://pending` placeholder before its lazy imports.  The following live-path issues need correction
before a real submission is safe.

## P1 — Missing option values can silently fall back to a sendable call

`parseArgs()` consumes values with `argv[++i]` without checking that one exists.  In particular,
`--feedback-uri` at the end of the command assigns `undefined`; later
`args.feedbackUri ?? call.feedback_uri` silently uses the URI embedded in the call file.  I reproduced
this in dry-run with `node attest/send.mjs --call - --feedback-uri`: it printed a valid plan using the
stdin call's URI rather than rejecting the malformed command.  The same issue applies to `--rpc` (and
every value-taking flag), so a malformed `--send` invocation can use an unintended URI/RPC.

Require a following, non-flag value for every value-taking option, reject duplicates, and fail before
reading the call/key or importing SDKs when parsing is malformed.  Treat an explicitly supplied empty URI
as invalid rather than as a fallback.

## P1 — Wildcard, unverified SDK can still silently change the live call semantics

`attest/package.json` declares `"8004-solana": "*"`, then `--send` invokes the documented-but-unverified
constructor and `giveFeedback` shape after checking only that a constructor export exists.  A future SDK
can retain a callable `giveFeedback` method while changing argument meaning; this code would submit it,
not fail safely.  The source comment asking the operator to verify it is not an executable guard.

Pin an audited SDK version (and commit its lockfile), verify the exact export/method contract in a
non-sending fixture, and make the sender reject an unexpected SDK surface before constructing a
transaction.  Until then the script should remain dry-run-only rather than expose a live `--send` path.

## P2 — Validate the FeedbackCall before a live submission

The sender only checks truthiness of `call.agent` and stringifies `call.value`; it accepts out-of-range,
non-integer values and arbitrary tags.  Before `--send`, require a valid base58 public key, integer
`value` in `0..=100`, exact `tag === "re-exec"`, and a non-placeholder URI.  This keeps the live bridge
bound to Task 021's `FeedbackCall` instead of signing arbitrary JSON.

`--receipt` is also parsed but never used; remove it or implement it so callers do not believe it affects
the receipt bound to a send.

## Validation performed

- `node attest/send.mjs --call -` with a piped FeedbackCall: dry-run only; no SDK/key/network access.
- Missing `--call` reports usage.
- `node attest/send.mjs --call - --feedback-uri` reproduced the P1 fallback above.
- Node ESM syntax is compatible with Node 18 (built-in `node:fs`, top-level `await`, optional chaining).
