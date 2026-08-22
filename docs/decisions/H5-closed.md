# H5 — Agent Release Tests for Solana — **CLOSED**

> **`CLOSED — all authorised candidates failed at the design gate; H5 remains unproven.`**

**Date:** 2026-08-22 · **Verdict by:** founder ruling
**Produced by:** Claude (spec/evidence role) · **Independently reviewed by:** Codex — three rounds,
each read-only, each on material Codex did not write:
[§8.1 candidate](../../reviews/H5-G0-agent-release-tests.md) `KILL AT DESIGN GATE — KILL-1` ·
[candidate search](../../reviews/H5-G0-candidates.md) `DISSENT — a candidate survives: C3` ·
[C3 G0](../../reviews/H5-C3-G0.md) `KILL AT DESIGN GATE`
**Artifacts:** `891a321` · `174fff2` · `f0f7ca5` · `d8eb10a` · `bb97f2d` · `cf34764` · `6fdcf1d`
**Gate:** [`docs/H5-GATE.md`](../H5-GATE.md) · **candidates:** [`docs/H5-CANDIDATES.md`](../H5-CANDIDATES.md)
· **C3's row:** [`docs/H5-C3-G0.md`](../H5-C3-G0.md)

> **H5's hypothesis ([`docs/H5-GATE.md`](../H5-GATE.md) §1):** a Solana AI agent that moves capital in
> production should be regression-tested before release against adversarial scenarios spanning real
> program BPF, real cloned state and **multiple transactions** — and such a test **finds failures a
> single `simulateTransaction` or a runtime wallet policy cannot find**.

**Only the second clause was ever on trial**, and it was never measured. **No H5 measurement was run.
No H5 implementation exists.** Every verdict below was reached at a **design gate**, before code.

---

## 1. What this record does and does not say

**Says:** every candidate H5 was authorised to pursue failed *before* measurement, and H5 therefore
never produced the reproducible experiment its own rules demand. **Not-proven is a KILL**, so the
phase closes.

**Does NOT say — and may not be cited as saying:**

- **NOT** that sequence-only failures do not exist on Solana. **That was never tested and is not
  claimed.** H5 is closed as **unproven**, not as **refuted**, and the official designation says so.
- **NOT** that pre-release agent testing is worthless. What failed is **this** formulation, against
  **these** baselines, on **the assets G0 was given**.
- **NOT** that any earlier hypothesis is vindicated. H1 is `KILLED`, H2 `FROZEN UNEXECUTED`, H3
  `KILLED`, H4 `KILLED`. Nothing here reopens any of them.

## 2. The four authorised candidates, and how each died

| candidate | claim | verdict, at the design gate |
|---|---|---|
| **§8.1** — `withdraw(u64::MAX)` balance mismatch | the agent believes it exited while 78% of the position remains | **`KILL-1`** — the failure is fully witnessed in one transaction's post-state; the "simulator does not know intent" escape was found *not structural*, since the intent is the agent's own source, in front of the same developer |
| **C1** — the failing transaction's bytes derived from earlier results | a single simulation cannot even *construct* the transaction | **not measured.** Its `KILL-1` was found circular, then it lapsed unpursued when the ruling authorised C3 alone |
| **C2** — a plan invalidated by slot advance and accrued interest | time-of-check / time-of-use | **not measured.** Undecided, then likewise lapsed |
| **C3** — duplicate execution after a lost confirmation | every transaction is individually admissible; only the **cumulative** diff is wrong | **`KILL AT DESIGN GATE`**, four P0s — §3 below |

**None of the four may be revived, repaired, or re-measured.** C1 and C2 are recorded as *lapsed*
rather than refuted, because that is what actually happened; **being unrefuted does not make either a
live candidate**, and neither may be picked up without §5.

## 3. Why C3 died — the P0s, in full

C3 was the strongest candidate H5 produced. It was pre-registered completely
([`docs/H5-C3-G0.md`](../H5-C3-G0.md)) with all seven founder-required freeze items, and reviewed once
under a founder directive to attack **in the kill direction first**.

**P0-1 — the three "meaningful" transactions were 件数合わせ.**

> *"No concrete operational reason justifies splitting a 200,000,000-atom withdrawal into two
> 100,000,000-atom calls when the cited fixture admits a single 486,657,686-atom call."*

C3's episode needed **three** capital actions. It got them by splitting one withdrawal into two
tranches so that one erroneous retry would make three. §2 of that row called chunking *"ordinary agent
behaviour"* — **an assertion, not a constraint**: no size limit, liquidity condition, timing rule or
risk policy required it. **CC had itself disclosed that the split was what made three actions
available, and could name no operational driver.** The design-time kill C3 pre-registered in its own
§9 therefore fired on C3.

**No repair existed on the committed fixture.** Removing the artificial split leaves *one* planned
action plus *one* duplicate — **two**, below the required three — and every route to a genuine third
planned action was closed: there is **no second collateral** to withdraw; the agent **holds no SOL**
with which to repay the single borrow, so a repay leg would mean **fabricating a balance**; and
"withdraw the maximum, iterating" is **not iterative here**, because H4 measured that `u64::MAX`
reaches the maximum **in one call**.

**P0-2 — the comparator could not carry the claim.** See §4; it is the deeper reason and has its own
section.

**P0-3 — material execution choices remained post-run freedoms.** The slot schedule was only
*"recorded by the run"* rather than pinned; further mutations were permitted if merely disclosed; and
no input contract existed for the developer's agent rule. **Slot progression can change lending state
and the retry's result**, so it must be fixed in advance, not observed afterwards.

**P0-4 — the CI entrypoint was not concrete.** It accepted only an episode JSON, not a rule artifact
or invocation contract, so it did not meet the gate's requirement that a developer run it **against
their own agent's rule** — a live `KILL-4` risk.

**Three P1s, all conceded:** sequential headroom was still unproven (H4's case A and case E are single
calls **from the initial fixture** and do not establish that the post-`W1`/post-`W2` solvency envelopes
admit another withdrawal — **the same path-dependence gap flagged a round earlier, repeated**); the
agent rule was **undefined off the intended path**; and `P3` **contradicted CC's own "state-bytes-only"
discipline**, since "returned `Ok`" is execution metadata, not state bytes.

## 4. The structural finding — C3 could not be differentiated from a runtime wallet policy

This is the reason H5 closes rather than merely losing a candidate, and it is stated plainly because
it is what a future attempt must answer.

The founder fixed the baselines before measurement:

- **`Pol`** — a **static policy**: destination allowlist plus a per-transaction cap.
- **`Sim`** — a **non-persistent** single `simulateTransaction`; neither baseline may retain, apply or
  chain post-state between transactions, and an executor that carries cloned state across
  transactions **is not a baseline — it is the mechanism H5 is tested on**.

Under that ruling C3's episode does evade both. **But H5's hypothesis names its competitor as *"a
runtime wallet policy"*, not as *"a policy without memory"*** — and a real one has memory. The review:

> *"Such controls are reasonable for capital-policy products and catch a duplicate ordinary
> withdrawal. The stipulated narrow `Pol` will miss, but that does not establish H5 condition B
> against 'a runtime wallet policy'. A GO would be unearned."*

**And this is not a defect of C3's construction — it is intrinsic to C3's failure class.** C3's failure
**is** a duplicate spend. A cumulative spend cap set at the agent's **own declared intent total**
catches a duplicate spend **by definition**. For C3 to have cleared condition B against a fair runtime
policy, it would have had to be *a duplicate that a cumulative cap misses* — **which is close to a
contradiction in terms.**

So C3 could have been given a passing baseline **only by keeping the comparator weaker than the one
the hypothesis names.** A `GO` obtained that way would have been unearned, and the honest reading is
that **C3 was not differentiable from a runtime wallet policy at all.**

## 5. Why `CLOSED`, and what reopening requires

**`CLOSED — all authorised candidates failed at the design gate; H5 remains unproven.`**

Four candidates, four design-gate failures, **zero measurements**, **zero lines of product code**.
Under this repo's standing rules — *a verdict is a number or a reproducible experiment*, and
*not-proven is a KILL* — a hypothesis that cannot reach a measurement is finished. **Adding a fifth
candidate to stay alive is forbidden**; that is 延命, not scope.

**Reopening requires all three, and no agent may waive or soften any of them:**

1. **A new founder ruling.** No agent may reopen H5, in whole or in part.
2. **A new, independent hypothesis** — not a repair of H5 and not a rewording of it. In particular it
   may not rest on §8.1, C1, C2 or C3, all of which are finished.
3. **A new pre-registration**, committed before any measurement, which must answer §4 head-on: it must
   name the comparator **the hypothesis itself names**, at full strength, and show the candidate
   failure survives it. **Choosing a weaker comparator is what made C3 unwinnable-yet-unfalsifiable,
   and it is the specific mistake a successor must not repeat.**

**Not authorised by anything, and not to be started:** acquiring a new fixture, searching for a new
candidate, implementation, measurement, UI, token, deploy.

## 6. Carried forward as input, not as a live hypothesis

- **The cheapest kill is the design gate.** All four candidates died before implementation; H5 cost
  **no product code and no measurement**. Three independent review rounds did the work, and one of
  them (`DISSENT`) **overturned CC's own proposal to kill early** — the loop caught an error in both
  directions, which is the point of it.
- **A comparator chosen for winnability is worse than a losing comparison.** It yields a result that
  cannot be defended, which is a slower and more expensive failure than an honest kill.
- **The fixture bounds the hypothesis.** `fixtures/h4/` holds one collateral against one borrow, which
  meant a single capital action was all it could express. **A hypothesis about sequences needs a
  fixture that can express one**, and that must be established **before** the hypothesis is committed
  to, not discovered at its fourth candidate.
- The mechanics — real BPF, cloned mainnet state, disclosed mutations, deterministic offline replay,
  no network — remain sound and reusable **as fixtures**. **Reusing mechanics is not reusing a
  hypothesis**, and nothing from H1–H5 may be revived as a foundation on the grounds that it exists.
