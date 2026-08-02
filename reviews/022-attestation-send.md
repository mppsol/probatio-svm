# Re-review 022 — Path A attestation preparer

**Branch:** `task/022-attestation-send` (`2782252`, fix `9fa891f`) · **Reviewer:** Codex · **Verdict: CHANGES**

The two P1 findings are resolved:

- Value-taking options now require a following non-flag argument and reject duplicates.  A missing
  `--feedback-uri` now fails before call parsing rather than falling back to the JSON URI.
- The live sender, SDK dependencies/imports, keypair read, and every network path are removed.
  `--send` only validates/prepares and explicitly refuses to submit, so even a supplied keypair path is
  never opened.  This is Node 18-compatible and uses only `node:fs` to read the declared call input.

## P2 — `isBase58Pubkey` does not actually validate a 32-byte Solana public key

The validator checks only base58 alphabet and a 32–44 character length.  It accepts strings that do not
decode to 32 bytes.  For example, a 44-character string of `z` characters is too large for a 32-byte
Solana public key, but this command reports `VALIDATED & READY`:

```sh
printf '%s\n' '{"agent":"zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz","value":100,"tag":"re-exec","feedback_uri":"ipfs://pinned"}' \
  | node attest/send.mjs --call - --send --keypair does-not-exist.json
```

No transaction is possible in this task, but the tool's claimed strict binding to Task 021's
`agent_asset: [u8; 32]` is false and would carry directly into 022b.  Decode base58 and require exactly
32 output bytes (without adding an SDK/key/network dependency) before labeling a call send-ready.  Also
reject a duplicate `--send` flag if the documented duplicate-rejection guarantee is intended to cover all
flags, not just value-taking flags.

## Validation performed

- Dry-run with a valid piped FeedbackCall: prints a plan only; no SDK/key/network access.
- Missing `--feedback-uri` value: errors; no fallback.
- Duplicate `--call`: errors.
- Source audit confirms no SDK imports, keypair reads, `fetch`, RPC client, or submit call remains.
