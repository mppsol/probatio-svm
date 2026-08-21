# STATUS — Probatio SVM

Binding gate: **[`docs/H3-GATE.md`](docs/H3-GATE.md)**. `docs/GATE.md` (H1) is **closed**;
`docs/H2-GATE.md` is **FROZEN UNEXECUTED**. A verdict is a number or a reproducible experiment.
Not-proven is a KILL.

## H3 — Composability Passport — **G0 (pre-registration), no verdict**

Gate: [`docs/H3-GATE.md`](docs/H3-GATE.md) · independent review:
[`reviews/H3-G0-passport.md`](reviews/H3-G0-passport.md) · **G0 commit: `PENDING`**

> A protocol's "CPI-able" claim decomposes into six capability fields, each decided by an executable
> probe against the protocol's **real mainnet binary and cloned mainnet state**; the Passport is
> byte-canonical, expires mechanically when the code or the state it depends on changes, generalises
> to a protocol it was not designed against, and has a consumer population that can be counted.

| # | Item | Status | Kill number fixed in advance |
|---|---|---|---|
| **G0** | Pre-registration: fields, expiry rules, harness, examples, primitive criteria, venue set | **written, awaiting independent review + founder ruling** | precondition, no verdict |
| G1 | Is there anyone to hand a Passport to? (CPI callers counted, not asserted) | not started | `N_multi` < 10 or `A_multi` < 5 |
| G2 | Discrimination on two real protocols (klend, Phoenix Eternal) | not started | any field `UNPROVEN`; all six fields agree; a failed C3 control; non-determinism; an undisclosed mutation |
| G3 | Generalisation to a blind third (marginfi v2) | not started | `F_new` > 0, or any field `UNPROVEN` |
| G4 | Expiry binds, on code **and** on state | not started | no demonstrated hash change; pre-check does not fire; no measured state flip; median `upgrades_per_90d` = 0 |
| G5 | Independent reconstruction, canonical equality, consumer cost | not started | any `stable_hash` differs; `Q_prose` ≥ 1; any field needs > 4 account reads |

**G1 is deliberately first**: it is the item H1 died on, it is the cheapest (~21,000 RPC reads,
~30 min, one script), and it can kill H3 before any harness is built.

Identity of all seven venues was verified read-only at slot **440,578,912** (§3 of the gate).
**7 of 7 carry a live upgrade authority — none is immutable.** Kamino Vaults and klend share the
authority `GzFgdRJXmaw…`, which is why the G1 count excludes same-authority programs.

The motivating measurement, disclosed in the gate as pre-G0 and deciding no item: `../solvo` captured
klend at slot 440,477,778 and klend was **redeployed 8,997 slots (≈ 1 h) later**, so a carefully
executed, independently recomputed capability verdict was already bound to a replaced binary before it
was written down. `code_hash` `8eab9f85…` (captured) vs `b1344d19…` (mainnet today).

**Next action: founder ruling on G0.** `G0` is a precondition and carries no verdict; committing it
does not start G1.

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
