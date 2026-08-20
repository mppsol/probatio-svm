# H2 / G0 — Pre-registration, **r2**

Binding gate: [`docs/H2-GATE.md`](./H2-GATE.md). This document is G0. It has **no verdict** — it is the
precondition that makes G1–G5 unarguable.

**r1 was reviewed by Codex and returned `CHANGES` with five P0s** —
[`reviews/H2-G0-prereg.md`](../reviews/H2-G0-prereg.md). Every one was valid. r2 closes them. The
changelog at the end says what changed and in which direction.

> **凍結宣言。** このコミット以降、W、V、損失定義、イベント単位、配分規則、制御判定、補償判定、
> 列挙の完全性要件、G3/G4 の手続き、G5 の許容差、および全閾値は**変更しない**。
> 項目が落ちた場合、別の期間・別の対象・緩めた閾値・通りやすい代理指標での再試行は**延命であり禁止**。

## What pre-registration binds — and what it does not

It binds **our discretion**, not **our knowledge of the chain**.

- **Frozen:** venues, window, what counts as a loss, the *unit* of an event, the allocation rule, what
  counts as controlled, what counts as covered *and by how much*, the completeness requirement, the
  G3 and G4 procedures, the G5 tolerance, and every threshold.
- **Not frozen:** byte offsets, account discriminators, schema versions. These are facts about the
  chain. Getting one wrong is a bug to fix; learning one at G1 time is not a degree of freedom.

**r1 put the enumeration address on the wrong side of this line.** Codex was right: until the event
schema and a deterministic completeness test are bound, choosing the address *is* a sampling decision.
r2 binds them (§ Enumeration).

## Attestation — what was looked at before freezing

To fix W, V and feasibility, only the following were queried: current slot and retention floor;
`getBlockTime` at both W boundaries; `getBlock` at three slots to confirm archival availability;
`getAccountInfo` for 7 candidate program IDs; `getSignaturesForAddress` limit 1000 per program for
density; `getBlock(440500000)` for its blockhash, used as the sub-window seed below.

**No loss event was decoded, no loss amount was computed, and no venue was added or removed on the
basis of a magnitude.** The one candidate dropped (`vAuLTQTvpZ8AiMbBLKN1QUbfsHhkPWFPPeNfnRLYqrE`) was
dropped because the account does not exist on mainnet.

This attestation is not independently auditable, and Codex correctly recorded that. It is a statement
of method, not evidence. What *is* auditable is that every number in this document is reproducible by
the commands it names.

---

## W — the measurement window

| | slot | block time |
|---|---:|---|
| start | **421,060,000** | 1779309788 — 2026-05-20T20:43:08Z |
| end | **440,500,000** | 1787240150 — 2026-08-20T15:35:50Z |

- span 19,440,000 slots, **7,930,362 s = 91.7866 days**
- both boundaries already in the past at G0 commit, so W cannot drift
- normalised by **measured wall-clock**, not slot count:
  `L_90 = L_total × (90 × 86400) / 7,930,362 = L_total × 0.980535`
- the $50M threshold is applied to `L_90`

```
curl -s -X POST https://api.mainnet-beta.solana.com -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","id":1,"method":"getBlockTime","params":[421060000]}'   # 1779309788
  --data '{"jsonrpc":"2.0","id":1,"method":"getBlockTime","params":[440500000]}'   # 1787240150
```

## V — the venue list, and the corpus it was drawn from

**Selection rule:** every venue in the candidate corpus below in which *a depositor or liquidity
provider absorbs a loss produced by another party's position or decisions*, and which is executable on
mainnet at G0 commit.

**Candidate corpus (frozen).** r1 asserted "every Solana venue" with no corpus, which Codex correctly
called untestable. The corpus is now an explicit list, so the rule's application can be checked:

| candidate | program ID | in V? | why |
|---|---|---|---|
| Kamino Lend | `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD` | **yes** | depositors absorb bad debt |
| marginfi v2 | `MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA` | **yes** | depositors absorb bad debt |
| Save (Solend) | `So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo` | **yes** | depositors absorb bad debt |
| Drift v2 | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` | **yes** | depositors absorb bad debt; vault depositors bear manager outcome |
| Jupiter Perps | `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu` | **yes** | JLP holders absorb trader PnL |
| Kamino Vaults | `KvauGMspG5k6rtzrqqn7WNn3oZdyKqLKwK2XWQ8FLjd` | **yes** | vault depositors bear manager outcome |
| Drift Vaults (candidate) | `vAuLTQTvpZ8AiMbBLKN1QUbfsHhkPWFPPeNfnRLYqrE` | **no** | **account does not exist on mainnet** — `getAccountInfo` returns `value: null` |
| AMM LP adverse selection (Orca, Raydium, Meteora) | — | **no** | excluded by rule: LP loss to arbitrageurs is a *price* outcome, not another party's *position or decisions*. Recorded as a deliberate exclusion, see the asymmetry note |

**Operational definitions**, frozen, so the rule is testable:

- **venue** — a single executable mainnet program ID whose accounts record both the loss and the
  bearer's claim on the pool.
- **live** — `getAccountInfo` returns `executable: true` at G0 commit.
- **"produced by another party's position or decisions"** — the loss is caused by the outcome of a
  position or instruction *the bearer neither owns nor authorises*. Price movement alone does not
  qualify; a counterparty's default or a manager's trade does.

**The omission asymmetry, stated because it is not symmetric.** An omitted venue lowers `L_gross`,
`N_addr` and `N_events` — that direction can only produce a KILL, never a pass. But it can also lower
`C_10` (a venue with one large event, omitted, reduces measured concentration), and *that* direction
can produce a pass. So the corpus is frozen rather than left open, and **`C_10` is computed per-venue
as well as pooled; the KILL fires on the pooled figure or on any single venue's figure** (§ Thresholds).

**No venue may be added to V after this commit**, including one a G1 result would make attractive.

## The loss definition

### The event, its unit, and its allocation — the r1 P0

r1 defined a loss event semantically and left the *unit* open. Codex's exploit was exact: one reserve
write-down borne pro rata by 500 depositors could later be reported as 1 event or 500, moving
`N_events`, `N_addr` and `C_10` in whichever direction was needed. Frozen now:

- **An event is one venue-side state transition.** A single bad-debt write-down is **one** event,
  regardless of how many addresses bear it. Events are never split by bearer.
- **An allocation is one (event, address) pair** with a non-zero share of that event's loss.
- `N_events` counts **events**. `N_addr` counts **distinct addresses across allocations**.
- **`C_10` is computed over events, never over allocations** — the concentration test asks whether ten
  *incidents* carry half the loss, and splitting an incident into depositor slices would defeat it.
- **Allocation rule:** pro rata by the bearer's share balance in the affected pool at the event slot,
  read from the venue's own share accounting. Balances at any other slot are not used.

### The four classes

| class | event | bearer | controlled by bearer? |
|---|---|---|---|
| **L-A** | a liquidation or write-down leaving an obligation with debt exceeding collateral, the shortfall absorbed by the reserve's depositors | reserve depositors, pro rata | **no** |
| **L-B** | realised trader profit paid out of a perp LP pool | LP token holders, pro rata | **no** |
| **L-C** | a managed vault's realised loss from the manager's positions | vault depositors, pro rata | **no** |
| **L-D** | an address's own position is liquidated | the position owner | **yes** |

**L-B and L-C are measured only from the venue's own recorded realised amount.** r1 said "reducing" and
"falling" without a baseline, which Codex correctly identified as selectable after the fact. There is
no share-price-difference method here: per-share value moves with oracle prices, fees, deposits and
withdrawals, and decomposing that after seeing the series is precisely the freedom G0 exists to remove.
**If a venue does not record the realised amount on-chain, its L-B/L-C events are `undetermined`.** This
is fail-closed and will likely make `L_und` large — which is what the 30% kill is for. A gate that
reports "cannot decide" is working; one that manufactures a decomposition is not.

**L-D is measured and reported in full but excluded from every threshold.** Reported, because
concealing the size of an exclusion would be H1's dust error again.

### USD derivation, and the r1 hole

Order frozen:

1. the venue's own on-chain **USD-denominated field** at the event slot;
2. else the venue's own **oracle account** value at the event slot;
3. else the venue's own oracle at the **nearest slot within ±150 slots** of the event.

r1 then said "else the event is undetermined and counts toward `L_und`" — but `L_und` is a *dollar*
quantity, so r1 assigned no dollars to an event with no dollars. Codex found the hole. Closed:

- an event that survives all three steps with **no derivable amount** is recorded in a separate counter
  **`E_unknown`** (a count, not a sum) and is excluded from every dollar aggregate;
- **KILL if `E_unknown` > 5% of `N_events`.** Past that the dollar aggregates are describing an
  unknown fraction of reality, and not-proven is a KILL.

**An external price source is not permitted.** A number we import is not a number the chain proves.

## Coverage — an amount, not a label

r1 classified an event `covered` if a remedy made the bearer whole "in whole or in part", then
subtracted `L_cov` in dollars. Codex's exploit: a partial payment could be called full coverage, or an
unlocated remedy called absent. Frozen now:

- coverage is an **amount** `c_i ∈ [0, loss_i]` per event, not a label. `L_cov = Σ c_i`.
- `c_i` is the amount **actually transferred** to the bearers by a mechanism in the frozen search
  universe below, in the same window, evidenced on-chain.
- a remedy that is **automatic** under the venue's own program logic counts at its programmatic amount
  even if not yet claimed; a remedy requiring a **discretionary** act (a governance vote, a treasury
  transfer) counts only for the amount actually transferred on-chain within W, and any residual is
  **`undetermined`**, not `uncovered`.

**Search universe (frozen).** "No such mechanism is found" may be asserted only after searching, for
each venue in V: (a) every account owned by that venue's program that holds SPL tokens or lamports and
is designated by the program as an insurance fund, backstop or reserve; (b) every socialised-loss field
in the venue's own accounting. **A remedy located outside this universe makes the event `undetermined`,
never `uncovered`.** Stopping the search early therefore cannot manufacture a pass — it moves loss into
`L_und`, which is subtracted from the residual and guarded by the 30% kill.

## Enumeration and its completeness — the r1 P0

r1 let G1 pick "the narrowest address that provably contains all events". Codex's exploit: 0.98
*event-count* recall permits missing the ten largest events, which lowers `C_10` and can turn a
concentration KILL into a pass. Frozen now:

1. **Completeness is proved from the schema, not from a sample.** The chosen address or state query
   must be shown, from the venue's published schema, to be touched by *every* event of its class. The
   sub-window test corroborates that argument; it never substitutes for it.
2. **Two recalls, both required, measured against a full enumeration of the sub-window:**
   **event-count recall ≥ 0.98** *and* **dollar-weighted recall ≥ 0.99**. The second is what closes the
   large-event exploit.
3. **The sub-window is selected deterministically and not by us.** W is divided into 91 whole 24-hour
   sub-windows indexed from the start. The index is
   `sha256(blockhash of slot 440,500,000) mod 91`. That blockhash is
   `9WgLiyHZD8cS2JJ7LZ8GNDhmi4LV9jkGu11z7zb5Tpx4`, giving index **46** — i.e. the 24 hours beginning
   `421,060,000 + 46 × (19,440,000 / 91.7866)` slots from W's start, computed by wall-clock from the
   window start time. Anyone can recompute it; nobody can choose it.
4. **The full enumeration procedure for the sub-window is:** every transaction touching the venue
   program in that 24 hours, fetched and decoded — feasible precisely because it is one day, not 92.

### Feasibility, re-measured (r1 P1 — corrected)

r1's Drift row said both `~0.03 tx/s` and `~15,000 txs/90d`, which cannot both be true. Codex was
right, and the `~0.03` was a figure my own measurement never produced — the correct rate is
**0.00194**. r1 also recorded no sample slots and no command, so it was not reproducible. Corrected,
with the command and the sampled slot range:

```
curl -s -X POST https://api.mainnet-beta.solana.com -H 'Content-Type: application/json' --data \
'{"jsonrpc":"2.0","id":1,"method":"getSignaturesForAddress","params":["<PROGRAM>",{"limit":1000,"commitment":"finalized"}]}'
# rate = (1000-1) / (max(blockTime) - min(blockTime))
```

| venue | sampled slots (newest/oldest) | sig/s | 90d signatures |
|---|---|---:|---:|
| Kamino Lend | 440560749 / 440556343 | 0.54471 | ~4,235,672 |
| marginfi v2 | 440560749 / 440549916 | 0.22161 | ~1,723,208 |
| Save | 440559755 / 440374336 | 0.01296 | ~100,772 |
| Drift v2 | 440558182 / 439317643 | **0.00194** | ~15,063 |
| Jupiter Perps | 440560742 / 440556106 | 0.51762 | ~4,024,986 |
| Kamino Vaults | 440560686 / 440546251 | 0.16628 | ~1,292,980 |
| **total** | | | **~11,392,681** |

These are 1,000-signature samples, not a census, and are stated as order-of-magnitude feasibility
inputs only. They agree with Codex's independent samples to within sampling noise. **~11.4M
per-transaction fetches is far above the ceiling below — so the narrow-address method is load-bearing,
not a convenience.**

### Request accounting and the terminal data-access rule — the r1 P0

r1 froze a 2,000,000-request ceiling without defining a request, and let an overrun return `BLOCKED`
with no consequence. Codex called it an escape hatch, correctly. Frozen now:

- **one request = one HTTP JSON-RPC call.** A batch of *k* counts as *k*. Every retry counts. A
  response served from a committed on-disk cache counts as 0 **only if that cache is committed to the
  repo or its hash is recorded**; otherwise it counts.
- **Ceiling: 2,000,000 requests.**
- If G1 cannot complete within it, G1 returns **`BLOCKED — data access`**, and per `docs/H2-GATE.md`
  that requires *a stated action on a stated date*: it must name the specific archival or indexed
  provider and a date **no more than 30 days after the G1 attempt**.
- **If that date passes without the data, H2 is `KILLED`.** `BLOCKED` is time-boxed and terminal. It is
  not a state H2 may rest in.
- G1 may **not** shrink W, drop a venue, or report a partial number instead.

## G3 — the procedure, frozen (r1 P0)

r1 said "the simplest honest predictor", which is an assessment — the exact thing `docs/H2-GATE.md`
forbids as a verdict. Frozen now:

| parameter | value |
|---|---|
| horizon `N` | **30 days** |
| `t0` per exposure | the slot at which the bearer's share balance first becomes non-zero, or W's start, whichever is **later** |
| features | balance, pool utilisation, pool total borrows, pool total deposits, the bearer's share of the pool, and the venue id — **all read at `t0` only**, no post-`t0` value |
| estimator | **logistic regression, L2, C = 1.0, no hyperparameter search, no feature selection** |
| split | train on exposures whose `t0` falls in the **first half of W**, test on the **second half**; `AUC` is reported on the test half only |
| missing data | the exposure is dropped and the dropped count reported; **KILL if > 10% dropped** |
| `D` | share of `L_total` whose triggering condition was **already true at `t0`** (obligation already insolvent, position already past its liquidation boundary) |

A richer predictor is not permitted, and neither is a poorer one.

## G4 — the payout functions, frozen (r1 P0)

Codex's sharpest catch: an indemnity function chosen after seeing the losses is `payout = loss`, giving
`median(e) = p90(e) = 0` and a guaranteed G4 pass. r1 named the three forms and specified none of them.
Frozen now — **every parameter is fit on the first half of W and evaluated on the second half, and
`median(e)`/`p90(e)` are reported on the second half only.** No parameter may see the data it is scored
on.

| form | payout function | parameters, fit on the train half |
|---|---|---|
| **parametric** | `payout = R × X` where `X` is a frozen on-chain observable at the event slot (the venue's own recorded shortfall for L-A; recorded realised PnL for L-B/L-C) and `R` a single scalar | `R` = the ratio minimising median basis error on the train half |
| **indemnity** | `payout = min(loss, cap)` | `cap` = the 95th percentile of train-half event loss |
| **staked bond** | `payout = min(loss, bond)`, `bond` fixed per venue | `bond` = the 90th percentile of train-half per-venue event loss |

An out-of-sample `median(e) = 0` is then a real result; an in-sample one would have been an artefact.

## Thresholds — frozen

| item | KILL condition |
|---|---|
| **G1** | `N_addr` < 1,000 · `N_events` < 300 · `L_90` < $50,000,000 · **`C_10` ≥ 50% pooled, or ≥ 50% for any single venue in V** · `L_und`/`L_gross` > 30% · `E_unknown` > 5% of `N_events` |
| **G2** | `R` < 0.95 · `A` < 0.95 |
| **G3** | `AUC` ≥ 0.90 · `D` ≥ 0.20 · dropped exposures > 10%  *(`AUC` is a ceiling, not a floor — a low `AUC` is not a failure)* |
| **G4** | no form with out-of-sample `median(e)` ≤ 0.20 **and** `p90(e)` ≤ 0.50 |
| **G5** | a deciding number that fails independent re-derivation beyond the tolerance below |

### G5 tolerance — frozen (r1 P1)

r1 said "beyond stated slot drift" and never stated it. A deciding number **reproduces** if it is within
**±0.5% relative** of the independent re-derivation, **or** the entire difference is accounted for by
named events occurring between the two snapshot slots. Anything else is a failure. Counts (`N_addr`,
`N_events`, `E_unknown`) are compared as integers with the same ±0.5% rule; shares (`C_10`, `R`, `A`,
`AUC`, `D`) are compared in **absolute** percentage points with a tolerance of **±0.5 pp**.

## Method requirements carried from H1's post-mortem

- Decode from **published schemas**, never inferred fixed offsets, and **fail closed**.
- **Every number carries its slot.**
- Cross-check any streamed or filtered population against an **independent full count**, and report the
  direction of the difference.
- Report the **distribution**, not only the headline.
- State the **direction of every discrepancy, in both directions** — an error favouring the KILL
  disqualifies as much as one favouring the project.

## Roles

Claude wrote G0 r1 and r2. **Codex reviews and does not co-author.** G0 is not frozen in practice until
a Codex review records `APPROVE`.

## Changelog r1 → r2, with direction

| finding | change | direction |
|---|---|---|
| P0 event unit | event = one venue-side transition; allocations separate; `C_10` over events | removes a two-way exploit; net **against** the project (splitting could no longer inflate `N_events`) |
| P0 L-B/L-C baseline | measured only from the venue's own recorded realised amount, else `undetermined` | **against** the project — `L_und` will grow |
| P0 USD hole | `E_unknown` counter + 5% kill | **against** the project |
| P0 coverage | an amount not a label; frozen search universe; outside-universe ⇒ `undetermined` | **against** the project |
| P0 enumeration | schema-level proof + dollar-weighted recall ≥ 0.99 + deterministic sub-window | closes the `C_10` exploit; **against** the project |
| P0 G3/G4 | fully specified; all parameters fit out-of-sample | removes a guaranteed G4 pass; strongly **against** the project |
| P0 request accounting | request defined; `BLOCKED` time-boxed to 30 days and terminal (then `KILLED`) | **against** the project |
| P1 density table | Drift corrected 0.03 → **0.00194**; slots and command recorded | corrects a fabricated figure; immaterial to any verdict |
| P1 venue corpus | explicit frozen corpus, operational definitions, per-venue `C_10` kill | closes a two-way exploit |
| P1 G5 tolerance | ±0.5% relative / ±0.5 pp absolute | closes a one-way exploit |

**Every r2 change moves the gate against the project.** That is the expected direction when a
pre-registration is tightened, and it is stated so a later reader can check that r2 did not quietly
buy H2 an easier pass.

## What happens next

1. Codex independently reviews r2.
2. On `APPROVE`, G0 is frozen and G1 may be implemented — the first code H2 is permitted to write.
3. G1 runs once, and its number is the verdict.
