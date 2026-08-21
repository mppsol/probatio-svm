<!--
Independent review by Codex (codex-cli 0.148.0-alpha.21), read-only sandbox, no network.
Artifact reviewed: docs/H3-GATE.md at commit 4ef5efd (branch h3/g0-composability-passport).
Produced by: codex exec -C /Users/hiroyusai/src/probatio-svm -s read-only
Transcribed verbatim; no finding was edited, reordered or softened by the author of the artifact.
-->

# H3 / G0 Composability Passport — independent review

**VERDICT: CHANGES**

This is not yet a measurement contract. It leaves several ways to convert an adverse capability result, an absent consumer population, or a non-generalising schema into a reported pass. The most serious defect is C4: byte-range containment cannot identify a deferral record without its semantics, and it produces both false passes and false fails. H3 also repeats H1’s adopter error: G1 counts program IDs observed in a thin sample, not consumers who would use a Passport.

## Findings

### 1. P0 — C3, C5, and C6 defer the deciding semantics

> “The venue's own scarcity mechanism — the state it reads to decide whether it can pay — is set below the request.” (§5, C3)

> “Candidate ranges may come only from (a) a published schema or IDL, or (b) published reader structs…” (§5, C6)

Neither clause identifies a source version/hash, complete candidate set, byte range, mutation values, request amount/unit, or a procedure proving that the mutated state is both a real scarcity condition and admin-reachable.

A later implementer can call a different checked field a “scarcity mechanism,” mutate it into an error path, and report C3 `PASS` because the stress transaction returns `Err`, even if the real underfunded path commits with no payment. That changes C3 from the adverse `FAIL` to `PASS`, preserving G2/G3. The C3 control only proves that one chosen non-stress run succeeds; it does not prove the stress error represents inability to pay.

C5 similarly tests only a fresh cloned state with no slot advance. A protocol whose settlement succeeds while its oracle/cache is fresh but requires a privileged keeper after data becomes stale can be reported `PASS_SELF_CONTAINED` or `PASS_WITH_PUBLIC_DATA`. C6 permits selection of a convenient subset of IDL/SDK ranges and single-byte corruptions. It neither proves the set is exhaustive nor that a demonstrated byte corruption is an authority-reachable state transition. An omitted admin kill switch makes `admin_can_disable: false` or an incomplete `state_deps` set look like a pass; a corrupting mutation can manufacture the G4 “state flip.”

Close this by pre-registering, per venue and field: immutable source revision/hash; exhaustive candidate-range derivation; exact range/value mutations; request units and amounts; control/stress transaction bytes; and the real authority instruction or a byte-level access-control proof for every claimed C6 dependency. If that requires venue-specific schemas, it is evidence that the proposed venue-agnostic field is not yet defined.

### 2. P0 — C4’s set-difference rule does not detect promises

> “PASS iff … every changed range in the stress run is contained in the union of changed ranges … in the control run.” (§5, C4)

This proposition is false: path-exclusive bytes are neither necessary nor sufficient for a deferral record.

False pass: an existing fixed-size request record at bytes `[0..64]` is overwritten on every withdrawal. The successful control writes `{state: Settled, amount, nonce}`, while a shortfall writes `{state: Queued, claim_amount, nonce}`. Both runs change `[0..64]`; no account is created and no length changes. The stress run has written a genuine payable-later promise, but C4 is `PASS`.

False fail: a venue commits a no-payment shortfall without any promise, but increments `failed_withdrawals` or writes `last_shortfall_slot` in an existing telemetry/accounting record. That stress-only range makes C4 `FAIL`, although no deferred claim exists.

The same hole exists for promises represented by a balance change in an already-existing escrow/token account, or a preallocated queue slot. C4 therefore can move the result either way and cannot support the statement that it “captures the thing exactly.”

Replace C4 with a semantic, source-pinned definition of a claimant’s future entitlement, including its account/field layout and a proof that it can later be redeemed. If a venue lacks sufficient published semantics to establish that, C4 must be `UNPROVEN`; byte-range novelty is only diagnostic evidence, not a verdict predicate.

### 3. P0 — program identity is not bound to its ProgramData account

> “`program_id`, `loader`, `programdata`, `code_hash`…” (§5, identity block)

> “Each venue's ELF is fetched by `getAccountInfo` on its programdata…” (§7)

E1 specifies a ProgramData header, but never specifies how `I.programdata` is derived and verified from `I.program_id`. The loader-v3 program account must itself be checked as executable, loader-owned, tag `Program`, and linked to the stated ProgramData pubkey. Without that check, a fixture can pair a program ID with an unrelated ProgramData account, load matching unrelated ELF bytes, satisfy `sha256(loaded_elf) == I.code_hash`, and report a Passport for code the program does not execute.

That permits every field to pass for an arbitrary benign binary while the named live program has adverse behaviour.

Define the complete loader-v3 identity procedure: raw RPC decoding; program-account owner/executable/length checks; tag `2` at `[0..4]`; ProgramData pubkey at `[4..36]`; ProgramData tag `3`; and equal, single-slot account contexts. Also specify `Option<Pubkey>` decoding: `[13..45]` is an authority only when the tag at byte 12 denotes `Some`; it is not an authority value when the tag denotes `None`.

### 4. P0 — G1 is neither the stated sample nor a count of independent consumers

> “take a deterministic systematic sample of `min(3000, N)` by selecting every `⌈N/3000⌉`-th signature…” (§9, G1)

The stated procedure does not produce `min(3000, N)` samples. For `N = 3,001`, the stride is two and the sample contains 1,501 signatures, not 3,000. This lowers observed `N_multi`, making a true market fail. More broadly, a program which calls each of two busy venues once has inclusion probability approximately `(3000/N_1) × (3000/N_2)`; ten real multi-venue users with occasional calls are overwhelmingly likely to yield fewer than ten observations. No prevalence assumption or power calculation supports the assertion that the threshold is reachable if the claim is true. The error direction is a false KILL.

The method also permits false passes. `N_multi` and `A_multi` count program IDs and upgrade-authority pubkeys, not teams or independent consumers. One operator can deploy ten programs with ten authorities, have each CPI into two venues, and satisfy both thresholds. The current-authority exclusion is not an attribution at the historical window; an authority can change after `W`. “Third-party” is consequently not established.

Finally, “flatten … in execution order” does not bind an algorithm for inserting each `meta.innerInstructions[index]` group after its top-level instruction, handling missing/null `stackHeight`, or resolving decoded versus partially decoded instructions. A natural concatenation of all top-level instructions followed by all inner instructions attributes a CPI to an unrelated later top-level program, inflating `N_multi`.

Specify an exact trace-normalisation algorithm and fixtures with expected caller outputs. Correct the sample formula or name its actual size. More importantly, pre-register a powered census/sample design and an independently auditable consumer identity rule. If the only available count is program IDs, G1 may establish technical CPI activity, but must not claim it establishes a consumer population.

### 5. P0 — G2 discrimination and G3 generalisation can pass by changing what is compared

> “the two Passports agree on all six fields” (§9, G2)

> “`F_new` > 0 — any field added, removed, or redefined…” (§9, G3)

“Agree” has no defined comparison tuple. An implementer can call two semantically identical `PASS` vectors different because their requests, errors, evidence, mutable-state snapshots, or quantities differ, avoiding the G2 KILL while the six capabilities convey no different information.

`F_new` counts only fields. It does not count new verdict categories, quantity members, candidate ranges, source schemas, mutation procedures, exception rules, or C6 predicates. Those are precisely the places where venue-specific knowledge can be added to make marginfi work while reporting `F_new = 0`. Naming marginfi before measurement does not make it blind to the author’s prior knowledge, and it does not constrain this tailoring.

Define a versioned, machine-readable field schema before G2: exact categorical comparison tuple for discrimination, all JSON members and permitted operations, and a structural diff rule under which any new member, predicate form, operation, exception, or venue-specific procedure contributes to `F_new`. Commit hashes of every permitted source must be part of the comparison.

### 6. P0 — `stable_hash` and `Q_prose` are not executable falsifiers; the primitive argument is circular

> “`stable_hash = sha256` over the Passport excluding `observed_at_slot` and live-state snapshot values…” (§8)

> “`Q_prose` = the count of these twelve that a consumer cannot answer…” (§8)

There is no complete Passport schema, JSON escaping/normalisation rule, absent-versus-null rule, array order for most arrays, path-level exclusion list, or definition of “same claim.” Two independent issuers can legitimately differ on evidence transaction IDs, account order, error representation, snapshot members, and candidate-range ordering. Conversely, they can agree after excluding an inconvenient value under the undefined category “live-state snapshot values.” Thus a `stable_hash` equality is not currently reproducible.

`Q_prose` is subjective, not a number derivable from bytes. Questions 6 and 8 are already not answered by the six fields: no field executes two venues and proves joint rollback, and `cu_total` is a one-venue C5 measurement, not “the whole settlement” across venues. Question 7 also needs exact refresh instruction/account/order data, while C5 records only program ID and count. Either G5 is predetermined to kill, or a reviewer can declare these questions “answered” by prose or field labels.

The stated defences do not break circularity. Solvo is external to H3’s directory, but it is not an independent question corpus from the authoring work. The blind third venue and `F_new = 0` test field extension, not whether the author-selected questions are necessary or whether their answers are mechanically present.

Replace each adoption question with a typed query, required Passport JSON paths, a deterministic answer algorithm, and a pre-registered expected answer domain. Source the question corpus independently. Add a cross-venue atomicity field if question 6 remains; otherwise it must remain unanswered and make G5 terminal.

### 7. P0 — G4 can report upgrade cadence where no upgrade occurred

> “count deployments … (`getSignaturesForAddress` on the programdata account, finalized)” (§9, G4)

A ProgramData account’s signature history is not a deployment history. It can include `SetAuthority` and other loader/account activity. The gate does not define a trailing-window endpoint, historical account mapping, loader instruction discriminator, account-index validation, or treatment of pagination/history gaps. Counting every signature can turn an actual zero-upgrade median into a positive `upgrades_per_90d` median, avoiding the G4 KILL.

Bind a finalized endpoint slot at run start; derive the ProgramData account at that slot; decode every candidate transaction; count only validated loader-v3 `Upgrade`/deployment instructions affecting that exact ProgramData account; and fail closed on unavailable history. Record the full classified signature list.

### 8. P0 — H3 is H1 in a different technical costume

> “is consumed by a population of on-chain programs that exists and can be counted.” (§1)

H1 failed because a technical population was substituted for an adopter. G1 repeats that substitution: observing ten program IDs directly CPI into two venues does not show that any team would obtain, trust, re-check, or act differently because of a Passport. A set of routers, internal tools, or one operator’s sybils can pass `N_multi ≥ 10` and `A_multi ≥ 5` while the product has zero consumers.

G1 first is cheaper than H1’s sequence, but it is not the missing consumer measurement. On its present rule, H3 can build a real instrument for a subject that does not exist—the H1 failure mode.

Pre-register a behaviour that distinguishes a Passport consumer from a CPI caller: a named independent integration decision, an independently sourced request, or an on-chain/off-chain use that changes accept/reject behaviour from the Passport alone. If this cannot be measured before harness construction, that is a KILL, not a reason to treat CPI frequency as adoption.

### 9. P1 — C2 permits a partial settlement to pass

> “PASS iff … `delta_destination / requested ≥ 0.999999`.” (§5, C2)

A venue can pay 999,999 of every 1,000,000 requested atoms and receive C2 `PASS`, despite the claim being “the value moves in the same transaction” and C3 being framed around full payment. The known Solvo rounding discrepancy is one hundredth of this tolerance; it does not justify a one-part-per-million allowance.

Either define atomic settlement as exact requested-amount semantics, or bind a unit-specific rounding function and a maximum exact atom difference. Record why that bound is permitted before measurement.

### 10. P1 — two motivating factual statements overstate the local evidence

> “Solvo measured Kamino klend and Phoenix Eternal under LiteSVM against cloned mainnet accounts and mainnet binaries…” (§4)

This is false for Phoenix. Solvo decision 005 says Phoenix used mainnet Eternal/Ember/Hawkeye binaries on a “freshly initialised exchange,” explicitly “not cloned mainnet state.”

Also, Solvo’s manifest records `max_response_context_slot: 440477778`, but its fixture-level `slot` is `440477781`. The claimed 8,997-slot difference is arithmetically correct relative to 440477778, but the document should not call that unqualifiedly “its klend fixture … captured at” that slot without identifying which account response had that context. Relative to the manifest fixture slot, the difference is 8,994.

Correct both descriptions and keep the motivation explicitly non-deciding.

### 11. P2 — status/audit wording and cost are imprecise

`STATUS.md` says “G0 commit: `PENDING`” although the artifact under review is commit `4ef5efd`. If “approval/freeze” is pending, say that and record the artifact commit hash.

The claimed G1 cost of “~21,000 RPC reads, ~30 min” omits signature pagination and the account reads needed to classify every observed caller’s loader and authority. It is a lower bound, not a reproducible cost estimate.

## Checks completed

Correctly verified locally:

- Stripping trailing zero bytes from `solvo/fixtures/g1/programs/klend.so.gz` produces exactly **2,414,913** bytes and SHA-256 **`8eab9f858da01fee962452ad3838570b3340cd43b80a236de8fe5297063d1cda`**.
- Solvo decision 005 accurately supports the cited facts: Phoenix’s genuine shortfall committed with destination delta zero and a queued record; the queued node records `InsufficientBudget`; B1 was not executed; klend measured **119,647,109** USDC out; and **99,999,999** moved on the 100,000,000 request.
- The local Solana loader-v3 source confirms the 45-byte ProgramData metadata size and, for a `Some` authority, tag at `[0..4]`, slot `[4..12]`, option tag at byte 12, authority `[13..45]`, and code beginning at byte 45.
- The H2 freeze banner is additive: the prior H2 text remains beneath it.

I could not re-derive the §3 live identity table, current program hashes, authorities, or live drift. The configured public RPC hostname could not be resolved in this environment, so those claims remain unverified rather than assumed.