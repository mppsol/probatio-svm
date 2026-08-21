# H4-01 — the sentinel harness (evidence tooling only)

**Gate:** [`docs/H4-GATE.md`](../H4-GATE.md) — binding, already committed and pushed (`35f8d0e`).
Read it first. This brief implements §3, §4 and §6 of that gate and adds nothing to them.
**Assignee: Codex (implementation).** The reviewer/adjudicator is the other model; the implementer
does not certify its own numbers.

## Hard constraints — violating any one of these is a KILL for H4, not a bug

- **`/Users/hiroyusai/src/solvo` is READ-ONLY.** Read it freely as reference; **never write to it**,
  not even a formatting change or a build artifact. That is `KILL-5`.
- **No protocol-independent schema, no adapter semantics, no cross-protocol scoring, no "Passport".**
  If a task here seems to need one, stop and report it — that is `KILL-4`.
- **No SBF program.** No `cargo build-sbf`, no probe program, no deploy, no UI, no token.
- **No product code.** This is evidence tooling: one crate, throwaway, no public API.
- Do not change `docs/H4-GATE.md`. If it is wrong, report it; do not edit around it.

## What already exists

- `fixtures/h4/programs/klend_old.so.gz` — the pre-upgrade binary, copied verbatim from
  `../solvo/fixtures/g1/programs/klend.so.gz`. Gzipped account payload **with** trailing zero
  padding: 10,485,715 bytes stored; **stripped of trailing `0x00` it is 2,414,913 bytes**, `sha256`
  `8eab9f858da01fee962452ad3838570b3340cd43b80a236de8fe5297063d1cda`.
- `fixtures/h4/programs/klend_new.so.gz` — the post-upgrade binary, fetched from mainnet
  **after** the gate was pushed. Stored **already stripped**: 2,431,953 bytes, `sha256`
  `b1344d1979daec34bea862a3ed5c44ca5dc8b8e72ec32f1a90ac5150229c22d9`. Identity (ProgramData
  `9uSbGW1y9H5Av6H5TKxQ1wnFApSq2t3oEpfF2YfjDQGA`, `last_deploy_slot` 440,486,775, context slot
  440,626,092, upgrade authority `GzFgdRJXmaw…`) is in `programs/klend_new.identity.json` and in
  `fixtures/h4/MANIFEST.json`.
- `fixtures/h4/programs/farms.so.gz` — dependency, identical in both runs.
- `fixtures/h4/accounts/*.json` — 17 cloned mainnet accounts at slot **440,477,781**, copied verbatim
  from Solvo. `fixtures/h4/MANIFEST.json` carries every pubkey, owner, length and `sha256`, plus
  Solvo's `derived` block (`obligation_owner_offset: 64`, `lending_market_authority`, the farm
  states and bumps).

**Both binaries must be trailing-zero-stripped before loading**, and the harness must assert the
`sha256` of what it loaded against the two hashes above, aborting if either differs.

## Reference implementation to adapt (read-only)

`../solvo/crates/g1-kamino/src/` is a working LiteSVM harness against this exact fixture set:
`fixtures.rs` (loading, `anchor_disc`), `keys.rs`, `preamble.rs` (the mandatory refresh trio),
`vm.rs`, `state.rs`, `main.rs`. `../solvo/crates/g1-kamino/Cargo.toml` has the proven dependency set.
`../solvo/docs/specs/012-g1-leg-a-kamino.md` documents klend's account constraints.

**One deliberate difference from Solvo:** Solvo called klend by CPI from an SBF probe, because its
question was about PDA authority. **H4 does not ask that question.** Call klend **top-level**, signed
by a plain harness keypair whose pubkey is written into the obligation's `owner` field (offset 64).
`v2` is used because it does not run `check_refresh_ixs!`; the refresh preamble is still executed
exactly as Solvo does it. The account order for the withdraw is the one Solvo's probe forwards, in
`main.rs::probe_instruction` — reproduce that order, dropping the probe indirection.

## Build constraints, carried rather than rediscovered

- New crate `crates/h4-sentinel` with **its own `[workspace]` table**, and an `exclude` entry in the
  root `Cargo.toml`. A second copy of the unbundled solana crates alongside litesvm breaks the build
  (Solvo decision 000 §3.4).
- Dependency set copied from `../solvo/crates/g1-kamino/Cargo.toml`, **including
  `arrayref = "=0.3.9"`. Do not relax it and do not remove it for looking unused** — `0.3.10` pulls a
  typosquat whose `build.rs` downloads and executes a binary, and it executed on this machine on
  2026-08-20 (Solvo decision 004 §6).
- No `solana-sdk` umbrella crate.
- The root workspace has a `[patch.crates-io] hermit-abi` shim for offline resolution; a nested
  workspace does not inherit it. If an offline build needs the same shim, copy the patch stanza —
  do not vendor anything new.
- If the build needs network and none is available, **say so and stop**; do not substitute a
  different litesvm version or a stubbed run.

## What to build

`crates/h4-sentinel`, one binary, `cargo run -p h4-sentinel` (or its own `cargo run` inside the
crate). It runs **eight executions**: four cases × two binaries.

**Per execution, from a freshly rebuilt VM** — state never carries across cases or binaries:

1. Load the 17 accounts and both programs (`klend` under test + `farms`); hash-assert the ELF.
2. Apply the disclosed mutations (below), identically for both binaries.
3. Run klend's refresh preamble: `refresh_reserve(usdc)`, `refresh_reserve(sol)`,
   `refresh_obligation`, exactly as `../solvo/crates/g1-kamino/src/preamble.rs` builds them.
4. Send the case's `withdraw_obligation_collateral_and_redeem_reserve_collateral_v2` instruction,
   top-level, signed by the harness keypair.
5. Record the decisive outputs.

**Cases.** `D` = the cloned obligation's USDC `deposited_amount`, read out of the fixture bytes and
**printed by the run**:

| id | `collateral_amount` |
|---|---|
| A | `100_000_000` |
| B | `D` |
| C | `D + 1` |
| E | `u64::MAX` |

**Disclosed mutations** — print each as `account, offset, len, before, after, why`, and apply the
same bytes for both binaries. Expected to be exactly two:
- the obligation's `owner` (offset 64, 32 bytes) → the harness keypair's pubkey;
- the destination USDC token account: create it in the VM with `authority` = the harness keypair.
**If any further mutation turns out to be necessary, disclose it the same way** — a mutation that
makes a run work and is not printed invalidates the whole measurement.

**Decisive outputs per execution:**
- `result`: `Ok`, or `Err` with the exact `TransactionError`/`InstructionError` **and** the numeric
  `Custom` code when there is one, verbatim.
- `token_deltas`: `amount` (SPL Token layout, **offset 64, `u64` LE**) before and after for the
  destination token account, the USDC liquidity supply vault, and the USDC collateral supply vault.
- `post_state`: for **every** fixture account, the post-transaction bytes, reduced to a `sha256` per
  account plus the list of `(offset, len)` ranges that differ from that account's pre-transaction
  bytes.

**Auxiliary, never asserted on:** program logs, compute units, `return_data`. Print them; do not
branch on them, do not compare them for a verdict.

**Per-case verdict**, computed mechanically per gate §4 — old vs new:
- `compatible` — `result` equal **and** all three `token_deltas` equal **and** every account's
  post-state `sha256` equal;
- `breaking` — `result` differs, or any `token_delta` differs;
- `unknown` — `result` and `token_deltas` equal but any post-state `sha256` differs; or the case
  could not be executed on both; or a rerun is not byte-identical.

**Do not compute or print the integration action** (`CONTINUE`/`ISOLATE`/`RE-VERIFY`) and do not
interpret the result. The verdict per case is mechanical and is yours; the adjudication is not.

## Acceptance criteria

1. `cargo run` (offline, from committed fixtures) produces the table and writes
   `evidence/h4-sentinel.json`.
2. **Two consecutive runs are byte-identical**, including the JSON. Provide the command that shows it.
3. The JSON contains: both binaries' identity blocks (path, stored bytes, `elf_len`, `code_hash`),
   the fixture manifest hash, `D`, every disclosed mutation, and for each of the eight executions
   every decisive output listed above, plus the four per-case verdicts.
4. No assertion anywhere reads a log line.
5. `git -C /Users/hiroyusai/src/solvo status --porcelain` is **empty** at the end. Include its output.
6. `cargo build` clean, no new warnings in the new crate.

## Report back

The four case verdicts, the exact numbers, the verbatim errors, `D`, the disclosed mutations, and —
stated plainly — anything you had to stub, skip or work around, **with the direction of the error**.
If a step cannot be executed, say so; **do not substitute a weaker step that passes.** If a case
cannot be run on one binary but can on the other, that is `unknown` and is a real result — report it,
do not engineer around it.
