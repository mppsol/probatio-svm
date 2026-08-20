# H2 — Discriminating probe (pre-registered before running)

**Purpose:** decide whether the precise G0 is worth writing at all, before writing it. G0 is three
review rounds and 331 lines deep with no measurement taken; `docs/GATE.md` names that exact failure.
This probe is the cheap test that stops it.

**This is not G1.** It produces no gate verdict for H2's criteria and no evidence for a `GO`.

## The one-directional guard — why this probe cannot be argued past

> **The probe's result may only ever KILL. It can never pass, and it can never widen scope.**

- If the probe's estimate of `L_total` is **below $50M per 90 days by a factor of 10 or more**, H2 is
  **KILLED**. A crude estimate built from direct on-chain reads cannot be wrong by a full order of
  magnitude, so a KILL at that distance is safe.
- If it is **within 10× of the threshold in either direction**, the probe returns **INCONCLUSIVE** and
  the only permitted next step is to write the precise G0 and run the real G1. The probe never reports
  a pass, so a favourable probe buys H2 nothing except the right to do the expensive work.

This asymmetry is what makes a rough measurement legitimate here. The thresholds it is compared against
were frozen in commit `e105b0f`, **before any measurement existed**, so no probe result can move them.

## What is measured

Current-state, order-of-magnitude only, over the venues already listed in
[`docs/H2-preregistration.md`](./H2-preregistration.md):

1. **Realised bad debt stock (L-A proxy)** — across the lending venues, the aggregate shortfall of
   positions whose debt exceeds their collateral, in the venues' own USD-denominated fields.
2. **Loss-bearing capital** — the depositor/LP capital that stands behind that loss, for scale.
3. **Backstop capacity** — insurance-fund/reserve balances, since loss they absorb is `covered` and
   therefore not addressable.

Schemas are taken from each program's **on-chain Anchor IDL**, not from inferred offsets — the H1
lesson, and the same artifact a precise G0 would need later.

## Stated in advance: what this probe cannot tell us

- It is a **stock**, not the 90-day **flow** G1 requires. A stock can understate a flow that was
  realised and written off within the window, or overstate one that has sat unresolved for years.
- It does not separate `covered` from `uncovered`, or `controlled` from `not-controlled`.
- It does not resolve the L-B gross-versus-net question (winning traders' gross profit versus the
  net reduction in LP value) that Codex flagged as unclosed in G0 r2.

Because of these, **only the 10× KILL is honest.** Any result closer than that is INCONCLUSIVE by
construction, not by judgement.

## Reporting

Every number carries its slot. Every venue that cannot be decoded is reported as undecodable, with the
reason — never silently dropped, since an omission lowers the estimate and would push toward a KILL.
