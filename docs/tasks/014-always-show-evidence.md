# Task 014 — Always show the replay evidence (proof panel, not a collapsed toggle)

**Owner:** Codex (frame-thick: UI/UX). **Reviewer:** CC (screenshot + honesty).
**Branch:** `task/014-always-show-evidence` (created; brief committed on it).
**Depends on:** Task 013 merged.

## Why

Probatio's whole claim is that a verdict is *provable from account state*. Today the proof lives inside
`<details><summary>See the replay details</summary>…</details>` — hidden behind a click. For a
verification product, hiding the receipts undercuts the credibility it's selling, and a judge may never
open it. Make the evidence **always visible** so "nothing to bypass" is shown, not asserted.

## The one rule that keeps this from re-introducing jargon (do NOT skip)

Task 013 deliberately demoted DeFi jargon for a non-expert judge. Always-showing the evidence must NOT
undo that. So:

- The **plain-language sentence leads** each evidence row (the existing `findingText` mapping —
  "Its account carried an undisclosed position", "The position went underwater", etc.), with the slot
  range next to it.
- The **raw finding kind** (`ClaimTracksExposure`, `IntraEpisodeInsolvency`, …) stays **demoted** — the
  same small monospace `<code>` treatment it has now. It is a subtitle for experts, never the headline.

If the raw kind becomes the visual lead, that's a CHANGES.

## Scope (in)

1. In `web/build.js`, replace the collapsed `<details>` proof with an **always-open evidence panel**
   (a titled block, e.g. heading "The evidence" / "What the replay recorded"). Same for BOTH the Node
   `proofMarkup` and the inlined browser `proofMarkup` — they must stay logically identical.
2. **Pass / no-findings case:** show a reassuring, always-visible line instead of an empty panel —
   e.g. "No mismatch found — the recorded position matched the claim of position `{claimed}`." And
   **FIX THE EXISTING BUG**: the Node no-findings branch currently returns a double-quoted string
   containing `${signed(t.claimed_delta)}`, which renders the literal text `${signed(t.claimed_delta)}`
   instead of the value (the browser twin uses string concatenation and is correct → the two paths
   diverge). Make the Node path interpolate correctly and match the browser path.
3. Keep the plain-first / demoted-code structure of each finding row (see the rule above).

## Honesty constraints (carry over — CC gate)

- Evidence text/slots come straight from the transcript `findings[]` via `findingText` +
  `evidenceRange`; do not editorialize beyond the existing plain mappings, do not invent findings.
- No overclaiming copy. Keep the honest framing ("computed offline… nothing to bypass").
- Nothing about the chart/verdict logic changes — this task only surfaces the already-computed proof.

## Technical constraints (carry over)

- Self-contained static; `web/build.js` stays the generator; `node web/build.js` deterministic (no
  `Date.now()`/`Math.random()`); commit the regenerated `web/index.html`.
- Node `proofMarkup` and the inlined browser `proofMarkup` stay in sync (byte-identical logic).
- Progressive enhancement preserved: the default-selected card's evidence is in the initial DOM and
  legible without JS/animation. Responsive (1200px + 390px), no horizontal scroll. `prefers-reduced-
  motion` respected.
- `web/` only. Do NOT touch `crates/`, `programs/`, `gallery/*.json`, or verifier semantics.
  `cargo test --offline` stays green.

## Acceptance criteria

- On every candidate the evidence is visible without any click. Deception cases list the plain-language
  findings (kind demoted to a small code tag) + slot ranges; Pass cases show the reassuring
  no-mismatch line with the correctly-interpolated claimed position.
- The Node no-findings interpolation bug is fixed; Node output == browser render for Pass cards.
- Honesty constraints hold; plain sentence leads, raw kind stays demoted.
- `node web/build.js` deterministic; committed `index.html` == fresh build. `cargo test --offline`
  green.
- Screenshot-verified by the reviewer at 1200px (a deception case + a Pass case) and 390px.

## Out of scope

- Changing verdict/chart/consequence logic or the transcript data.
- New candidates/venues; enabling Pages (already live).
