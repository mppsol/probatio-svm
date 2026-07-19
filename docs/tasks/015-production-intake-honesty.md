# Task 015 — Make the demo honest about what it is and how it's used in production

**Owner:** Codex (frame-thick: UI/UX copy + placement). **Reviewer:** CC (screenshot + honesty).
**Branch:** `task/015-production-intake-honesty` (created; brief committed on it).
**Depends on:** Task 014 merged.

## Why

The "pick a candidate to manage the fund" UI is a pedagogical device — the six candidates are fixed,
recorded certification episodes, not a live marketplace of real funds. A judge could misread it as
"an agent marketplace," which misrepresents the actual product: Probatio is a **pre-deployment
certifier** — it certifies an agent *before* it manages real capital. Two small, honest additions
close that gap:

1. Signal that the six are **illustrative example episodes**, not a live roster.
2. Make the **production intake** explicit: in real use you bring the agent you're about to trust, and
   Probatio runs it through the same replay and certifies it before it touches capital.

Keep this SMALL and non-disruptive — do not weaken the felt moment (hook, roster, 3-act stage). This is
copy + light placement, not a redesign.

## Scope (in)

1. **Roster clarifier (one muted line).** Under the "Choose a candidate" heading, add a short line, e.g.:
   *"Six illustrative agents — each a recorded certification episode. In production, you bring the agent
   you're about to trust."* One line, muted, must not compete with the roster.
2. **Production-intake framing.** Make the intended production flow explicit — fold into the existing
   "How Probatio checks an agent" section and/or the CTA rather than adding a whole new section. Convey
   the pre-deployment certification model in plain words, e.g.: *"In production, the agent you're about
   to trust is run through this same replay — brought by its developer, or by the vault about to delegate
   to it — and certified from account state before it manages real capital."* Keep the existing CTA
   ("certify your own agent") consistent with this.

## HONESTY constraints (CC gate — anticipatory, never overclaiming)

- Frame the production flow as **design intent** ("Probatio is built to…", "In production…"), NEVER as
  current adoption ("teams use…", "agents are certified by…"). Demand is unproven; do not imply a live
  customer base or a running hosted service beyond the static demo.
- Keep the existing honest lines: "static replay of committed certification episodes" and "computed
  offline by reading account state — nothing to bypass."
- Do not claim "agents can't cheat", realtime monitoring, or any exploit/demand statistic.
- Nothing about verdicts/chart/evidence changes — this is framing copy only.

## Technical constraints (carry over)

- Self-contained static; `web/build.js` stays the generator; `node web/build.js` deterministic; commit
  the regenerated `web/index.html`. If any string is duplicated in the inlined browser script, keep Node
  and browser in sync.
- Responsive (1200px + 390px), no horizontal scroll; new copy must wrap cleanly on mobile.
- `web/` only. Do NOT touch `crates/`, `programs/`, `gallery/*.json`, or verifier semantics.
  `cargo test --offline` stays green.

## Acceptance criteria

- A first-time judge can tell the six are illustrative example episodes (not a live fund marketplace),
  and can read, in plain words, how certification is meant to work in production (bring an agent →
  standard replay → certified from account state before it manages capital).
- Additions are small and do not disrupt the hook / roster / 3-act stage. Existing honest framing
  retained. Anticipatory voice (no adoption/demand claims).
- `node web/build.js` deterministic; committed `index.html` == fresh build. `cargo test --offline` green.
- Screenshot-verified by the reviewer at 1200px and 390px.

## Out of scope

- Any change to verdict/chart/evidence/consequence logic or transcript data.
- A real submission backend / hosted certification service (this stays a static demo).
- New candidates/venues.
