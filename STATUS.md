# STATUS — Probatio SVM

Binding gate: **[`docs/H5-GATE.md`](docs/H5-GATE.md)** — H5, *Agent Release Tests for Solana*, at
**G0 — pre-registration only, nothing measured**. The four earlier gates are closed and none is
reopened: `docs/GATE.md` (H1) **`KILLED`** · `docs/H2-GATE.md` **FROZEN UNEXECUTED** ·
`docs/H3-GATE.md` **`KILLED` at the design gate** · `docs/H4-GATE.md` **`KILLED` at G0
(KILL-2 + KILL-3), 2026-08-22**.
A verdict is a number or a reproducible experiment. Not-proven is a KILL.

## H5 — Agent Release Tests for Solana — **G0 pre-registered; nothing measured**

Gate: [`docs/H5-GATE.md`](docs/H5-GATE.md) · founder ruling **2026-08-22** · **no measurement has
been run, and no implementation exists, at the time this row was written**

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

**A concrete proposal for all five is on the table — [`H5-GATE.md` §8.1](docs/H5-GATE.md), marked
`PROPOSED`, NOT IN FORCE. Awaiting a founder ruling; nothing runs until then.** In outline: workflow
`W` = klend USDC withdraw on the already-committed `fixtures/h4/` set (17 cloned accounts, slot
**440,477,781**, real BPF, no new fetch), as **three** transactions carrying state forward.
Candidate failure `F` is **read off H4's own frozen evidence, not hunted inside H5**: case E requested
`u64::MAX` collateral, received **`Ok`**, and moved **486,657,686** of **2,248,785,777** — leaving
**1,762,128,091 atoms (78%) still deposited after a call the caller reads as "I exited"**, with no
error and no log line saying "partial". Detector `P` is state-bytes-only. The scripted agent's rule is
written out in full, and **both baselines are pinned, including the generous one** (`S-generous`: one
`simulateTransaction` **plus** inspection of the state it returns).

**`S-generous` is pre-registered as governing, deliberately, and it is where CC expects H5 to die.**
An `err`-only baseline is a strawman. The narrow surviving claim, if any: a single simulation returns
*a state*, but does not know **what the agent intended across transactions**, so it cannot tell that
`deposited_amount = 1,762,128,091` contradicts an intent to exit — **the oracle is the agent's
declared intent, not the chain.** That is also the structural answer to `KILL-3`: H4 had two binaries
and no intent, hence no ground truth; H5 tests an agent, so intent exists. If `S-generous` plus any
fixed rule catches `F` without that intent, **`KILL-1` fires and H5 is dead.**

**Prediction on the record: overall open, leaning `KILL-1`.**

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
