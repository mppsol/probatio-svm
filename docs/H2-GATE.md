# H2 — Kill Gate — **FROZEN UNEXECUTED (2026-08-21)**

> ⛔ **This gate is frozen and is no longer binding.** The binding gate is
> [`docs/H3-GATE.md`](./H3-GATE.md).
>
> **Founder ruling, 2026-08-21:** H2's risk-transfer hypothesis is **frozen without being executed**.
> Not `KILLED` — no G1 measurement was ever run, and no number here is a verdict. Not `BLOCKED` —
> nothing external is being waited on. It stopped because G0 entered a specification spiral (three
> rounds, 331 lines, zero measurements) and the founder chose a different hypothesis over a fourth
> round. **Nothing below is rewritten.** H2 may be resumed only by a founder ruling, from its own G0,
> with its pre-registration re-frozen; resuming it inside H3 would be 延命.

**Status when frozen: frame fixed, no evidence.** This document superseded [`docs/GATE.md`](./GATE.md)
as the binding gate until 2026-08-21. `GATE.md` is H1's gate; H1 is **KILLED** and stays killed
([`docs/decisions/P1-real-target.md`](./decisions/P1-real-target.md)).

> **H1 の結論は書き換えない。** H1 は KILLED であり、その証拠・数値・レビューはそのまま残す。
> H2 は H1 の**反証を入力として**開始する新しい仮説であり、H1 の延命ではない。
>
> H2 の各判定項目について:
> - 数値または再現可能な実験で結論を出す
> - 結論が NO、未証明、または重大な OPEN_RISK なら `KILLED` または `BLOCKED` に更新して停止する
> - **仮説を追加して延命しない**
> - **事前登録したパラメータ（期間・対象・閾値）は測定後に変更しない**
> - テスト、証拠、レビューが揃った場合のみ `GO`
>
> **実資金、mainnet deploy、force push、秘密情報の追加は禁止する。**
> **このフェーズではコードを実装しない。** G0 が終わるまで書くのは仕様と計画だけである。

---

## What H1 proved, and what H2 must therefore not do

H1's hypothesis was: *certify autonomous agents in Solana DeFi before capital is trusted to them.*
It died on P1 — not because the tool failed, but because **the subject did not exist**:

| H1 measurement (mainnet slot 440479436) | value |
|---|---:|
| registered on-chain agents with state the harness could certify | **0** of 1,767 identities |
| live Jupiter Perps open interest under program control | **0.05%** ($32,423 of $64.5M) |
| holders indistinguishable on-chain from human traders | **96.4%** ($62.2M) |
| the headline PASS the project shipped | a **$16** position, below a $50 rounding band |

Four lessons are **inputs** to H2, not history:

1. **The adopter was asserted, never counted.** H1 named "agent developers" and never once counted
   them. → *H2 counts the buyer before it describes the buyer.*
2. **A working tool with no subject is worth nothing.** "It runs" was never the question.
   → *H2's first gate item is the existence of the subject, not the capability.*
3. **A headline number can hide a degenerate distribution.** 16.87% PASS looked like a verdict;
   98.4% of it was sub-$50 dust. → *Every H2 measure reports its distribution and concentration, and
   a degenerate distribution is a KILL even when the headline passes.*
4. **Errors are not symmetric, and they are not always flattering.** H1's one numeric defect was
   **non-monotone**: a bogus field widened the search (favoured the project) while the omitted
   `agent_wallet` narrowed it (favoured the kill). → *H2 decodes from published schemas, fails closed,
   and states the direction of every discrepancy — in both directions.*

**H2 does not mention agents.** The word, the framing, and the H1 artifacts are excluded from H2's
hypothesis. H1's code and gallery remain in the tree as evidence of a closed question, not as
foundation.

---

## The hypothesis

> **There exists, on Solana today, a countable population that bears a reconstructible financial loss
> it does not control, whose loss can be matched by a payout within a stated error band, at a moment
> when the outcome is not yet determined — and a third party can verify every one of those claims from
> public chain state.**

This is a **risk-transfer** hypothesis, not a verification hypothesis. It is falsifiable at every
clause, and each clause is a gate item below.

Deliberately **not fixed** by this document, because fixing them by assertion is what killed H1:

- **who the buyer is** — G1 measures the loss-bearing population and the buyer is whatever that
  measurement names. If the measurement names nobody, H2 is KILLED at G1.
- **the payout form** — G4 tests parametric / indemnity / staked-bond against the same historical loss
  set and selects numerically. If none clears the basis band, H2 is KILLED at G4.

---

## Non-goals

Out of scope for the whole of H2, without exception:

- **Any code implementation before G0 is committed.** This phase produces specification only.
- **Reviving H1 in any form** — certification, attestation, verifiers, invariants, the guard program,
  the `MandateSpec`, the Agent Registry, the gallery. Reusing an H1 artifact because it exists is
  exactly the "it would work if we also had X" move the gate forbids.
- **Any framing built on autonomous agents.** H1 measured that population at ~0.
- **Building a book, quoting a price, or taking any risk.** H2 measures whether a book *could* exist.
- **New on-chain programs, new ingestion adapters, UI, dashboards, demos, or deployment.**
- **Real funds, mainnet writes, testnet or mainnet deploy, force push, committed secrets.**
- **Market sizing by analogy, comparable, or narrative.** Only measured on-chain quantities count.

---

## The gate

Items run **in order**. Each is decided by a number computed from public chain state at a named slot.
A failure at any item is a `KILLED` for H2 as a whole — not a reason to reorder, re-scope, or retry.

### G0 — Pre-registration (no verdict; a precondition)

Before *any* measurement is run, the following are written down and committed, and the commit hash is
recorded in `STATUS.md`:

- **W** — the measurement window (a start slot and an end slot, both named).
- **V** — the venue list: the exact programs whose state will be read, fixed in advance.
- **The loss definition** — what counts as a loss event, per venue, in bytes: which accounts, which
  fields, which state transition, and how the USD amount is derived.
- **The coverage determination** — what counts as an *existing remedy* for a loss event (an insurance
  fund that paid, a protocol backstop, a socialised-loss mechanism, a purchased cover), and exactly how
  each event is classified `covered` / `uncovered` / `undetermined`. Pre-registered for the same reason
  as everything else: "covered" defined *after* seeing the number is the easiest place in this gate to
  manufacture a pass.
- **Every threshold in G1–G5 below**, unchanged from this document unless changed *before* G0 commits.

> **The anti-延命 rule.** After G0 commits, **W, V, the loss definition and the thresholds are frozen.**
> If a gate item fails, it fails. Re-running with a different window, a different venue, a looser
> threshold, or a proxy measure that is easier to pass is **延命** and is forbidden. A different W or V
> is a *different gate* and starts over from G0 with its own pre-registration.

### G1 — Does the buyer exist, and is the loss poolable?  *(criteria 1, 2)*

Over W and V, measure and report:

| symbol | quantity |
|---|---|
| `N_addr` | distinct addresses that bore ≥1 **uncovered** loss event |
| `N_events` | distinct **uncovered** loss events |
| `L_gross` | aggregate loss, USD — the headline number |
| `L_cov` | of that, loss an existing remedy actually paid or was contractually available for |
| `L_und` | of that, loss whose coverage status could not be determined |
| **`L_total`** | **`L_gross − L_cov − L_und` — the uncovered residual. This, not `L_gross`, is the addressable loss and the number every threshold below is applied to.** |
| `C_10` | share of `L_total` from the 10 largest events |
| — | the full loss distribution (deciles), not only the headline |

**Why `L_total` is the residual and not the gross.** This is H1's error in its other form. H1's
headline was $64.5M of open interest; the number that decided the gate was ~0, because the gross figure
counted a population the product could not address. Loss that an existing mechanism already makes good
is not a market — it is someone else's book. Measuring the gross and calling it addressable would
reproduce exactly the mistake that killed H1, one layer up.

**`L_und` counts against the project, not for it.** Undetermined coverage is subtracted from the
addressable residual rather than added to it. The burden is on the project, and not-proven is a KILL —
so a loss we cannot show was uncovered is not counted as ours.

**KILLED if any of:**

| condition | why this number |
|---|---|
| `N_addr` < **1,000** | below this there is no book: idiosyncratic risk cannot be diversified, and any risk-transfer structure needs many independent exposures |
| `N_events` < **300** | below ~300 events a loss frequency cannot be estimated to better than roughly ±6% standard error; pricing would be guesswork presented as a number |
| `L_total` < **$50M** per 90 days | the arithmetic, stated openly: $50M/90d ⇒ ~$200M/yr insured loss; at a 5–10% premium rate that is a $10–20M/yr *total addressable* premium pool; at a realistic 5–10% share that is $0.5–2M/yr — the floor at which a team is fundable. Below it the business cannot exist even under generous assumptions |
| `C_10` ≥ **50%** | a book where ten events carry half the loss is not poolable — it is a bet on ten outcomes, not insurance. This is the H1 dust lesson: a passing headline over a degenerate distribution is still a KILL |
| `L_und` / `L_gross` > **30%** | if nearly a third of the loss cannot be classified as covered or not, the residual carries an error bar wider than the decision it is supposed to make. The measurement cannot decide, and **not-proven is a KILL** — it is not a reason to widen the classifier until it can |

If W is not 90 days, `L_total` is scaled to a 90-day equivalent **before** comparison, and the scaling
is stated.

### G2 — Is the loss reconstructible and attributable?  *(criteria 2, 5)*

| symbol | quantity |
|---|---|
| `R` | share of `L_total` reconstructible from public chain state alone, each event pinned to a slot or signature |
| `A` | share of `L_total` attributable to exactly one address, without ambiguity |

**KILLED if `R` < 0.95 or `A` < 0.95.** An unverifiable or unattributable loss cannot be adjudicated;
the payout becomes discretionary, and criterion 5 fails by construction. Any shortfall is itemised by
cause — a loss that is merely *hard* to reconstruct is reported as unreconstructible.

### G3 — Was the outcome undetermined at participation?  *(criterion 4)*

At `t0` = the hypothetical moment of participation, using **only public state visible at `t0`**:

| symbol | quantity |
|---|---|
| `AUC` | discrimination of the simplest honest predictor of "bears a loss within the next N days" |
| `D` | share of eventual `L_total` already determined at `t0` (e.g. already past the liquidation boundary, already insolvent) |

**KILLED if `AUC` ≥ 0.90 or `D` ≥ 0.20.** If the outcome is near-deterministic from public data at
participation, it is not fortuitous — it is a wager on a known result, and it would be bought only by
those already losing.

**A low `AUC` is not a kill.** Pure pooling of unpredictable loss is exactly what risk transfer is for.
This item has a ceiling, not a floor — stating that explicitly, because the tempting mistake is to
read a low number as failure and start adding predictive machinery, which is scope, not evidence.

### G4 — Can payout and loss be made to coincide?  *(criterion 3)*

Against the **same** event set established in G1–G2, for each of the three candidate forms —
**parametric**, **indemnity**, **staked/slashed bond** — compute the per-event basis error
`e = |payout − loss| / loss` and report `median(e)` and `p90(e)`.

**KILLED if no form achieves `median(e)` ≤ **0.20** and `p90(e)` ≤ **0.50**.** If payout and loss cannot
be made to coincide within that band on real historical events, criterion 3 fails by definition and the
instrument is a wager, not risk transfer.

Otherwise **the payout form is fixed by this measurement**: the passing form with the lowest `p90(e)`
that also clears G5. No form is chosen for elegance, precedent, or implementability.

### G5 — Can a third party reconstruct all of it?  *(criterion 5)*

- Every deciding number is produced by one committed script, from public RPC, and **carries its slot**.
- Every deciding number is **independently re-derived from its definition, not from the code**, by the
  model that did not produce it, which states **the direction of every discrepancy**.
- Any streaming or sampled computation is cross-checked against an independent full count.

**KILLED if any deciding number fails independent re-derivation beyond stated slot drift.**

### G6 — The numbers are the verdict  *(criterion 6)*

Every item above carries its kill number in advance. There is no item whose verdict is an assessment.
If a number is missing, the item is **not-proven**, and **not-proven is a KILL**.

---

## Reproducible verification plan

Order is fixed. **Each step stops the phase on failure** — there is no step that runs "anyway".

| step | produces | decided by | on failure |
|---|---|---|---|
| **G0** | `docs/H2-preregistration.md` + commit hash in `STATUS.md` | — (precondition) | cannot proceed |
| **G1** | `N_addr`, `N_events`, `L_gross`/`L_cov`/`L_und`/`L_total`, `C_10`, decile distribution, all @slot | one script, one command | `KILLED`, stop |
| **G2** | `R`, `A`, itemised shortfall by cause | same event set, one script | `KILLED`, stop |
| **G3** | `AUC`, `D`, and the `t0` visibility rule used | one script, point-in-time state only | `KILLED`, stop |
| **G4** | `median(e)`, `p90(e)` × 3 forms; the selected form | one script over the G1–G2 event set | `KILLED`, stop |
| **G5** | independent re-derivation of every deciding number | the other model, from definitions | `KILLED`, stop |

**Method, carried forward from H1's post-mortem — these are requirements, not style:**

- **Decode from the published schema, never from inferred fixed offsets**, and **fail closed**: an
  unreadable field is an error, never a silently skipped record. H1's P0 was exactly this.
- **Every number carries a slot.** Live-chain figures drift; a number without a slot is not a number.
- **Cross-check any streamed or filtered population against an independent full count**, and report the
  direction of the difference.
- **Report the distribution, not only the headline.** A measure that separates size rather than
  substance is a failed measure even when its headline passes.
- **State the direction of every discrepancy, in both directions.** An error that favours the KILL is
  as disqualifying as one that favours the project.

**Roles — at most two, never concurrent on the same artifact:**

| role | who | does |
|---|---|---|
| specification, evidence, adjudication | **Claude** | what the gate means, what would falsify it, what runs next |
| implementation **or** independent review | **Codex** | one role at a time, chosen deliberately; never both |

No model reviews its own output. The model that computed a number does not certify it.

---

## Differentiation — resolved into G1, not added as a seventh item

H1's `KILLED` clause was *"the differential cannot be shown, or the adopter is vague."* The founder's
six criteria do not name differentiation, and it is **not** added here as a seventh gate item. It is
folded into G1's definition instead, for three reasons:

1. **It is the same error H1 died of, one layer up.** Gross loss counts a population the product cannot
   address, exactly as H1's $64.5M of open interest counted a population it could not certify. The fix
   belongs in the denominator, not in a bolted-on item.
2. **It costs almost nothing.** Classifying coverage runs over the *same* event set G1–G2 already
   produce. A separate item would re-derive the same data to ask a question the residual already answers.
3. **It makes the existing threshold correct rather than merely stricter.** The $50M/90d figure was
   derived from an *addressable* premium pool. Applying it to gross loss was the sloppy version;
   applying it to the uncovered residual is what the arithmetic always meant.

**What this does test:** whether uncovered loss exists at all. If every measured loss already has a
remedy, there is no room, and H2 is KILLED at G1 rather than after a full cycle.

**What this deliberately does not test, and is deferred to H3:** *why this rather than the incumbent* —
product differentiation, pricing, distribution. At gate stage the answerable question is whether the
room exists; whether our version of the thing is better is a question you can only ask once it does.
Recorded here so it is a known deferral rather than an omission.

## On reaching the gate: stop

`GO` ends H2; it does not start H3. On any verdict — `GO`, `KILLED`, `BLOCKED` — the run stops,
`STATUS.md` is updated, the evidence is committed and pushed, and the founder decides what happens
next. **Auto-entering the next phase is the failure this document exists to prevent.**
