# Review 016 — Dogfood: Crucible momentum bot

**Reviewer:** CC. **Branch:** `task/016-dogfood-crucible-momentum` @ `26b98fe`.
**Method:** diffed the port against the Crucible source; ran the suite; checked the transcript's delta
timeline; screenshotted the new card at 1200px.

## CHANGES (one P1 — a degenerate demonstration; the port itself is perfect)

**The port is faithful — confirmed, and nicely done.** I diffed `CrucibleMomentum` (policy.rs) against
`crucible/packages/bot-simple/src/index.ts`. It matches the rule exactly, including the subtle
push-then-MA ordering: Crucible pushes the current price (index.ts:158) *before* computing `getMA()`
over the last 5 (161), then tests `price > ma` (173). The Rust port pushes `obs.mark`, sums the last 5
**including** it, and tests `obs.mark*5 > sum` — the integer cross-multiply preserves the strict float
`>` exactly. MA window 5, flip = close-then-open, fixed qty, `<5` samples = flat: all faithful. Honest
non-strawman framing on the card ("directional by design… tests a hypothetical neutral claim, not the
bot's own claim") is exactly right. Gates all green: 58 harness tests pass; `gallery --core` and
`node web/build.js` deterministic; committed == fresh build.

**P1 — on the clean episode the "momentum" bot shows no momentum.** The standard episode price path is
flat (100) then a single crash (→40) — it never rises. So the momentum rule (`price > MA`) resolves to
**short for the entire run**: `distinct measured_delta = {0, -10}`, no long leg. The card literally
titled *"Momentum bot"* renders a **flat short line** (betting-down, slots 5–60) — no direction change,
and it's the mirror image of the existing flat "Quiet long" card. For a non-expert judge this is
confusing ("where's the momentum?") and redundant, which is exactly the "not half-baked / intuitive"
bar we're holding. The faithful port is correct; the *episode* makes the demonstration degenerate.

**Fix — drive this transcript on the hostile (multi-shock) episode, not the clean one.**
`hostile.rs::lagged_multi_shock` already does 100 (flat) → drop → **partial recovery (up)** → second
drop. A momentum bot on that path genuinely flips: short on the drops, **long during the recovery** —
a dynamic, on-label chart that actually shows momentum, and is visually distinct from Quiet long.
Concretely:
- Emit `core-crucible-momentum.json` by running `CrucibleMomentum` through the hostile path
  (`run_episode_ref_hostile` / the `MarkParams::hostile()` scenario already in the tree), keeping the
  same faithful policy, the neutral claim, and the due-diligence framing.
- **Verify the transcript contains BOTH a long (+) and a short (−) `measured_delta` segment** (add it to
  the policy's unit test). If for some reason the recovery doesn't cross the MA, add a minimal
  deterministic oscillating path rather than shipping a monotone line — the acceptance is "the recorded
  line visibly flips long↔short at least once."
- Keep it deterministic; adjust the card copy to note it's a choppy/multi-shock market if helpful
  (honest either way). Still flagged against the neutral claim (`ClaimTracksExposure` across the
  directional legs).

Everything else is APPROVE-ready. Re-request review after the episode swap (Round 2): I'll confirm the
chart shows real long↔short momentum, re-diff nothing (port unchanged), and re-run the gates.

## Round 2 — APPROVE (`69c5c33`)

Fixed by driving the transcript on the hostile multi-shock path; the `CrucibleMomentum` decision rule
is byte-for-byte unchanged (verified via diff 26b98fe..69c5c33 — only the unit test and the gallery/main
wiring changed). The transcript now carries both long (+10) and short (−10) legs (26 direction flips),
verdict ShortcutDetected, `ClaimTracksExposure [5-60]`. Screenshotted the card: the recorded line
whipsaws up/down across the flat neutral claim with hazard bands on both sides — a momentum bot
visibly never-neutral, on-label and distinct from the flat "Quiet long". Honest: a naive momentum rule
overtrades on a noisy oracle, which is exactly what's shown. Gates: 56 harness tests pass; `gallery
--core` + `node web/build.js` deterministic; committed == fresh build. Faithfulness + honest framing
from Round 1 stand. Merges to `master`.
