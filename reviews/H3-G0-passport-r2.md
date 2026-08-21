<!--
Independent review by Codex (codex-cli 0.148.0-alpha.21), read-only sandbox, no network.
Artifact reviewed: docs/H3-GATE.md at commit 8d92bf1 (r2), against its own r1 review at
reviews/H3-G0-passport.md and the r1 artifact at 4ef5efd.
Produced by: codex exec -C /Users/hiroyusai/src/probatio-svm -s read-only
Transcribed verbatim; no finding was edited, reordered or softened by the author of the artifact.
-->

# H3 / G0 Composability Passport — independent review, r2

**VERDICT: CHANGES**

r2 genuinely fixes identity binding, C2 tolerance, the two Solvo factual errors, and deployment-history counting. It does not finish the decisive semantics for C3/C4/C6, makes the pair-valued C7 field expire incorrectly, leaves the claimed canonical Passport non-canonical, and turns H1’s adopter question into two easily satisfied written formalities. These are P0s of the same deferred-semantics/adopter class as r1. Per §13, this is the finding to report; it is not a request for r3.

## r1 finding triage

| r1 | Status | Basis |
|---|---|---|
| 1. C3/C5/C6 semantics deferred | **PARTIALLY CLOSED** | C5 now tests a stale clock, but C3 proves a threshold rather than inability to pay, and C6’s universe/mutations remain implementer-selected. |
| 2. C4 containment neither necessary nor sufficient | **PARTIALLY CLOSED** | Redemption is a better signal, but one delayed/claimant-authorised promise can still pass; the procedure also contradicts itself on range novelty. |
| 3. Program identity not bound to ProgramData | **GENUINELY CLOSED** | E1 now derives ProgramData from the executable Program account at one slot and handles loader tags and `Option<Pubkey>` fail-closed. |
| 4. G1 sample broken / not independent consumers | **PARTIALLY CLOSED** | The stride error and trace attribution are fixed, but unequal windows remain underpowered and consumer independence is merely moved to inadequate G1b statements. |
| 5. G2 comparison and G3 `F_new` undefined/narrow | **GENUINELY CLOSED** | The seven-verdict comparison tuple and structural `F_new` rule close the stated r1 hole. |
| 6. `stable_hash` / `Q_prose` not executable | **APPARENTLY CLOSED** | The document calls the schema complete but retains `{…}` placeholders and gives invalid sort keys; typed paths lack selector and answer semantics. |
| 7. ProgramData signatures are not deployment history | **GENUINELY CLOSED** | G4 now requires decoded, target-validated loader deployment instructions and fails closed on gaps. |
| 8. H3 is H1 in a different costume | **PARTIALLY CLOSED** | G1b acknowledges the problem but measures two hypothetical statements, not a demonstrated Passport consumer or independent adopter. |
| 9. C2 permits material partial settlement | **GENUINELY CLOSED** | `requested − delta_destination ≤ 1 atom` closes the one-part-per-million tolerance. |
| 10. Solvo / slot factual overstatements | **GENUINELY CLOSED** | The klend fixture slot and Phoenix localnet distinction are now accurate. |
| 11. STATUS and G1 cost imprecise | **PARTIALLY CLOSED** | Cost now mentions pagination, but `STATUS.md` still omits r2’s actual commit and falsely says every finding is closed. |

## Findings

### 1. P0 — C3’s boundary sweep proves a gate, not “cannot pay”

> “If the outcome does not change exactly at the boundary between `S=` and `S−`, the range is not the paying constraint.” (§5, C3)

A genuine `withdrawal_cap`, rate quota, or account-level policy limit can satisfy all four runs: `r+1` and `r` pay; `r−1` and `0` error. The venue can still have ample liquid assets. C3 then reports `PASS` for a policy/error path rather than the adverse full-payment shortfall the claim names.

The converse is also possible: a correct available-liquidity field can have a boundary at `r + fee`, `r + minimum_reserve`, or a lot-size threshold rather than exactly `r`; the sweep rejects a venue that is actually fine.

“Does not pay in full” is additionally unpartitioned. C2 accepts a one-atom shortfall as `PASS`; an `S− = r−1` run can therefore both be ordinary-language “not full” and C2 `PASS`, leaving C3’s admissibility and final verdict undefined.

Close this only with a source-pinned definition that proves the range is the actual current settlement resource, specifies fees/reserves/rounding in the requested unit, and gives a total outcome table for every sweep result.

### 2. P0 — C4 still lets a genuine promise report `PASS`

> “From the post-`S−` state … restore the scarcity range to `r + 1` and advance the clock by `AdapterManifest.stale_horizon` slots.” (§5, C4)

Construct a venue which overwrites a fixed queue slot with a genuine claimant-owned promise, redeemable by `claim(request_id, claimant-signature)` during the next ten slots. Let `stale_horizon` be the venue oracle horizon, 150 slots. At the prescribed C4 clock position the claim has expired or been swept; no value arrives and no permissionless settlement instruction succeeds. If the successful control overwrites the same fixed slot, there is no range novelty either. C4 reports `PASS` even though the shortfall wrote a real promise.

The shared `stale_horizon` is a C5 freshness constant, not a C4 settlement/redeemability schedule. Advancing directly to it does not establish “at any point,” despite C4’s wording. A claimant-authorised settlement also need not be invocable by “anyone,” so the current test can miss it.

There is also an internal contradiction:

> “PASS iff … no changed byte range in `S−` is absent …” (§5, C4)

versus

> “A stress-only changed range with no redemption is reported, not fatal.” (§5, C4)

The first makes novelty non-PASS; the second says it is nonfatal, without assigning a verdict.

Close this with a pinned pending-record and settlement-state-machine procedure: test every pre-registered valid redemption time, with the claimant authority and any permissionless crank path; define expiry/cancellation outcomes; and make range novelty either diagnostic only or a terminal condition, not both.

### 3. P0 — The adapter manifest relocates C6’s freedom instead of constraining it

> “Candidates are every field … of every account named in the value-out instruction’s account list.” (§5, C6)

> `value_out_ix: { "discriminator": hex, "accounts": [ {role, pubkey|derivation} ] }` (§2)

This is enumerable only after an implementer selects a particular value-out route and its account expansion. The manifest does not define “value-out,” require all equivalent withdrawal routes, require every remaining/dynamic account, or prove that the selected route is representative. A venue can expose a small withdrawal instruction/account list while an admin-controlled config, alternate route, or valid account derivation controls whether the broader capability succeeds. C6 can then return `NOT_FOUND_IN_UNIVERSE` and G4 receives no dependency, even though the authority can disable the capability.

Nor are C6 mutation values pre-registered. Mutating each field to one arbitrary corrupt value can fail to flip a verdict, yielding `NOT_FOUND_IN_UNIVERSE`, even though a valid authority-reachable value does flip it.

This also violates §2’s claim that venue-specific input enters “only” through the manifest: C4’s “schema names as settling pending requests” and C6’s writer/access-control selections are not manifest members.

Close this by pinning a complete instruction-family/account-expansion algorithm, including remaining accounts and alternative value-out routes; defining the finite legal mutation set per field; and requiring `UNPROVEN`, not `NOT_FOUND_IN_UNIVERSE`, when source semantics cannot show that the selected universe covers all state capable of changing C1–C5.

### 4. P0 — C7 is a pair property but Passport expiry is only per venue

> “With `R = AdapterManifest.reference_venue`…” (§5, C7)

> “A Passport whose probe, fixtures or adapter manifest changed must be re-issued.” (§6, E4)

C7’s result is about `(V, R, fixture state of both, call ordering)`. Yet `Identity` contains only the subject program’s code identity. If `R` upgrades or relevant `R` state changes, the `V` Passport remains valid: neither E1/E2 nor the consumer’s stated four-read check expires the C7 claim. `adapter_manifest_hash` does not change when a referenced program upgrades.

The same omission weakens G3: a marginfi adapter can select a different `reference_venue` without `F_new > 0`, because only manifest *shape* changes count. G5 can still reproduce an equal hash for the same selected pair; equality does not establish that the per-venue Passport means what it says after `R` changes.

Close this by making C7 a pair-scoped artifact or by adding a fully identified reference-leg identity/state dependency to C7 and E1–E4, with a globally frozen reference-pair mapping for G2/G3.

### 5. P0 — The “complete” Passport schema cannot produce a deterministic `stable_hash` or `Q_prose`

> `Field := { … quantities: {…the members named in §5, all present…} … }` (§8.1)

> “`deltas`, `changed_ranges`, `created_accounts` and `valid_if` [are] sorted by `(account, offset, len)`.” (§8.1)

The schema remains schematic. `quantities` is not typed per C1–C7; several arrays named in §5 have no element schema or ordering (`settlement_ix_tried`, `range_novelty`, `deltas_composed`, and others). More concretely, `deltas` has no `offset` or `len`, and `created_accounts` is a scalar account string, so the declared sort key is not evaluable.

The twelve paths are not typed queries either. `fields[C1]` has no selector grammar; `fields[*]` has no aggregate semantics; C3’s path to `.quantities.error` is not defined as a field in the schema; and question 12’s path only reads E3 predicates while the question asks when E1–E4 invalidate the claim. Presence of an empty array can count as an answer under:

> “`Q_prose` = the count … whose path is absent, `null`, or `UNPROVEN`.” (§8.2)

That permits irrelevant or empty values to report all twelve questions answered.

Close this with a literal machine-readable field schema, JSON-pointer/selector grammar, exact array element types and sort comparators, and an answer predicate/domain for each question—not merely a present path.

### 6. P0 — G1b is still H1’s asserted adopter, now in written form

> “At least 2 independent parties … state in writing … which fields they would gate on and what they would do differently given a `FAIL`.” (§9, G1b)

Two people can supply identical hypothetical statements, be employees or contractors of one operator, control no capital, and never retrieve, trust, or act on a Passport. The clause neither defines “independent,” binds the writer to a public program/entity, requires a current multi-venue allocation, nor requires an actual decision/change of behavior. A party need only operate a G1a program *or* allocate across venues; neither proves Passport demand.

Thus H3 can pass G1b with two courteous form letters and still build for no consumer—the H1 failure mode.

Close this with pre-registered independently verifiable identity and independence criteria plus a concrete, attributable behavior: for example, a signed policy/integration decision by operators of named public multi-venue capital programs, identifying the Passport inputs and a real accept/reject action they will take. If that cannot be obtained before G2, it is a KILL.

### 7. P1 — G1a is an exhaustive scan of incomparable windows, not a powered market census

> “`W_v` is the longest window … containing at most `B = 10,000` finalized transactions … capped at 24 hours.” (§9, G1a)

Phoenix receives roughly 50 seconds while Save receives 24 hours. A composer must apparently be observed at two venues, each inside that venue’s own `W_v`, but that conjunction is not stated as the counting algorithm. `N_multi` therefore has radically different inclusion probabilities across pairs; a periodic composer can be real and never appear in Phoenix’s 50-second window.

The document honestly accepts false KILLs, but it calls the result a “complete census” and uses it as H1’s population measurement without a common exposure interval or power argument.

Close this by explicitly defining multi-venue inclusion as one observed CPI in each relevant `W_v`, reporting pairwise windows/exposure, and either pre-registering a common observation window or characterising the result solely as a bounded lower bound rather than a consumer-population count.

### 8. P1 — §13 direction audit is not fully correct

| Row | Direction assessment |
|---|---|
| 1 | **Incorrect/incomplete.** Boundary and stale checks are adverse, but limiting C6 to a selected `value_out_ix` account list can make `NOT_FOUND_IN_UNIVERSE` easier to obtain by selecting a small route. That favorable freedom is not labelled. |
| 2 | **Correctly mixed in intent.** Redemption prevents some false passes; demoting telemetry novelty prevents the named false fail. The unresolved C4 procedure is separate. |
| 3 | **Correct.** More identity fail-closed checks are adverse. |
| 4 | **Correctly mixed.** A census improves observation within a window; Phoenix’s short window can lower counts. |
| 5 | **Correct.** A verdict-only equality rule and broader `F_new` increase defined kill paths. |
| 6 | **Incomplete.** C7 adds a field, but path-presence-based `Q_prose` can make G5 easier to pass by treating empty/irrelevant values as answers. |
| 7 | **Correct.** Classifying only deployment instructions can expose a zero cadence. |
| 8 | **Correct as a formal direction.** G1b adds a kill condition, though it does not measure adoption adequately. |
| 9 | **Correct.** One atom is stricter than the prior ratio. |
| 10 | **Correct.** Forbidding Phoenix’s local fixture exposes a G2 KILL path. |
| 11 | **Neutral direction, false completion claim.** The status change does not record r2’s artifact commit. |

### 9. P1 — STATUS is inconsistent with the gate and the reviewed commit

> “G0 artifact commits: r1 `4ef5efd`, r2 `PENDING_FREEZE`” (`STATUS.md`)

The reviewed r2 artifact is commit `8d92bf1`, but `STATUS.md` does not record it. It also says:

> “every finding is closed in r2”

That is not supportable given the P0s above. The linked `reviews/H3-G0-passport-r2.md` is absent at the reviewed commit, which is expected before this review is committed but should not be presented as an existing artifact.

Close this by recording `8d92bf1` as the r2 artifact and describing the review as pending, without asserting its outcome.

## Internal-consistency result

Not internally consistent:

- §2’s “only through manifest” rule conflicts with C4 settlement-path selection and C6 writer/access-control selection outside the manifest.
- C3 has no total result partition because its one-atom C2 allowance conflicts with `S− = r−1`.
- C4’s PASS requirement conflicts with its “not fatal” novelty rule.
- C7 is pair-scoped in §5 but per-program in §§6 and 8, so E4/G3/G5 do not carry its necessary subject identity.
- §8.1’s schema and canonicalisation do not type or sort their own data; §8.2 paths cannot be evaluated from that schema.
- `STATUS.md` claims r2 has closed every r1 finding while omitting the actual r2 commit.

## Local verification

Correctly verified locally:

- `solvo/fixtures/g1/MANIFEST.json` records fixture `slot: 440477781` and `max_response_context_slot: 440477778`.
- `440486775 − 440477781 = 8,994` slots.
- Stripping trailing zero bytes from `solvo/fixtures/g1/programs/klend.so.gz` yields **2,414,913** bytes and SHA-256 **`8eab9f858da01fee962452ad3838570b3340cd43b80a236de8fe5297063d1cda`**.
- Solvo Decision 005 and Spec 012 accurately support r2’s corrected distinction: klend used cloned mainnet accounts; Phoenix used mainnet binaries on a freshly initialised localnet exchange, not cloned mainnet state.
- The cited Solvo shortfall facts are accurate: Phoenix committed with destination delta zero and queued an `InsufficientBudget` request; B1 was not executed; klend’s A2 amount was 119,647,109 and A3 moved 99,999,999 of 100,000,000.

Network remains unavailable: `solana-rpc.publicnode.com` could not be resolved. I therefore did not assume the §3 live identities, authorities, hashes, traffic rates, or “mainnet today” claims hold.