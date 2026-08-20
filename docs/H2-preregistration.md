# H2 / G0 — Pre-registration (frozen on commit)

Binding gate: [`docs/H2-GATE.md`](./H2-GATE.md). This document is G0. It has **no verdict** — it is the
precondition that makes G1–G5 honest.

> **凍結宣言。** このコミット以降、W（期間）、V（対象）、損失定義、制御判定、補償判定、および
> G1〜G5 の全閾値は**変更しない**。項目が落ちた場合、別の期間・別の対象・緩めた閾値・通りやすい
> 代理指標での再試行は**延命であり禁止**する。別の W / V は別の Gate であり、G0 からやり直す。

## What pre-registration binds — and what it does not

It binds **our discretion**. It does not bind **our knowledge of the chain**.

- **Frozen (discretion):** which venues, which window, what counts as a loss, what counts as
  controlled, what counts as covered, and every threshold. Changing any of these after seeing a number
  is how a gate is argued past.
- **Not frozen (knowledge):** byte offsets, account discriminators, which address most cheaply
  enumerates an event. These are facts about the chain, not choices. Getting one wrong is a bug to be
  fixed; discovering one at G1 time is not a degree of freedom.

The line matters because H1's P0 was a byte-layout bug (a fact, correctly fixed) while H1's fatal error
was an unmeasured adopter (a discretion, never fixed). They are different failures and G0 guards only
the second.

## Attestation — what was looked at before freezing

Written before any loss measurement. To fix W and V, the following were queried and **only** the
following:

| checked | result | why it is not a peek at the answer |
|---|---|---|
| current slot, `getFirstAvailableBlock` | 440,558,508 / 0 | retention, not loss |
| `getBlockTime` at both window boundaries | see W below | calendar arithmetic |
| `getBlock` at slots ~90d / ~40d / ~1d back | all served | archival availability |
| `getAccountInfo` for 6 candidate program IDs | 5 exist, 1 does not | existence, not magnitude |
| `getSignaturesForAddress` limit 1000, per program | tx/s density | RPC cost, not loss |

No loss event was decoded, no amount was computed, no venue was added or removed on the basis of a
magnitude. The one venue dropped (`vAuLTQTvpZ8AiMbBLKN1QUbfsHhkPWFPPeNfnRLYqrE`) was dropped because
**the account does not exist on mainnet**, not because of anything it contained.

---

## W — the measurement window

| | slot | block time |
|---|---:|---|
| start | **421,060,000** | 1779309788 — 2026-05-20T20:43:08Z |
| end | **440,500,000** | 1787240150 — 2026-08-20T15:35:50Z |

- span 19,440,000 slots, **7,930,362 s = 91.79 days**
- Both boundaries are already in the past at G0 commit time, so W is immutable and cannot drift.
- Slot time is not constant, so W is normalised **by measured wall-clock**, not by slot count:
  `L_90 = L_total × (90 × 86400) / 7,930,362 = L_total × 0.9805`.
  The $50M threshold is applied to `L_90`.

## V — the venue list

**Selection rule, fixed before enumeration:** every Solana venue in which *a depositor or liquidity
provider absorbs a loss produced by another party's position or decisions*, and which is live on
mainnet at G0 commit. The rule is what is frozen; the list is the rule applied.

| venue | program ID | verified on mainnet | loss classes |
|---|---|---|---|
| Kamino Lend | `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD` | executable | L-A, L-D |
| marginfi v2 | `MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA` | executable | L-A, L-D |
| Save (Solend) | `So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo` | executable | L-A, L-D |
| Drift v2 | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` | executable | L-A, L-C, L-D |
| Jupiter Perps | `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu` | executable | L-B, L-D |
| Kamino Vaults | `KvauGMspG5k6rtzrqqn7WNn3oZdyKqLKwK2XWQ8FLjd` | executable | L-C |

**No venue may be added to V after this commit**, including one that a G1 result would make attractive.
A venue whose loss cannot be reconstructed contributes to `L_und`; it is not quietly dropped.

## The loss definition

A **loss event** is a state transition, inside W, at a venue in V, that reduces the recoverable value
held by an identifiable address, in one of these four classes:

| class | event | who bears it | controlled by the bearer? |
|---|---|---|---|
| **L-A** | a liquidation or write-down that leaves an obligation with debt exceeding collateral, the shortfall being absorbed by the reserve's depositors | depositors of that reserve, pro rata | **no** |
| **L-B** | trader profit paid out of a perp LP pool, reducing the pool's per-share value | LP token holders | **no** |
| **L-C** | a managed vault's per-share value falling as a result of the manager's positions | vault depositors | **no** |
| **L-D** | an address's own position is liquidated | the position owner | **yes** |

**USD derivation**, in this order, and the order is frozen:

1. the venue's **own on-chain USD-denominated field** at the event (ground-truth recovery — the H1
   principle: account state is the world);
2. failing that, the venue's **own oracle account** value at the event slot;
3. failing both, the event is **`undetermined`** and counts toward `L_und`.

An external price source is **not** permitted. A number we import is not a number the chain proves.

## Control classification — which loss counts toward the thresholds

The hypothesis is about loss the bearer **does not control**. So:

- **L-A, L-B, L-C count** toward `N_addr`, `N_events`, `L_gross` and the thresholds.
- **L-D is measured and reported in full, but excluded from the thresholds.** It is reported because a
  reader is entitled to see the size of what we excluded and why — not reporting it would be the same
  concealment as H1's dust.
- An event that cannot be classified as controlled or not is **`undetermined`**.

This is not a judgement about who deserves cover. It is the moral-hazard boundary: a loss the buyer
controls cannot be underwritten without changing the behaviour that produces it.

## Coverage determination — `covered` / `uncovered` / `undetermined`

An event is **`covered`** if, at the time it occurred, a remedy existed that made the bearer whole in
whole or in part, evidenced **on-chain**:

- a protocol insurance fund, backstop or reserve paid out against it;
- a socialised-loss mechanism that transferred the loss away from the bearer;
- a purchased cover contract was in force for that address and that loss.

**`uncovered`** if the loss stayed with the bearer and no such mechanism is found.

**`undetermined`** if coverage may have occurred off-chain (a treasury reimbursement, a governance
vote executed elsewhere, an off-chain policy) and cannot be settled from chain state.

`L_und` is **subtracted** from the addressable residual. Loss we cannot show was uncovered is not
counted as ours.

## Enumeration method, and its completeness requirement

Signature-walking a program for 91.79 days is cheap; fetching every transaction is not. Measured at G0:

| program | tx/s | 90d transactions | signature calls @1000 | per-tx fetches if unfiltered |
|---|---:|---:|---:|---:|
| Kamino Lend | 0.8 | ~5,963,000 | ~5,963 | infeasible |
| marginfi v2 | 0.2 | ~1,784,000 | ~1,784 | infeasible |
| Jupiter Perps | 0.6 | ~4,342,000 | ~4,342 | infeasible |
| Drift v2 (program address) | ~0.03 | ~15,000 | ~15 | feasible |

So each venue is enumerated from the **narrowest address or state query that provably contains all
events of its class**. Which address that is, is chain knowledge and may be determined at G1 time — but
it carries a **completeness requirement that is frozen here**:

> The narrow enumeration must be validated against a full enumeration over a **randomly chosen
> contiguous sub-window of W of at least 24 hours**, and its **recall reported**. A narrow enumeration
> with recall < 0.98 may not be used. Cheapness never substitutes for completeness — an undercount
> would move `L_total` down and is exactly the kind of error that would be mistaken for a clean KILL.

**RPC budget ceiling (frozen): 2,000,000 requests.** If G1 cannot complete within it, G1 returns
**`BLOCKED` — data access**, naming the resolving action (an archival/indexed provider) and a date. It
does **not** return a partial number, and it does not shrink W to fit.

## Thresholds — restated verbatim from `docs/H2-GATE.md`, frozen

| item | KILL condition |
|---|---|
| G1 | `N_addr` < 1,000 · `N_events` < 300 · `L_90` < $50,000,000 · `C_10` ≥ 50% · `L_und`/`L_gross` > 30% |
| G2 | `R` < 0.95 · `A` < 0.95 |
| G3 | `AUC` ≥ 0.90 · `D` ≥ 0.20  *(a ceiling, not a floor — a low `AUC` is not a failure)* |
| G4 | no payout form with `median(e)` ≤ 0.20 **and** `p90(e)` ≤ 0.50 |
| G5 | any deciding number fails independent re-derivation beyond stated slot drift |

## Method requirements carried from H1's post-mortem

- Decode from **published schemas**, never inferred fixed offsets, and **fail closed**: an unreadable
  field is an error, never a silently skipped record.
- **Every number carries its slot.**
- Cross-check any streamed or filtered population against an **independent full count**, and report the
  direction of the difference.
- Report the **distribution**, not only the headline.
- State the **direction of every discrepancy, in both directions** — an error favouring the KILL
  disqualifies as much as one favouring the project.

## Roles

Claude wrote this G0 (specification is frame-thin work). **Codex reviews it, and does not co-author
it** — no model reviews its own output. G0 is not frozen in practice until that review is recorded.

## What happens next

1. Codex independently reviews this pre-registration.
2. On `APPROVE`, G0 is frozen and G1 may be implemented — **the first code H2 is permitted to write.**
3. G1 runs once, and its number is the verdict.
