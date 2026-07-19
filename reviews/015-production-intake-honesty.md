# Review 015 — Production-intake honesty

**Reviewer:** CC. **Branch:** `task/015-production-intake-honesty` @ `971a3df`.
**Method:** rendered with Puppeteer; screenshotted the roster note and CTA at 1200px and the CTA at 390px.

## APPROVE

Two small, honest additions land exactly as intended without disrupting the hook / roster / 3-act stage:

- **Roster clarifier** (muted single line under "Choose a candidate"): "Six illustrative agents — each a
  recorded certification episode. In production, you bring the agent you're about to trust." Reads as a
  quiet caption, does not compete with the roster. Removes the "live fund marketplace" misread.
- **Production intake** (folded into the existing CTA, no new section): "In production, Probatio is built
  to replay the agent you're about to trust — brought by its developer or by the vault about to delegate
  to it — and certify it from account state before it manages real capital." States the A/B intake
  (self-cert / gatekeeper vault) and the pre-capital timing in plain words.

### Honesty audit — PASS

- **Anticipatory voice, no adoption claim:** "In production…", "is built to…". No "teams use", no live
  customer base, no hosted-service claim beyond the static demo. Demand-unproven boundary respected.
- Existing honest lines retained: CTA keeps "computed offline by reading account state as ground truth —
  nothing to bypass"; footer keeps "static replay of committed certification episodes."
- No "agents can't cheat" / realtime / exploit / demand statistic. No verdict/chart/evidence/consequence
  logic touched (diff is framing copy only).

### Gates

- Scope: `web/build.js` + generated `web/index.html` (+ brief) only.
- `node web/build.js` deterministic (`f0f96f26…50a82` twice); committed `index.html` == fresh build.
- Responsive: CTA copy wraps cleanly at 390px, button stacks; roster note wraps; no horizontal scroll.
- No Rust changed → `cargo test` surface identical to green master; Codex confirmed `--offline` passes.
- The CTA/footer link `github.com/psyto/probatio-svm` 301-redirects to the live `mppsol/probatio-svm`
  (verified) — resolves, not broken (pre-existing link, unchanged by this task).

Merges to `master`.
