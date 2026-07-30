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

