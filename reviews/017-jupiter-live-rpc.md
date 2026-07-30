# Review — Task 017: live Jupiter Perps ingestion

**Reviewer:** Codex (independent adversarial review)  
**Reviewed commit:** `12b2c521b189ad9974972097eb37009ab59b5a0e`  
**Base:** `origin/master` (`febb712`)  
**Verdict:** **CHANGES**

The account-field offsets are consistent with the public `Position` schema: after
the 8-byte discriminator, four pubkeys and two `i64`s place `side` at 152,
then `price`, `size_usd`, and `collateral_usd` at 153/161/169.  The committed
216-byte fixture therefore tests a real long account successfully.  There is no
evidence that a custody changes that layout: custody is a fixed `Pubkey` field.

However, the ingestion boundary currently treats malformed, truncated, and
wrong-account data as an empty/closed position.  That permits a live verdict
over an incomplete set.  The generated card also loses the mandatory
"unsolicited due-diligence / mandate declared by us" framing once the console
output is gone.

## P1 — block merge

### P1-1: Truncated or malformed account bytes are silently dropped, allowing a partial live snapshot

**Locations:** `crates/harness/src/jupiter.rs:219-240`, `:255-268`, `:292-295`

`base64_decode` trims every trailing `=` and never validates quartet length,
padding placement, or remaining bits.  For example, removing four characters
from a valid 216-byte account's unpadded base64 leaves a valid-character string
that decodes to 213 bytes.  `decode_position` returns `None` for that bad
length, and `parse_gpa_response` silently omits it.  With several accounts,
the remaining positions are certified as if the omitted directional position
did not exist; if every entry is truncated, the CLI reports “no open” rather
than an acquisition failure.

This violates the ground-truth boundary: `None` currently represents both a
known closed slot and an untrusted/invalid account.  Make the base64 decoder
strict and make decoding return an error for a non-216-byte account (and for
an unknown discriminator/side where the account was expected to be a
Position).  Preserve `None` only for a structurally validated closed slot.
Add offline tests for missing/extra/misplaced base64 padding and a GPA array
containing one valid account plus one truncated account; both must return
`Err`, never a partial vector.

### P1-2: The GPA query and decoder do not verify the Anchor Position discriminator

**Locations:** `crates/harness/src/jupiter.rs:219-240`, `:308-315`

The RPC scan filters only program owner, account length, and bytes at the
assumed owner offset.  It never constrains nor verifies bytes `0..8`, even
though the decoded account begins with an Anchor discriminator.  Consequently
any other 216-byte account of the Jupiter program with those bytes at offset 8
is accepted if coincidental values at 152/161/169 look like a side and
notional; other variants are silently dropped through P1-1.  This is precisely
the class of schema drift/type confusion the fixed-offset recovery path must
reject rather than certify through.

Add the known Position discriminator as both a `memcmp` filter at offset zero
and a decoder check.  Add a test that mutates the fixture discriminator and
asserts `parse_gpa_response` returns `Err`.  The decoder's public schema lists
the discriminator as the first field before `owner`, `pool`, `custody`, and
the monetary fields, so validating it is part of validating these offsets.

### P1-3: The persisted live card omits the required unsolicited-DD provenance

**Locations:** `crates/harness/src/main.rs:341-345`; emitted schema in
`crates/harness/src/transcript.rs:61-102`

The console banner correctly says “unsolicited due-diligence” and “declared by
us”.  The card written to `gallery/jupiter-live-*.json` does not: it serializes
only the generic `NEUTRAL_MM.system` (“You are a delta-neutral market maker…”),
a synthetic claim, and a verdict.  Viewed in the gallery (the artifact that
outlives the console), it reads as though the wallet operator was given that
mandate.  This fails Task 017's non-negotiable honesty condition and the
Task 015/016 banner+card requirement.

Persist explicit provenance in the live transcript/card, e.g.
`assessment_kind: "unsolicited_due_diligence"`,
`mandate_source: "declared_by_probatio"`, and wording that a FLAG is not an
assertion that the owner lied.  Cover the serialized card in an offline test.

## P2 — address before a public live-demo follow-up

### P2-1: Only the long layout is recovered from chain bytes

**Locations:** `crates/harness/src/jupiter.rs:403-430`

The one committed mainnet fixture establishes the long path.  The short test
manufactures bytes at the same offsets, so it cannot detect a real-world
short/custody/layout misunderstanding.  Keep the existing fixture, but add a
redacted/consented real short Position fixture (ideally a different custody)
or record an independently reproducible schema source/version alongside the
fixture.  `sizeUsd == 0` is a sensible delta filter for a structurally valid
closed slot; it must not remain the way malformed accounts disappear (P1-1).

### P2-2: `--mark` is neither validated as positive nor actually an “on-chain override”

**Locations:** `crates/harness/src/main.rs:285-290`, `:328`

The parser accepts zero and negative integers although its error says
“positive”, and the output calls an arbitrary CLI value an “on-chain override”.
Reject non-positive values and call it a user-supplied mark override.  The
delta verdict is correctly mark-independent (`jupiter_to_snapshots` derives
`measured_delta` solely from signed `size_usd`), but this avoids overstating
the advisory liquidation model.

### P2-3: The card records synthetic slot 0, not the RPC snapshot slot

**Locations:** `crates/harness/src/jupiter.rs:333-335`,
`crates/harness/src/transcript.rs:83-90`

`live_slot` hard-codes slot 0 and the RPC request does not use a context slot.
The gitignore is correct, but a point-in-time DD card cannot identify which
on-chain snapshot it assessed.  Request `withContext`, preserve the returned
Solana slot (and RPC endpoint/capture time if appropriate), and serialize it
in the live card.

## Checks performed

- `git diff origin/master...origin/task/017-jupiter-live-rpc -- Cargo.lock` is
  empty: no HTTP/RPC/base64 dependency was introduced.
- `.gitignore` correctly ignores `gallery/jupiter-live-*.json`; no matching
  card is tracked.
- `cargo test -p probatio-svm-harness`: **61 library + 2 binary tests passed**.
- `cargo build -p probatio-svm-harness`: **passed** with no new warnings.
- Mark-independence test and implementation are sound for delta: changing
  `mark_usd` affects equity/liquidation fields, not signed-notional-derived
  `measured_delta` or `aggregate_delta`.

## Resolution (CC) — all three P1 fixed; P2-2 folded in; P2-1/P2-3 deferred

Applied on `task/017-jupiter-live-rpc`; `cargo test -p probatio-svm-harness`
now **64 lib + 2 bin** green, `Cargo.lock` still empty vs base.

- **P1-1** — `decode_position` now returns `Result<Option<JupPosition>>`:
  `Err` = untrusted/invalid account (wrong length/discriminator/side), `Ok(None)`
  = structurally validated closed slot. `parse_gpa_response` propagates the `Err`
  with `?` instead of dropping it. `base64_decode` is strict (positive
  multiple-of-4 length, ≤2 trailing pads, zero padding bits). New tests:
  strict-padding cases and a GPA array with a 213-byte account → `Err`.
- **P1-2** — added `POSITION_DISCRIMINATOR` (`[170,188,143,228,122,64,247,208]`,
  extracted from the committed fixture) as both a `memcmp` filter at offset 0 in
  `fetch_owner_positions` and a decode-time check. New test mutates the
  discriminator and asserts `Err`.
- **P1-3** — `Transcript` now carries `assessment_kind` / `mandate_source` /
  `provenance_note`, derived from the `"jupiter-live"` backend, and serializes
  them. The live card states it is unsolicited DD, that the mandate was declared
  by Probatio, and that a FLAG is not an assertion that the operator lied. New
  offline test asserts the serialized live card contains this.
- **P2-2** — `--mark` now rejects zero/negative (matches its error text); the
  banner calls it a "user-supplied mark override", not an "on-chain override".
- **Deferred** (as scoped): **P2-1** (a consented real *short* fixture needs a
  fresh mainnet capture) and **P2-3** (`withContext` snapshot slot lives on the
  untested network `fetch` path) — both belong to the pre-public-live-demo
  follow-up, not this merge.

