# Review 012 — Felt-moment hero

**Reviewer:** CC. **Branch:** `task/012-felt-moment-hero` @ `2b2c0c7`.

## APPROVE

The hero stages the scripted-drift certification as the intended felt moment — promise (mandate +
claimed delta) → reveal (measured-vs-claimed over 60 slots) → catch (FLAG + exact evidence slots) →
one honest ground-truth line — and the six-card gallery is preserved below. Scope is clean: only
`web/build.js` + generated `web/index.html` changed (plus this task's brief); no `crates/`,
`programs/`, or `gallery/*.json` data touched.

### Honesty constraints (the review gate) — all hold, verified against the data

1. **Measured line keyed off `measured_delta !== claimed_delta` per slot, not findings.** `heroChart`
   (`build.js:191`) and the card `chart` (`build.js:161`) both derive colour solely from
   `measured_delta` vs `claim`; neither reads `evidence_slots`. Confirmed by replaying every
   transcript: `core-phantom` is FLAG with **0/60 diverged slots** → its measured line stays
   on-claim/green, and the catch is carried by the FLAG badge + findings. `core-gamer` = 59/60,
   `jupiter/scripted-drift` = 60/60, Pass cases = 0/60. Colour matches truth everywhere.
2. **Non-delta findings get a distinct treatment.** `nonDeltaKinds` (`build.js:149`) tags
   `PhantomExposure` ("hidden in another account") and `IntraEpisodeInsolvency` ("went underwater
   after the price shock") as account-state findings in the card list, and the gallery legend now
   explicitly says account-state findings "may not appear on the net-delta line." This is a real
   improvement over the 011 legend.
3. **No overclaiming copy.** Ground-truth line = "computed offline by reading account state as ground
   truth — nothing to bypass." No "agents can't cheat", no realtime-monitor, no exploit/demand claims.
   Footer framing unchanged.
4. **On-screen numbers match the transcript.** claimed_delta 0, verdict ShortcutDetected (shown as
   FLAG + "Verdict: ShortcutDetected"), evidence `slots 1–60` (ClaimTracksExposure) + `slot 60`
   (ClaimMismatch), headline "net long 10" (= final `measured_delta`). All verified.

### Gates

- `node web/build.js` deterministic: identical SHA-256 across two runs
  (`f07524f3…c6b2371`); committed `web/index.html` == fresh build (empty `git diff`).
- Static/self-contained preserved: no fetch/server/external assets; inline CSS/JS only.
- Responsive: hero collapses to one column at ≤720px, evidence to one column at ≤440px; no horizontal
  page scroll introduced.
- No Rust changed (diff is `web/` + brief only) → `cargo test` surface is identical to the green
  master `a7e4b55`; Codex confirmed `--offline` passes. Not re-run (the diff proves it unaffected).

### Non-blocking follow-ups (P2 — do NOT gate the merge)

- **P2 — `build.js:212,232,237`: hero strings are hardcoded to this transcript.** The aria-label
  ("Measured net delta is 10 from slots 1 through 60; claimed delta is 0."), the "net long" wording,
  and the "FLAG" verdict word are literals, currently accurate for the fixed scripted-drift hero. They
  would go stale/wrong if `HERO` is ever swapped to another (or a net-short) transcript. Cheap to
  derive from `t` — worth doing if the hero is ever made switchable, not needed now.
- **P2 — `web/preview.png` regeneration (environmental blocker, not a code defect).** Codex's in-app
  browser forbids `file://` navigation, so the committed screenshot is still the 011 dashboard. Nothing
  references `preview.png` (not the page, not README), so this is out-of-band from the deployed
  artifact. Note it is also both tracked *and* in `.gitignore:20` — recommend `git rm --cached
  web/preview.png` (or force-regenerate the pitch still separately) as cleanup. Handle outside this
  merge.

Merges to `master`.
