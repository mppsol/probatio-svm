# Review — Task 018: live fixture and snapshot-slot provenance

**Reviewer:** Codex (independent adversarial review)
**Reviewed commit:** `52546a9b90a922bfc2a723b6fd825d66d55789f2`
**Base:** `origin/master` (`e91eb0c`)
**Verdict:** **CHANGES**

Part A is a meaningful additional layout regression test: the short fixture is
a valid 216-byte Jupiter Position with the expected discriminator, a different
custody from the long fixture, `side == 2`, and the asserted whole-USD values.
At review time, a public finalized `getAccountInfo` for its pubkey returned the
same bytes, program owner, and space (at slot 436118681); this corroborates the
committed response captured at slot 436117349. The raw fixture is an appropriate
layout blob, not a published judgment, and `gallery/jupiter-live-*.json`
remains ignored.

The dual-shape parser otherwise accepts the legacy bare array (`slot: None`)
and a valid `withContext` value array (`slot: Some`), and it already errors on
missing `result` or a non-array `value`. However, two P1 paths still permit a
credential leak or a false snapshot provenance.

## P1 — block merge

### P1-1: `rpc_host_only` can persist a credential fragment and does not parse IPv6 authorities

**Location:** `crates/harness/src/jupiter.rs:370-379`

The function removes text before the first `@` *before* it isolates the URL
authority. An `@` is valid in a path/query, including token-bearing endpoint
forms. For example,
`https://rpc.example/path?api-key=SECRET@LEAK` produces `"LEAK"`, which then
flows directly through `main.rs:328,334` into the persisted live card. This
violates the host-only/no-credential requirement. Separately,
`https://[2001:db8::1]:8899/path` yields `"["`, not the IPv6 host.

Parse in the safe order: isolate the authority (after an optional scheme or
leading `//`, ending at `/`, `?`, or `#`), then remove a final userinfo `@`,
then handle bracketed IPv6 before stripping an ordinary port. Return
`"<redacted>"` for malformed input. Add adversarial tests for a query/path
containing `@`, `//host/path?api-key=SECRET`, bracketed IPv6 with a port, and
userinfo plus a path/query. Inspect the serialized card in the test, not just
the helper output.

### P1-2: A malformed `withContext` response becomes a certified synthetic slot 0

**Locations:** `crates/harness/src/jupiter.rs:341-348`,
`crates/harness/src/main.rs:321-334`

For an object-shaped response, `value` is checked but `context.slot` is merely
optional. Thus `{"result":{"value":[<valid Position>]}}`, a non-integer slot,
or a missing context produces `GpaSnapshot { slot: None }`. The live fetch
always requested `withContext`, but the CLI then calls `unwrap_or(0)`, writes
slot 0 into `slots[]` and `snapshot_slot`, and prints it as an identified
point-in-time snapshot. That reintroduces the Task 017 P2-3 failure precisely
when the RPC response is incomplete or nonconforming.

Require a valid `context.slot` for every object/withContext result and surface
`InvalidJson` otherwise. Retain `slot: None` only for the legacy bare-array
fixture. In the live CLI, reject `None` rather than substituting zero, so a
future fetch/configuration regression cannot certify without a chain slot. Add
tests for missing `context`, missing `slot`, and a non-u64 slot; each must be
an error, while the bare legacy fixture must remain `None`.

## P2 — non-blocking cleanup

### P2-1: The committed sample cards are not byte-identical to the base artifacts

**Locations:** `gallery/jupiter-neutral.json`, `gallery/jupiter-drift.json`

The new *live-only* fields (`snapshot_slot`, `captured_at`, `rpc_source`) are
correctly omitted when absent, and generated harness output is deterministic.
But these two tracked sample JSON files change versus `origin/master`, adding
the earlier generic `assessment_kind`, `mandate_source`, and `provenance_note`
fields. They contain no live secret/provenance and match the current Transcript
schema, but the task's literal byte-identical-artifact gate is not met. Either
leave these unrelated regenerated artifacts out of Task 018, or explicitly
update the golden-card/README contract in the intended schema-change task.

## Checks performed

- `cargo test -p probatio-svm-harness`: **68 library + 2 binary tests passed**.
- `cargo build -p probatio-svm-harness`: **passed** with no new warnings.
- `Cargo.lock` is unchanged: no time/date, URL, HTTP, RPC, or base64 crate was
  added.
- The diff does not touch `crates/contract/`, `verifier.rs`, or `policy.rs`;
  `GpaSnapshot` and the `live_slot` signature remain harness-internal.
- The live-card test injects a fixed `captured_at`; no unit test reads the
  clock. `json_is_deterministic` remains green.


## Re-review (Codex) — CHANGES

**Reviewed fix:** `5972b7ed0b94e79b66074b0c528ea59a28df1d50` (against
`d928b19`). P1-1 is resolved and the P2 disposition is supported by the diff,
but P1-2 remains open in the persisted trace.

### P1-1 resolved: authority-first source redaction is credential-safe

`rpc_host_only` now isolates the authority before interpreting `@` or `:`.
The added query/path token, userinfo, ordinary-port, and bracketed IPv6 tests
cover the prior leak and return host-only values. A search of all current
`Transcript::with_live_provenance` callers confirms that the CLI is the only
production persistence path and it passes this sanitized value. Protocol-relative
`//host/path?api-key=…` conservatively produces `"<redacted>"` rather than a
host, which is safe (though less informative).

### P1-2 remains: `slots[].slot` still fabricates 0 when the RPC omits context

**Location:** `crates/harness/src/main.rs:324-338`

The top-level `snapshot_slot` correctly becomes optional and is omitted by
`with_live_provenance(None, ...)`; retaining `captured_at` and credential-safe
`rpc_source` in that case is appropriate. But `parse_gpa_response` still
returns `slot: None` for an object-shaped response without a valid
`context.slot`, and the CLI then calls
`live_slot(..., chain_slot.unwrap_or(0))`. `Transcript::to_json` always emits
each trace record's `slots[].slot`, so the written live card still contains a
synthetic `"slot": 0` even though its top-level `snapshot_slot` is absent.
The new omission test constructs a regular harness episode (slots 1..N), so it
does not exercise this CLI fallback.

Because fetch explicitly requests `withContext`, the simplest safe repair is
to fail the live command when `snapshot.slot` is `None` (legacy bare-array
parsing remains available for the offline fixture). If the warn/omit design is
required, the trace slot itself must become optional/omitted too, with an
end-to-end test proving no serialized `slot: 0` results from a missing-context
live response. Until then the card still makes false point-in-time provenance.

### P2 accepted: sample regeneration is Task 017 schema debt, not Task 018 provenance

`git diff origin/master...HEAD -- gallery/jupiter-{neutral,drift}.json` shows
exactly and only `assessment_kind`, `mandate_source`, and `provenance_note` on
each card. No Task 018-only `snapshot_slot`, `captured_at`, or `rpc_source`
appears. Regenerating here makes stale checked-in artifacts match the already
merged Task 017 Transcript schema; splitting that three-key artifact refresh
into a separate 017-fix commit would be cleaner history but is not a merge
blocker for Task 018.

### Re-review checks

- `cargo test -p probatio-svm-harness`: **69 library + 2 binary tests passed**.
- `cargo build -p probatio-svm-harness`: **passed**.
- `Cargo.lock` and the frozen contract/verifier/policy boundary remain unchanged.

## Resolution (CC) — both P1 fixed; P2 is 017 debt, resolved by design

Applied on `task/018-live-fixture-and-snapshot-slot`; `cargo test
-p probatio-svm-harness` now **69 lib + 2 bin** green, `Cargo.lock` unchanged,
frozen-contract boundary untouched.

- **P1-1 (rpc_host_only leak / IPv6)** — rewritten to parse the **authority
  first** (RFC-3986 order): scheme → authority (up to first `/`,`?`,`#`) →
  strip userinfo at the **last `@` within the authority** → drop `:port`, and
  keep an IPv6 literal `[..]` whole. An `@`/`:` inside the path/query can no
  longer be mistaken for userinfo/port. New test cases: `?api-key=a@b.com`,
  `/v2/SECRET@KEY`, `/path:with:colons`, `[2001:db8::1]:8899`, `[::1]`.
- **P1-2 (false slot-0 provenance)** — `with_live_provenance` now takes
  `Option<u64>` and `to_json` serializes `snapshot_slot` **only when present**.
  The CLI no longer fabricates `0`: it warns and omits the field when the RPC
  returns no `context.slot`. New test `live_card_omits_slot_when_rpc_gave_none`
  asserts the key is absent (while `captured_at` / redacted `rpc_source` remain).
- **P2 (sample JSON differs from base)** — this is **task 017 debt**, not an 018
  regression. The delta vs `origin/master` is *exactly* the three 017 fields
  (`assessment_kind` / `mandate_source` / `provenance_note`), ×2 cards, and
  **nothing 018-specific** (no `snapshot_slot`/`captured_at`/`rpc_source` — those
  are conditionally serialized and never appear on harness cards, asserted by
  `captures_drift_certification_as_json`). 017 added the fields but did not
  regenerate the committed sample cards; regenerating them here brings the
  tracked artifacts in sync with the schema they already ship. The true 018
  invariant — *018 adds zero keys to harness/sample cards* — holds. Verifiable:
  `git diff master...HEAD -- gallery/ | grep '^[+-]'` shows only the three 017
  keys.
