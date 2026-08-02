# Review 019 — MandateSpec (CC reviews Codex)

**Branch:** `task/019-mandate-spec` · **Reviewer:** CC · **Verdict: APPROVE** (fix commit `cbec9d7`).
Initial verdict was CHANGES (1×P1, 2×P2); all three resolved in `cbec9d7` — verified below.

## Resolution (fix commit `cbec9d7`, re-reviewed)

- **P1 resolved.** `jupiter_to_snapshots_with_mandate` now binds `let instrument = 0;` (v1 SOL) and
  checks `instrument == mandate.instrument`; the `MANDATE_INSTRUMENT` import is removed (no references
  remain in the module). The clause now tests the traded instrument against the authored spec.
- **P2 (test) resolved.** `authored_mandate_controls_jupiter_snapshot_compliance` exercises default
  (within), tightened `max_size` (out), and mismatched `instrument` (out).
- **P2 (sentinel) resolved.** `spec_hash` replaces the `[0;32]` return with `debug_assert!` + `unreachable!`.
- Gates: `cargo test` (85) green, `cargo build` + `build-sbf` clean. No new issues. **Ready to merge.**

---

## Original review (verdict CHANGES)

One clean fix and the branch is an APPROVE. The core change — promoting the mandate to an authored,
hashable `MandateSpec` and threading it through `within_mandate`/`check_position`/verifier capture,
defaulting to `stage0_default()` — is correct, behaviour-preserving, and well-tested (84 green,
`build-sbf` clean, determinism guarded by `default_mandate_preserves_episode_trace`).

## P1 — jupiter mandate check is written against the global constant, not the authored spec

`crates/harness/src/jupiter.rs` (`jupiter_to_snapshots_with_mandate`):

```rust
within_mandate: measured_delta.abs() <= mandate.max_size
    && mandate.instrument == MANDATE_INSTRUMENT,   // <-- spec compared to the global constant
```

Two problems, both defeating the point of the task on the **live certification surface**
(`certify-jupiter --live`, the DD attestation path from tasks 017/018):

1. **Leaky/incorrect basis.** The mandate-instrument test must compare *the traded instrument* to
   `mandate.instrument`, mirroring `Position::within_mandate` (`self.instrument == spec.instrument`).
   The snapshot's instrument is hardcoded `0` two lines up. The correct clause is
   `instrument_of_snapshot == mandate.instrument` (i.e. `0 == mandate.instrument`). As written,
   `mandate.instrument == MANDATE_INSTRUMENT` is behaviour-equivalent **only by the coincidence** that
   both the snapshot instrument and `MANDATE_INSTRUMENT` are `0`. The moment the jupiter path carries a
   non-zero instrument (the documented multi-token future) or a spec authors `instrument != 0`, the
   check no longer tests what it claims to.
2. **Reintroduces the constant the task removes.** This path is supposed to read the *authored spec*;
   importing and comparing to `MANDATE_INSTRUMENT` re-couples it to the global. (The original code had
   **no** instrument clause at all — `measured_delta.abs() <= MAX_MANDATE_SIZE` — so this also adds
   unrequested logic.)

**Fix:** bind the snapshot instrument to a local and compare it to the spec —
`&& instrument == mandate.instrument` — and drop the `MANDATE_INSTRUMENT` import from this module.
(Equivalently, if the intent is "size-only on the jupiter path for v1", drop the instrument clause
entirely to match the original; but threading the spec's instrument is the more useful choice.)

Severity note (honest): this is **behaviour-equivalent today** — no current test fails. It is P1 for
*abstraction correctness on the product's live path*, not a live regression. If you'd rather ship now
and correct it in the Step-2 (Custos `MandateConformance`) task, it is safe to defer — your call.

## P2 — the one new public API on the live path is untested

`jupiter_to_snapshots_with_mandate` is the authored-spec entry point for the live/certification surface,
and nothing exercises it. This is exactly why the P1 slipped. Add a test: a jupiter trace that passes
under `stage0_default()` but flags `MandateDeviation` (via `within_mandate == false`) under a tightened
`max_size`, and — once P1 is fixed — under a non-matching `instrument`.

## P2 — `spec_hash` error sentinel collides with a real digest

`crates/harness/src/mandate.rs` returns `[0; 32]` on `encode()` failure. `encode` into a fixed
`[0u8; LEN]` buffer is unreachable-failure, so this is fine in practice, but `[0;32]` is also a value a
real FNV digest could (astronomically) take. Prefer `debug_assert!(spec.encode(&mut encoded).is_ok())`
or a comment stating the branch is unreachable, so the sentinel isn't mistaken for a valid tag.

## Notes (not findings)

- `run_episode` now calls `run_episode_with_mandate(policy, stage0_default())` directly instead of
  `run_episode_with_backend(policy, Backend::Ref)`. Both converge on `run_episode_ref_with_mandate(_,
  stage0)`, so traces are identical (guarded by the new determinism test) — no lost behaviour, though
  `run_episode` no longer exercises the `Backend::Ref` dispatch arm. Acceptable.
- `MandateSpec` encode/decode, `stage0_default`, and the guard/perp `stage0_default()` threading (no new
  account) all match the brief. Contract crate stays `#![no_std]`/dep-free; FNV-1a lives in the harness
  with the correct `TODO(reexec-core)` keccak note. Good.

## Gate check

`cargo test` (84) green, `cargo build` clean, `build-sbf` clean per the implementer's report; determinism
test added; no network/clock in tests. Gates hold. Fix P1 (+ P2 test) and this merges.
