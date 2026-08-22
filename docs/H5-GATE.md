# H5 — Agent Release Tests for Solana — **CLOSED (2026-08-22)**

> ⛔ **This gate is closed.** `CLOSED — all authorised candidates failed at the design gate; H5 remains unproven.`
> Verdict and reasoning: [`docs/decisions/H5-closed.md`](./decisions/H5-closed.md).
>
> **Four authorised candidates, four design-gate failures, ZERO measurements and zero lines of
> product code.** §8.1 → `KILL-1`; `C1` and `C2` → lapsed unpursued after the ruling authorised `C3`
> alone; `C3` → `KILL AT DESIGN GATE` with four P0s ([`docs/H5-C3-G0.md`](./H5-C3-G0.md),
> [review](../reviews/H5-C3-G0.md)). **None may be revived, repaired or re-measured.**
>
> **The structural reason, and what a successor must answer:** C3's failure *is* a duplicate spend,
> and a cumulative spend cap set at the agent's own intent total catches a duplicate spend **by
> definition**. C3 could be given a passing baseline **only by keeping the comparator weaker than the
> one this hypothesis itself names** — *"a runtime wallet policy"*. **C3 was not differentiable from a
> runtime wallet policy at all.**
>
> **H5 is `unproven`, NOT `refuted`.** Nothing here says sequence-only failures do not exist on
> Solana — **that was never tested and is not claimed.**
>
> **Reopening requires all three:** a new founder ruling · a **new, independent hypothesis** (not a
> repair or rewording of H5, and not resting on §8.1/`C1`/`C2`/`C3`) · a new pre-registration that
> names the comparator **the hypothesis itself names, at full strength**. Not authorised and not to be
> started: a new fixture, a new candidate search, implementation, measurement, UI, token, deploy.
>
> **Nothing below is rewritten.** It is kept as the pre-registration the verdict is read against.

## The pre-registration, as committed before measuring


**Status: G0 pre-registration. Written and committed BEFORE any measurement.**
Founder ruling, 2026-08-22. This is the binding gate; `STATUS.md` records the phase.
**No measurement has been run. No implementation exists.**

> **The four closed gates are not reopened by this one.**
> H1 `KILLED` ([`docs/GATE.md`](./GATE.md), [`decisions/P1-real-target.md`](./decisions/P1-real-target.md)) ·
> H2 `FROZEN UNEXECUTED` ([`docs/H2-GATE.md`](./H2-GATE.md)) ·
> H3 `KILLED` at the design gate ([`docs/H3-GATE.md`](./H3-GATE.md)) ·
> H4 `KILLED` at G0, KILL-2 + KILL-3 ([`docs/H4-GATE.md`](./H4-GATE.md),
> [`decisions/H4-sentinel-kill.md`](./decisions/H4-sentinel-kill.md)).
> No verdict above is rewritten, resumed, or worked around. **H5 is a new hypothesis, not a revival.**

Rules carried forward unchanged: a verdict is a **number or a reproducible experiment**;
**not-proven is a KILL**; pre-registered parameters are not changed after measurement; every number
carries a **slot**; every discrepancy is reported **with its direction**, in both directions; at most
two agents, and **no model reviews its own output**. Forbidden without exception: real funds, mainnet
or devnet deploy, force push, committed secrets, and writing to `../solvo`.

---

## 1. The hypothesis

> **A Solana AI agent that moves capital in production should be regression-tested before release
> against adversarial scenarios that span real program BPF, real cloned state, and multiple
> transactions — and such a test finds failures that a single `simulateTransaction` or a runtime
> wallet policy cannot find.**

Working product name: **Probatio — Agent Release Tests for Solana**.
*"Before you give an AI agent a Solana wallet, make it survive the chain."*

**The claim that must be measured is the second half.** "Should be tested" is an opinion and is worth
nothing here. `finds failures a single simulation or a wallet policy cannot find` is a
**reproducible experiment**, and it is the only thing G0 pre-registers a verdict on.

## 2. Scope, fixed here

**In scope**

- The buyer: **agent developers and agent-wallet developers, before release**.
- The point of use: **CI and regression testing** for an agent that takes capital actions on Solana.

**Out of scope — naming these is binding, not decorative**

- a **runtime wallet policy engine** (H5 tests before release; it does not sit in the transaction path);
- a **general-purpose transaction simulator**;
- **production monitoring, insurance, or a certification market**;
- a **generic protocol capability schema** — H3 died of exactly this (`KILL-3` below);
- **"agent certification"**, and any claim that presupposes the existence of an agent population;
- **reusing H1's evidence or conclusions as demand evidence.** H1's asserted adopters, its gallery,
  its attestations and its framing are refuted material. They may be cited as *what was refuted*;
  they may never be cited as *why someone would buy this*.

## 3. What H5 claims to do differently

Each line is a property the measurement must exhibit, not a slogan:

1. **Inspects a state transition across several transactions**, not a single transaction's
   allow/deny.
2. Runs against **real BPF programs and cloned mainnet state**.
3. Covers failures of the shape **"the call returned success but the funds are not usable"**,
   **"a wrong retry after a revert"**, and **"a halted price, stale state, or a partial fill"**.
4. **Decides on state diffs, balance diffs and invariants — never on logs.** (Carried from H4/Solvo
   harness discipline: no assertion may read a log line.)
5. Ships a **reproducible failure trace and a regression test** as its deliverable.

## 4. Structural difference from H1, stated so it cannot be blurred

H1 was **`KILLED`**. This gate does not revive it, and nothing below rests on an H1 conclusion.

| | H1 (`KILLED`) | H5 |
|---|---|---|
| what it was | a **market that certifies agents already operating** | a **development/CI test run before an agent reaches production** |
| buyer | whoever would pay for third-party assurance about someone else's agent | the **agent developer**, on their own agent, pre-release |
| point of use | continuously, in operation | **once per release**, in the developer's own pipeline |
| unit of failure | an agent's trustworthiness, asserted | **one reproducible multi-transaction episode** that ends in a bad state |
| deliverable | an attestation / a certificate | a **failing trace and a regression test the developer keeps** |

**These are four different things, not four descriptions of one thing.** If H5's measurement ends up
needing an attestation, a certificate, a gallery, a `MandateSpec`, or an assertion about an agent
population, that is not a scope change — see `KILL-3` and `KILL-4`.

Reusing H1/H4 **mechanics** (LiteSVM driver, cloned-state fixtures, disclosed mutations,
deterministic offline replay) is permitted **as fixtures**. Reusing an H1 **conclusion** as evidence
for H5 is forbidden. Reusing an artifact *because it exists* is 延命, not scope.

## 5. The pre-registered GO conditions

`GO` requires **all four**, for **one fixed capital-action workflow**, with numbers:

| | condition |
|---|---|
| **A** | A **reproducible multi-transaction episode** can be built on **real BPF + cloned state**: ≥ 3 transactions, deterministic, offline once fixtures are committed, byte-identical across two runs. |
| **B** | **One** failure is detected **by state diff** that **neither** a static policy **nor** a single `simulateTransaction` can detect. Both baselines are **implemented and actually run** on the same episode, and both are shown to return "no problem". A baseline that is described but not run does not count. |
| **C** | That failure is shown to **change the agent's next action**, using a **fixed scripted agent** — deterministic, no LLM in the loop, its decision rule written down before the run. |
| **D** | Existing Probatio/Solvo assets are usable **as demonstration fixtures**, with **every input, mutation and hash recorded**, and **without overstating Solvo's conclusions** — each borrowed number carries its slot and its source, and Solvo's own verdicts are quoted as they stand. |

**B is the whole hypothesis.** A, C and D are the conditions that make B credible; a `PASS` on
A + C + D with a failed B is a **`KILL`**, not a partial success.

## 6. The kill conditions

| # | fires when |
|---|---|
| **KILL-1** | H5 turns out to be **equivalent to a single simulation, or to a destination/cap policy** — i.e. either baseline in **B**, once run, also catches the failure |
| **KILL-2** | a **stateful failure cannot be reproduced on real BPF** at all |
| **KILL-3** | deciding a verdict requires a **protocol-independent schema or arbitrary adapter semantics** — H3's grave, and H4's blocked rescue |
| **KILL-4** | **no concrete CI entrypoint** can be defined that attaches to an agent developer's release process |

**Not-proven is a KILL.** If B cannot be demonstrated, H5 is dead; it is not "pending more work", and
no fifth condition may be added to keep it alive.

## 7. What G0 authorises, and what it does not

**G0 produces this document and the `STATUS.md` row. Nothing else.** Specifically **not** authorised
at G0: product implementation, UI, token, deploy, wallet, policy engine, a second protocol, a second
workflow, an LLM in the loop, or any measurement.

**The measurement is a separate, founder-authorised step.** Before it runs, §8 must be closed.

## 8. Left open by the ruling, and what must be fixed before any measurement

The founder's ruling fixes the hypothesis, the scope, the four `GO` conditions and the four `KILL`
conditions. It does **not** name the workflow, the failure, the agent, or the entrypoint. Those four
are exactly where a measurement can be tuned after the fact, so **they are pre-registered here as
open, and must be frozen — by a founder ruling, in this document, before anything is run.**

1. **The one capital-action workflow** (`A`). It must be named concretely — protocol, instructions,
   account set, source slot — and frozen. Candidate available offline today: the **Kamino klend**
   fixture set already committed at `fixtures/h4/` (17 cloned mainnet accounts, slot **440,477,781**,
   both real klend binaries and `farms`), whose `refresh_reserve × 2 → refresh_obligation →
   withdraw_obligation_collateral_and_redeem_reserve_collateral_v2` path is **already proven to
   execute** under LiteSVM by H4's completed run. Using it costs no new fetch and no network.
2. **The candidate failure** (`B`). It must be named **before** it is hunted, together with the state
   predicate that detects it. A named candidate that fails to reproduce is `KILL-2` and is an honest
   result; a failure discovered first and named afterwards is not a measurement.
3. **The scripted agent** (`C`). Its decision rule must be written down in full before the run —
   deterministic, no model in the loop — and the "next action" it changes must be a specific
   instruction, not a mood.
4. **The CI entrypoint** (`KILL-4`). One command an agent developer runs in their own pipeline, with
   its exit-code contract. If it cannot be written as a command, `KILL-4` fires at the design gate.

**Both baselines in `B` must also be specified before the run**: what "a single
`simulateTransaction`" means exactly (which transaction, from which state), and what the static
policy is exactly (destination allowlist plus per-transaction cap, with its values). Writing them
after seeing the failure would make `B` unfalsifiable.

**The first concrete proposal for all five is in §8.1. It was independently reviewed and is
`REJECTED at the design gate — KILL-1`.** §8 is therefore still open, nothing has been ratified, and
**nothing runs**. A replacement candidate requires a **new founder ruling and a new G0 row** — it may
not be swapped in by an agent.

**§8.2 — the candidate search that followed.** By founder ruling (2026-08-22) a *design investigation
only* was run to ask whether **any** qualifying candidate exists:
[`docs/H5-CANDIDATES.md`](./H5-CANDIDATES.md). Three candidates were enumerated and **all three died
at the design gate** — `C1` and `C2` to `KILL-1`, `C3` to `KILL-2` — **without implementation or
measurement**. The decisive finding is about the fixture, not about the candidates: the committed
`fixtures/h4/` obligation holds **one collateral against one borrow**, so there is **no second capital
leg** to make ordering matter, and **klend's own clamping reaches the solvency limit in a single
transaction**, so a cumulative walk has nowhere further to go. What remains is a **single capital
action**, whose failure is by construction witnessed inside one transaction's post-state.

That selected **branch A: H5 proposed `KILLED`** — **which the independent review then overturned.**
[`reviews/H5-G0-candidates.md`](../reviews/H5-G0-candidates.md) returned **`DISSENT — a candidate
survives: C3`** with two P0s, CC concurs, and **branch A is withdrawn. H5 is not killed and no kill
commit was made.** `H5-CANDIDATES.md` **§8 governs**; its §6–§7 are superseded.

**§8.3 — the baseline, RULED AND CLOSED (founder, 2026-08-22, before any C3 measurement).**
The definition that stalled H5 is now fixed, and it supersedes every earlier reading in this document,
including the one §8.1's review relied on:

| baseline | definition |
|---|---|
| **`Pol`** | a **static policy** — destination allowlist plus a per-transaction cap |
| **`Sim`** | a **non-persistent single `simulateTransaction`** |

- **Neither may retain, apply or chain post-state between transactions.**
- **An episode executor that carries cloned state across transactions is NOT a baseline — it is the
  mechanism H5 is tested on.**

**Ruled before measurement, and not revisable after seeing a result.** Doing so afterwards would be
the post-hoc criterion change this gate forbids.

**§8.4 — C3 is pre-registered as a new G0 row:** [`docs/H5-C3-G0.md`](./H5-C3-G0.md) — *duplicate
execution after a lost confirmation*. All seven founder-required items are frozen there before any
measurement. **The §8.1 first candidate, `C1` and `C2` are NOT revived** and nothing in that row
depends on them. **No implementation and no measurement are authorised**; the independent-review
payload is committed at [`reviews/H5-C3-G0.codex-prompt.md`](../reviews/H5-C3-G0.codex-prompt.md).

## 8.1 PROPOSED freeze of the five open items — **REJECTED at the design gate**

> ⛔ **This proposal is dead. Do not revive it, do not measure against it, do not adapt it.**
> Independent review — [`reviews/H5-G0-agent-release-tests.md`](../reviews/H5-G0-agent-release-tests.md),
> Codex, read-only against `174fff2` — returned **`KILL AT DESIGN GATE — KILL-1`**, one P0:
>
> > `F` is `P1 && P2`. Both are knowable after a **single** simulation of T2 from post-T1 state if
> > returned account data is available. … The proposed escape — that the simulator "does not know
> > intent" — **is not structural.** The intent is the scripted agent source, available to the same
> > developer invoking simulation. A simulator-side regression assertion can simply state: "after
> > `withdraw(u64::MAX)`, `deposited_amount == 0`". **No third transaction or new oracle is required.**
>
> `S-generous` — which this section deliberately pre-registered as **governing** — catches `F` at T2.
> `KILL-1` is *"H5 turns out to be equivalent to a single simulation"*, and on this candidate it is.
> The reviewer also found T3 to be **theatre** (P1: `F` is fully witnessed by T2's own post-state, so
> the multi-transaction differentiator does no work here), the `Pol` cap **beatable by construction**
> (P1: 1,000,000,000 sits above the *already known* realized 582,271,854, and a real pre-sign policy
> reading the *requested* value would reject the `u64::MAX` sentinel outright), the CI entrypoint a
> **placeholder** (P1), and the proposal missing a pinned BPF hash and the required H4 mutations (P2).
>
> **Cost of this kill: zero implementation.** No code was written, no measurement run, no fetch made.
> That is the design gate working as intended, and it is the cheapest place H5 could have died.
>
> **What this does and does not decide.** It kills **this candidate**, not H5 by fiat: §8.1 was never
> in force, and this section itself pre-registered that **no substitute failure may be swapped in
> after the fact — a replacement candidate requires a new founder ruling and a new G0 row.** That
> constraint binds CC too, and CC has not proposed a replacement. **Whether H5 itself is closed is a
> founder ruling.** The reviewer's P0 is structural, not candidate-specific: it defeats *any*
> candidate whose failure is fully witnessed inside one transaction's post-state. The only door it
> leaves is a failure that **no single transaction's post-state witnesses** — a defect in the
> *sequence*, where each transaction's own end state looks correct. **Nothing here shows such a
> failure exists**, and under this repo's rules not-proven is a KILL, so that door is not a plan and
> must not be treated as one.
>
> **Nothing below is rewritten.** It is kept as the refuted proposal the verdict is read against.

### Provenance of the candidate, disclosed first

The candidate failure below was **not** found by hunting inside H5. It is read off **H4's completed,
committed and independently reviewed measurement** (`evidence/h4-sentinel.json`, verdict
[`H4-sentinel-kill.md`](./decisions/H4-sentinel-kill.md)), where it appears as case E's token deltas.
It is therefore already public in this repo, already reviewed, and **cannot have been tuned to make
H5 pass** — it predates H5's existence. This is stated because a candidate named after seeing data is
normally illegitimate; here the data is a prior phase's frozen artifact, and the direction of the
concern is recorded rather than argued away.

### The number that motivates it

From H4 case E, identical on both binaries, fixture slot **440,477,781**:

| requested `collateral_amount` | `result` | collateral actually moved | USDC actually received |
|---:|---|---:|---:|
| `u64::MAX` (18,446,744,073,709,551,615) | **`Ok`** | **486,657,686** | 582,271,854 |
| `D` = 2,248,785,777 (exactly the deposit) | `Err … Custom(6011)` | 0 | 0 |

Requesting the sentinel "everything" **succeeds and moves 2.2% of the position**; requesting the
literal full deposit **fails**. `2,248,785,777 − 486,657,686 = 1,762,128,091` collateral atoms —
**78% of the position — remain deposited after a call the caller reads as "I exited"**. H4's own run
log corroborates it: `SetStakeDelegated: prev:2248785777 -> new:1762128091`.

**No error is returned. No log line says "partial".** The only witness is the state.

### 1 — The one capital-action workflow `W` (condition A)

**Kamino klend, USDC reserve, on the fixture set already committed at `fixtures/h4/`** (17 cloned
mainnet accounts, slot 440,477,781, real `klend` + `farms` BPF). No new fetch, no network. Three
transactions, in three separate VM slots, state carried forward between them — **this is the only
structural difference from H4, and it is the point**:

| tx | content |
|---|---|
| **T1** | the mandatory refresh preamble: `refresh_reserve(USDC)`, `refresh_reserve(SOL)`, `refresh_obligation` |
| **T2** | `withdraw_obligation_collateral_and_redeem_reserve_collateral_v2`, `collateral_amount = u64::MAX` — the agent's "exit my whole position" call |
| **T3** | whatever the agent's decision rule (below) emits next, given what it believes T2 accomplished |

### 2 — The candidate failure `F` and its state predicate `P` (condition B)

**`F`: the agent believes it exited its position; 1,762,128,091 collateral atoms are still deposited,
and the agent stops.** Every transaction in the episode returns `Ok`.

`P` is evaluated on **state bytes only** — never a log line, per §3.4:

```
P1  obligation.deposits[USDC].deposited_amount  != 0  at end of episode
P2  agent terminated in state EXITED (its own rule believed the position closed)
P3  realized collateral delta (486,657,686) != the delta the agent assumed (2,248,785,777)
F fires iff P1 AND P2.   P3 is recorded with its direction, both ways.
```

**If `F` does not reproduce, that is `KILL-2` and it is an honest result.** No substitute failure may
be swapped in after the fact; a replacement candidate requires a new founder ruling and a new G0 row.

### 3 — The scripted agent (condition C)

Deterministic, **no LLM in the loop**, rule written here in full **before** any run:

```
s0  read obligation.deposits[USDC].deposited_amount -> believed_start
T1  emit refresh preamble
T2  emit withdraw(collateral_amount = u64::MAX)          # "exit everything"
    on result Ok  -> believed_remaining := 0              # <-- the defect under test
    on result Err -> believed_remaining := believed_start
T3  if believed_remaining == 0 -> emit EXIT-COMPLETE action, terminate in state EXITED
    else                       -> emit withdraw(believed_remaining), loop
```

**The action that changes is a specific instruction, not a mood:** with true state the agent emits a
second `withdraw` at T3; with the false belief it emits the exit action and stops. The two
instruction sequences are compared byte-for-byte and both are recorded.

### 4 — Both baselines in `B`, defined before the run

`B` is unfalsifiable unless the baselines are pinned now, so **both variants are pre-registered, and
the generous one governs**:

| baseline | definition | verdict if it also catches `F` |
|---|---|---|
| **S-strict** | a single `simulateTransaction` of **T2 alone**, from the post-T1 state, reading only the transaction-level `err` field | — |
| **S-generous** | the same simulation, but **also** allowed to read the post-state accounts it returns and diff them | **`KILL-1` fires** |
| **Pol** | destination allowlist `{agent's USDC ATA}` + per-transaction cap **1,000,000,000 USDC atoms** (realized 582,271,854 is under it; the cap is fixed here and not adjusted later) | **`KILL-1` fires** |

**Pre-registering `S-generous` is deliberate and cuts against H5.** The strict variant is a strawman:
of course an `err`-only check misses a transaction that succeeds. The honest question is whether a
developer who simulates one transaction *and inspects the returned state* still misses `F`. **CC's
assessment is that this is where H5 most likely dies**, and the gate says so in §9 rather than
discovering it later.

The surviving claim, if any, is narrow and must be stated narrowly: a single simulation returns *a
state*; it does not know **what the agent intended across transactions**, so it cannot tell that
`deposited_amount = 1,762,128,091` contradicts an intent to exit. **The oracle comes from the agent's
declared intent, not from the chain.** If, once run, `S-generous` plus any fixed rule catches `F`
without that intent, `KILL-1` fires and H5 is dead.

### 5 — The CI entrypoint (`KILL-4`)

Contract only — **no code is authorised at G0**:

```
<one command> --episode <episode.json>
  exit 0        every pre-registered invariant held
  exit non-zero one named invariant failed; a reproducible trace is written
  stdout        the failing invariant, the accounts and offsets that witness it,
                every input, mutation and hash (condition D)
  offline       once fixtures are committed; no network
```

It must be runnable in an agent developer's own pipeline against their own agent's decision rule. **If
it cannot be written as a command with an exit-code contract, `KILL-4` fires at the design gate**,
before anything is built.

### Why this is not H3's or H4's grave (`KILL-3`)

`P` reads **one named protocol's own field at a known offset**, for **one** workflow. H5 claims **no**
generality across protocols and defines **no** adapter: a second protocol would need its own
pre-registration, and saying so is the whole point. H4 died because, with only two binaries and no
declared intent, **no ground truth existed** to say whether a state difference mattered. H5 supplies
that ground truth from **the agent's own stated intent** — which is available precisely because H5
tests an agent, not a protocol. **If deciding `F` ever needs a rule that generalises beyond `W`, that
is `KILL-3` and not a scope change.**

### What the founder is being asked to rule on

Ratify, amend, or reject: `W`, `F`+`P`, the agent rule, the two baselines (**especially whether
`S-generous` governs**), and the entrypoint contract. **Until then §8 remains open and nothing runs.**

## 9. Predictions, recorded before measurement

Wrong predictions are **not** a kill; refusing to record them would be.

| condition | prediction | confidence |
|---|---|---|
| A — reproducible multi-transaction episode | **holds** — H4 already produced a deterministic offline replay on this fixture set | medium–high |
| B — a failure both baselines miss | **RESOLVED AGAINST H5 for the §8.1 candidate, before any measurement** — the review found `S-generous` catches `F` at T2. The prediction below is what CC recorded beforehand, and it was right. | — |
| ~~B (as predicted 2026-08-22, kept on the record)~~ | **open, and now sharper.** Against `S-strict` and `Pol` (§8.1) CC expects `F` to survive. Against **`S-generous`** — one simulation *plus* inspection of the state it returns — CC expects `KILL-1`, unless the agent's declared intent is admitted as the oracle. **This single choice is where H5 most likely dies**, which is why §8.1 pre-registers `S-generous` as governing | low |
| C — the failure changes the agent's next action | **holds if B holds**, since a scripted agent reading state must branch on it | medium |
| D — assets usable as fixtures, fully recorded | **holds** — the recording discipline already exists | high |
| **overall** | **open, leaning `KILL-1`** — and the candidate failure `F` itself is *not* a prediction: it is already observed in H4's frozen evidence. What is unpredicted is whether any baseline also catches it | low |

## 10. On reaching the gate

`GO` ends this phase; it does not start the next one. On any outcome: update `STATUS.md`, commit the
evidence and the decision record, push, and **hand back to the founder**.
