APPROVE

Reviewer: CC. Implementer: Codex. Branch: task/002-pinocchio-perp-litesvm @ 29dc633.

## Verdict rationale

The core deliverable is met and verified: the harness drives a **real compiled BPF program**, not a
native mock. `ensure_sbf_program()` runs `cargo build-sbf --features bpf-entrypoint` → produces
`target/deploy/probatio_perp_program.so` (confirmed present, 16 KB) → `LiteSVM::add_program_from_file`
loads it → `send_transaction` executes it and reports **real `compute_units_consumed`** (Open=348,
SettleFunding=356). No P0/P1 correctness issues found. Remaining items are all P2 (hygiene / docs /
out-of-scope), so this merges; the two pure-doc items are applied by the reviewer as editorial touch-ups
(no logic change, no re-review needed) and the rest are tracked.

## What was verified (positives)

- **Real-program proof, not a mock.** Backend `Svm` executes the on-chain `.so`; CU is measured, not
  simulated. This is exactly the Stage-0b thesis (`STAGE0_DESIGN.md` §0b).
- **Thorough parity.** `honest_trace_matches_litesvm_trace` (full-trace equality) +
  `verifier_results_match_across_backends_for_all_policies` (verdict + evidence_slots for all 3) +
  `measure_honest_compute_units_is_non_zero`. 18 tests green offline.
- **Program correctness.** `trade()` mirrors the reference model exactly, with `checked_*` arithmetic.
  Authorization present: owner signer + owner-key match (Deposit/Open/Hedge/Close), harness-authority
  signer (CrankOracle), writable + program-owned account checks on every read.
- **Codec.** Fixed-offset little-endian, bounds-checked `put`/`take`, clean round-trip; `Market::LEN`=24
  / `Position::LEN`=65 consistent with offsets; field layout unchanged from Task 001; `StateSnapshot`
  shape unchanged; `crates/contract` cleanly `#![no_std]` (only core used).
- **CLI.** `--backend ref|svm` with proper error handling; `run_episode` default preserves Task 001
  semantics so existing verifier tests are untouched.

## Findings (all P2)

P2 [Cargo.toml + vendor/hermit-abi]: a global `[patch.crates-io]` replaces `num_cpus`'s transitive
`hermit-abi` with an empty `#![no_std]` stub. It is functionally **safe** — `hermit-abi` is only compiled
under `target_os = "hermit"`, never on the host or BPF targets this project builds — but it is an
undocumented offline-resolution crutch: a reader of a (soon) public repo sees a mysterious vendored crate
+ workspace-wide patch with no explanation. Fix: document *why* (offline resolution of a never-compiled
cfg-gated dep), or drop the patch and cache real `hermit-abi`. → **applied**: explanatory comment added to
`Cargo.toml` and a `vendor/hermit-abi/README.md`.

P2 [contract doc drift]: `crates/contract/src/lib.rs:7-8` still says the crate "keeps this dependency-free
(pure std)", but it is now `#![no_std]` with an on-chain codec. → **applied**: comment corrected.

P2 [programs/perp/src/lib.rs:200-217 SettleFunding]: unlike every other mutating instruction,
`SettleFunding` performs no signer/authority check. This is **defensible** (funding settlement is fully
determined by on-chain state, so a permissionless crank is standard and desirable), but in a program
whose product thesis is authorization rigor it should not read as an *accidental* missing check.
Recommend a one-line comment stating it is intentionally permissionless. Not a blocker. Track for
Task 003 (guard) where authorization semantics get formalized.

P2 [build-sbf `sol_memcpy_` warning]: `cargo build-sbf --offline` succeeds but post-processing emits one
`sol_memcpy_` warning, so the "build-sbf clean / no warnings" gate is technically unmet. This is the
common benign SBF-linker note for an implicit `memcpy` (struct/slice copy) that resolves to the syscall;
the program executes correctly in LiteSVM with real CU. Not a merge blocker. Track as a polish item
(silence via explicit copies, or accept-and-document) before the repo is presented.

P2 [ref vs program overflow divergence]: the reference `trade()` uses unchecked `i64` arithmetic while
the program uses `checked_*` returning `Err`. Parity holds on the 3 scripted policies (no overflow
occurs), but the two backends could diverge on an overflowing input. Out of scope for Stage 0; note for
when adversarial/red-team policies (Task 004) probe extreme sizes — the ref model should adopt the same
saturating/checked behavior so parity survives hostile inputs.

## Follow-ups to track (not blocking this merge)

- Silence or document the `sol_memcpy_` build-sbf warning.
- One-line comment marking `SettleFunding` as intentionally permissionless (or fold into Task 003).
- Align ref-model `trade()` overflow behavior with the program before Task 004 red-teaming.
