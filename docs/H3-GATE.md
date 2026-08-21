# H3 — Composability Passport — **KILLED at the design gate (2026-08-21)**

> ⛔ **This gate is closed. H3 is `KILLED`** — verdict and reasoning:
> [`docs/decisions/H3-design-gate-kill.md`](./decisions/H3-design-gate-kill.md).
>
> Two independent review rounds returned `CHANGES` with P0s of the same class, §13 pre-registered
> that there would be no r3, and the founder ruled. **What died is the fixed, protocol-independent
> capability schema**: the freedom to choose which effect satisfies a field was relocated into the
> adapter manifest, not removed, so the deliverable would be a report per protocol rather than a
> comparable primitive. **No gate item was ever executed** — G1a–G5 were never started, no code was
> written, and nothing here says anything about how klend or Phoenix actually behave.
>
> **Nothing below is rewritten.** It is kept as a reproducible refutation asset.

**Status when killed: G0 pre-registration, r2, not frozen. No verdict was ever measured. No code.**
This document superseded [`docs/H2-GATE.md`](./H2-GATE.md) as the binding gate until 2026-08-21.

Review history: r1 → Codex [`reviews/H3-G0-passport.md`](../reviews/H3-G0-passport.md) `CHANGES`
(eight P0) → r2 (this document). §13 lists every change and **the direction it moves the gate**.

> **Founder ruling, 2026-08-21.** H2's risk-transfer hypothesis is **FROZEN UNEXECUTED**. It is not
> `KILLED` — no G1 measurement was ever run, and nothing here rewrites that. It is not `BLOCKED`
> either: nothing external is being waited on. It stopped because its G0 entered a specification
> spiral (three rounds, 331 lines, zero measurements) and the founder chose a different hypothesis
> rather than a fourth round. **H2 may be resumed only by a founder ruling, from its own G0, with
> its own pre-registration re-frozen.** H1 remains `KILLED` and is not rewritten.

H1's rules carry forward unchanged and are not restated in full:

- A verdict is a **number or a reproducible experiment**, never an assessment or a plan.
- **Not-proven is a KILL.** Adding a hypothesis to stay alive is forbidden.
- **Pre-registered parameters are not changed after measurement.**
- Every number carries a **slot**. Every discrepancy is reported **with its direction**, in both
  directions — an error that favours the KILL is as disqualifying as one that flatters the project.
- Forbidden without exception: real funds, mainnet or devnet deploy, force push, committed secrets.
- **At most two agents.** Claude writes specification and evidence; Codex implements **or** reviews,
  never both at once. No model reviews its own output.

---

## 1. The hypothesis

> **A protocol's claim to be "composable" or "CPI-able" decomposes into a small fixed set of
> capability fields, each decided by an executable probe against that protocol's real mainnet binary
> and cloned mainnet state; the resulting Passport is byte-canonical, expires mechanically when the
> code or the state it depends on changes, generalises to a protocol it was not designed against,
> and has a consumer that can be named and counted.**

Every clause is a gate item below, and every gate item is decided by a number fixed in advance.

**What this is not.** It is not a security audit, not a claim that a protocol is safe, and not a
score. A Passport says *what happens when you call it*, in bytes, at a named code hash — nothing
about whether calling it is wise.

## 2. The design principle that keeps this finite, and its stated limit

H2's G0 died because it needed a byte-level loss schema for six venues before it could measure
anything. H3 keeps the *field definitions* venue-agnostic:

> **Every capability field is decided by effects observable in any transaction — token-account
> balance deltas, byte-range diffs of writable accounts, the transaction's error result, returned
> data, compute consumed, and the signer set.**

Venue-specific input enters **only** through a per-venue **adapter manifest** whose shape is fixed
here and whose content is committed **before** that venue is measured:

```
AdapterManifest := {
  "venue":            program id (base58),
  "source":           { "kind": "idl" | "sdk_reader_structs", "url": string,
                        "commit_or_hash": string, "sha256": hex },
  "value_out_ix":     { "discriminator": hex, "accounts": [ {role, pubkey|derivation} ] },
  "value_in_ix":      { … same shape … },
  "scarcity_range":   { "account": base58, "offset": dec, "len": dec, "encoding": enum },
  "request_unit":     { "mint": base58, "decimals": dec },
  "request_amount":   dec (atoms),
  "candidate_ranges": [ {account, offset, len, encoding, schema_field_name} … ],
  "stale_horizon":    dec (slots),
  "reference_venue":  program id (base58)
}
```

**Every range in the manifest carries the schema field name it came from, and every schema is pinned
by hash.** A range with no named field in a hash-pinned schema is inadmissible: guessing an offset is
forbidden (H1's P0 was exactly that). `F_new` at G3 counts departures from this shape (§9, G3).

**The limit, stated plainly.** This makes venue knowledge *bounded and disclosed*; it does not make
it zero. Codex's r1 finding 1 is correct that a field defined only over observable effects can be
satisfied by the wrong effect. Each of C3, C4, C6 therefore carries a **positive test** below that
the wrong effect cannot pass, not merely an observation that some effect occurred.

## 3. Subjects — identity, verified read-only

Verified by `getAccountInfo` against `solana-rpc.publicnode.com`, `commitment: finalized`, on
2026-08-21. **`S_G0 = 440,578,912`** is the pre-registration reference slot; the four count-only rows
were read at context slots 440,579,564–440,579,569 and carry that drift openly.

Reproduce any row: `getAccountInfo(program_id)` → check `owner`, `executable`, `data.len == 36`,
`u32le(data[0..4]) == 2`; take `programdata = base58(data[4..36])`;
`getAccountInfo(programdata)` → check `u32le(data[0..4]) == 3`; then `last_deploy_slot =
u64le(data[4..12])`, `upgrade_authority = data[12] == 1 ? base58(data[13..45]) : null`, and the
hashes per §6 E1. The full procedure, including every fail-closed condition, is E1.

| role | protocol | program id | last deploy slot | upgrade authority | elf bytes | `code_hash` |
|---|---|---|---:|---|---:|---|
| **probe, PASS candidate** | Kamino klend | `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD` | 440,486,775 | `GzFgdRJXmawPhGeBsyRCDLx4jAKPsvbUqoqitzppkzkW` | 2,431,953 | `b1344d1979daec34bea862a3ed5c44ca5dc8b8e72ec32f1a90ac5150229c22d9` |
| **probe, FAIL candidate** | Phoenix Eternal | `EtrnLzgbS7nMMy5fbD42kXiUzGg8XQzJ972Xtk1cjWih` | 437,447,068 | `GPgADQrhzGoUgLqxsZMKvSpwcLaJFVTq6gEixKhmcwpm` | 3,142,561 | `2f1b8a5d60696c7f18d1888a721c0659ec82208abe21e1be1fe8dade5f296e0e` |
| **probe, blind third** | marginfi v2 | `MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA` | 432,875,565 | `J3oBkTkDXU3TcAggJEa3YeBZE5om5yNAdTtLVNXFD47` | 3,161,329 | `26dda5e1a060d8fa5d8cf122518f26be3bdaab68e1dc525f74061e2e74cb38f4` |
| count only | Save (Solend) | `So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo` | 350,614,570 | `RY93CZYe5g6drtG7W9PmHRPzaBLZ1uwihTzayQTmJfh` | 603,425 | `ef621872c5b7c3fdef61fed1e042609f433cff1c408457b526ff665ef5df33f9` |
| count only | Drift v2 | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` | 429,731,225 | `8jj7zJgdr5bDndc7evM74FMGwzLPmd4u4QxNzFi1BMai` | 193,713 | `8299a9b8554010b96b9e09ee6cf1c884870d626d623a617cf56424017215e81f` |
| count only | Jupiter Perps | `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu` | 413,092,478 | `5myNNmEmPm3UAnJ2ggLEpnTFb9t9Gk8369wKw6n3uAKx` | 2,602,905 | `417954cc67bad9291388bfd480a671072ec43b886b04d88d4df2efa7215b6ee0` |
| count only | Kamino Vaults | `KvauGMspG5k6rtzrqqn7WNn3oZdyKqLKwK2XWQ8FLjd` | 432,874,668 | `GzFgdRJXmawPhGeBsyRCDLx4jAKPsvbUqoqitzppkzkW` | 1,195,577 | `5b1eac1695f68e4e3889a10bce0797e96f9cfd0d0aaa8d7b8f4e37d730fbc609` |

**This table has been verified by one party only** — the author of this document, from the commands
above. Codex's r1 review could not resolve the RPC hostname in its sandbox and correctly declined to
assume it. It is re-derived by the model that did not produce it at **G5**, and until then it is
recorded as unconfirmed.

Three facts from that table are load-bearing and were **not** assumed in advance:

- **7 of 7 carry a live upgrade authority. None is immutable.** Every one can be replaced by a single
  transaction from a single key. A Passport that is not re-checked at the moment of use therefore
  asserts nothing about the code that will run — §6 is built on this, not on a hypothetical.
- **Kamino Vaults and klend share the upgrade authority `GzFgdRJXmaw…`.** The G1 count of callers
  must therefore exclude same-authority programs, or it counts a team's own toolkit as a market.
- **klend was upgraded at slot 440,486,775** — see §4, which is the reason this project exists.

**Venue set V is frozen here.** Adding a venue later — to the probe set or the count set — is a
different gate and starts over at G0. Widening V would make G1 easier to pass; that direction is
disclosed and the door is closed now, before the count is known.

## 4. The observation that motivates the hypothesis, disclosed in full

The sibling repo `../solvo` measured Kamino klend and Phoenix Eternal under LiteSVM against mainnet
binaries (`solvo/docs/decisions/005-safe-capital-movement-verdict.md`, 2026-08-21). **Solvo is a
read-only reference for H3 and is not edited by this work.**

**Its two legs did not use the same kind of state, and the difference matters** (Codex r1, P1 finding
10 — the r1 text of this section conflated them):

- **klend (Leg A)** ran against **cloned mainnet accounts**, fixture manifest `slot` **440,477,781**
  (`max_response_context_slot` 440,477,778).
- **Phoenix (Leg B)** ran against a **freshly initialised localnet fixture**, *not* cloned mainnet
  state, with mainnet Eternal/Ember/Hawkeye binaries. Solvo's own §7 records that fixture as
  **more permissive than mainnet on every axis**.

Solvo's committed `klend.so`, stripped of trailing zero bytes exactly as E1 defines, is
**2,414,913 bytes**, `sha256` = `8eab9f858da01fee962452ad3838570b3340cd43b80a236de8fe5297063d1cda`
— independently re-derived by Codex in its r1 review.

The klend binary on mainnet today is **2,431,953 bytes**, `sha256` = `b1344d19…9c22d9`, deployed at
slot **440,486,775** — **8,994 slots (≈ 1 hour) after that fixture's slot**.

So a capability verdict produced by a careful, executed, independently recomputed experiment was
bound to a binary that had already been replaced **before the verdict was written down**. That is the
entire argument for this hypothesis, and it is a measurement, not a thesis. It is also a warning
about H3 itself: if H3 ships a Passport without §6, H3 ships the same stale claim in a nicer format.

**Consequence for H3, and it cuts against the project.** §7 requires **cloned mainnet state**. Solvo
did not obtain it for Phoenix. If H3 cannot obtain it either, Phoenix's fields are `UNPROVEN` and
that is a **KILL at G2** — substituting a fresh localnet fixture, as Solvo legitimately did for a
different question, is not permitted here and is not a fallback.

**Disclosure:** this comparison, and the traffic rates in §9/G1, were computed *before* G0 froze.
They decide no gate item. No number that decides a G-item has been measured.

## 5. Item 1 — the Passport schema and the executable proof of each field

Seven capability fields. Every field carries `verdict`, the `quantities` the verdict was computed
from, `evidence` (every run, with its byte diffs), and `valid_if` (§6, E3). A consumer may re-apply
its own predicate to `quantities`; the `verdict` is the canonical predicate, not the only one.

`verdict ∈ {PASS, FAIL, UNPROVEN}` unless a field states otherwise. **`UNPROVEN` is a KILL** wherever
the gate counts fields — a capability that could not be decided is not recorded as unknown-but-fine.

### C1 — `pda_authority`

**Claim.** A program that is not the venue, holds no privileged role on it, and appears on no
whitelist, can be the *sole* authority for a value-moving instruction, signing via `invoke_signed`.

**Proof.** The probe's PDA is the authority on both the venue-side position/obligation account and
the destination token account. The transaction's fee payer is **not** an authority on any account
whose balance changes. The probe calls the venue by CPI.

**PASS** iff the destination token account's `amount` (SPL Token layout, offset 64, `u64` LE), read
back out of the VM after execution, increased, **and** no signer appears in the flattened instruction
trace other than the fee payer and the probe PDA. **FAIL** iff the venue errors on the authority, or
an additional signature is required. **UNPROVEN** iff the probe cannot be placed in the authority
seat (record the exact obstacle).

*Quantities:* `signers[]`, `delta_destination`, `error` (verbatim), `whitelist_checked`.

### C2 — `atomic_settlement`

**Claim.** The value moves in the same transaction as the request.

**PASS** iff `requested − delta_destination ≤ 1` **atom** and `delta_destination > 0`. **FAIL**
otherwise. One atom is the smallest indivisible unit and is exactly the discrepancy klend produced in
Solvo's A3 (99,999,999 against 100,000,000). r1 used a `1e-6` *ratio*, which would have passed a
999,999-of-1,000,000 partial fill; Codex's P1 finding 9 is correct and the bound is now absolute.

*Quantities:* `requested`, `delta_destination`, `shortfall_atoms`, `slots_elapsed` (must be 0).

### C3 — `revert_on_shortfall`

**Claim.** When the venue cannot pay in full, the instruction **returns an error** rather than
committing a transaction that moves nothing.

**The scarcity range is pre-registered, not chosen at measurement time.** It is
`AdapterManifest.scarcity_range`, committed before the venue is measured, and it must name a field in
a hash-pinned schema. This closes the r1 hole where an implementer could mutate any convenient field
into an error path and report `PASS`.

**The boundary sweep — the positive test that a wrong range cannot pass.** With `r =
request_amount`, run the value-out instruction at scarcity values `r+1`, `r`, `r−1`, and `0`:

| run | scarcity value | required outcome for the range to be admissible |
|---|---|---|
| `S+` | `r + 1` | pays in full (C2 `PASS`) |
| `S=` | `r` | pays in full (C2 `PASS`) |
| `S−` | `r − 1` | **does not** pay in full |
| `S0` | `0` | **does not** pay in full |

**If the outcome does not change exactly at the boundary between `S=` and `S−`, the range is not the
paying constraint and C3 is `UNPROVEN`.** An unrelated field mutated into an error path will not
produce a transition that tracks the request amount, and a field whose effect is monotone but not at
the boundary is equally rejected.

**Given an admissible range: PASS** iff `S−` and `S0` both return `Err`. **FAIL** iff either commits
with `delta_destination == 0`, or commits with `0 < requested − delta_destination` beyond one atom.

Where the venue has a **second** independent limiting parameter, a fifth run holds that parameter at
its binding value while the scarcity range does not bind, as an attribution control; without it the
field records `attribution: single-parameter` and the gate counts it as `UNPROVEN`.

None of this is procedural caution. Solvo's first Phoenix run reported the opposite verdict because
writing `remaining_budget` alone is neutralised by a token bucket refilling from `last_update_slot =
0`: **no shortfall was ever created and it read exactly like a clean pass.** The `S+`/`S=` runs make
that failure impossible to mistake for a result.

*Quantities:* `error` per run (verbatim), `delta_destination` per run, `return_data_len`,
`scarcity_range` (account, offset, len, schema field name, source hash), `boundary_admissible`.

### C4 — `no_deferral_record`

**Claim.** The venue does not answer a request it cannot satisfy by writing a promise.

Codex's r1 P0 finding 2 is correct: byte-range novelty is neither necessary nor sufficient. A venue
that overwrites a fixed-size request record in place writes a genuine promise and would have passed;
a venue that increments a telemetry counter writes no promise and would have failed. **The decisive
test is redemption, and byte-range novelty is demoted to corroborating evidence.**

**The entitlement test — the positive test.** From the post-`S−` state, **without issuing a new
request**, restore the scarcity range to `r + 1` and advance the clock by
`AdapterManifest.stale_horizon` slots. Then:

- **FAIL** if the destination's `amount` increases at any point without a new request, or if any
  venue-side instruction that the pinned schema names as settling pending requests can be invoked
  successfully by *anyone* and moves value to the destination. A promise is exactly state that pays
  without being re-requested; this detects it whether it lives in a new account, an overwritten
  record, a preallocated slot, or an escrow balance.
- **PASS** iff no value arrives, **and** no changed byte range in `S−` is absent from the union of
  changed ranges of the same account in `S+` — the r1 containment rule, kept as a *necessary*
  condition and reported as `range_novelty`, never as the verdict on its own.
- **UNPROVEN** if the pinned schema names no settlement path and none can be established. A venue
  whose deferral semantics cannot be tested does not get a `PASS` by default.

**A stress-only changed range with no redemption is reported, not fatal**, and is recorded as
`range_novelty: {…}` with `deferral: false` — the telemetry-counter case Codex named.

*Quantities:* `entitlement_paid` (bool), `settlement_ix_tried[]`, `range_novelty[]`,
`created_accounts[]`, `len_deltas[]`.

### C5 — `permissionless_settlement`

**Claim.** Settlement needs no keeper, crank, admin, or freshness precondition the caller cannot
satisfy inside its own transaction — **and this remains true when the venue's data is stale.**

**Proof — two clock positions.** Codex's r1 finding 1 is correct that a fresh clone hides
staleness-gated keepers.

- **`K_fresh`** — the control case at the cloned slot, no slot advance, probe building every
  precondition instruction itself.
- **`K_stale`** — the same, after advancing the clock by `AdapterManifest.stale_horizon` slots
  (declared per venue from the pinned schema's own staleness constant, never guessed).

Four-valued, and **the verdict is the worse of the two runs**: **`PASS_SELF_CONTAINED`** (value moves
with no precondition instructions); **`PASS_WITH_PUBLIC_DATA`** (moves, but requires precondition
instructions the probe builds from public accounts); **`FAIL_PRIVILEGED`** (requires a signature the
caller cannot produce, a second transaction, or an action by a named keeper/admin — *in either run*);
**`UNPROVEN`** (requires an artifact whose permissionless availability could not be established, e.g.
an off-chain signed price blob with no public posting path; record which, and which run).

*Quantities:* `precondition_ixs[]` — each `{program_id, discriminator (hex, first ≤8 bytes of
instruction data), account_count, order_index}` — `extra_accounts`, `privileged_signers[]`,
`cu_total` per run, `stale_horizon`.

### C6 — `admin_reachable_surface`

**Claim.** The state the venue's own authority can change, without a code upgrade, to flip any of
C1–C5 is **measured**, and each recorded dependency is shown to be **authority-reachable**.

**The candidate universe is stated and exhaustive relative to it.** Candidates are **every field, in
the hash-pinned schema, of every account named in the value-out instruction's account list** — an
enumerable set, not a selection. `AdapterManifest.candidate_ranges` must equal that enumeration;
a shorter list is inadmissible. If a venue publishes neither an IDL nor SDK reader structs covering
those accounts, C6 is `UNPROVEN` and that venue is a KILL.

**Two conditions per recorded dependency**, both required (Codex r1 finding 1: a corrupting mutation
is not an admin capability):

1. **Flip** — mutating the range changes the verdict of a named field. The mutation is disclosed by
   the run (§7, rule 5).
2. **Reachability** — a venue instruction that writes that range is named from the pinned schema,
   together with the account its access control requires to sign. If no such instruction can be
   named, the dependency is recorded with `reachability: UNPROVEN` and **does not count** toward G4.

`admin_can_disable` is three-valued: **`DEMONSTRATED`** (≥1 dependency with both conditions),
**`NOT_FOUND_IN_UNIVERSE`** (the full enumeration was swept and none flipped), **`UNPROVEN`**.
It is not a `bool`, because "we did not find one" and "there is none" are different claims.

*Quantities:* `state_deps[] = {account, offset, len, schema_field_name, field_flipped, writer_ix,
required_signer, reachability}`, `candidate_count`, `swept_count`, `admin_can_disable`.

### C7 — `composed_rollback`

**Claim.** This venue composes with another in one transaction, and a failure in the second leg rolls
the first back to the byte.

Added in r2 because Codex's r1 finding 6 is correct that adoption questions 6 and 8 (§8) were not
answerable from the six fields. Dropping the questions would have been the easier move and would have
made G5 easier to pass; adding the field makes it harder. The direction is stated in §13.

**Proof.** With `R = AdapterManifest.reference_venue`, in a single transaction: value-out on `V`,
then value-in on `R`; and a second transaction: value-out on `V`, then a deliberate `Err` from the
probe.

**PASS** iff the composed transaction succeeds with both legs' balance deltas matching their
standalone values, **and** the failing variant leaves **every** balance and **every** writable
account byte at its pre-transaction value. **FAIL** otherwise. **UNPROVEN** if `R` cannot be run in
the same harness.

*Quantities:* `reference_venue`, `deltas_composed[]`, `deltas_standalone[]`,
`rollback_bytes_changed` (must be 0), `cu_composed`.

### Aggregate

There is **no score and no grade**. The Passport carries seven fields; the adoption decision is the
consumer's policy applied to them (§8).

## 6. Item 2 — expiry and revocation

Four rules. E1 and E2 void the whole Passport; E3 suspends one field; E4 voids issuance.

**E1 — identity and code.** The Passport is bound to a program **and to the ProgramData account that
program actually points at**. Codex's r1 finding 3 is correct that r1 never bound the two; without
this a fixture could pair a program id with unrelated ProgramData, load matching bytes, and pass every
field for code the program does not execute. The procedure, fail-closed at every step:

1. `getAccountInfo(program_id)` at one slot. Require `executable == true`, and record `owner` as the
   loader.
2. **loader-v3** (`BPFLoaderUpgradeab1e11111111111111111111111`): require `data.len == 36` and
   `u32le(data[0..4]) == 2` (`Program`). `programdata := base58(data[4..36])`.
3. `getAccountInfo(programdata)` at the **same slot context**. Require `owner` = the same loader and
   `u32le(data[0..4]) == 3` (`ProgramData`). Then `last_deploy_slot := u64le(data[4..12])`; the
   authority `Option` tag is `data[12]` — `1` ⇒ `upgrade_authority := base58(data[13..45])` and
   `mutable := true`; `0` ⇒ `upgrade_authority := null`, `mutable := false`, and **`data[13..45]` is
   not an authority and is not recorded as one**. Any other tag value ⇒ `UNPROVEN`.
4. `code_hash := sha256(data[45..]` with **all trailing `0x00` removed**`)`;
   `programdata_hash := sha256(data)` over the untrimmed account; `elf_len` recorded. **All three**,
   because trimming is not injective in principle and the triple closes it.
5. **non-upgradeable** (`BPFLoader2111…`, `BPFLoader1111…`): `code_hash := sha256(account.data)`,
   `programdata := null`, `programdata_hash := code_hash`, `upgrade_authority := null`.
6. **Any other loader, including loader-v4: `UNSUPPORTED_LOADER`**, whole Passport `UNPROVEN`.
   Guessing a header offset is the failure H1's post-mortem names.

A mismatch on **either** hash ⇒ `EXPIRED`. The consumer's cheap pre-check is `last_deploy_slot`
(12 bytes at a fixed offset): if it exceeds the recorded value the Passport is `EXPIRED` **without
hashing anything**. Conservative on purpose — redeploying identical bytes bumps the slot and expires
the Passport, which errs toward re-verification.

**E2 — authority.** `upgrade_authority` differing from the recorded value ⇒ `EXPIRED`; the party able
to swap the code is part of the claim. And the honest consequence of §3: **for a program with a live
upgrade authority, no Passport is valid ahead of time.** What it provides is not a guarantee but a
**cheap re-check** — one account read at decision time instead of a re-audit. A Passport for a mutable
program that a consumer does not re-check is worth nothing, and the schema says so in a field
(`mutable: true`) rather than in prose.

**E3 — state.** Each field carries `valid_if`: predicates over live state, not recorded values (a
budget that refills is not a change). A predicate is
`{account, offset, len, encoding, op, rhs}` where `rhs.kind ∈ {const, request}` — e.g.
*"`remaining_budget` at (account, offset, 8, `u64le`) `≥` request"*. Any false predicate ⇒ that field
is `SUSPENDED`, and a consumer policy requiring it must reject. `valid_if` may contain **only**
ranges that C6 recorded with `reachability` established; a hand-written predicate is inadmissible.

**E4 — harness.** `harness_hash`, `fixture_manifest_hash` and `adapter_manifest_hash` are part of the
identity block. A Passport whose probe, fixtures or adapter manifest changed must be re-issued, so a
verdict cannot outlive the code that produced it.

**Evaluation cost, which is part of the claim.** A consumer's full check must be ≤ **4 account reads
per field at a single slot**, with no historical scan and no indexer. A field whose `valid_if` cannot
be evaluated inside that budget is not a primitive (see G5).

## 7. Item 3 — the minimal harness

Tier **V2** in the sibling repo's ladder: BPF under LiteSVM against real cloned venue accounts. Not
V3 — nothing is deployed anywhere, and no claim may be stated at a tier above the one that produced it.

1. **Binaries.** Each venue's ELF is fetched by `getAccountInfo` on the ProgramData account **that
   E1's procedure derived from the program id**, and trimmed exactly as E1 defines. **The harness
   asserts `sha256(loaded_elf) == I.code_hash` before executing anything** and aborts otherwise.
2. **State — cloned mainnet only.** Accounts cloned at one named slot `S1` (the first finalized slot
   at or after the G0 commit), with `MANIFEST.json` recording `pubkey, owner, len, sha256, slot` per
   account and the RPC's `max_response_context_slot` for the batch.
   **A freshly initialised or localnet fixture may not be substituted for cloned mainnet state.** A
   venue whose state cannot be cloned yields `UNPROVEN` fields, which is a KILL — see §4.
3. **The probe.** One SBF program per venue-shaped call, on no whitelist, holding every authority
   through a single PDA. Ops: `MOVE_OUT`, `MOVE_IN`, `MOVE_OUT_THEN_FAIL`, `CONTROL`,
   `MOVE_OUT_UNDER_STRESS`, `COMPOSE_WITH_REFERENCE`, `SETTLE_PENDING_ATTEMPT`.
   `harness_hash = sha256` of the probe ELF.
4. **Assertions read account bytes back out of the VM. No assertion may read a log line.**
5. **Disclosed mutations.** Every byte of cloned state the harness writes — repointing an `owner` to
   the probe PDA, setting a scarcity value, a C6 sweep — is printed by the run itself: account,
   offset, len, before, after, and why. **A mutation that is not printed invalidates the run.**
6. **Determinism.** Two runs byte-identical, including the emitted Passport JSON and its hash.
7. **Offline.** Once fixtures are committed the run touches no network.
8. **Output.** The canonical Passport JSON (§8) plus a human table. The JSON is the artifact.

Known traps, carried from the sibling repo's executions rather than rediscovered: `cargo build-sbf`
defaults to `--arch v0` and can abort on stack frame size — use `--arch v1` or later with a separate
`CARGO_TARGET_DIR` per architecture; a probe crate needs its own `[workspace]` and a root `exclude`
entry, because a second copy of the unbundled solana crates alongside litesvm breaks the build.

## 8. Item 5 — why this is a primitive and not a test report

Argued nowhere; decided by three numbers at G5.

### 8.1 The complete Passport schema

Every member is always present. **Absent members are forbidden; `null` marks inapplicable.** Numbers
are never JSON numbers — every integer is a decimal string, so no float ever appears.

```
Passport := { "schema": "probatio.passport/v0", "identity": Identity, "fields": [Field × 7] }

Identity := { program_id, loader, programdata, code_hash, programdata_hash, elf_len,
              upgrade_authority, mutable, last_deploy_slot, observed_at_slot,
              harness_hash, fixture_manifest_hash, adapter_manifest_hash }

Field    := { id: "C1".."C7", verdict: string, quantities: {…the members named in §5, all present…},
              evidence: { runs: [Run…] }, valid_if: [Predicate…] }

Run      := { label, result: "Ok"|"Err", error: string|null, cu,
              deltas: [{account, mint, before, after}…],
              changed_ranges: [{account, offset, len}…],
              created_accounts: [account…], return_data_len }

Predicate:= { account, offset, len, encoding: "u64le"|"u8"|"i64le"|"bytes",
              op: "ge"|"gt"|"le"|"lt"|"eq"|"ne", rhs: {kind: "const"|"request", value} }
```

**Canonicalisation.** UTF-8. Object keys sorted ascending by Unicode code point. No insignificant
whitespace. Strings escaped with exactly `\"`, `\\`, and `\u00XX` (lowercase hex) for U+0000–U+001F;
no other character is escaped. Hashes and byte strings lowercase hex; pubkeys base58. Array order:
`fields` by `id`; `runs` in execution order; `deltas`, `changed_ranges`, `created_accounts` and
`valid_if` sorted by `(account, offset, len)` as unsigned; `state_deps` and `precondition_ixs` by
`(account, offset, len)` and `order_index` respectively.

`passport_hash = sha256(canonical bytes)`. **`stable_hash` is the same hash computed with exactly one
path replaced by the empty string: `identity.observed_at_slot`.** No other exclusion exists — r1's
"live-state snapshot values" was an undefined category and Codex's r1 finding 6 is correct that it
made equality unreproducible. Two independent issuers of the same claim, from the same committed
fixtures, produce the **same `stable_hash`**. That equality is the G5 kill.

### 8.2 The adoption questions, as typed queries

Each question is answered by evaluating a path against the Passport. **A question with no path is
unanswered by definition** — this is a computation, not a judgement (Codex r1 finding 6).

| # | question | answering path | source of the question |
|---|---|---|---|
| 1 | Can my PDA be the authority? | `fields[C1].verdict` | klend CPI whitelist gate, `solvo` d005 §2 |
| 2 | If I am not whitelisted, does it revert? | `fields[C1].quantities.error` | klend `CpiDisabled` `Custom(6080)` |
| 3 | Does the money arrive in the same transaction? | `fields[C2].verdict`, `.quantities.shortfall_atoms` | `solvo` d005 §3 |
| 4 | If the venue cannot pay, error or successful no-op? | `fields[C3].verdict`, `.quantities.error` | `solvo` d005 §0 |
| 5 | Does a failed call leave a record that pays me later? | `fields[C4].verdict`, `.quantities.entitlement_paid` | `solvo` d005 §3 queue node |
| 6 | Can I compose two venues and roll both back? | `fields[C7].verdict`, `.quantities.rollback_bytes_changed` | `solvo` d005 §2 A4a/A4b |
| 7 | What must I refresh first, and can I do it myself? | `fields[C5].quantities.precondition_ixs[]` | klend `check_refresh`, `solvo` spec 012 Leg A |
| 8 | What does the whole settlement cost in compute? | `fields[C5].quantities.cu_total`, `fields[C7].quantities.cu_composed` | `solvo` d004 — 1,037,241 CU at 74% of the ceiling |
| 9 | Can an admin turn any of this off without redeploying? | `fields[C6].quantities.admin_can_disable`, `.state_deps[]` | `solvo` d000 §1.5 |
| 10 | Is the program I audited the program that will run? | `identity.code_hash`, `identity.last_deploy_slot` | §4 — klend, 8,994 slots |
| 11 | Who can replace the code, and is it immutable? | `identity.upgrade_authority`, `identity.mutable` | §3 — 7 of 7 mutable |
| 12 | When does what I was told stop being true? | `fields[*].valid_if[]`, E1–E4 | §6 |

**`Q_prose` = the count of these twelve whose path is absent, `null`, or `UNPROVEN` in a Passport
produced at G2/G3.** The list and the paths are frozen here. Editing either afterwards is forbidden.

**On circularity, honestly.** Codex's r1 finding 6 is right that the questions and the fields were
written by the same author, and that sourcing them from `../solvo` makes them *externally grounded*
but not *independent*. Two defences remain and both are numbers: `F_new = 0` at G3 (the schema was not
extended to fit a venue it did not know), and `stable_hash` equality at G5 (a second party derived the
same object from the definitions). Neither is a full answer, and this document does not claim one.

## 9. The gate

Items run **in order**. A failure at any item is `KILLED` for H3 as a whole — not a reason to
reorder, re-scope, or retry with a different venue.

### G1 — Is there anyone to hand a Passport to? *(the H1 item, first on purpose)*

H1 died because the adopter was asserted and never counted. This item is placed **before** any harness
work so H3 cannot repeat it at greater expense.

**G1 is split in two, because Codex's r1 finding 8 is correct**: counting programs that CPI into two
venues measures *technical exposure*, not *demand for a Passport*. G1a is the on-chain count; **G1b is
the demand test, and G1a passing without G1b is not an adopter.**

#### G1a — the counted surface

**Complete census, not a sample.** r1 specified a systematic sample whose stride formula did not
produce the size it claimed (Codex r1 finding 4: `N = 3,001` yields 1,501, not 3,000) and whose
inclusion probability for an occasional caller was too low to reach the threshold even if the claim
were true. r2 replaces it with an exhaustive scan inside a **per-venue budgeted window**.

For each venue `v`: `W_v` is the **longest window ending at `S_G0` containing at most `B = 10,000`
finalized transactions for `v`, capped at 24 hours**. Every transaction in `W_v` is fetched — no
sampling, no stride. `W_v` is recorded per venue.

Measured traffic at 2026-08-21 (1,000-signature trailing samples, cost input only, deciding nothing):

| venue | rate (sig/s) | implied `W_v` at `B = 10,000` |
|---|---:|---|
| Phoenix Eternal | **199.8** | **≈ 50 s** |
| Kamino klend | 1.142 | ≈ 2.4 h |
| Jupiter Perps | 0.763 | ≈ 3.6 h |
| marginfi v2 | 0.380 | ≈ 7.3 h |
| Kamino Vaults | 0.217 | ≈ 12.8 h |
| Save | 0.0118 | 24 h cap (≈ 1,020 tx) |
| Drift v2 | 0.0013 | 24 h cap (≈ 113 tx) |

**Direction, stated: `B` is a cost bound and a smaller `B` can only lower `N_multi`.** Choosing it
after seeing the rates cannot manufacture a pass — it can only manufacture a kill. Phoenix's window is
under a minute at present rates, so the count is a **hard lower bound** and a G1a failure may be a
limit of the window rather than of the market. **That is not appealable.** Re-running with a larger
`B` after seeing a near-miss is 延命 and is forbidden.

**Trace normalisation, exactly.** From `getTransaction(sig, {maxSupportedTransactionVersion: 0,
encoding: "jsonParsed"})`: for each top-level instruction at index `i`, emit it at height 1, then emit
`meta.innerInstructions[index == i].instructions` in array order, each at its `stackHeight`
(`null` ⇒ 2). **Groups are never merged**: the caller of an instruction at height `h` is the nearest
preceding entry **within the same group `i`** at height `h − 1`. If none exists, the instruction is
attributed to no caller and is discarded. Partially decoded instructions are attributed by
`programId`, which is present whether or not the instruction was parsed.

**Excluded from the caller set:** the venue programs themselves; System, Compute Budget, both BPF
loaders, SPL Token, Token-2022, and Associated Token Account; and any program whose
`upgrade_authority` at `S_G0` equals the `upgrade_authority` of any venue in V. The last exclusion is
evaluated at `S_G0`, not historically — an authority can change after `W_v`; the direction is that the
exclusion only ever **removes** callers, so it lowers `N_multi`.

| symbol | quantity |
|---|---|
| `N_any` | distinct third-party programs observed calling ≥1 venue by CPI within its `W_v` |
| **`N_multi`** | distinct third-party programs observed calling **≥2 distinct venues** by CPI |
| `A_multi` | distinct upgrade authorities among those `N_multi` programs, read at `S_G0` |
| — | the full caller frequency distribution, not only the headline |

**KILLED if `N_multi` < 10, or `A_multi` < 5.**

*Why these numbers.* `N_multi` counts the population for whom a **shared schema** beats reading two
sets of docs. Ten is the point below which every consumer can be served by hand, which is a
consultancy and not a primitive. `A_multi ≥ 5` exists so that ten programs under three authorities —
three teams' internal tooling — cannot pass. Both are judgement calls, stated openly, frozen now.

**What `A_multi` does not close, admitted:** one operator can deploy ten programs under ten
authorities. On-chain data cannot distinguish that from ten teams. This is why G1b exists.

*Cost:* ≤ 51,000 `getTransaction` calls, plus signature pagination and two `getAccountInfo` per
distinct caller for the loader/authority classification. **~1 hour, one script.** It is the cheapest
item in the gate and it can kill the project on its own.

#### G1b — the demand test *(requires founder authorisation; not executed at G0)*

**At least 2 independent parties** — operating one of the programs counted in G1a, or a vault, router
or agent that allocates capital across venues — state **in writing, before G2 runs**, which fields
they would gate on and what they would do differently given a `FAIL`. Their statements are committed
verbatim.

**KILLED if fewer than 2 such statements are obtained.**

H1 never asked a single adopter, and that is why it built an instrument for nobody. This is the only
item in the gate that is not an on-chain measurement, and it is here deliberately: no on-chain query
can distinguish a program that would use a Passport from a program that merely calls two venues. It
involves contacting third parties, so **it needs an explicit founder ruling before it runs** — it is
pre-registered here, not performed.

### G2 — Discrimination on two real protocols *(items 4 and 6)*

The same probe suite, unchanged, against **Kamino klend** and **Phoenix Eternal** at the hashes in §3,
each with its adapter manifest committed first. Both Passports emitted; all seven fields decided.

**The comparison tuple is defined**: the ordered 7-tuple of `verdict` strings, and nothing else.
Differences in quantities, errors, evidence or slots do **not** count as discrimination (Codex r1
finding 5).

**KILLED if any of:**

| condition | why |
|---|---|
| any field on either venue is `UNPROVEN` | not-proven is a KILL |
| the two verdict 7-tuples are **equal** | the schema does not discriminate, so it carries no information |
| any C3 boundary sweep is inadmissible, or a C4 entitlement test cannot be run | the field measures the fixture, not the venue |
| either venue cannot be run against **cloned mainnet state** | §7 rule 2 — the Phoenix risk named in §4 |
| two runs of the same Passport are not byte-identical | not reproducible |
| any state mutation is found that the run did not print | the harness can manufacture a pass |

*Predictions, recorded before measurement.* Wrong predictions are **not** a kill — they are the
evidence that the instrument tells us something we did not know.

| field | klend (new hash `b1344d19…`) | confidence | Phoenix Eternal | confidence |
|---|---|---|---|---|
| C1 | PASS | high | **open** — Solvo's B1 was never executed | — |
| C2 | PASS | high | PASS in control | high |
| C3 | PASS (reverts) | **low** — klend's withdrawal-cap branch has never been executed | **FAIL** | high |
| C4 | PASS | medium | **FAIL** | high |
| C5 | `PASS_WITH_PUBLIC_DATA` | high | **open** — the stale-clock run has no precedent | — |
| C6 | `DEMONSTRATED` | medium | `DEMONSTRATED` | medium |
| C7 | PASS | high | **open** | — |

klend is re-measured from fresh fixtures at the **new** hash. Solvo's PASS is **not inherited** — per
§4 it belongs to a binary that no longer exists.

*Cost:* the largest item — two probe programs, two fixture sets, the boundary sweep, the entitlement
test, the C6 enumeration. Days.

### G3 — Generalisation to a protocol the schema was not designed against

**marginfi v2**, named in §3 before any field was measured, probed with the **same field definitions
and the same probe structure**, with only its adapter manifest filled in.

**`F_new` counts every departure from the frozen shape**, not only added fields (Codex r1 finding 5):
a new field; a changed verdict domain; an added, removed or renamed member of any `quantities`,
`evidence` or `valid_if` object; a new predicate `encoding` or `op`; a new `AdapterManifest` member; a
change to any procedure text in §5–§7; or any exception carved for this venue.

**KILLED if `F_new` > 0, or if any field is `UNPROVEN`.**

Predictions: C1 `PASS` (medium), C2 `PASS` (medium), C3 `PASS`/reverts (low), C4 `PASS` (medium),
C5 `PASS_WITH_PUBLIC_DATA` (medium), C6 `DEMONSTRATED` (medium), C7 `PASS` (medium).

### G4 — Expiry binds, and it binds on state as well as code *(item 2)*

1. **E1 fires on a real upgrade.** Two real slots, two different `code_hash` values, one program,
   re-derived inside the harness from the definition. The klend pair in §4 is the candidate.
2. **The 12-byte pre-check fires.** `last_deploy_slot` alone flags the stale Passport, without hashing.
3. **E3 fires on state.** ≥1 field on ≥1 venue is flipped by a disclosed byte mutation in a
   `state_deps` range that C6 recorded **with `reachability` established**.
4. **Upgrade cadence, counted correctly.** For each venue in V: bind a finalized endpoint slot at run
   start; derive its ProgramData account at that slot; enumerate that account's signatures over the
   trailing 90 days; **decode every transaction and count only validated loader-v3 deployment
   instructions targeting that exact ProgramData account**, counting `SetAuthority` /
   `SetAuthorityChecked` separately as E2 events. **Fail closed on any pagination or history gap** and
   record the full classified signature list. r1 counted raw signatures, which Codex's finding 7
   correctly identifies as able to manufacture a non-zero cadence out of authority changes.

**KILLED if any of:** no code-hash change can be demonstrated end to end; the `last_deploy_slot`
pre-check does not fire; no field can be flipped by a reachability-established state dependency; any
`valid_if` predicate is inadmissible under E3; **or the median `upgrades_per_90d` across V is 0.**
The last is the item's real teeth: if venues never change, a human reads the docs once and the
expiry machinery — the whole reason to be machine-readable — is decoration.

### G5 — Reconstruction, canonical equality, and consumer cost

- Every field verdict, **and the §3 identity table**, is re-derived from its definition, not from the
  code, by the model that did not produce it, which states **the direction of every discrepancy**.
- The re-derivation emits a Passport whose **`stable_hash` equals** the original's.
- `Q_prose` is computed against the twelve frozen paths of §8.2.
- The per-field consumer check is measured in **account reads at one slot**.

**KILLED if:** any `stable_hash` differs; **or `Q_prose` ≥ 1**; **or any field needs > 4 account reads
per check**; or any deciding number fails independent re-derivation beyond declared slot drift.

### G6 — The numbers are the verdict

Every item above carries its kill number in advance. **If a number is missing, the item is
not-proven, and not-proven is a KILL.**

| step | produces | decided by | on failure |
|---|---|---|---|
| G0 | this document + commit hash in `STATUS.md` | — (precondition) | cannot proceed |
| G1a | `N_any`, `N_multi`, `A_multi`, distribution, per-venue `W_v`, all @slot | one script | `KILLED`, stop |
| G1b | ≥2 written consumer statements | founder-authorised outreach | `KILLED`, stop |
| G2 | 2 Passports × 7 fields, boundary sweeps, entitlement tests, determinism | probe + fixtures | `KILLED`, stop |
| G3 | 1 Passport, `F_new` | same probe structure | `KILLED`, stop |
| G4 | hash pair, pre-check firing, state flip, classified `upgrades_per_90d` | harness + RPC | `KILLED`, stop |
| G5 | `stable_hash` equality, `Q_prose`, reads-per-check, identity re-derivation | the other model | `KILLED`, stop |

## 10. Non-goals, and the anti-延命 rules

Out of scope for the whole of H3, without exception:

- **Any code before G0 is committed, reviewed and released by a founder ruling.**
- **Reviving H1 in any form** — certification, attestation, receipts, the verifier, `MandateSpec`, the
  guard program, the gallery, `attest/`, the agent framing. A Passport has no signer, no receipt, no
  score and no gallery; it is a measurement record with an expiry rule.
- **Resuming H2** — frozen; resuming needs a founder ruling and its own re-frozen G0.
- **Editing the sibling `../solvo` repo.** Read-only reference.
- Registries, hosted services, indexers, UI, dashboards, demos, tokens, deployment.
- Real funds, mainnet or devnet writes, force push, committed secrets.
- **Any claim stated at a tier above the one that produced it.** Everything here is V2 at best.

**The anti-延命 rules.** After G0 commits: the venue set V, the budget `B` and the 24-hour cap, the
field definitions, the adapter-manifest shape, the twelve questions and their paths, the
canonicalisation rules, and every threshold above are **frozen**. If a gate item fails, it fails.
Re-running with another venue, a longer window, a larger budget, a looser bound, or a proxy that is
easier to pass is 延命 and is forbidden. A different V or `B` is a **different gate** and starts over
at G0.

## 11. Three ways H3 is already dead, stated by the author

Adjudicated by the independent review and the founder, not here.

1. **It may be H1 wearing a different hat, and r1's version of G1 did not fix it.** Codex's finding 8
   is that counting CPI callers substitutes a technical population for an adopter — precisely H1's
   error. r2 splits the item and adds **G1b**, which asks real parties in writing. If G1b cannot be
   satisfied, H3 dies of H1's disease, and that will be the second time in this repo.
2. **It may be true and useless.** Every field could be answerable by a competent integrator reading
   the docs once. The counter is measurable and is G4's cadence number: a one-time human read decays
   at the rate the code changes, and klend's binary changed 8,994 slots after a careful capture. If
   the median venue never upgrades, that counter is false and the gate says so.
3. **The FAIL example is borrowed knowledge, and one leg of it may not be reproducible.** Phoenix's
   failure is known from `../solvo` — and Solvo obtained it on a *localnet fixture*, not cloned
   mainnet state (§4). §7 forbids that substitution, so H3 may be unable to reproduce its own FAIL
   example at all. That is a KILL at G2, and it is named here rather than discovered later.

## 12. What G0 does not authorise

`G0` is a precondition and carries **no verdict**. Committing this document does not start G1.
Nothing is built, no fixture is fetched, no probe is written, and **no third party is contacted**,
until the founder rules on this gate after reading the independent review.
**`GO` ends a phase; it does not start the next one.**

## 13. Changelog r1 → r2, with the direction of every change

Against `reviews/H3-G0-passport.md` (Codex, `CHANGES`, 8×P0 + 2×P1 + 1×P2). **Direction is stated per
change**; H2's post-mortem records that a changelog claiming every change is adverse, when some are
not, is itself the flattering-error pattern.

| # | finding | change | direction |
|---|---|---|---|
| 1 | C3/C5/C6 semantics deferred | scarcity range pre-registered in a hash-pinned adapter manifest; **boundary sweep** `r+1/r/r−1/0` required for admissibility; C5 measured at a **stale clock** too, verdict = worse of the two; C6 candidate universe = the full enumeration of schema fields of every account in the value-out instruction | **against the project** — three new ways to reach `UNPROVEN`, which is a KILL |
| 2 | C4 containment neither necessary nor sufficient | **entitlement test is decisive** (does value arrive with no new request); range novelty demoted to corroborating evidence; `UNPROVEN` when no settlement path can be established | **against** on false passes, **for** on the telemetry-counter false fail Codex named. Mixed, and stated as mixed |
| 3 | identity not bound to ProgramData | E1 is now a 6-step fail-closed procedure incl. program-account tag `2`, ProgramData linkage, same-slot context, `Option` tag semantics | **against** — more ways to fail closed |
| 4 | G1 sample broken and underpowered | sampling **removed**; complete census in a per-venue budgeted window; exact trace-normalisation algorithm; exclusion direction stated | **for the project** on power (a census sees more than a broken sample), **against** on Phoenix's ~50 s window. Net effect unknown before the measurement, and that is the honest statement |
| 5 | G2 "agree" undefined; `F_new` too narrow | comparison tuple = the 7 verdicts only; `F_new` counts any departure from the frozen shape | **against** |
| 6 | `stable_hash`/`Q_prose` not executable; Q6/Q7/Q8 unanswered | complete Passport schema and canonicalisation; `stable_hash` excludes exactly one path; questions become typed paths; **C7 added** rather than dropping question 6 | **against** — a seventh field to decide, and dropping the question would have been the easy move |
| 7 | ProgramData signatures ≠ upgrades | decode and count only loader deployment instructions; `SetAuthority` counted separately; fail closed on gaps | **against** — the cadence KILL becomes reachable |
| 8 | H3 is H1 in costume | G1 split; **G1b added** — ≥2 written consumer statements before G2, founder-authorised | **against** — a new item that can kill the project on its own |
| 9 | C2 tolerance too loose | ratio `1e-6` replaced by **≤ 1 atom absolute** | **against** |
| 10 | two factual overstatements | §4 now distinguishes klend's cloned mainnet state from Phoenix's localnet fixture; slot corrected to 440,477,781 and the gap to **8,994**; §7 forbids substituting a fresh fixture | **against** — it names a way G2 can fail that r1 hid |
| 11 | STATUS wording, cost estimate | `STATUS.md` records the artifact commit; G1a cost restated with pagination and classification reads | neutral |

**One r1 claim is withdrawn as wrong, not softened:** r1 said C4's containment rule "captures the
thing exactly". It does not, and Codex's counterexamples are correct.

**Rounds are themselves evidence.** H2's G0 reached three review rounds and 331 lines without a
measurement, and `docs/GATE.md` names that failure mode explicitly. **This gate will not go to r3.**
If the r2 review returns `CHANGES` with P0s of the same class — deciding semantics still deferred —
that is the finding, it is reported to the founder as-is, and the phase stops.
