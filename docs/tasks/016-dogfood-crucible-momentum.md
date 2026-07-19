# Task 016 — Dogfood: certify our own Crucible momentum bot

**Owner:** Codex (frame-thick: faithful port of a known rule + gallery wiring + web card).
**Reviewer:** CC (faithfulness vs the Crucible source, honesty/non-strawman framing, screenshot, gates).
**Branch:** `task/016-dogfood-crucible-momentum` (created; brief committed on it).
**Depends on:** Task 015 merged.

## Why (dogfooding — with an honest frame)

We want a certification card that runs on **our own real code**, not a synthetic policy. Crucible's
`bot-simple` momentum bot is real, shipped logic. Porting its decision rule into the harness and
certifying it makes the demo "we ran Probatio on our own bot," and exercises the harness path on real
strategy logic.

**Honest framing (this is the review gate — avoid a strawman).** The Crucible bot is a **momentum**
strategy — directional *by design*; it never claimed to be neutral. So the card is NOT "our bot lied."
It is a **due-diligence demonstration**: *if an operator handed you this bot and claimed it was
market-neutral, could you verify that?* Probatio replays what the code actually does and shows it is
fully directional — so the neutral claim would be false. The value shown is "verify the claim against
the code, don't take the operator's word" — the product thesis, on our own bot. The card copy must be
transparent that the bot is honestly directional; do not imply the bot itself was deceptive.

## Source of truth (port faithfully — CC will diff against this)

`/Users/hiroyusai/src/crucible/packages/bot-simple/src/index.ts`, `tick()` (lines ~148–211). The rule:
- Keep a moving average of the **last 5** prices (`getMA()`, lines 120–124).
- `signal = price > MA ? long : short` (line 173). Below 5 samples (MA not ready) → do nothing (flat).
- If the current position side already equals `signal` → hold. If it's the opposite → close, then open
  the new side (a flip). Fixed size, fixed leverage (`POSITION_SIZE_USD=100`, `MAX_LEVERAGE=5`).
- Net effect: **always in-market once warmed up** — long above the 5-MA, short below, flipping on
  crossover. Maximal one-sided delta at all times.

The port must preserve THIS rule (MA window = 5, `mark > MA → long/short`, flip on crossover,
fixed size). Sizing is adapted to the harness's integer-`qty` position model (use a fixed `QTY` like the
existing policies) — that's a faithful adaptation of "fixed size", not a change to the decision rule.
Cite the Crucible file:line in a doc comment on the new policy.

## Scope (in)

1. **New policy** in `crates/harness/src/policy.rs`: `CrucibleMomentum` implementing `Policy`.
   - Reads `obs.mark` each slot (the `Observation` already carries `mark: i64` — no contract change),
     maintains a 5-sample price history, applies the rule above, emits `Open`/`Close`/`Noop` `Action`s
     against `AgentAccountRef::Measured`.
   - `claim()` → `AgentClaim { claimed_delta: 0, claims_solvent: true }` (we certify it against a
     **neutral** mandate; the flag is about exposure-vs-claim).
   - `provisioning()` → enough collateral to stay **solvent** through the shock, so the finding is a
     clean exposure/claim mismatch (`ClaimTracksExposure` / `ClaimMismatch`), NOT insolvency. We want
     the clean discriminator, not a pile-on.
   - Ships with a unit test asserting the warmed-up policy holds a non-zero directional delta on the
     episode price path (so the neutral claim is flagged) and that the run is deterministic.
2. **Gallery emitter** in `crates/harness/src/main.rs`: under `gallery --core`, add
   `write_core("core-crucible-momentum", &mut CrucibleMomentum, &<neutral mandate>)` writing
   `gallery/core-crucible-momentum.json` deterministically. Commit the generated transcript.
3. **Web card** in `web/build.js`: add the new transcript to `CARDS` as a candidate with honest,
   plain-language copy per the framing above, e.g.:
   - title: "Momentum bot (our own)" · venue: "ported from Crucible"
   - short: "A real momentum bot from our Crucible project — long above its moving average, short below.
     Certified against a *neutral* claim: Probatio flags the directional exposure it would carry."
   Regenerate `web/index.html` (deterministic) and commit.

## Honesty constraints (CC gate)

- **Faithful port:** the rule must match the Crucible source (MA=5, mark>MA→long/short, flip, fixed
  size). CC diffs against `bot-simple/src/index.ts`. A reimplemented/different strategy = CHANGES.
- **Non-strawman copy:** transparent that the bot is directional by design and was never claimed
  neutral; the card demonstrates verifying a neutral *claim* against real code, not accusing the bot.
- Existing chart/verdict/evidence logic is reused unchanged; coloring still keys off
  `measured_delta !== claimed_delta` per slot. No overclaim ("agents can't cheat" / realtime / demand).
- Numbers/verdict/slots shown come from the generated transcript.

## Technical constraints

- `cargo build` + `cargo test --offline` green; the new policy ships with a test; **episode run is
  deterministic** (same bytes on re-emit). No network in tests.
- `gallery --core` regenerates `core-crucible-momentum.json` deterministically; `node web/build.js`
  deterministic; committed `web/index.html` == fresh build.
- No contract change (`Observation.mark` already exists). Don't touch verifier semantics, the chart, or
  other transcripts' data. Keep the private solinv catalog out of the tree.
- Commit as `psyto <saito.hiroyuki@gmail.com>`.

## Acceptance criteria

- `gallery --core` emits `gallery/core-crucible-momentum.json`; verdict = flagged (directional exposure
  vs neutral claim); the delta timeline visibly swings long/short across the episode (a dynamic chart,
  not a flat line).
- A new candidate card renders on the demo with honest, non-strawman copy; picking it shows the
  promise (neutral claim under test) → reveal (directional swings) → catch (flagged, exact slots).
- Port is faithful to `bot-simple/src/index.ts` (CC-verified). `cargo test --offline` green; both
  `gallery --core` and `node web/build.js` deterministic; committed `index.html` == fresh build.
- Screenshot-verified by the reviewer at 1200px + 390px.

## Out of scope

- Certifying Yogi's real on-chain trace (that's the next task — a Drift trace adapter).
- Extending the verifier to express a "momentum mandate" PASS (the verifier is neutrality-centric; we
  use the honest neutral-claim due-diligence framing instead).
- Any change to the contract, verifier logic, or other candidates.
