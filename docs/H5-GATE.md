# H5 — Agent Release Tests for Solana — kill gate and G0 pre-registration (binding)

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

## 9. Predictions, recorded before measurement

Wrong predictions are **not** a kill; refusing to record them would be.

| condition | prediction | confidence |
|---|---|---|
| A — reproducible multi-transaction episode | **holds** — H4 already produced a deterministic offline replay on this fixture set | medium–high |
| B — a failure both baselines miss | **open.** This is the hypothesis and the most likely place H5 dies, most plausibly to `KILL-1` | — |
| C — the failure changes the agent's next action | **holds if B holds**, since a scripted agent reading state must branch on it | medium |
| D — assets usable as fixtures, fully recorded | **holds** — the recording discipline already exists | high |
| **overall** | **open, leaning `KILL-1`** | low |

## 10. On reaching the gate

`GO` ends this phase; it does not start the next one. On any outcome: update `STATUS.md`, commit the
evidence and the decision record, push, and **hand back to the founder**.
