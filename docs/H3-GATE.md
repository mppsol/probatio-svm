# H3 — Composability Passport — kill gate and G0 pre-registration (binding)

**Status: G0 pre-registration. No verdict. No code.**
This document supersedes [`docs/H2-GATE.md`](./H2-GATE.md) as the binding gate.

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
> and is consumed by a population of on-chain programs that exists and can be counted.**

Every clause is a gate item below, and every gate item is decided by a number fixed in advance.

**What this is not.** It is not a security audit, not a claim that a protocol is safe, and not a
score. A Passport says *what happens when you call it*, in bytes, at a named code hash — nothing
about whether calling it is wise.

## 2. The design principle that keeps this finite

H2's G0 died because it needed a byte-level loss schema for six venues before it could measure
anything. H3 avoids that by construction:

> **Every capability field is defined over effects observable in any transaction — token-account
> balance deltas, byte-range diffs of writable accounts, the transaction's error result, returned
> data, and the signer set — and never over a venue-specific struct field.**

Venue-specific knowledge enters at exactly two places, both bounded and both disclosed:

1. choosing *which instruction* to call (one instruction per venue: the one that moves value out);
2. choosing *which state to perturb* to create the stress condition of C3, which must then be
   validated by a control pair (§5, C3) before any verdict is read from it.

Nowhere else may a field's definition mention a venue.

## 3. Subjects — identity, verified read-only

Verified by `getAccountInfo` against `solana-rpc.publicnode.com`, `commitment: finalized`, on
2026-08-21. **`S_G0 = 440,578,912`** is the pre-registration reference slot; the four count-only
rows were read at context slots 440,579,564–440,579,569 and carry that drift openly.

`code_hash` is defined in §6, E1. All seven are loader-v3 (`BPFLoaderUpgradeab1e11111111111111111111111`).

| role | protocol | program id | last deploy slot | upgrade authority | elf bytes | `code_hash` |
|---|---|---|---:|---|---:|---|
| **probe, PASS candidate** | Kamino klend | `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD` | 440,486,775 | `GzFgdRJXmawPhGeBsyRCDLx4jAKPsvbUqoqitzppkzkW` | 2,431,953 | `b1344d1979daec34bea862a3ed5c44ca5dc8b8e72ec32f1a90ac5150229c22d9` |
| **probe, FAIL candidate** | Phoenix Eternal | `EtrnLzgbS7nMMy5fbD42kXiUzGg8XQzJ972Xtk1cjWih` | 437,447,068 | `GPgADQrhzGoUgLqxsZMKvSpwcLaJFVTq6gEixKhmcwpm` | 3,142,561 | `2f1b8a5d60696c7f18d1888a721c0659ec82208abe21e1be1fe8dade5f296e0e` |
| **probe, blind third** | marginfi v2 | `MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA` | 432,875,565 | `J3oBkTkDXU3TcAggJEa3YeBZE5om5yNAdTtLVNXFD47` | 3,161,329 | `26dda5e1a060d8fa5d8cf122518f26be3bdaab68e1dc525f74061e2e74cb38f4` |
| count only | Save (Solend) | `So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo` | 350,614,570 | `RY93CZYe5g6drtG7W9PmHRPzaBLZ1uwihTzayQTmJfh` | 603,425 | `ef621872c5b7c3fdef61fed1e042609f433cff1c408457b526ff665ef5df33f9` |
| count only | Drift v2 | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` | 429,731,225 | `8jj7zJgdr5bDndc7evM74FMGwzLPmd4u4QxNzFi1BMai` | 193,713 | `8299a9b8554010b96b9e09ee6cf1c884870d626d623a617cf56424017215e81f` |
| count only | Jupiter Perps | `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu` | 413,092,478 | `5myNNmEmPm3UAnJ2ggLEpnTFb9t9Gk8369wKw6n3uAKx` | 2,602,905 | `417954cc67bad9291388bfd480a671072ec43b886b04d88d4df2efa7215b6ee0` |
| count only | Kamino Vaults | `KvauGMspG5k6rtzrqqn7WNn3oZdyKqLKwK2XWQ8FLjd` | 432,874,668 | `GzFgdRJXmawPhGeBsyRCDLx4jAKPsvbUqoqitzppkzkW` | 1,195,577 | `5b1eac1695f68e4e3889a10bce0797e96f9cfd0d0aaa8d7b8f4e37d730fbc609` |

Three facts from that table are load-bearing and were **not** assumed in advance:

- **7 of 7 carry a live upgrade authority. None is immutable.** Every one can be replaced by a single
  transaction from a single key. A Passport that is not re-checked at the moment of use therefore
  asserts nothing about the code that will run — §6 is built on this, not on a hypothetical.
- **Kamino Vaults and klend share the upgrade authority `GzFgdRJXmaw…`.** The G1 count of adopters
  must therefore exclude same-authority programs, or it counts a team's own toolkit as a market.
  The rule in §9/G1 exists because of this row, not in the abstract.
- **klend was upgraded at slot 440,486,775** — see §4, which is the reason this project exists.

**Venue set V is frozen here.** Adding a venue later — to the probe set or the count set — is a
different gate and starts over at G0. Widening V would make G1 easier to pass; that direction is
disclosed and the door is closed now, before the count is known.

## 4. The observation that motivates the whole hypothesis, disclosed in full

The sibling repo `../solvo` measured Kamino klend and Phoenix Eternal under LiteSVM against cloned
mainnet accounts and mainnet binaries (`solvo/docs/decisions/005-safe-capital-movement-verdict.md`,
2026-08-21). **Solvo is a read-only reference for H3 and is not edited by this work.**

Its klend fixture was captured at slot **440,477,778**. Its committed `klend.so`, stripped of
trailing zero bytes exactly as E1 defines, is **2,414,913 bytes**, `sha256` =
`8eab9f858da01fee962452ad3838570b3340cd43b80a236de8fe5297063d1cda`.

The klend binary on mainnet today is **2,431,953 bytes**, `sha256` =
`b1344d19…9c22d9`, deployed at slot **440,486,775** — **8,997 slots (≈ 1 hour) after that capture**.

So a capability verdict produced by a careful, executed, independently recomputed experiment was
bound to a binary that had already been replaced **before the verdict was written down**. That is the
entire argument for this hypothesis, and it is a measurement, not a thesis. It is also a warning
about H3 itself: if H3 ships a Passport without §6, H3 ships the same stale claim in a nicer format.

**Disclosure:** this comparison was computed *before* G0 froze. It decides no gate item — it is the
motivation and it is stated so a reviewer can discount it. No number that decides a G-item below has
been measured.

## 5. Item 1 — the Passport schema and the executable proof of each field

A Passport is a JSON object with an **identity block** and **six capability fields**. Every field
carries `verdict`, the raw `quantities` the verdict was computed from, `evidence` (the transaction
that produced it), and `valid_if` (§6, E3). A consumer may re-apply its own predicate to
`quantities`; the `verdict` is the canonical predicate, not the only one available.

`verdict ∈ {PASS, FAIL, UNPROVEN}` unless stated otherwise. **`UNPROVEN` is a KILL** wherever the
gate below counts fields — a capability that could not be decided is not recorded as unknown-but-fine.

### Identity block `I`

`program_id`, `loader`, `programdata`, `code_hash`, `programdata_hash`, `elf_len`,
`upgrade_authority` (base58 or `null`), `last_deploy_slot`, `observed_at_slot`, plus
`harness_hash` (§7) and `fixture_manifest_hash` (§7).

### C1 — `pda_authority`

**Claim under test.** A program that is not the venue, holds no privileged role on it, and appears on
no whitelist, can be the *sole* authority for a value-moving instruction, signing via `invoke_signed`.

**Proof.** The probe program's PDA is the authority on both the venue-side position/obligation account
and the destination token account. The transaction is submitted by a fee payer that is **not** an
authority on any account whose balance changes. The probe calls the venue by CPI.

**PASS** iff the destination token account's `amount` (SPL Token layout, offset 64, `u64` LE), read
back out of the VM after execution, increased, **and** the flattened instruction trace contains no
signer other than the fee payer and the probe PDA. **FAIL** iff the venue returns an error naming the
authority, or the move requires an additional signature. **UNPROVEN** iff the probe cannot be placed in
the authority seat at all (record the exact obstacle).

*Quantities:* `signers[]`, `delta_destination`, `error` (verbatim), `whitelist_checked` (bool + the
account or constant consulted).

### C2 — `atomic_settlement`

**Claim.** The value moves in the same transaction as the request.

**Proof.** Same successful transaction as C1. Read `amount` before and after **from account bytes in
the VM**, never from logs.

**PASS** iff `delta_destination > 0` **and** `delta_destination / requested ≥ 0.999999`.
**FAIL** otherwise. The ratio bound is stated so a caller can distinguish a rounding-down (klend
returned 99,999,999 of 100,000,000 requested in Solvo's A3 — a `1e-8` shortfall) from a partial fill.
The raw pair is always recorded, so a consumer with a stricter tolerance applies its own.

*Quantities:* `requested`, `delta_destination`, `ratio`, `slots_elapsed` (must be 0).

### C3 — `revert_on_shortfall`

**Claim.** When the venue cannot pay in full, the instruction **returns an error** rather than
committing a transaction that moves nothing.

**Proof, and the control pair that makes it valid.** The venue's own scarcity mechanism — the state it
reads to decide whether it can pay — is set below the request. Two runs are mandatory:

- **control** — mechanism set so it does **not** bind. Must produce C2 `PASS`. If it does not, the
  stress condition was never created and **C3 is `UNPROVEN`**, not `PASS`.
- **stress** — mechanism set so it binds.

**PASS** iff the stress run's transaction result is `Err`. **FAIL** iff it commits with
`delta_destination == 0`, or commits with `0 < delta_destination < requested`. Where the venue has a
**second** independent limiting parameter, a third run holds that parameter at the binding value while
the first does not bind, as an attribution control; without it the field records
`attribution: single-parameter` and the gate counts it as `UNPROVEN`.

This control requirement is not procedural caution. Solvo's first Phoenix run reported the opposite
verdict because writing `remaining_budget` alone is neutralised by a token bucket refilling from
`last_update_slot = 0`: **no shortfall was ever created and it read exactly like a clean pass.**

*Quantities:* `error` (verbatim), `delta_destination` in all runs, `return_data_len`,
`mechanism` (account, offset, len, before, after — as disclosed by the run itself).

### C4 — `no_deferral_record`

**Claim.** The venue does not answer a request it cannot satisfy by writing a promise.

**Proof — venue-agnostic, no struct knowledge required.** For every writable account in the
transaction, diff its bytes before and after and reduce to *changed ranges* (maximal contiguous runs
of differing bytes). Then:

**PASS** iff **all** of:
1. every changed range in the **stress** run is contained in the union of changed ranges of the
   **same account** in the **control** run;
2. no account is created (lamports `0 → >0`) in the stress run that is not created in the control run;
3. no account's data length increases in the stress run.

**FAIL** if any is violated; the violating account and byte ranges are recorded in the Passport.

The definition captures the thing exactly: *a deferral record is state written on the failure path
that the success path does not write.* Phoenix's queue node — `state Queued`,
`last_transition_reason InsufficientBudget`, appearing only under shortfall — is caught by rule 1
without H3 ever knowing Phoenix's queue layout.

*Quantities:* `changed_ranges` per account for both runs, `created_accounts[]`, `len_deltas[]`.

### C5 — `permissionless_settlement`

**Claim.** Settlement needs no keeper, crank, admin, or freshness precondition the caller cannot
satisfy inside its own transaction.

**Proof.** Run the control case with **no slot advance** between request and settlement, with the probe
building every precondition instruction itself.

Four-valued: **`PASS_SELF_CONTAINED`** (value moves with no precondition instructions);
**`PASS_WITH_PUBLIC_DATA`** (moves, but requires precondition instructions the probe builds from
public accounts — klend's three refreshes are expected to land here); **`FAIL_PRIVILEGED`** (requires a
signature the caller cannot produce, a second transaction, or an action by a named keeper/admin);
**`UNPROVEN`** (requires an artifact whose availability could not be established — e.g. an off-chain
signed price blob with no permissionless posting path; record which).

*Quantities:* `precondition_ixs[]` (program id + count), `extra_accounts` (count),
`privileged_signers[]`, `cu_total` for the whole settlement transaction.

### C6 — `admin_reachable_surface`

**Claim.** The set of state that the venue's own authority can change, without a code upgrade, to flip
any of C1–C5, is **measured** rather than asserted.

**Proof.** For each candidate byte range, mutate the cloned account, re-run the affected field, and
record the range **iff the verdict flips**. Candidate ranges may come only from (a) a published schema
or IDL, or (b) published reader structs of the venue's own SDK. **Guessed offsets are forbidden**; a
venue with neither source yields `UNPROVEN` for C6 and, per the rule above, a KILL at that venue.
Every mutation is printed by the run: account, offset, len, before, after (§7).

**Value:** `state_deps[] = {account, offset, len, field_flipped, predicate}` and
`admin_can_disable: bool`. This is not a judgement of the venue. It is what makes E3 executable: it
names the state a consumer must re-read, and it is the difference between "we believe this is
config-dependent" and "we flipped it".

### Aggregate

There is **no score and no grade**. The Passport carries six fields; the adoption decision is the
consumer's policy applied to them (§8).

## 6. Item 2 — expiry and revocation

Four rules. E1 and E2 void the whole Passport; E3 suspends one field; E4 voids issuance.

**E1 — code.** `code_hash` is defined byte-exactly, per loader:

- loader-v3 (`BPFLoaderUpgradeab1e…`): programdata layout is `[0..4]` tag `= 3`, `[4..12]` deploy slot
  `u64` LE, `[12]` authority `Option` tag, `[13..45]` authority, `[45..]` ELF followed by zero
  padding. `code_hash = sha256(data[45..]` with **all trailing `0x00` removed**`)`, and
  `programdata_hash = sha256(data)` over the untrimmed account. **Both are recorded**, with
  `elf_len`: trimming is not injective in principle, and the triple closes it.
- non-upgradeable (`BPFLoader2111…`, `BPFLoader1111…`): `code_hash = sha256(account.data)`,
  `programdata_hash = code_hash`, `upgrade_authority = null`.
- **any other loader, including loader-v4: `UNSUPPORTED_LOADER`.** The whole Passport is `UNPROVEN`.
  Fail closed; guessing a header offset is exactly the failure H2's post-mortem names.

A mismatch on **either** hash ⇒ `EXPIRED`. The consumer's cheap pre-check is `last_deploy_slot`
(12 bytes at a fixed offset): if it exceeds the recorded value the Passport is `EXPIRED` **without
hashing anything**. It is conservative on purpose — redeploying identical bytes bumps the slot and
expires the Passport, which errs toward re-verification.

**E2 — authority.** `upgrade_authority` differing from the recorded value ⇒ `EXPIRED`; the party able
to swap the code is part of the claim. And the honest consequence of §3: **for a program with a live
upgrade authority, no Passport is valid ahead of time.** What it provides is not a guarantee but a
**cheap re-check** — one account read at decision time instead of a re-audit. A Passport for a
mutable program that a consumer does not re-check is worth nothing, and the schema says so in a
field (`mutable: true`) rather than in prose.

**E3 — state.** Each field carries `valid_if`: predicates over live state, not recorded values (a
budget that refills is not a change). A predicate is
`{account, offset, len, encoding, op, rhs}` where `rhs` may reference the caller's own request —
e.g. *"`remaining_budget` at (account, offset, 8, u64le) `≥` request"*. Any false predicate ⇒ that
field is `SUSPENDED`, and a consumer policy requiring it must reject. `valid_if` may contain **only**
predicates measured by C6; a hand-written predicate is not admissible.

**E4 — harness.** `harness_hash` and `fixture_manifest_hash` are part of the identity block. A
Passport whose probe or fixtures changed must be re-issued, so a verdict cannot outlive the code that
produced it. This closes "we changed the probe and kept the verdict".

**Evaluation cost, which is part of the claim.** A consumer's full check must be ≤ **4 account reads
per field at a single slot**, with no historical scan and no indexer. A field whose `valid_if` cannot
be evaluated inside that budget is not a primitive and is recorded as such (see G5).

## 7. Item 3 — the minimal harness

Tier **V2** in the sibling repo's ladder: BPF under LiteSVM against real cloned venue accounts. Not
V3 — nothing is deployed anywhere, and no claim may be stated at a tier above the one that produced it.

1. **Binaries.** Each venue's ELF is fetched by `getAccountInfo` on its programdata and trimmed
   exactly as E1 defines. **The harness asserts `sha256(loaded_elf) == I.code_hash` before executing
   anything** and aborts otherwise. The binary tested and the binary named by the Passport are then
   the same bytes by construction.
2. **State.** Accounts cloned at one named slot `S1` (the first finalized slot at or after the G0
   commit), with `MANIFEST.json` recording `pubkey, owner, len, sha256, slot` per account and the
   RPC's `max_response_context_slot` for the batch. `fixture_manifest_hash = sha256` of the canonical
   manifest.
3. **The probe.** One SBF program per venue-shaped call, on no whitelist, holding every authority
   through a single PDA. Ops: `MOVE_OUT`, `MOVE_IN`, `MOVE_OUT_THEN_FAIL`, `CONTROL`,
   `MOVE_OUT_UNDER_STRESS`. `harness_hash = sha256` of the probe ELF.
4. **Assertions read account bytes back out of the VM. No assertion may read a log line.** Logs may
   be printed; they may not be asserted on.
5. **Disclosed mutations.** Every byte of cloned state the harness writes — repointing an `owner` to
   the probe PDA, setting a scarcity parameter — is printed by the run itself: account, offset, len,
   before, after, and why. **A mutation that is not printed invalidates the run.** Every other byte
   stays as fetched.
6. **Determinism.** Two runs byte-identical, including the emitted Passport JSON and its hash.
7. **Offline.** Once fixtures are committed the run touches no network. Fetching is a separate,
   explicitly-invoked step.
8. **Output.** The canonical Passport JSON (§8) plus a human table. The JSON is the artifact; the
   table is a convenience.

Known traps, carried from the sibling repo's executions rather than rediscovered: `cargo build-sbf`
defaults to `--arch v0` and can abort on stack frame size — use `--arch v1` or later with a separate
`CARGO_TARGET_DIR` per architecture; a probe crate needs its own `[workspace]` and a root `exclude`
entry, because a second copy of the unbundled solana crates alongside litesvm breaks the build.

## 8. Item 5 — why this is a primitive and not a test report

Argued nowhere; decided by three numbers at G5.

1. **Canonical form.** Serialization is fixed: UTF-8 JSON, object keys sorted by Unicode code point,
   no insignificant whitespace, **no floating point anywhere** (integers as decimal strings, ratios as
   explicit numerator/denominator pairs), bytes as lowercase hex, pubkeys as base58, arrays in the
   order each field specifies. `stable_hash = sha256` over the Passport **excluding**
   `observed_at_slot` and live-state snapshot values; `passport_hash = sha256` over everything. Two
   independent issuers of the same claim produce the **same `stable_hash`** — an equality, not an
   opinion, and the G5 kill.
2. **The decision is a pure function.** `decide(passport, policy, live_state) → ADOPT | REJECT |
   EXPIRED | SUSPENDED`, where `policy` is a consumer-written predicate set over fields and
   quantities. No prose is read. The falsifier: **`Q_prose`** — of a list of adoption questions
   pre-registered below, how many cannot be answered from the Passport alone.
3. **It generalises.** The blind third venue (G3): **`F_new`**, the number of fields that had to be
   added or redefined to decide a protocol the schema was not designed against. A schema that needs a
   new field per protocol is a report format, not a primitive.

**The adoption questions, drawn from what an integrator actually failed on, not invented to fit the
schema.** Each is sourced; sources are external to H3.

| # | question | source |
|---|---|---|
| 1 | Can my program's PDA be the authority, or do I need a human signer? | klend `CPI_WHITELISTED_ACCOUNTS` gate, `solvo` decision 005 §2 |
| 2 | If I am not whitelisted, does the call revert? | klend `CpiDisabled` (`Custom(6080)`), same |
| 3 | Does the money arrive in the same transaction? | `solvo` decision 005 §3 |
| 4 | If the venue cannot pay, do I get an error or a successful no-op? | `solvo` decision 005 §0 — the measured Phoenix branch |
| 5 | Does a failed call leave a record that pays me later? | `solvo` decision 005 §3, queue node |
| 6 | Can I compose two venues in one transaction and roll both back? | `solvo` decision 005 §2, A4a/A4b |
| 7 | What must I refresh first, and can I do it myself? | klend `check_refresh` / `refresh_farms!`, `solvo` spec 012 Leg A |
| 8 | How much compute does the whole settlement cost? | `solvo` decision 004 — 1,037,241 CU, 74% of the real ceiling |
| 9 | Can an admin turn any of this off without redeploying? | Phoenix budget/queue parameters, `solvo` decision 000 §1.5 |
| 10 | Is the program I audited the program that will run? | §4 of this document — klend, 8,997 slots |
| 11 | Who can replace the code, and is it immutable? | §3 of this document — 7 of 7 mutable |
| 12 | When does what I was told stop being true? | §6, E1–E4 |

**`Q_prose` = the count of these twelve that a consumer cannot answer from a Passport plus ≤4 account
reads per field.** The list is frozen here, before any field was measured, and may not be edited to
match what the schema turns out to support.

## 9. The gate

Items run **in order**. A failure at any item is `KILLED` for H3 as a whole — not a reason to
reorder, re-scope, or retry with a different venue. Costs are stated so the founder can stop early.

### G1 — Is there anyone to hand a Passport to? *(the H1 item, first on purpose)*

H1 died because the adopter was asserted and never counted. This item is placed **before** any
harness work so that H3 cannot repeat it at greater expense.

**Window** `W = [S_G0 − 216,000, S_G0]` — 216,000 slots ≈ 24 h ending at slot 440,578,912.

**Method.** For each of the seven venues in V: enumerate signatures in `W` via
`getSignaturesForAddress` (paginated, finalized); take a **deterministic systematic sample** of
`min(3000, N)` by selecting every `⌈N/3000⌉`-th signature from the newest, no RNG; fetch each with
`getTransaction(maxSupportedTransactionVersion: 0, encoding: "jsonParsed")`; flatten
`instructions` + `meta.innerInstructions` in execution order, taking a top-level instruction's stack
height as 1. For every instruction whose program is a venue at stack height `h ≥ 2`, the **caller** is
the nearest preceding instruction in flattened order with stack height `h − 1`.

**Excluded from the caller set:** the venue programs themselves; System, Compute Budget, both loaders,
SPL Token, Token-2022, and Associated Token Account; and **any program whose `upgrade_authority`
equals the `upgrade_authority` of any venue in V** — the `GzFgdRJXmaw…` row in §3 is why.

| symbol | quantity |
|---|---|
| `N_any` | distinct third-party programs observed calling ≥1 venue by CPI in `W` |
| **`N_multi`** | distinct third-party programs observed calling **≥2 distinct venues** by CPI in `W` |
| `A_multi` | distinct upgrade authorities among those `N_multi` programs |
| — | the full caller frequency distribution, not only the headline |

**KILLED if `N_multi` < 10, or `A_multi` < 5.**

*Why these numbers.* `N_multi` counts the population for whom a **shared schema** beats reading two
sets of docs; a single-venue integrator reads one page once and needs no primitive. Ten is the point
below which every consumer can be served by hand, individually, which is a consultancy and not a
primitive. `A_multi ≥ 5` exists because ten programs under three authorities is three teams' internal
tooling — the counted form of H1's mistake. Both are judgement calls stated openly and **frozen now**.

*Direction of the error, disclosed.* The sample under-counts: a composer with few transactions in a
24 h window may be missed entirely. The measured `N_multi` is a **lower bound**, so the item fails
closed — and a fail may be a limit of the sample rather than of the market. **That is not appealable.**
Re-running with a longer window or a bigger sample after seeing a near-miss is 延命 and is forbidden.
Both parameters are frozen here, before the count is known.

*Cost:* ~21,000 RPC reads, ~30 min, no code beyond a single script. **This is the cheapest item and
it can kill the project on its own.**

### G2 — Discrimination on two real protocols *(items 4 and 6)*

The same probe suite, unchanged, against **Kamino klend** and **Phoenix Eternal** at the hashes in §3.
Both Passports emitted; all six fields decided on both.

**KILLED if any of:**

| condition | why |
|---|---|
| any field on either venue is `UNPROVEN` | not-proven is a KILL; an undecidable field is not a capability |
| the two Passports agree on **all six** fields | the schema does not discriminate, so it carries no information |
| any C3 control run fails (the stress condition was never created) | the field measures the fixture, not the venue |
| two runs of the same Passport are not byte-identical | not reproducible |
| any state mutation is found that the run did not print | the harness can manufacture a pass |

*Predictions, recorded before measurement.* Wrong predictions are **not** a kill — they are the
evidence that the instrument tells us something we did not know. Refusing to record them would be.

| field | klend (new hash `b1344d19…`) | confidence | Phoenix Eternal | confidence |
|---|---|---|---|---|
| C1 | PASS | high | **open** | — Solvo's B1 was never executed; no basis for a prior |
| C2 | PASS | high | PASS in control | high |
| C3 | PASS (reverts) | **low** — klend's withdrawal-cap branch has never been executed | **FAIL** | high |
| C4 | PASS | medium | **FAIL** | high |
| C5 | `PASS_WITH_PUBLIC_DATA` | high | `PASS_WITH_PUBLIC_DATA` | low |
| C6 | non-empty | high | non-empty | high |

klend is re-measured from fresh fixtures at the **new** hash. Solvo's PASS is **not inherited** — per
§4 it belongs to a binary that no longer exists, and if the new binary behaves differently that is a
finding, not a failure.

*Cost:* the largest item — two probe programs, two fixture sets, the C3/C4 control machinery. Days.

### G3 — Generalisation to a protocol the schema was not designed against *(item 5, clause 3)*

**marginfi v2**, named in §3 **before** any field was measured, probed with the **same field
definitions and the same probe structure**, adding only the venue-specific instruction choice and
stress parameter that §2 permits.

Predictions, recorded now: C1 `PASS` (medium), C2 `PASS` (medium), C3 `PASS`/reverts (low),
C4 `PASS` (medium), C5 `PASS_WITH_PUBLIC_DATA` (medium), C6 non-empty (high).

**KILLED if `F_new` > 0** — any field added, removed, or redefined to make marginfi decidable — **or
if any field is `UNPROVEN`.**

### G4 — Expiry binds, and it binds on state as well as code *(item 2)*

1. **E1 fires on a real upgrade.** Two real slots, two different `code_hash` values, one program. The
   klend pair in §4 is the candidate; it must be re-derived inside the harness, from the definition.
2. **The 12-byte pre-check fires.** `last_deploy_slot` alone flags the stale Passport, without hashing.
3. **E3 fires on state.** ≥1 field on ≥1 venue is flipped by a **disclosed byte mutation** in a
   `state_deps` range that C6 produced by measurement.
4. **Upgrade cadence.** For each venue in V, count deployments in the trailing 90 days
   (`getSignaturesForAddress` on the **programdata** account, finalized) and report
   `upgrades_per_90d` per venue.

**KILLED if any of:** no code-hash change can be demonstrated end to end; the `last_deploy_slot`
pre-check does not fire; no field can be flipped by a measured state dependency; any `valid_if`
predicate is hand-written rather than produced by C6; **or the median `upgrades_per_90d` across V is
0.** The last is the item's real teeth: if venues never change, a human reads the docs once and the
Passport's expiry machinery — its whole reason to be machine-readable — is decoration.

### G5 — Reconstruction, canonical equality, and consumer cost *(item 5, clauses 1 and 2)*

- Every field verdict is **re-derived from its definition, not from the code**, by the model that did
  not produce it, which states **the direction of every discrepancy**.
- The re-derivation emits a Passport whose **`stable_hash` equals** the original's.
- `Q_prose` is computed against the twelve frozen questions of §8.
- The per-field consumer check is measured in **account reads at one slot**.

**KILLED if:** any `stable_hash` differs; **or `Q_prose` ≥ 1**; **or any field needs > 4 account reads
per check**; or any deciding number fails independent re-derivation beyond declared slot drift.

`Q_prose ≥ 1` is deliberately unforgiving. A Passport that answers eleven of twelve questions and
sends the reader to the docs for the twelfth has not replaced reading the docs.

### G6 — The numbers are the verdict

Every item above carries its kill number in advance. **If a number is missing, the item is
not-proven, and not-proven is a KILL.**

| step | produces | decided by | on failure |
|---|---|---|---|
| G0 | this document + commit hash in `STATUS.md` | — (precondition) | cannot proceed |
| G1 | `N_any`, `N_multi`, `A_multi`, distribution, all @slot | one script, one command | `KILLED`, stop |
| G2 | 2 Passports, 6 fields each, control runs, determinism proof | probe + fixtures | `KILLED`, stop |
| G3 | 1 Passport, `F_new` | same probe structure | `KILLED`, stop |
| G4 | hash pair, pre-check firing, state flip, `upgrades_per_90d` | harness + RPC | `KILLED`, stop |
| G5 | `stable_hash` equality, `Q_prose`, reads-per-check | the other model, from definitions | `KILLED`, stop |

## 10. Non-goals, and the anti-延命 rules

Out of scope for the whole of H3, without exception:

- **Any code before G0 is committed, reviewed and released by a founder ruling.** This phase produces
  specification only.
- **Reviving H1 in any form** — certification, attestation, receipts, the verifier, `MandateSpec`, the
  guard program, the gallery, `attest/`, the agent framing. None of it is H3's foundation. A Passport
  has no signer, no receipt, no score and no gallery; it is a measurement record with an expiry rule.
- **Resuming H2** — it is frozen, and resuming needs a founder ruling and its own re-frozen G0.
- **Editing the sibling `../solvo` repo.** It is a read-only reference.
- Registries, hosted services, indexers, UI, dashboards, demos, tokens, deployment.
- Real funds, mainnet or devnet writes, force push, committed secrets.
- **Any claim stated at a tier above the one that produced it.** Everything here is V2 at best.

**The anti-延命 rules.** After G0 commits: the venue set V, the window `W`, the sample size, the field
definitions, the twelve questions, and every threshold above are **frozen**. If a gate item fails, it
fails. Re-running with another venue, a longer window, a larger sample, a looser bound, or a proxy
that is easier to pass is 延命 and is forbidden. A different V or W is a **different gate** and starts
over at G0.

## 11. Three ways H3 is already dead, stated by the author

Adjudicated by the independent review and the founder, not here. No model reviews its own output.

1. **It may be H1 wearing a different hat.** H1 built a real instrument for a subject that did not
   exist. H3 builds a real instrument for a *consumer* that may not exist. The only defence is that
   G1 counts that consumer first, cheaply, with a number fixed before the count — and if G1 fails,
   H3 dies of H1's disease, which will be the second time in this repo.
2. **It may be true and useless.** Every field could be answerable by a competent integrator reading
   the docs once. The counter is measurable and is G4's cadence number: a one-time human read decays
   at the rate the code changes, and klend's binary changed 8,997 slots after a careful capture. If
   the median venue never upgrades, that counter is false and the gate says so.
3. **The FAIL example is borrowed knowledge.** Phoenix's failure is known from `../solvo`, so G2 is
   partly a re-measurement of a known answer. The defences are the blind third venue at G3 with
   `F_new = 0`, and the predictions recorded in §9 — including the honest `open` for Phoenix C1,
   which Solvo never executed.

## 12. What G0 does not authorise

`G0` is a precondition and carries **no verdict**. Committing this document does not start G1.
Nothing is built, no fixture is fetched, no probe is written until the founder rules on this gate
after reading the independent review. **`GO` ends a phase; it does not start the next one.**
