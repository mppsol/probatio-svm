# Review 013 — Judge-first, pick-an-agent experience

**Reviewer:** CC. **Branch:** `task/013-judge-first-interactive` @ `44731a7`.
**Method:** rendered with Puppeteer at 1200px and 390px, and drove the roster interaction to inspect the
default (Measurement gamer), a Pass case (Steady holder), the phantom case (Hidden-wallet trader), and
Jupiter drifter. Screenshots reviewed for appeal + honesty, not just code.

## CHANGES (one P1 + one bundled P2 — everything else is APPROVE-ready)

The rebuild genuinely clears the appeal bar 012 missed: plain-language hook ("Would you trust an AI
agent with a fund?"), an interactive candidate roster with jargon-free one-liners, plain-English axis
(BETTING UP / FLAT / BETTING DOWN), the hazard band that makes the promise↔reality gap the hero, and a
grounded consequence ("price fell 60%… went underwater… surfaced before the loss, not after"). Honesty
verified below. Two fixes stand between this and merge — both on the **Pass path**, which a judge WILL
click:

### P1 — Pass-case reveal headline is broken copy. `build.js` `stageMarkup` (line ~191) + its JS twin (~246)

For a candidate whose record matches its promise (Steady holder, Balanced Jupiter trader), the reveal
headline renders literally: **"The gap appears in no recorded gap."** — verified on screen. It is
ungrammatical AND misleading (asserts a gap on an honest agent). Root cause: the headline is
`phantom ? "…" : "The gap appears in " + gap + "."`, and for a matched record `gap =
formatRanges(divergenceRanges(t))` returns the string `"no recorded gap"`.

Fix: make the headline a three-way, keyed off divergence (not verdict), matching how phantom is already
special-cased:
- phantom → "The line is flat. The risk is not." (unchanged)
- else if `divergenceRanges(t).length === 0` → a matched-line headline, e.g. **"The record matches the
  promise."**
- else → "The gap appears in {gap}." (unchanged)

Apply in BOTH the Node `stageMarkup` and the inlined browser `stageMarkup` (they must stay identical).

### P2 (fix together) — reveal headline stays amber on a Pass. `build.js` CSS `.reveal-act h3` (line ~211)

`.reveal-act h3 { color:#ffd2a4; }` paints the reveal headline amber (danger signal) for every
candidate, including the matched/Pass ones — so "The record matches the promise." would read in an
alarm colour. When the record matches (the new matched branch above), render the headline in the safe
colour (`var(--safe)`), e.g. via a modifier class on the `<h3>`. Deception cases stay amber.

## Honesty audit — PASS (verified in the new code AND on screen)

The generator was substantially rewritten (465 lines), so I re-audited rather than assuming carry-over:

1. **Chart geometry keyed off `measured_delta !== claimed_delta` per slot, never findings.** `chartMarkup`
   (line ~93) computes `isDiverged`, and bands (~96), line segments (~111), and dots (~123) all derive
   from it. Comment at line 95 states the invariant. Confirmed on screen: **Hidden-wallet trader renders a
   flat green line with NO hazard band** (0/60 diverged) — the review-011 failure mode does not recur.
2. **Non-delta findings get distinct treatment.** Phantom → "risk found in another wallet" key +
   "Caught — risk was hidden in another wallet"; insolvency → the consequence box. Not faked onto the
   delta line.
3. **No overclaiming copy.** Anticipatory/honest; "computed offline by reading account state as ground
   truth — nothing to bypass" retained in the CTA. No "can't cheat" / realtime / exploit / demand claims.
4. **Numbers grounded.** `consequenceMarkup` derives `before`/`after` from the transcript's `mark`
   (100→40) around `IntraEpisodeInsolvency.evidence_slots[0]` (=30) → "fell 60% at slot 30", underwater
   range from the finding. Gap ranges derived from `divergenceRanges`. Nothing invented; no unlabeled
   money figures.

## Gates

- Scope: only `web/build.js` + generated `web/index.html` (+ this task's brief). No `crates/`,
  `programs/`, or `gallery/*.json` touched.
- `node web/build.js` deterministic: `77b71794…0672d7` on repeated runs; committed `index.html` == fresh
  build.
- Progressive enhancement: default-selected state is fully rendered in initial DOM (screenshot-legible
  without animation); `prefers-reduced-motion` block present (line ~217). Responsive at 390px verified
  (single column, no horizontal scroll). Roster is keyboard-accessible (radiogroup + arrow-key handler).
- No Rust changed → `cargo test` surface identical to green master; Codex confirmed `--offline` passes.

Re-request review after the P1+P2 fix (Round 2). This is close — one honest agent shouldn't be told it
left a gap it never left.

## Round 2 — APPROVE (`61fa8a1`)

Both fixes verified by re-screenshotting and reading the computed style of `.reveal-act h3`:
- **Steady holder** (Pass): headline now "The record matches the promise." in `rgb(86,216,140)` =
  `var(--safe)` (green). Coheres with the green "Kept its word" verdict — the card now reads
  trustworthy end-to-end.
- **Balanced Jupiter trader** (Pass): same — "The record matches the promise." + green.
- **Quiet long** (deception): unchanged — "The gap appears in slots 1–60." in `rgb(255,210,164)`
  (amber). Deception path retained.
- Fix scope minimal (11 lines `build.js`, 6 lines `index.html`); Node + inlined browser logic stay in
  sync. `node web/build.js` deterministic (`5801ff70…eea534` on two runs); committed `index.html` ==
  fresh build. `cargo test --offline` green (no Rust changed). Honesty audit from Round 1 unaffected.

Merges to `master`.
