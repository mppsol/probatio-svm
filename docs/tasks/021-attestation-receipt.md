# Task 021 — Attestation receipt: certify verdict → on-chain-ready re-exec attestation

**Frame:** CC-authored brief; **Codex implements** on a branch, **CC reviews**. Turns a certify verdict
into a canonical, re-runnable **attestation receipt** plus the exact `giveFeedback` call parameters, so
Probatio can write an on-chain, agent-identity-tied re-execution verdict to Solana's **live,
permissionless Reputation Registry** (Path A of `docs/GTM-agent-registry.md`). **Offline-only this task**
— the actual devnet send (TS `8004-solana` SDK) is deferred to task 022, so no test touches the network.

## Why

The Solana Agent Registry's Validation module is archived, but its **Reputation Registry is live and
permissionless** (`giveFeedback(agentAsset, { value, tag1, feedbackUri })`, mainnet program
`8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`, npm `8004-solana`; agents are Metaplex Core NFTs,
agentId = asset pubkey). So Probatio can attest **today** — filed under Reputation, not Validation. The
verdict content and the call args are the same shape as ERC-8004's `validationResponse` (`response` 0–100,
`responseURI`), so this receipt is cross-registry by construction. This task builds the deterministic,
offline-testable receipt + call args; task 022 sends it.

## Scope — new `crates/harness/src/attest.rs`

Types + one function, dependency-light (reuse `spec_hash`; no network, no clock inside logic):

```rust
pub struct Reproduce { pub policy: String, pub backend: String, pub n_slots: u64 } // how a 3rd party re-runs it
pub struct FeedbackCall { pub agent_asset: [u8;32], pub value: u8 /*0..=100*/, pub tag: String, pub feedback_uri: String }
pub struct Attestation { pub receipt_json: String, pub call: FeedbackCall }

pub fn attest(
    agent_asset: [u8;32],
    spec: &probatio_contract::MandateSpec,
    report: &crate::verifier::ShortcutReport,
    reproduce: Reproduce,
    feedback_uri: String,   // where the receipt_json will be pinned (ipfs://… / https://…); caller supplies
    captured_at: u64,       // epoch secs, INJECTED (never read from the clock here — mirrors task 018)
) -> Attestation
```

Behaviour:
- `value` = **100 if `report.verdict == Pass`, else 0** (binary today; graded severity is future).
- `tag` = `"re-exec"`.
- `receipt_json` — canonical, dependency-free JSON (reuse the `ShortcutReport::to_json` style), schema
  `"probatio.attestation.v1"`, containing: `agent` (base58 of `agent_asset`), `mandate_spec_hash`
  (hex of `spec_hash(spec)`), `reproduce` (policy/backend/n_slots — enough for a third party to re-run
  `run_episode` + `verify` and reproduce the verdict), `verdict` (`"PASS"`/`"FLAG"`), `findings`
  (kinds + evidence_slots), `report_hash` (FNV-1a over `report.to_json()`, same identity-tag convention
  as `spec_hash`, with the same `TODO(reexec-core)` keccak note), and `captured_at`.
- Deterministic: same inputs ⇒ byte-identical `receipt_json`.

**CLI:** extend the harness binary with `--attest <agent_asset_base58> [--feedback-uri <uri>]` on the
existing certify path: after producing the `ShortcutReport`, print the `receipt_json` and the
`FeedbackCall` (agent, value, tag, feedback_uri) as JSON to stdout, so task 022 (or a manual step) can
pin the receipt and send `giveFeedback`. No network, no signing here.

## Honesty / safety

- **Offline-only.** No RPC, no keypair, no send in this task or any test. `captured_at` is injected.
- `report_hash` is an **identity tag** (FNV), not a security hash — say so, same as `spec_hash`.
- The receipt is an **attestation by Probatio**, not a claim by the agent operator — keep the same
  unsolicited-DD honesty stance as the live Jupiter path (a FLAG means "did not satisfy the mandate
  Probatio checked", never "the operator lied").

## Files to touch

```
crates/harness/src/attest.rs   NEW — Reproduce/FeedbackCall/Attestation + attest() + canonical JSON + tests
crates/harness/src/lib.rs      pub mod attest; re-export attest/Attestation/FeedbackCall/Reproduce
crates/harness/src/main.rs     --attest <asset> [--feedback-uri <uri>] on the certify path: print receipt + call
reviews/021-attestation-receipt.md   CC review verdict
```

## Acceptance criteria / gates

- `cargo test` green (all existing + new). New tests: PASS report ⇒ `value == 100`, receipt `"verdict":"PASS"`,
  contains the `mandate_spec_hash` hex and `agent` base58; FLAG report ⇒ `value == 0`, `"verdict":"FLAG"`,
  findings serialized; `receipt_json` is **byte-identical** for identical inputs (determinism);
  `report_hash` changes when the report changes and is stable when it doesn't.
- No test hits the network or reads the clock; `captured_at` injected. `cargo build` clean, no new warnings;
  on-chain crates still `build-sbf` clean; contract crate untouched.

## Out of scope (task 022 / follow-ups)

- The actual devnet send: a thin TS script using the `8004-solana` SDK to pin `receipt_json` and call
  `giveFeedback(agentAsset, { value, tag1:"re-exec", feedbackUri })`, plus a `--live` manual path.
- ERC-8004 `validationResponse` EVM emitter (same `value`/`responseURI` shape — later, Reckn/EVM lane).
- Graded (non-binary) `value` from finding severity; pinning/IPFS automation.
