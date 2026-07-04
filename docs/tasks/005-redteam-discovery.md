# Task 005 — Red-team discovery loop (the coverage moat)

**Owner:** CC (frame-thin: invariant-set completeness / verifier soundness).
**Reviewer:** Codex (adversarial — try to beat the promoted invariant).
**Branch:** `task/005-redteam-discovery`.
**Depends on:** Task 004 merged.
**Motivation:** external critique (2026-07-04) point ⑤ — the hand-authored `measurement_gamer` /
`phantom_hider` policies prove the *mechanism* but not *coverage* of unknown shortcuts. This task shows
the invariant set is incomplete and *mechanically extends it* — the [[solinv]] invariant-fuzzing DNA.

## Design

- **Search space.** A parametric `ParamAttack { open_slot, close_slot, size, side }` policy that
  **claims delta = 0 (neutral)** while actually holding a directional position between `open_slot` and
  `close_slot`. Adequately collateralized (survives the shock) so it is not caught by insolvency.
- **Escape = a shortcut the baseline set misses.** Baseline verdict is `Pass` AND the agent breached its
  own neutral claim (`|measured_delta(h)| > tol` for some `h`) — i.e. it ran hidden directional risk
  while claiming neutral, yet passed.
- **The known gap.** Baseline `ContinuousNeutrality` only inspects the `[N-W, N-1]` window (Task 001 P2).
  An attacker that flattens *before* the window (e.g. `close_slot = 50`) holds risk through the
  shock@30 yet leaves the window clean ⇒ **baseline PASSES**.
- **Promotion.** Add a claim-aware invariant `ClaimedNeutralityHeld`: *if the agent claims neutral
  (`|claimed_delta| ≤ tol`), then `|measured_delta(h)| ≤ tol` must hold for ALL h.* Subsumes the window
  `ContinuousNeutrality` for claimed-neutral agents; does **not** apply to honest directional traders
  (honest claims delta=10), so no false positive.
- **Invariant-set-driven verifier.** `verify` runs the PROMOTED set (includes `ClaimedNeutralityHeld`);
  `verify_baseline` runs the pre-promotion set (excludes it). The loop shows: escape ⇒ `verify_baseline`
  = Pass, `verify` = ShortcutDetected(`ClaimedNeutralityHeld`); honest ⇒ Pass under both.
- **Backend.** Discovery sweeps on the fast deterministic `ref` backend (ref==svm parity already
  proven). No SBF build in the loop.

## Public / private boundary (load-bearing)

The public repo ships **one** demonstrator: `ParamAttack` + a deterministic `discover()` that finds the
one window-gap escape + the single `ClaimedNeutralityHeld` promotion. The **exhaustive** multi-dimensional
search, the full invariant catalog, and any mainnet corpus stay in **private [[solinv]]**. Do not commit
catalog breadth here — this is the wedge, not the moat's interior.

## Scope (in)

- `crates/harness/src/policy.rs`: `ParamAttack` policy (claims neutral, holds a directional position).
- `crates/harness/src/verifier.rs`: `FindingKind::ClaimedNeutralityHeld`; `InvariantSet {Baseline,
  Promoted}`; `verify_with(policy, trace, claim, set)`; `verify` = Promoted, `verify_baseline` = Baseline.
  Adding to Promoted must NOT change any existing verdict (honest Pass; gamer/phantom still FLAG).
- `crates/harness/src/redteam.rs`: deterministic `discover() -> Vec<Escape>` sweeping a fixed grid of
  `close_slot`; `demonstrate()` picks the first escape and shows baseline-Pass → promoted-FLAG → honest
  still Pass.
- `main.rs`: a `redteam` subcommand printing the discovered gap + the promotion contrast.

## Acceptance criteria

- `discover()` is deterministic (same grid ⇒ same escapes) and finds ≥1 escape (the pre-window flatten).
- The escape: `verify_baseline` = `Pass`; `verify` (promoted) = `ShortcutDetected` with
  `ClaimedNeutralityHeld` and correct evidence slots.
- Honest passes under BOTH sets; `measurement_gamer` / `phantom_hider` still FLAG under both.
- `cargo test --offline` green (25 prior + new). No warnings.

## Out of scope

- Exhaustive / multi-dimensional search, LLM red-teamer, full catalog (private solinv).
- Hostile-episode realism (slippage / oracle-lag / adversarial ordering) — separate roadmap task.
