# Task 019 — MandateSpec: promote the implicit mandate to an authored, hashable contract type

**Frame:** contract-surface change (thin frame / CC-authored ADR, per `AGENTS.md` §"The contract").
Implemented by **Codex** on a branch; **CC reviews** (adversarial: does the spec stay `#![no_std]`/dep-free,
is the canonical encoding stable, does threading the spec break determinism or the guard's on-chain check?).

## Why

Today the agent mandate is **two hardcoded constants** — `MAX_MANDATE_SIZE = 100` and
`MANDATE_INSTRUMENT = 0` (`crates/contract/src/lib.rs:19-22`) — read by `Position::within_mandate()`
(`:164-166`), the guard's `check_position()` (`:194-202`), and captured by the off-chain verifier
(`crates/harness/src/verifier.rs:23,254-266`). The mandate is therefore **not an object**: it can't be
authored per-agent, can't be hashed, can't be referenced by a certificate.

This is the keystone of the certify → screen → adjudicate fold (`docs/adr/` / tri-lane plan): the SAME
declared mandate must be checkable at three stations —

- **certify** (this repo): an episode stays within the mandate envelope (`MandateDeviation`),
- **screen** (Custos repo, later task): a single prospective tx stays within the *same* mandate
  (a new `MandateConformance` invariant — NOT Custos F6, which is hidden-instruction/malice),
- **adjudicate** (Reckn repo, later task): a disputed tx is replayed against the mandate predicate,
  bound by its `spec_hash`.

None of that is possible while the mandate is two `const`s. Step 1 (this task) makes the mandate an
**authored, canonically-encodable, hashable `MandateSpec`** — single-repo, no cross-repo work yet.

## Scope

### Part A — `MandateSpec` in the shared contract

- Add to `crates/contract/src/lib.rs` a `MandateSpec` struct carrying the Stage-0 envelope fields that
  are currently the two constants:
  ```rust
  pub struct MandateSpec { pub max_size: i64, pub instrument: u8 }
  ```
- `impl MandateSpec`:
  - `pub const fn stage0_default() -> Self` = `{ max_size: MAX_MANDATE_SIZE, instrument: MANDATE_INSTRUMENT }`
    (preserves today's behaviour exactly; keep the two consts as the default's source of truth).
  - `LEN` + fixed-offset little-endian `encode(&self, out) -> Result<(), ContractError>` /
    `decode(data) -> Result<Self, ContractError>`, mirroring `Market`/`Position` (dep-free, `#![no_std]`).
    This canonical encoding is the **hash preimage**.
- **`spec_hash`:** the contract crate stays dep-free, so it exposes only the canonical `encode()` bytes.
  Add `spec_hash(&self) -> [u8; 32]` in the **harness** (`crates/harness`, host-side, may use a dep) over
  those bytes. Use FNV-1a-64 zero-extended to 32 bytes **for now**, with a `// TODO(reexec-core): replace
  with keccak256 when the shared engine is extracted (Reckn already ships it)` — this is an identity tag
  for Stage 0, not yet a security commitment. Document the exact preimage (the `encode()` bytes) beside it.

### Part B — thread the spec through the three readers

- `Position::within_mandate(&self, spec: &MandateSpec) -> bool` (was arg-less). Body becomes
  `self.size.abs() <= spec.max_size && self.instrument == spec.instrument`.
- `check_position(market, position, spec: &MandateSpec)` — thread the spec into the guard check.
  Update `programs/guard` and any `programs/perp` caller to pass `MandateSpec::stage0_default()` (or a
  provisioned spec if one is already threaded through genesis — check `world.rs`).
- Verifier: `AccountState.within_mandate` is captured in `verifier.rs` (`AccountState::capture`, `:27-36`).
  Thread the episode's `MandateSpec` into capture so `MandateDeviation` checks against the authored spec,
  not the constant. Default to `stage0_default()` where no per-episode spec exists yet.
- **On-chain guard note:** the guard program reads the spec too. Keep the spec as `stage0_default()` on
  the guard path unless it is already provisioned in market/genesis state — do NOT invent a new account to
  store it in this task (that is a layout change; out of scope). If threading a runtime spec into the guard
  is non-trivial, keep the guard on `stage0_default()` and thread the authored spec only through the
  off-chain verifier + host `check_position` callers; note the split in the review.

## Honesty / safety (non-negotiable)

- **Behaviour-preserving:** with `stage0_default()` everywhere, every existing test must stay green and all
  episode traces byte-identical. This task adds an *authoring seam*, it does not change any verdict.
- `spec_hash` is an **identity tag, not a security hash** at Stage 0 (FNV-1a). Say so in the doc comment;
  do not let a demo describe it as tamper-proof until keccak lands in reexec-core.
- Contract crate stays `#![no_std]` and **dependency-free** (`Cargo.lock` for `probatio-contract`
  unchanged). Any hash dep lives in the harness only, and must not reach the network or the clock.

## Files to touch

```
crates/contract/src/lib.rs        NEW MandateSpec (struct + stage0_default + LEN/encode/decode);
                                  within_mandate(&self, spec); check_position(.., spec); roundtrip test
crates/harness/src/verifier.rs    AccountState::capture takes spec; MandateDeviation vs authored spec
crates/harness/src/world.rs       thread the episode MandateSpec into capture / check calls
crates/harness/src/<hash>.rs      NEW spec_hash (FNV-1a-64 → [u8;32]) + preimage doc + test
programs/guard/src/lib.rs         check_position(.., stage0_default()) (or provisioned spec — see note)
programs/perp/src/lib.rs          update any check_position caller
reviews/019-mandate-spec.md       CC review verdict
```

## Acceptance criteria / gates

- `cargo test` green across contract + harness; **all existing tests unchanged and passing** (behaviour-
  preserving). New tests: `MandateSpec` encode/decode roundtrip; `within_mandate` flips when the spec is
  tightened (`max_size: 5`) or loosened; `check_position` honours a passed spec; `spec_hash` is stable for
  equal specs and differs when `max_size` or `instrument` changes; a verifier episode flags
  `MandateDeviation` against a tightened spec that `stage0_default()` would pass.
- Determinism intact: sample episode traces byte-identical to before (the default spec reproduces today's
  constants). No test hits the network or reads the clock.
- `cargo build` clean, no new warnings; on-chain crates build under `cargo build-sbf`; `probatio-contract`
  stays `#![no_std]` and dependency-free.

## Out of scope (future tasks / other repos)

- Custos `MandateConformance` invariant reading this same `MandateSpec` (Step 2, Custos repo).
- Reckn attestation binding `spec_hash` for keyless re-verification; keccak in reexec-core (Step 3).
- Richer envelope fields (allowed programs / counterparties / value-delta bounds), time-varying mandates,
  and any new on-chain account to *store* a per-agent spec (layout change — separate ADR).
