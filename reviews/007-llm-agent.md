APPROVE

Reviewer: CC. Implementer: Codex. Branch: task/007-llm-agent @ e00073a.

## Verdict rationale

Clean and correct. Probatio is now literally an agent proving ground: a real Claude agent runs behind the
`Policy` trait via the `Decider` seam, and the verifier certifies whether it honored its mandate. The
single highest-risk item — the Opus 4.8 request shape — is correct. No P0/P1.

## What was verified (positives)

- **Opus 4.8 request shape is correct (the #1 risk).** `build_request_body` sends only `model`,
  `max_tokens`, `system`, `messages`, `tools`, `tool_choice`. It does NOT send `temperature`, `top_p`,
  `top_k`, or any `thinking` field — all of which 400 on Opus 4.8. Headers are right
  (`anthropic-version: 2023-06-01`, `x-api-key`, `content-type`), and `tool_choice` forces
  `submit_action`.
- **Response extraction.** Iterates `content[]`, matches `type == "tool_use"`, reads `.input`, maps to
  `contract::Action` — the correct Messages-API shape.
- **The certification demo lands.** `scripted_neutral_agent_is_deterministic_and_passes` (all-Noop,
  claims delta 0 ⇒ Pass, deterministic) and `scripted_drift_agent_triggers_claim_tracks_exposure`
  (opens long 10 while claiming neutral ⇒ `ClaimTracksExposure` ⇒ ShortcutDetected). This is exactly the
  product thesis: an agent that violates its stated mandate is caught.
- **Testability seam.** `Decider` trait + `ScriptedDecider` keeps the whole Policy wiring deterministic
  and offline; `CurlClaude` is the live impl. No test hits the network (confirmed by grep).
- **`parse_submit_action`** is a pure, well-tested function with strict key validation (`expect_keys`
  rejects unexpected fields; `reject_unexpected_fields` test covers it). qty→u64, target_delta→i64,
  side→Side — all match the contract.
- **Graceful degrade.** No `ANTHROPIC_API_KEY` ⇒ `agent` subcommand prints a clear message and exits 1
  (manually confirmed). `CurlClaude::decide` maps any curl/parse error to `Action::Noop` + stderr, so a
  mid-episode failure degrades safely rather than hanging or faking a result.
- **Contract change** is minimal and non-breaking: `PartialEq, Eq` added to `Action`/`AgentAccountRef`
  for `assert_eq!` in tests. serde_json 1.0 added (cached, builds offline). 47 tests green, no new
  Rust warnings.

## Findings (all P2 — live-path / hygiene, non-blocking)

P2 [llm.rs:139 `strict: true` with optional properties]: the `submit_action` schema marks only `action`
as `required` while `side`/`qty`/`target_delta` are optional, under `strict: true`. Anthropic strict tool
use may (like structured outputs) require every property in `required` — if so, the forced tool call
400s at runtime on every `agent` invocation. This path is untested in CI (no key). Before relying on the
live agent, either verify the schema against the live API once, or drop `strict: true` — the defensive
`parse_submit_action` already validates the input robustly, and the forced `tool_choice` already
guarantees the tool is called, so `strict` is belt-and-suspenders here. Track.

P2 [llm.rs:60 API key in argv]: the key is passed as a `curl` argument (`x-api-key: <key>`), so it is
visible in the process list (`ps`) for the duration of the call. Low severity (local, single-user dev
tool), but prefer passing it via an env-substituted header or stdin (`--header @-`). Track.

P3 [nit]: the observation prompt includes `funding_index`, which is always 0 in Stage 0 — harmless.

## Follow-ups to track

- Verify the live `submit_action` schema (or drop `strict: true`) before the `agent` demo is presented.
- Move the API key off argv.
- A future task could add a small LLM-agent "gallery" (a couple of mandates, run live, save transcripts)
  for the pitch — out of scope here.
