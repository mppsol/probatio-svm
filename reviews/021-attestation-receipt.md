# Review 021 — Attestation receipt (CC reviews Codex)

**Branch:** `task/021-attestation-receipt` (`5dafbbf`) · **Reviewer:** CC · **Verdict: APPROVE** (no P0/P1;
one P2 nit). Independently verified in-tree.

## Correctness audit

- **Verdict mapping** `Pass→100 / ShortcutDetected→0`, `tag="re-exec"`; receipt carries `schema`,
  `attester`, `agent` (base58 via `Address`), `mandate_spec_hash` (hex of `spec_hash`), `reproduce`
  (policy/backend/n_slots — enough to re-run), `verdict`, `findings` (kind+slots), `report_hash`,
  `captured_at`. Matches the brief and the ERC-8004 `value`/`responseURI` shape (cross-registry).
- **`report_hash`** = FNV-1a over `report.to_json()`, zero-extended, with the same `TODO(reexec-core)`
  keccak note as `spec_hash` — labeled an identity tag, not a security hash. Correct.
- **Receipt ≠ call separation is right:** `feedback_uri` is in the `FeedbackCall` (where the receipt will
  be pinned), *not* inside `receipt_json` (which is the pinned content) — no self-reference. Good.
- **JSON is well-formed:** `json_escape` handles quotes/backslash/control/`\u`; hex + base58 + fixed enum
  strings need no escaping; `finding.detail` is deliberately omitted from the receipt (avoids escaping a
  free-text field). The PASS receipt is round-tripped through `serde_json` in tests; the live CLI emits
  valid canonical JSON for both PASS and FLAG.
- **Offline & deterministic (the load-bearing safety property):** `attest()` is pure; `captured_at` is a
  parameter. CLI `parse_attest_args` validates the base58 asset via `Address::from_str`, defaults
  `captured_at` to `0` (**never reads the clock**), and **refuses `--attest` with `--live`** (exit 2) so
  the attest path can never touch the network. Determinism test asserts byte-identical receipts.

## Independent verification (CC ran these)

- `cargo test --offline`: **91 passed** (harness 77 lib incl. the 4 new `attest` tests; contract 7;
  reexec-spec 3; main 2; perp/guard 1) — 0 failed. `cargo build` clean, **no warnings**; `serde_json`
  used only in tests, no new runtime dep in `Cargo.toml`; contract crate untouched.
- `certify-jupiter --attest <asset> --feedback-uri ipfs://demo --captured-at 1700000000 --sample`:
  prints a valid `probatio.attestation.v1` receipt + `giveFeedback` call — PASS card ⇒ `value:100`,
  drift card ⇒ `verdict:"FLAG"`, `value:0`, findings serialized. Sample gallery cards regenerated
  **byte-identical** (determinism intact).

## P2 (nit, non-blocking)

- Only the PASS receipt is parse-validated with `serde_json` in tests; add the same
  `serde_json::from_str` round-trip to the FLAG test so the findings-array serialization is guaranteed
  well-formed by a test, not just by the CLI eyeball. Low risk (CLI output is valid JSON today).

**Ready to merge.** Path A core is in place: a deterministic, offline, re-runnable attestation receipt +
`giveFeedback` call args. Task 022 (the thin `8004-solana` TS send + `--live`) turns this into an on-chain
attestation.
