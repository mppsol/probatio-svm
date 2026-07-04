# Task 008 — Harden the live agent path (review 007 P2s)

**Owner:** CC (frame-thin: hygiene / live-path de-risking).
**Reviewer:** Codex.
**Branch:** `task/008-agent-hardening`.
**Depends on:** Task 007 merged.

Closes the two P2s from `reviews/007-llm-agent.md`. Both touch the untested live path in `llm.rs`.

## Changes

1. **Drop `strict: true` from the `submit_action` schema.** Anthropic strict tool use may require every
   property in `required`; with `side`/`qty`/`target_delta` optional, the forced tool call could 400 at
   runtime on every `agent` invocation — and this path is untested in CI. The forced `tool_choice`
   already guarantees the tool is called, and `parse_submit_action` already validates the input
   defensively, so `strict` is belt-and-suspenders. Dropping it removes the tail risk. (Re-add later if
   live testing confirms strict-with-optional is accepted.)
2. **Keep the API key out of argv.** Pass the `x-api-key` header via a curl config file on **stdin**
   (`curl --config -`) instead of as a command-line argument, so the key is not visible in the process
   list (`ps`). The request body stays an inline `--data` arg (not sensitive).

## Acceptance criteria

- `submit_action` schema no longer sets `strict: true`; the rest of the schema is unchanged.
- `CurlClaude` no longer passes the key as an argv element; it writes a curl config (`header =
  "x-api-key: …"`) to the child's stdin and runs `curl --config -`.
- `parse_submit_action` and all offline tests unchanged and green (`cargo test --offline`); no new
  warnings. Live path compiles.

## Out of scope

- Live API verification (no key in CI). CPI guard promotion, gallery, pitch video.
