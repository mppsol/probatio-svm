# STATUS — Probatio SVM

Binding gate: **[`docs/H5-GATE.md`](docs/H5-GATE.md)** — H5, *Agent Release Tests for Solana*, at
**G0 — pre-registration only, nothing measured. The baseline is now RULED and CLOSED (founder,
2026-08-22, before measurement), and the surviving candidate `C3` is pre-registered as its own G0 row
at [`docs/H5-C3-G0.md`](docs/H5-C3-G0.md), awaiting independent review. H5 was never killed.** The four earlier gates are closed and none is
reopened: `docs/GATE.md` (H1) **`KILLED`** · `docs/H2-GATE.md` **FROZEN UNEXECUTED** ·
`docs/H3-GATE.md` **`KILLED` at the design gate** · `docs/H4-GATE.md` **`KILLED` at G0
(KILL-2 + KILL-3), 2026-08-22**.
A verdict is a number or a reproducible experiment. Not-proven is a KILL.

## H5 · C3 — duplicate execution after a lost confirmation — **reviewed `KILL AT DESIGN GATE`; NOT measured**

Row: [`docs/H5-C3-G0.md`](docs/H5-C3-G0.md) · gate: [`docs/H5-GATE.md`](docs/H5-GATE.md) §8.3–§8.4 ·
founder ruling **2026-08-22** · review payload committed, **not run**:
[`reviews/H5-C3-G0.codex-prompt.md`](reviews/H5-C3-G0.codex-prompt.md) · **no implementation exists
and no measurement has been run.**

**The baseline is RULED and CLOSED, before measurement** — this is what unblocked H5: `Pol` = a static
policy (destination allowlist + per-transaction cap); `Sim` = a **non-persistent** single
`simulateTransaction`; **neither may retain, apply or chain post-state between transactions**; and an
**executor that carries cloned state across transactions is not a baseline — it is the mechanism H5 is
tested on.** Not revisable after seeing a result.

> **The claim:** an agent that **loses a confirmation** and retries **without resolving transaction
> status** executes a capital action **twice**. Every individual transaction is admissible — to `Pol`,
> to `Sim`, and to klend — and only the **cumulative** state diff is wrong. A per-transaction check
> cannot express a property of a **sum**; that is the one structural gap C3 sits in.

**Three meaningful capital actions, counted honestly** — `W1` `withdraw(100,000,000)` → `Ok`; `W2`
same → **commits on chain**, agent is told **`Timeout`**; `W3` the erroneous retry → commits.
**klend's refresh preambles are explicitly NOT counted**, bundled or not; **there is no no-op, filler
or decorative transaction.** The amount is not arbitrary: H4 **measured** `withdraw(100,000,000)`
moving exactly 100,000,000 (case A) and a 486,657,686 single-call ceiling (case E), so
**3 × 100,000,000 = 300,000,000** has measured headroom.

**Fault injection is pinned to one place:** the **agent-facing** result of `W2` only, value `Timeout`,
at the harness's submit/confirm boundary — **not** in the VM, the transaction, or any account. `W2`
executes on real BPF and **commits**. **The chain is not lied to; the agent is.** The retry uses a
**new blockhash**, and the harness **asserts `sig(W3) ≠ sig(W2)`** so the episode can never silently
test the runtime's dedup instead of the agent.

**Predicate — cumulative, state-bytes-only, no log ever read:** `P1` the collateral supply vault fell
by **3A**; `P2` `deposited_amount` fell by 3A (2,248,785,777 → 1,948,785,777) while the agent's ledger
says 2A; `P3` **every individual action returned `Ok` and moved exactly A — none is anomalous alone.**
Fires iff `P1 ∧ P2 ∧ P3`. **`P3` is the point**: it is the pre-registered proof no per-transaction
check could have flagged anything.

**Both baselines are implemented and run, and `GO` requires both to miss.** `Pol`'s cap is fixed at
**150,000,000** — above one tranche, below two — chosen so it is *not* trivially beaten: **the
duplicate is not a large transaction, it is an extra ordinary one.** If either flags the episode,
**`KILL-1` fires and C3 is dead.**

**Pinned:** `klend_new` `b1344d19…9c22d9` (2,431,953 B) is the binary under test and `klend_old` is
**not used**; farms `9ca00de8…5a240`; cloned state `fixtures/h4/` at slot **440,477,781**, manifest
`b5db5e51…4fc6a`; exactly **two** mutations (obligation owner @64 len 32; destination token account),
both disclosed by the run; CI entrypoint frozen as an exit-code contract (0 / 1 / 2), offline and
deterministic, taking **the developer's own agent rule as an input**.

**Pre-registered prediction: `GO` is genuinely possible here for the first time in H5, and the
likeliest death is `KILL-2`** — `W3` refused or clamped — **not `KILL-1`.** Named risk: C3's strength
rests entirely on the §0 baseline ruling; if that definition is wrong, C3 falls with it, and no later
result can repair that.

**Not revived:** the §8.1 first candidate, `C1`, `C2`. **Nothing here depends on them.**

### Review outcome — `KILL AT DESIGN GATE`, and measurement did not proceed

[`reviews/H5-C3-G0.md`](reviews/H5-C3-G0.md) — Codex, read-only, one round, run by founder ruling with
a directive to attack six points **in the kill direction first**. **Four P0s, three P1s.** Per the
founder's rule a non-`PASS` verdict means **no measurement**, and none was run. **CC concurs with
every P0.** C3 is **not** declared dead by CC — that is a founder ruling.

| # | finding | CC |
|---|---|---|
| **P0-1** | the `W1`/`W2` split is **件数合わせ** — *"no concrete operational reason justifies splitting a 200,000,000-atom withdrawal into two 100,000,000-atom calls when the cited fixture admits a single 486,657,686-atom call"*. "Ordinary agent behaviour" was **assertion, not constraint** | **concur** — CC's own §2 disclosed the split was what made three actions available, and could name no operational driver. The design-time kill C3 pre-registered fires on itself |
| **P0-2** | **`Pol` is too weak to carry condition B.** With no cumulative cap, no intent total and no transaction history it misses by construction, but the hypothesis's competitor is *"a runtime wallet policy"*, and a real one **catches a duplicate ordinary withdrawal** | **concur, and this is the deeper one** — a duplicate spend is the **canonical** target of a cumulative limit |
| **P0-3** | **post-run freedoms remain**: the slot schedule is only *"recorded by the run"*, further mutations are permitted if disclosed, and no input contract exists for the developer's agent rule | **concur** — slot progression can change lending state and `W3`'s result, so it must be pinned, not observed |
| **P0-4** | the **CI entrypoint takes only an episode JSON**, not a rule artifact or invocation contract, so it does not yet meet the gate's own-agent requirement — a live **`KILL-4`** risk | **concur** |
| **P1** | **sequential headroom is still unproven** — H4's case A and case E are single calls **from the initial fixture**; they do not establish that the post-`W1`/post-`W2` solvency envelopes admit another 100,000,000. The same path-dependence gap flagged a round earlier | **concur** — CC used a single-call ceiling as headroom for a *sequence*, which is precisely what was already flagged |
| **P1** | the agent rule is **undefined off the intended path** — no rule for `Err`, an unexpected status, or a repeated timeout; a refused `W3` leaves no defined transition | **concur** |
| **P1** | **`P3` contradicts "state-bytes-only"** — it requires knowing each transaction *returned `Ok`*, which is execution metadata, not state bytes | **concur — a real internal inconsistency in CC's own predicate** |

### Remediation: CC finds none available on the committed fixture

**P0-1 has no fix here.** Removing the artificial split leaves **one planned action plus one
duplicate = two**, below the required three. Every way to obtain a genuine third planned action is
closed by the fixture: there is **no second collateral** to withdraw; the agent **holds no SOL** with
which to repay the one borrow, so a repay leg would mean fabricating a balance; and "withdraw the
maximum, iterating" is not iterative here because **H4 measured that `u64::MAX` reaches the maximum in
a single call**.

**P0-2 may be unfixable in principle, not just here.** C3's failure *is* a duplicate spend, and a
cumulative spend cap set at the agent's own intent total catches it by definition. For C3 to clear
condition B against a fair runtime policy it would have to be a duplicate that a cumulative cap
misses — which is close to a contradiction in terms.

**Both P0s point the same way, and CC does not have a repair to offer.** No replacement candidate was
searched for, per the standing instruction. **Awaiting a founder ruling.** No measurement,
implementation, fetch, UI, token or deploy was performed.

## H5 — Agent Release Tests for Solana — **NOT killed; the review dissented and candidate `C3` survives**

Gate: [`docs/H5-GATE.md`](docs/H5-GATE.md) · founder ruling **2026-08-22** · review:
[Codex, `KILL AT DESIGN GATE — KILL-1`](reviews/H5-G0-agent-release-tests.md) · **no measurement has
been run and no implementation exists.** Nothing is ratified; **awaiting a founder ruling on whether
H5 itself is closed.**

> A Solana AI agent that moves capital in production should be regression-tested before release
> against adversarial scenarios spanning **real program BPF, real cloned state, and multiple
> transactions** — and such a test **finds failures a single `simulateTransaction` or a runtime
> wallet policy cannot find**.

**Only the second half is measurable, and only it is on trial.** "Should be tested" is an opinion.

**In scope:** agent developers / agent-wallet developers, **before release**; CI and regression
testing for agents that take capital actions on Solana.
**Out of scope, bindingly:** runtime wallet policy engine · general-purpose transaction simulator ·
production monitoring, insurance, certification market · generic protocol capability schema ·
"agent certification" and any claim presupposing an agent population · **reusing H1's evidence or
conclusions as demand evidence**.

**H5 is not H1 revived.** H1 was a market certifying agents *already operating*, and it is `KILLED`.
H5 is a *pre-release developer/CI test*. Different buyer (the agent's own developer), different point
of use (once per release, in their pipeline), different unit of failure (one reproducible
multi-transaction episode), different deliverable (a failing trace and a regression test, not an
attestation). Reusing H1/H4 **mechanics as fixtures** is allowed; reusing an H1 **conclusion** as
evidence is not.

**Pre-registered `GO` — all four, for one fixed capital-action workflow:**

| | condition |
|---|---|
| A | a reproducible ≥3-transaction episode on **real BPF + cloned state**, deterministic and offline |
| B | **one** failure caught **by state diff** that **neither** a static policy **nor** a single `simulateTransaction` catches — **both baselines implemented and actually run**, both shown to say "no problem" |
| C | that failure **changes the next action** of a **fixed scripted agent** (deterministic, no LLM, rule written before the run) |
| D | existing Probatio/Solvo assets used **as fixtures**, every input/mutation/hash recorded, **Solvo's conclusions not overstated** |

**B is the hypothesis.** A + C + D passing with B failing is a `KILL`, not a partial success.

| KILL | fires when |
|---|---|
| 1 | H5 is **equivalent to a single simulation or a destination/cap policy** — either baseline, once run, also catches it |
| 2 | a **stateful failure cannot be reproduced on real BPF** |
| 3 | deciding needs a **protocol-independent schema or arbitrary adapter semantics** (H3's grave) |
| 4 | **no concrete CI entrypoint** can be defined that attaches to a developer's release process |

**Still open, and frozen as open** ([`H5-GATE.md` §8](docs/H5-GATE.md)) — the founder's ruling did not
name them, and each is a place a measurement could be tuned after the fact, so **all must be fixed by
a founder ruling before anything runs**: the one workflow · the candidate failure and its state
predicate · the scripted agent's decision rule · the CI entrypoint command · and the exact definition
of both baselines in B.

**The first concrete proposal for all five was written, independently reviewed, and is
`REJECTED at the design gate` — [`H5-GATE.md` §8.1](docs/H5-GATE.md),
[review](reviews/H5-G0-agent-release-tests.md) (Codex, read-only, `174fff2`):
**`KILL AT DESIGN GATE — KILL-1`**, one P0.**

The proposal was: workflow `W` = klend USDC withdraw on the committed `fixtures/h4/` set as **three**
transactions; candidate failure `F` = the agent believes it exited while **1,762,128,091 atoms (78%)
are still deposited** — read off **H4's own frozen evidence** (case E requested `u64::MAX`, got
**`Ok`**, moved **486,657,686** of **2,248,785,777**, no error, no "partial" in any log). Both
baselines were pinned, with the **generous** one (`S-generous`: one `simulateTransaction` **plus**
inspection of the state it returns) pre-registered as **governing**.

**`S-generous` catches `F` at T2, so `KILL-1` fires on this candidate.** CC's "the simulator does not
know the agent's intent" argument was found **not structural**: the intent is the scripted agent's own
source, available to the same developer running the simulation, who can simply assert *"after
`withdraw(u64::MAX)`, `deposited_amount == 0`"* — no third transaction and no new oracle needed. The
reviewer also found **T3 to be theatre** (`F` is fully witnessed by T2's post-state, so the
multi-transaction differentiator does no work), the **`Pol` cap beatable by construction** (fixed at
1,000,000,000 above the *already known* realized 582,271,854; a real pre-sign policy reading the
*requested* value would reject the `u64::MAX` sentinel outright), the **CI entrypoint a placeholder**,
and the freeze missing a pinned BPF hash and the required mutations.

**Cost of this kill: zero implementation.** No code, no measurement, no fetch. CC's own §9 prediction
had named `S-generous` as where H5 would most likely die, and it did — before anything was built.

**This kills the candidate, not H5 by fiat.** §8.1 was never in force, and it pre-registered that **no
substitute failure may be swapped in after the fact — a replacement needs a new founder ruling and a
new G0 row.** That binds CC too; **CC has not proposed a replacement.** The P0 is structural, not
candidate-specific: it defeats *any* candidate whose failure is fully witnessed inside one
transaction's post-state. The only door left is a failure **no single transaction's post-state
witnesses** — a defect in the *sequence*. **Nothing shows such a failure exists**, and not-proven is a
KILL, so that door is not a plan. **Whether H5 is closed is a founder ruling. Awaiting it; nothing
runs.**

**The candidate search that followed (founder ruling 2026-08-22, *design investigation only*):**
[`docs/H5-CANDIDATES.md`](docs/H5-CANDIDATES.md). Three candidates, **all dead at the design gate,
none implemented and none measured** — `C1` (failing transaction's bytes derived from earlier
results) **`KILL-1`**, because the failure is still fully witnessed in that transaction's own
post-state, the identical shape §8.1 died of; `C2` (a plan invalidated by slot advance and accrued
interest) **`KILL-1`**, because its last transaction is constructible in advance and one simulation of
it at submission time determines the failure; `C3` (wrong retry after an apparent revert, double
execution) **`KILL-2`**, because the trigger is a **client-side** event — an RPC timeout or a lost
confirmation — that real BPF on cloned state cannot produce, and only harness fabrication supplies.

**The reason is the fixture, and it was read from bytes.** The committed obligation holds **exactly
one deposit** (USDC, 2,248,785,777 atoms, ≈$2,690.34) against **exactly one borrow** (SOL, BF-adjusted
debt ≈$1,343.59; allowed ≈$2,152.28, unhealthy ≈$2,421.31). So there is **no second capital leg** to
make ordering matter, and **klend's clamping already reaches the solvency limit in one transaction**
(H4 case E: `u64::MAX` → 486,657,686 moved), leaving a cumulative walk nowhere to go. What remains is
a **single capital action**, whose failure is by construction witnessed in a single transaction's
post-state.

**Branch A was proposed — and then WITHDRAWN by the review it asked for.**
[`reviews/H5-G0-candidates.md`](reviews/H5-G0-candidates.md) (Codex, read-only, one round, run by
founder ruling from the committed payload) returned **`DISSENT — a candidate survives: C3`** with
**two P0s against CC's own reasoning**, and **CC concurs with both**:

- **`C3` was killed on a wrong premise.** `KILL-2` fires when *a stateful failure cannot be reproduced
  on real BPF*. C3's stateful failure is the **cumulative double withdrawal**, and that **is**
  reproduced on real BPF; CC conflated the *trigger* with the *failure*. A dropped confirmation is
  *"an explicit client/transport fault input to the agent under test"* — **the system under test is
  the agent, not the chain**, so injecting a transport fault is ordinary fault injection, not
  fabricating chain state. **`C3` survives.**
- **`C1`'s `KILL-1` was circular.** Observing `T4`'s post-state requires having executed `T1`–`T3` —
  *"that is H5's distinguishing machinery, not one RPC `simulateTransaction`."* CC granted the
  baseline H5's own executor and then found H5 redundant. **`C1` is not killed** (nor frozen — `R` and
  the exit rule are still unspecified). **`C2` is undecided**, for the same reason.

**Everything reduces to one unresolved definition, and it is a founder ruling:** `simulateTransaction`
**cannot advance chain state**, so a baseline that carries post-state across transactions **is** a
local-fork episode executor — i.e. H5 itself. **If the baseline is one non-persistent RPC simulation**,
multi-transaction failures are genuinely beyond it and `C1`/`C3` are live. **If it may carry cloned
state**, no candidate can ever clear condition 2 and **H5 is dead in general, not per-candidate.**
Choosing this *after* seeing a result would be the post-hoc criterion change the ruling forbids, so it
must be settled **before** any freeze, measurement or kill.

**Disclosed, not acted on:** the review also finds that §8.1's kill rested on the same invalid grant
(*"post-`T1` state"*). **§8.1 stays finished** — the founder ruled it so and forbade rescuing it, and
that stands. It is recorded because the reasoning has been undermined, not as a proposal to revive it.

**Confirmed by independent re-derivation:** exactly one live deposit slot and one live borrow slot, and
the four `_sf` values. The two-collateral ordering shape stays unavailable — but the review notes that
**`C3` needs neither a second capital leg nor a partial fill**, so §7's resume condition **does not
govern**. The cumulative-walk bound is confirmed **unproven**.

**Not claimed:** that no sequence-only failure exists on Solana. **Not claimed:** that any past
hypothesis supports H5's demand. **Not proposed:** any replacement candidate — none was searched for,
no fetch made, no code written, no measurement run.

**Prediction on the record, made before the review: overall open, leaning `KILL-1`** — and `KILL-1`
is what the design gate returned, on the exact baseline CC named as the likely cause of death.

**G0 authorises this document and this row — nothing else.** No implementation, UI, token, deploy,
wallet, policy engine, second protocol, LLM in the loop, or measurement.

## H4 — Upgrade Behavior Sentinel — **KILLED at G0 — KILL-2 + KILL-3**

**Verdict:** [`docs/decisions/H4-sentinel-kill.md`](docs/decisions/H4-sentinel-kill.md) · gate:
[`docs/H4-GATE.md`](docs/H4-GATE.md) (closed) · review:
[Codex, `MEASUREMENT SOUND`, no P0](reviews/H4-G0-sentinel.md) · artifacts `35f8d0e` · `2d7b5c3` ·
`75f3cb9` · `7efbb37` · **verdict date 2026-08-22**

> H4's hypothesis: for **one** named Solana integration, the behaviour it depends on is re-executed
> against the real pre- and post-upgrade BPF binaries, on identical cloned state with identical
> inputs, and yields a `compatible` / `breaking` / `unknown` verdict **a byte hash comparison cannot
> produce**.

**The measurement ran, and it is the verdict.** Unlike H3, H4 pre-registered numbers and produced
them. Fixture slot **440,477,781**; `D` = **2,248,785,777**; old `code_hash` `8eab9f85…3d1cda`
(2,414,913 B), new `b1344d19…9c22d9` (2,431,953 B, deploy slot 440,486,775).

| case | amount | old `result` | new `result` | deltas | `state_equal` | verdict |
|---|---:|---|---|---|---|---|
| A | 100,000,000 | `Ok` | `Ok` | equal | false | `unknown` |
| B | `D` | `Err … Custom(6011)` | same | equal | false | `unknown` |
| C | `D + 1` | `Err … Custom(6011)` | same | equal | false | `unknown` |
| E | `u64::MAX` | `Ok` | `Ok` | equal | false | `unknown` |

**4 of 4 `unknown` → KILL-2. Action `RE-VERIFY` → KILL-3.** `RE-VERIFY` is the only answer the
differing hash could already give; H4 spent a real measurement to return to its own baseline.

**What actually differed.** In every case both binaries agree on `result` and on all three
`token_deltas` (case A moves the same 119,647,109 USDC atoms). The sole divergence is that the **new
binary writes 4 bytes at offset 28** of `obligation`, `reserve_sol` and `reserve_usdc`, where the old
binary leaves the fixture's `00000000`.

**Why that is not rescuable.** Calling those 4 bytes immaterial is a klend-specific semantic
judgement (`KILL-4`); generalising it to "ignore reserved ranges" is a protocol-independent schema
(also `KILL-4`). **The rescue and the kill are the same door.** Gate §4 pre-registered `unknown` for
exactly this shape, and pre-registered why — so the rule could not be softened after seeing it.

**KILL-1 explicitly did *not* fire:** both ELFs loaded, 8/8 executions ran, and three consecutive
runs produced a byte-identical `evidence/h4-sentinel.json`
(`f05c0ea6…8f0cf0`, 106,573 B). An earlier run (`75f3cb9`) loaded neither binary — the brief wrongly
required trailing-zero stripping, and both section-header tables end **15 bytes past** the last
non-zero byte; corrected in `7efbb37` and in the loader change committed here, identically for both
binaries. **KILL-5 did not fire:** the old fixture is byte-identical to Solvo's committed copy
(`adc2b55b…`), i.e. copied not edited. `git -C ../solvo status --porcelain` is *not* empty — it shows
one untracked `docs/H3-GATE.md` written 2026-08-22 08:26 by a **separate session working inside
Solvo** on Solvo's own H3; no write originated from here. Recorded because the brief asked for that
output to be empty.

**Carried forward as input, not as a live hypothesis:** byte-equality across two binaries is
**decidable but not actionable** — a useful verdict needs a notion of which differences matter, and
that notion is venue-specific. H3 needed meaning up front; H4 avoided meaning entirely and found it
needed meaning at the end. The harness *mechanics* (real BPF, cloned state, disclosed mutations,
deterministic offline replay) are sound; **reusing mechanics is not reusing the hypothesis**, and
nothing here may be revived as a foundation because it exists.

## H3 — Composability Passport — **KILLED at the design gate**

**Verdict:** [`docs/decisions/H3-design-gate-kill.md`](docs/decisions/H3-design-gate-kill.md) ·
gate: [`docs/H3-GATE.md`](docs/H3-GATE.md) (closed) · reviews:
[r1 `CHANGES`](reviews/H3-G0-passport.md) 8×P0 → [r2 `CHANGES`](reviews/H3-G0-passport-r2.md) 6×P0,
same class · artifacts `4ef5efd` · `f5cb3ec` · `8d92bf1` · `91bb712` · **verdict date 2026-08-21**

**What died:** the fixed, protocol-independent capability schema. Two review rounds established that
r2 did not remove the implementer's freedom to choose which effect satisfies a field — it
**relocated** it into the adapter manifest (`scarcity_range`, the `value_out_ix` account list, the
settlement-path and writer selections that are not manifest members at all). If capability semantics
are irreducibly venue-specific, the deliverable is a **report per protocol, not a schema**, which is
precisely what the brief's item 5 asked H3 to prove it was not. Two further P0s point the same way:
C7 is a *pair* property stored in a *per-venue* Passport, so nothing expires it when the reference
venue upgrades; and G1b can be satisfied by two form letters — H1's asserted adopter in written form.

**Not rescued by more specification** (pinning per-venue semantics for every field is H2's failure
mode one layer down), **not rescued by running G1a** (a consumer count does not repair a schema that
cannot be specified — it was ready to run and was deliberately not run), and **no r3** (§13
pre-registered that there would not be one).

**No gate item was ever executed.** G1a, G1b, G2, G3, G4, G5 — none started. No capability field was
measured on any protocol, no code was written, no fixture fetched, no third party contacted, and
`../solvo` was not edited. **This kill is about the specification, not about klend or Phoenix.**

**Kept as reproducible refutation assets:** the E1–E4 identity/expiry machinery bound to real bytes
(ProgramData linkage, loader tags, `Option<Pubkey>` semantics, the hash triple, the 12-byte
`last_deploy_slot` pre-check); G4's deployment-instruction counting; the harness discipline
(hash-asserted mainnet binary, cloned mainnet state only, disclosed mutations, no assertion on logs,
byte-identical reruns); and the read-only measurements below.

Verified read-only at slot **440,578,912**, by one party only — the reviewer had no network in either
round and correctly declined to assume it: **7 of 7 venues carry a live upgrade authority, none is
immutable**; klend and Kamino Vaults share the authority `GzFgdRJXmaw…`; `../solvo`'s klend fixture
sits at slot 440,477,781 and klend was **redeployed 8,994 slots (≈ 1 h) later** (`8eab9f85…` →
`b1344d19…`); Phoenix Eternal runs at **≈ 199.8 sig/s**. The reviewer independently re-derived the
fixture slot, the gap, the captured hash, and that Solvo's Phoenix leg used a **localnet fixture, not
cloned mainnet state**.

**Founder note (2026-08-21), recorded as a deferral and NOT a new phase:** what died is the
general-purpose, comparable Passport; the strong remaining asset is re-runnable verification tied to a
real binary, real state and a real upgrade. If anything moves next it should be a **narrow evidentiary
product that reconstructs a specific failure or incident after the fact**, not a general-purpose
scorecard. **This is not started** — no gate, no hypothesis, no kill number, no authorisation. A
successor requires its own G0.

**No hypothesis is currently live.** H1 `KILLED`, H2 `FROZEN UNEXECUTED`, H3 `KILLED at the design
gate`. Nothing may be built until a founder ruling opens a new gate.

## H1 — certify autonomous agents before capital is trusted to them — **KILLED**

| # | Item | Status | Evidence | Verdict date |
|---|---|---|---|---|
| **P1** | A real target — an actual on-chain agent or agent vault | **KILLED** | [`docs/decisions/P1-real-target.md`](docs/decisions/P1-real-target.md) + Codex [r1](reviews/P1-real-target.md) → [r2 `APPROVE`](reviews/P1-real-target-r2.md) | 2026-08-20 |
| P2 | Reproducible certification | not reached | — | — |
| P3 | The constraint binds | not reached | — | — |
| P4 | Differentiation vs an existing sandbox/fuzzer | not reached | — | — |

**Founder decision (2026-08-20): the H1 KILL stands and is not rewritten.** H2 starts from its
refutation as input.

## H2 — a countable population bearing a reconstructible loss it does not control — **FROZEN UNEXECUTED**

| # | Item | Status | Kill number fixed in advance | Evidence |
|---|---|---|---|---|
| **G0** | Pre-registration of window, venues, loss definition, thresholds | **NOT FROZEN — r2 `CHANGES`, founder decision pending** | precondition, no verdict | [`docs/H2-preregistration.md`](docs/H2-preregistration.md) · reviews [r1](reviews/H2-G0-prereg.md) · [r2](reviews/H2-G0-prereg-r2.md) |
| G1 | Does the buyer exist, and is the loss poolable? | not started | `N_addr`<1,000 / `N_events`<300 / `L_total`<$50M per 90d / `C_10`≥50% / `L_und`÷`L_gross`>30% | — |
| G2 | Is the loss reconstructible and attributable? | not started | `R`<0.95 or `A`<0.95 | — |
| G3 | Was the outcome undetermined at participation? | not started | `AUC`≥0.90 or `D`≥0.20 | — |
| G4 | Can payout and loss be made to coincide? | not started | no form with median(e)≤0.20 and p90(e)≤0.50 | — |
| G5 | Can a third party reconstruct all of it? | not started | any deciding number fails independent re-derivation | — |

The buyer and the payout form are **deliberately not fixed by assertion** — G1 names the buyer by
measurement, G4 selects the payout form by measurement. Fixing them by assertion is what killed H1.

**Founder ruling, 2026-08-21: H2 is FROZEN UNEXECUTED.** Not `KILLED` — no G1 measurement was ever
run and no number below is a verdict. Not `BLOCKED` — nothing external is being waited on. G0 was in a
specification spiral (three rounds, 331 lines, zero measurements) and the founder chose a different
hypothesis (H3) over a fourth round. Nothing below is rewritten. **Resuming H2 requires a founder
ruling and its own re-frozen G0**; continuing it inside H3 would be 延命.

The record of why it stopped, kept as written:

G0 r2 was reviewed and returned `CHANGES` again. The findings are valid and verified: the sub-window
seed is genuinely ambiguous (`sha256` of the base58 *string* gives 46, of the decoded *bytes* gives 3);
67,962 s of W fall outside the 91 whole days; G4's "parametric" form is the identity `payout = loss` by
construction, so it cannot test payout/loss coincidence even out-of-sample; and the G5 tolerance
accepts `C_10 = 49.8%` against an independent `50.2%` — one a pass, one a KILL — as "reproducing".

The changelog claim "every r2 change moves the gate against the project" was **false**, as Codex found:
the density correction is neutral, and the G5 tolerance was favourable. That is the flattering-error
pattern `docs/H2-GATE.md` warns about, and here it ran in the direction of making r2 look more rigorous
than it was.

Root cause: `docs/H2-GATE.md` requires the loss definition **in bytes**, while G0 r2 deferred byte
offsets and discriminators as "chain knowledge". Those contradict. Closing it honestly means a
byte-level schema for each of six venues — and G0 is now three rounds and 331 lines deep **without a
single measurement**. `docs/GATE.md` names this exact failure: *"a specification reached 3,000 lines and
three review rounds before anyone asked whether the thing it specified could be built at all."*

**Differentiation is resolved into G1, not added as a seventh item** (founder: "推奨で"). `L_total` is
the **uncovered residual** `L_gross − L_cov − L_und`, and every G1 threshold applies to it — so a loss an
incumbent already covers cannot pass the gate as if it were addressable. Undetermined coverage counts
*against* the project. Deferred to H3 by design: *why this rather than the incumbent* (product
differentiation, pricing, distribution) — answerable only once the room is shown to exist.

---

### H1 / P1 — KILLED (2026-08-20) — detail

Reproduce: `python3 docs/decisions/repro/p1_real_target.py` (~60s, stdlib only).
Independently reviewed by Codex: [r1 `CHANGES`](reviews/P1-real-target.md) (one P0, fixed) →
[r2 **`APPROVE`**](reviews/P1-real-target-r2.md).

At mainnet slot 440479436, of the **1,767** distinct agent identities carried by the **1,471**
`AgentAccount` records in the Solana Agent Registry (`8oo4dC4Jv…`) — `creator`, `owner`, `asset`,
`agent_wallet`, `parent_asset`, decoded against the registry's published Borsh schema — **6** have ever
held a Jupiter Perps position and **0** have an open one. The set of registered on-chain agents this
harness can certify is empty. Of the **4,708** addresses that do hold open Jupiter positions,
**4,366 (96.4%, $62.2M)** are plain System-Program wallets indistinguishable from human traders;
**337** are rent-collected accounts of which **0 are off-curve**, so none is a vault PDA; and only
**5 ($32,423 = 0.05%)** are program-controlled, none identifiable as an agent vault.

The repo's headline live PASS (`gallery/jupiter-live-AhUvhrHH.json`) was recomputed from the
definition of `delta_units`: PASS ⟺ `|net notional| < $50`, and that wallet's whole position is **$16**
— it passes by being too small to measure. Across the live population **98.4% of the PASS class is
sub-$50 dust**; 83.16% FLAG.

Not `BLOCKED`: no external dependency with a stated action on a stated date. Not rescued by a new
ingestion adapter — `docs/GATE.md` treats *"it would work if we also had X"* as a KILL, not a new scope.

**The gate stops here. Per `docs/GATE.md`, the founder decides what happens next.**
