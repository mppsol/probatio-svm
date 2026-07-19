# Review 014 — Always show the replay evidence

**Reviewer:** CC. **Branch:** `task/014-always-show-evidence` @ `e411d5c`.
**Method:** rendered with Puppeteer; drove the roster and inspected the default deception card
(Measurement gamer), a Pass card (Steady holder), and the phantom card. Read the `.proof` text of each.

## CHANGES (one P2 — visible on the Pass path; everything else is APPROVE-ready)

The core of the task is done well: the `<details>` toggle is gone, evidence is an always-open
`<section class="proof">` panel, the plain-language sentence leads each row with the slot range, and
the raw finding kind stays a small monospace subtitle. The old Node no-findings interpolation bug is
fixed (the branch is now a template literal; the Pass card correctly shows `position +10`, and Node ==
browser render). Determinism holds; honesty intact (see below). Deception cards read excellently.

**P2 — orphaned period on the Pass evidence sentence.** `web/build.js` CSS `.proof code { display:block }`
On the Pass card the evidence reads: *"No mismatch found — the recorded position matched the claim of
`position +10`"* followed by a lone **"."** on its own line. Cause: the `.proof code { display:block }`
rule (correct for the finding-row *kind* subtitle) also matches the **inline** `<code>position +10</code>`
in the `.proof-clear` sentence, forcing a block break so the trailing period drops to the next line.
On the reassuring Pass path this reads as sloppy.

Fix (one line): scope the block rule to finding-row codes only — e.g. change `.proof code` to
`.proof li code`, leaving the `.proof-clear` inline `<code>` to render inline so the sentence + period
stay on one flow. (Alternatively give `.proof-clear code { display:inline }`.) Apply in the CSS; no JS
change. Re-verify the Steady holder / Balanced Jupiter cards render the sentence as one clean line.

## Honesty audit — PASS

- Evidence rows come straight from the transcript: plain sentence via `findingText`, ranges via
  `evidenceRange`; raw `kind` shown verbatim as the demoted tag. No invented findings, no editorializing
  beyond the existing plain mappings. Verified on gamer (ContinuousNeutrality / ClaimTracksExposure /
  IntraEpisodeInsolvency), phantom (PhantomExposure / IntraEpisodeInsolvency), honest (no-mismatch line).
- No chart/verdict/consequence logic changed (diff is 8 lines `build.js`). The "computed offline …
  nothing to bypass" framing is untouched. Plain-first / kind-demoted rule honored.

## Gates

- Scope: `web/build.js` + generated `web/index.html` (+ brief) only.
- `node web/build.js` deterministic (`231053ff…8a7cc` twice); committed `index.html` == fresh build.
- No `<details>`/`<summary>` remain; no double-quoted `${…}` literal remains (old bug class gone).
- No Rust changed → `cargo test` surface identical to green master; Codex confirmed `--offline` passes.
- Progressive enhancement preserved (default card evidence in initial DOM); responsive.

Re-request review after the one-line CSS fix (Round 2).
