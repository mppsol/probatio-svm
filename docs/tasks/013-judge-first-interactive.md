# Task 013 — Judge-first, pick-an-agent certification experience

**Owner:** Codex (frame-thick: UI/UX craft — copy, interaction, motion, layout).
**Reviewer:** CC — this time the review judges **appeal + intuitiveness by actually rendering
screenshots**, not just the honesty gate. (012 passed honesty but read as an engineer's dashboard; a
non-expert bounced off the jargon and the flat hero chart. That is the bar we are now clearing.)
**Branch:** `task/013-judge-first-interactive` (created; this brief is committed on it).
**Depends on:** Task 012 merged (hero + gallery + `web/build.js` exist).

## The problem we are fixing (verified by screenshotting 012)

A non-expert judge looking at the current page does NOT understand it in 3 seconds:
- **Jargon wall:** "net delta", "delta-neutral", "measured ≠ claimed", "ShortcutDetected",
  "ClaimTracksExposure", "reference perp", "transcript". No plain-language "what is this / why care".
- **Dead hero:** the featured chart is two flat parallel lines (scripted-drift holds a constant long),
  so the "watch the lie unfold" moment has no motion and no drama. The gap between promise and reality
  is never emphasized.
- **No stakes:** "net long 10" — 10 of what? The price shock (mark 100→40) and the insolvency findings
  are in the data but the page never shows the *consequence* (a real fund would have lost money).
- **Passive:** nothing to do. The user asked for a page where the *intent and how to use it* are
  immediately obvious.

## Decided direction (from the product owner)

- **Primary audience = a NON-EXPERT hackathon judge.** Plain language first. DeFi jargon is either
  removed or given an inline plain-English gloss. Story + stakes over precision-for-experts. (Experts
  are served by the detail still being available lower down / on demand — don't delete the rigor,
  demote it.)
- **Interaction = pick-an-agent → certify.** Recast the six transcripts as **candidates auditioning to
  manage a fund**. The judge picks one; Probatio certifies it in front of them: promise → replay →
  verdict + consequence. This makes "what is this and how do I use it" obvious by *doing*.

## The experience to build

### 1. The hook (top of page, ~3-second read, no jargon)

Replace the current header paragraph with a plain-language framing. Convey, in human words:
*AI agents are about to manage real money. Would you take an agent's word that it's playing it safe?
Probatio doesn't — it replays exactly what the agent did on-chain and catches it when its actions
betray its promise.* (Rewrite in your own voice; the constraint is **no undefined jargon** and it must
land the "promise vs what it actually did, proven from the chain" idea.)

Then a one-line instruction that makes usage obvious: e.g. *"Pick a candidate to manage the fund —
Probatio audits it live."*

### 2. The roster (the "how to use it" affordance)

A selectable set of candidates (the six transcripts), each with a **plain-English one-liner** and a
PASS/FLAG-agnostic neutral presentation until selected (don't spoil the verdict in the roster — the
reveal is the point). Human descriptions, e.g.:
- Honest agent → "Says it'll hold a steady bet — and does."
- Measurement gamer → "Looks flat exactly when it's measured; swings the rest of the time."
- Phantom exposure → "Claims it's flat, hides the real bet in a second wallet."
- Jupiter market-neutral → "Real Jupiter Perps agent, balanced long+short."
- Jupiter drifted → "Says balanced, actually leaning long."
- Scripted drift → "Promises neutral, quietly holds a long."

Selecting a candidate drives the stage below. **Default selection on load** = the most legible
deception (recommend **Measurement gamer** — its line visibly rides high then dives to zero at the
measurement slot, so the "it faked the metric" story reads at a glance; or scripted-drift IF you use
the hazard-band treatment in §3 to make the flat gap dramatic).

### 3. The stage: promise → reveal → catch (keep the 3-act scaffold from 012, fix the payload)

- **Promise** — the mandate in plain words + a plain gloss of what it means ("delta-neutral = promised
  to bet neither up nor down").
- **Reveal** — the chart, but make the *divergence* the hero, not the lines:
  - **Fill the area between measured and claimed** with a hazard tint labeled in plain words
    ("undisclosed directional risk — the bet it didn't admit to"). Even a flat gap becomes a dramatic
    band.
  - **Animate the reveal on selection** (slot-by-slot draw) so the viewer *watches* it happen. This is
    an enhancement: see Progressive-enhancement below — the final drawn state must be fully legible
    without the animation and under `prefers-reduced-motion`.
  - Plain-English y-axis framing ("betting UP" / "flat / neutral" / "betting DOWN") instead of raw
    numbers as the primary label; raw numbers may stay as secondary detail.
- **Catch + CONSEQUENCE** — verdict in plain words ("Caught — its actions broke its promise") AND the
  stakes: use the price shock and insolvency findings to show *why it matters*. e.g. "Then the market
  fell 60% (slot 30). This hidden long went underwater — a real fund would have taken the loss."
  - **HONESTY on consequence (hard):** the consequence must be grounded in the transcript — the
    `IntraEpisodeInsolvency` finding, the `mark` drop (100→40), the `measured_delta`. If you show any
    monetary figure it must be **transparently derived from numbers on screen** (e.g. position ×
    price move) and **labeled illustrative** ("illustrative, at the episode's prices"). Do NOT invent
    a loss the data doesn't support. For cases with no insolvency finding, keep the consequence
    qualitative ("undisclosed risk the fund never agreed to").

### 4. How it works + CTA (make the mechanism and next step obvious)

A plain 3-step ("1. The agent promises how it'll behave. 2. Probatio replays what it actually did
on-chain. 3. If actions ≠ promise, it's caught — with proof.") and a CTA: link to
github.com/psyto/probatio-svm and a plain "how to certify your own agent" pointer. Keep the honest
"computed offline by reading account state — nothing to bypass" line.

## HARD honesty constraints (carry over from 012 — CC review gate; any violation = CHANGES)

1. Chart colour/emphasis keyed off `measured_delta !== claimed_delta` **per slot**, never off a
   finding's `evidence_slots`. On `core-phantom` (0/60 diverged) the measured line stays on-claim; the
   catch is carried by the FLAG + the "hidden in another wallet" treatment. The hazard-band fill also
   follows measured-vs-claimed, and for phantom there is **no on-line band** — its risk is off-line, so
   say so.
2. Non-delta findings (`PhantomExposure`, `IntraEpisodeInsolvency`) get a distinct treatment, not a
   fake divergence on the delta line.
3. No overclaiming copy: no "agents can't cheat", no realtime-monitor claim, no exploit/demand claim.
   Anticipatory + honest only.
4. Every number/verdict/slot range on screen matches the transcript. Consequence framing grounded per
   §3 (no invented losses; monetary figures labeled illustrative and derived from shown numbers).

## Technical constraints (carry over)

- Self-contained static: no fetch/server/CORS/external assets/CDN/fonts. `web/build.js` stays the
  generator; it inlines `gallery/*.json`. `node web/build.js` is **deterministic** (no
  `Date.now()`/`Math.random()`); commit the regenerated `web/index.html`.
- **Progressive enhancement:** default rendered state (a candidate selected, chart fully drawn) is
  legible and screenshot-ready **without** animation. Selection interaction is JS; if JS-selection is
  used, render a sensible default-selected state in the initial DOM so a no-JS/first-paint screenshot
  still shows the full story. Respect `prefers-reduced-motion: reduce` (no autoplay motion).
- Responsive: works on laptop and phone; no horizontal page scroll. Keyboard-accessible selection
  (roster items are buttons/tabs, focusable, ARIA-labelled).
- Only `web/` changes. Do NOT touch `crates/`, `programs/`, `gallery/*.json` data, or verifier
  semantics. `cargo test --offline` stays green.

## Acceptance criteria

- A non-expert can, in one glance: (a) read what this is without hitting undefined jargon, (b) see it's
  interactive and pick a candidate, (c) watch promise-vs-reality diverge with the gap emphasized,
  (d) get the verdict AND why it matters (consequence).
- Picking each of the six candidates drives the stage correctly; verdicts/slots match transcripts;
  honest cases show "kept its word", deception cases show the catch + (where grounded) consequence.
- Honesty constraints 1–4 hold — in particular phantom's measured line stays on-claim.
- `node web/build.js` deterministic; committed `web/index.html` == fresh build. `cargo test --offline`
  green; no new warnings.
- **Screenshot-verified** by the reviewer at 1200px and 390px widths.

## Out of scope

- Any change to `crates/`, `programs/`, verifier semantics, or the transcript *data*.
- A real backend / running agents live in the browser (this stays a static replay of committed
  transcripts; "interactive" = client-side selection over inlined data).
- New venues / new transcripts (separate data task).
- Enabling GitHub Pages (submission-time logistics).
