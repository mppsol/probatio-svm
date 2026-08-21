# STATUS — Probatio SVM

Binding gate: **[`docs/H4-GATE.md`](docs/H4-GATE.md)** — H4, *Upgrade Behavior Sentinel*, at **G0**.
`docs/GATE.md` (H1) is **closed — `KILLED`**; `docs/H2-GATE.md` is **FROZEN UNEXECUTED**;
`docs/H3-GATE.md` is **closed — `KILLED` at the design gate**. None of the three is reopened.
A verdict is a number or a reproducible experiment. Not-proven is a KILL.

## H4 — Upgrade Behavior Sentinel — **G0 pre-registration committed; measurement pending**

Gate: [`docs/H4-GATE.md`](docs/H4-GATE.md) · G0 artifact commit: `PENDING` · **no measurement has
been run at the time this row was written**

> For **one** named Solana integration, the concrete behaviour it depends on is re-executed against
> the real pre- and post-upgrade BPF binaries, on identical cloned state with identical inputs, and
> yields a `compatible` / `breaking` / `unknown` verdict **a byte hash comparison cannot produce**.

**Forbidden by name, because H3 died of it:** a general-purpose Capability Passport, any
protocol-independent schema or adapter semantics, and cross-protocol scoring of any kind. Needing one
is `KILL-4`, not a scope change.

**Why this is not H3:** H3 needed to know what a behaviour *means*, and every meaning turned out to
be venue-specific. H4 holds input, state and case fixed, varies **only the binary**, and compares the
two outputs for **equality**. Equality is not an interpretation.

**Fixed before measurement:** klend `withdraw_obligation_collateral_and_redeem_reserve_collateral_v2`
top-level on the USDC reserve · old binary = `../solvo`'s committed `klend.so` (2,414,913 B,
`8eab9f85…`, fixture slot 440,477,781) · new binary = fetched from mainnet *after this gate is
pushed*, with ProgramData, slot and `code_hash` recorded · four cases (success `100,000,000`,
boundary `D`, failure `D + 1`, sentinel `u64::MAX`) · decisive outputs = `Ok`/`Err` with exact code,
token deltas at SPL offset 64, and full post-state byte equality; **logs are auxiliary and never
asserted on**.

| KILL | fires when |
|---|---|
| 1 | the two real binaries cannot be run in one reproducible environment, or reruns are not byte-identical |
| 2 | all four cases are `unknown` |
| 3 | the action is `RE-VERIFY` — no more actionable than the hash difference already was |
| 4 | deciding needs a protocol-independent schema or arbitrary adapter semantics |
| 5 | `../solvo` must be changed — it is read-only, without exception |

**`PASS` does not start Gate 1 or any implementation.**

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
