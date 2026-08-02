# Task 020 — Generic MandateSpec + `reexec-spec` seed (the cross-station fold)

**Frame:** cross-repo architecture ADR + contract-surface change (CC-authored). This is the frame that
makes "one authored mandate, checked at certify AND screen" real. Split into **2a (this repo)** and
**2b (Custos repo, separate brief)**. 2a lands first (2b depends on `reexec-spec`).

## Why

Task 019 gave `MandateSpec { max_size, instrument }` — a **perp-specific** envelope. The screen station
(Custos) evaluates a **generic token-account `Outcome`** (pre/post balances, delegate/authority), not a
perp `Position`, so it **cannot** check `max_size`/`instrument`. To share ONE mandate across certify and
screen, the spec needs (a) a **neutral home** both repos can depend on, and (b) at least one **generic,
outcome-checkable field**. This task adds the minimum of each; richer fields stay deferred.

## Decision 1 — seed the `reexec-spec` crate (reexec-core's first extraction)

- New crate `reexec-spec` (dep-free, `#![no_std]`) holding `MandateSpec` (+ `encode`/`decode`/`LEN`/
  `stage0_default`) and the host-side `spec_hash` seam.
- `probatio-contract` **re-exports** `MandateSpec` (`pub use reexec_spec::MandateSpec;`) so nothing in
  this repo breaks and 019's callers are untouched.
- Both `probatio-svm` and `custos` depend on `reexec-spec` → **one definition, no copy**. This is the
  neutral shared home the tri-lane calls `reexec-core`; we seed it with the one type that must be shared.

## Decision 2 — add a generic, screen-checkable field: `max_value_out`

- Extend `MandateSpec` to `{ max_size: i64, instrument: u8, max_value_out: u64 }`.
  - **certify** (probatio) keeps enforcing `max_size`/`instrument` exactly as in 019 — unchanged behaviour.
  - **screen** (custos, 2b) enforces `max_value_out`: net value leaving the user-controlled accounts in a
    simulated tx must be `<= max_value_out`, else `MandateConformance` fires RED.
- `stage0_default()` sets `max_value_out: u64::MAX` (no cap ⇒ **behaviour-preserving**: every existing
  certify episode and trace is byte-identical; the field is inert until authored).
- Canonical encoding grows by 8 bytes (`LEN = 8 + 1 + 8 = 17`), appended after `instrument` to keep the
  existing prefix stable; `spec_hash` now covers the new field. Update the 019 roundtrip/hash tests.

## Scope — 2a (THIS repo)

- Create `reexec-spec` crate; move `MandateSpec` + `encode`/`decode`/`LEN`/`stage0_default` into it;
  add `max_value_out` (default `u64::MAX`); `probatio-contract` re-exports `MandateSpec`.
- Move/keep `spec_hash` (harness) over the new canonical bytes; update its preimage doc + tests.
- Update 019 tests: `mandate_spec_roundtrip` (new LEN/bytes), `spec_hash` sensitivity to `max_value_out`.
- **Certify unaffected:** `within_mandate` still checks size/instrument only; `max_value_out` is not read
  on the certify/perp/guard path (it is a screen-station field). All existing tests stay green,
  traces byte-identical.

## Scope — 2b (Custos repo, separate brief `custos/docs/…`, Codex implements, CC reviews)

- `custos` depends on `reexec-spec`.
- New `MandateConformance` invariant (spec-relative tier, **distinct from the F1–F6 malice tier**):
  over the simulated `Outcome`, sum the net token value leaving accounts owned by `o.user`; if that net
  outflow `> mandate.max_value_out`, emit a RED `Finding{ code: "M1-mandate", … }`.
- Demo wiring: an authored `{ max_value_out: X }` passes a benign tx and fires RED on a tx moving `> X`
  even when the tx "succeeds" — the Grok/Bankr story (a tricked agent still can't exceed its mandate).

## Out of scope

- Richer envelope fields (allowed-programs / counterparties / per-account caps), time-varying mandates.
- Extracting the `Verdict`/`Finding`/`Invariant` vocabulary into `reexec-core` (that is Step 3, and it
  unifies certify+screen+adjudicate reporting — a separate ADR).
- Reckn attestation binding `spec_hash` for keyless re-verification (Step 3).

## Assignment & sequencing

- **2a** — this repo's brief→branch→review→merge loop (CC frames; per the window split CC may implement
  here, **Codex reviews**). Contract-surface change ⇒ this ADR is the required shared frame.
- **2b** — Custos window: **Codex implements**, **CC reviews**. Cannot start until `reexec-spec` (2a) is
  merged, since it provides the shared `MandateSpec`.
- Both are lane ① (certify+screen); no offensive (lane ③) coupling — the tri-lane walls are untouched.
