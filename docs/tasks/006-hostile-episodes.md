# Task 006 — Hostile episodes (robustness audit of the verifier)

**Owner:** CC (frame-thin: verifier robustness / soundness).
**Reviewer:** Codex (adversarial — hunt for a hostility-induced escape or false positive).
**Branch:** `task/006-hostile-episodes`.
**Depends on:** Task 005 merged.
**Motivation:** external critique (2026-07-04) point ③ — the deterministic single-shock episode is too
clean to certify robustness. This task injects realistic hostility (slippage, lagged multi-shock oracle,
deterministic noise) and turns "clean episode" from a limitation into a *tested* surface.

## The key insight (drives the whole task)

`measured_delta` is the position **size** — it does not depend on price. Policies act on slots, not on
the mark, so the size timeline is identical no matter how hostile the price path is. Therefore the
**misrepresentation invariants (delta-based: `ClaimTracksExposure`, `ContinuousNeutrality`,
`PhantomExposure`) are price-noise invariant** — hostility cannot mask or fabricate them. The only
value-based invariant, `IntraEpisodeInsolvency`, IS price-sensitive: it is **stress-relative**.

The task makes both facts explicit and tested.

## Scope (in)

- `crates/harness/src/hostile.rs` (new): `HostileParams { slippage, scenario, noise_amp }` +
  `MarkScenario { Clean, LaggedMultiShock }` + deterministic `mark_at_hostile(slot, &params)` (a staged,
  lagged multi-step drop with a partial recovery and a second drop, plus a deterministic bounded
  per-slot wiggle — NO RNG). `HostileParams::clean()` reproduces the current episode exactly.
- Refactor `apply` → `apply_with(world, action, slippage)` so fills cross the spread adversarially
  (`fill = mark + sign(delta)*slippage`); `apply` = `apply_with(.., 0)`. Add
  `run_episode_ref_hostile(policy, &HostileParams)` (ref backend only — this is an off-chain verifier
  question).
- Prove **stress-invariance of misrepresentation**: for `measurement_gamer` and `phantom_hider`, the
  `measured_delta` sequence AND the delta-based findings (`evidence_slots`) are byte-identical between
  the clean and a hostile episode.
- Demonstrate **stress-relativity of solvency**: a `StressBoundary` policy (honest directional, claims
  its true delta, collateral sized to survive the mild clean shock but NOT the deeper hostile path) →
  `Pass` under clean, `IntraEpisodeInsolvency` under hostile. This is CORRECT (a stress test), and shows
  why the episode must declare its stress.
- Verdict stability: `honest` = Pass, `measurement_gamer`/`phantom_hider` = FLAG under hostility.
- `main.rs`: extend/print a `hostile` view (or fold into the `redteam` output).

## Acceptance criteria

- `HostileParams::clean()` yields a trace byte-identical to `run_episode` (no regression; existing 30
  tests untouched).
- Misrepresentation findings for gamer/phantom are identical clean vs hostile (a test asserts equality).
- `StressBoundary`: `Pass` clean, `IntraEpisodeInsolvency` hostile (a test asserts both).
- honest Pass / gamer / phantom FLAG under hostile (a test asserts).
- Deterministic: same params ⇒ byte-identical hostile trace.
- `cargo test --offline` green (30 prior + new), no warnings.

## Out of scope

- SVM backend under hostility (the robustness question is off-chain; ref backend only).
- LLM agent (Task 007), CPI guard promotion, pitch video.

## Honest framing to record

Hostile episodes HARDEN the claim rather than fix a bug: they prove the misrepresentation moat is
price-noise invariant (a real strength, because it is position-based), and make solvency's
stress-relativity explicit. If Codev finds a hostility-induced escape, that becomes the next promotion.
