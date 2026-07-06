# Task 011 — Certification dashboard (touchable demo)

**Owner:** CC (frame-thin: web tooling + a harness `gallery --core` transcript emitter).
**Reviewer:** Codex.
**Branch:** `task/011-dashboard`.
**Depends on:** Task 010 merged.
**Motivation:** win-path gap ③ — judges can't run `cargo`. A static, self-contained dashboard that
visualizes the committed certification transcripts makes the demo clickable (and hostable on GitHub Pages),
and is the visual asset for the pitch.

## Scope (in)

- `gallery --core` (harness): write the canonical trio as transcripts — `core-honest.json` (Pass),
  `core-gamer.json` / `core-phantom.json` (ShortcutDetected). Deterministic, committed.
- `web/build.js`: inline the committed `gallery/*.json` transcripts into a self-contained
  `web/index.html` (no fetch/server/CORS — works via file:// and Pages). One card per certification:
  verdict badge, mandate, a per-slot net-delta-vs-claim SVG chart (flagged slots coloured), findings.
- `web/{package.json,README.md}`, committed built `web/index.html`.
- `.github/workflows/pages.yml`: rebuild + deploy `web/` to GitHub Pages on push (public repo required).

## Acceptance criteria

- `gallery --core` writes the three transcripts deterministically; `web/build.js` regenerates
  `index.html` deterministically. Cards render honest=PASS, gamer/phantom=FLAG, jupiter-neutral=PASS,
  jupiter-drift=FLAG, scripted-drift=FLAG (verified by screenshot).
- All prior tests still green; `cargo test --offline` green; no warnings.

## Out of scope

- A live/interactive backend (this is static, reads pre-computed transcripts). Actually enabling Pages
  (repo must be public) is a submission-time logistics step, not this task.
