# Review 020 — Generic `MandateSpec` / `reexec-spec`

**Branch:** `task/020-generic-mandate-spec` (`33be9b2`, `8f621f2`) · **Reviewer:** Codex · **Verdict: APPROVE**

No P0/P1/P2 findings.

## Contract-surface audit

- `reexec-spec` is a workspace-local, dependency-free `#![no_std]` crate.  `probatio-contract`
  re-exports the moved `MandateSpec`, `MAX_MANDATE_SIZE`, and `MANDATE_INSTRUMENT`; no duplicate
  mandate type/constants remain in the contract crate, while its account-codec helpers remain live.
- The canonical preimage is exactly 17 bytes: signed little-endian `max_size` at offset 0,
  `instrument` at offset 8, and little-endian `max_value_out` at offset 9.  The owning-crate test
  checks those bytes and round-trips them; the harness FNV preimage documentation and sensitivity
  test cover all three fields.
- `max_value_out` is inert on certify: `Position::within_mandate` and `check_position` still read
  only `max_size`/`instrument`; no verifier, perp, or guard reader consumes the new field.
  `stage0_default()` supplies `u64::MAX`, preserving the existing certify envelope.
- The reference/SVM episode capture and Jupiter default wrapper still use `stage0_default()`.
  Since snapshots/transcripts do not serialize the new field and their size/instrument inputs are
  unchanged, the deterministic episode traces and `jupiter-neutral`/`jupiter-drift` card content
  remain unchanged.

## Validation

- `git diff --check master...HEAD`
- `cargo test` — 85 tests passed
- `cargo build`
- `cargo build-sbf --manifest-path programs/perp/Cargo.toml --features bpf-entrypoint`
- `cargo build-sbf --manifest-path programs/guard/Cargo.toml --features bpf-entrypoint`
