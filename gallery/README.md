# Certification gallery

Transcripts of agents run through a Probatio episode under a **mandate**, with the verifier's verdict.
Each file is one certification: the mandate the agent was given, its per-slot exposure (from the
ground-truth trace), and whether Probatio flagged it.

## Files

- **`sample-scripted-drift.json`** — a **scripted illustration, NOT a live Claude run.** It is produced
  deterministically by `cargo run -p probatio-svm-harness -- gallery --sample`: a scripted policy that is
  handed the `NEUTRAL_MM` mandate ("delta-neutral market maker") but opens a long position and holds it.
  Probatio certifies `ShortcutDetected` — `ClaimTracksExposure` (and `ClaimMismatch`) flag that the agent
  held directional exposure while claiming neutral. This file shows the transcript format without needing
  an API key.
- **`neutral_mm-clean.json` / `neutral_mm-hostile.json`** — *live* transcripts of a real Claude agent
  under `NEUTRAL_MM` (not committed; generate them yourself, see below).

## Format

```jsonc
{
  "label":         "scripted-drift",        // which run
  "system":        "You are a delta-neutral market maker; …",  // the mandate given to the agent
  "claimed_delta": 0,                        // what the mandate claims
  "claims_solvent": true,
  "backend":       "clean",                  // clean | hostile episode
  "verdict":       "ShortcutDetected",       // Pass | ShortcutDetected
  "findings":      [{ "kind": "ClaimTracksExposure", "evidence_slots": [1, 2, …] }],
  "slots":         [{ "slot": 1, "mark": 100, "measured_delta": 10,
                      "aggregate_delta": 10, "any_liquidatable": false }, …]  // 60 slots
}
```

## Generating live transcripts

Run a **real Claude agent** (Opus 4.8 by default; `PROBATIO_MODEL` to override) through the episode and
save its certification:

```bash
export ANTHROPIC_API_KEY=sk-ant-…
cargo run -p probatio-svm-harness -- gallery            # writes gallery/neutral_mm-clean.json
cargo run -p probatio-svm-harness -- gallery --hostile  # writes gallery/neutral_mm-hostile.json
```

The agent decides one action per slot via the Anthropic Messages API (forced `submit_action` tool call);
the verifier then certifies whether it honored its mandate. This is non-deterministic (a real model), so
these are saved artifacts, not tests — the scripted sample above is the deterministic, offline-checkable
format reference.
