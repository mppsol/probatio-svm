# H4 — Upgrade Behavior Sentinel — **KILLED at G0 (2026-08-22) — KILL-2 + KILL-3**

> ⛔ **This gate is closed. H4 is `KILLED`** — verdict, numbers and reasoning:
> [`docs/decisions/H4-sentinel-kill.md`](./decisions/H4-sentinel-kill.md). Independent review:
> [`reviews/H4-G0-sentinel.md`](../reviews/H4-G0-sentinel.md) — Codex, `MEASUREMENT SOUND`, no P0.
>
> **The measurement in §3–§4 was executed.** All four cases returned `unknown` (**KILL-2**), so the
> integration action is `RE-VERIFY` (**KILL-3**) — the only answer the differing code hash could
> already give. `KILL-1` and `KILL-5` explicitly did **not** fire: both real binaries loaded, all
> eight executions ran, and three consecutive runs were byte-identical. In every case the two
> binaries agreed on `result` and on all three `token_deltas`; the sole divergence is 4 bytes the
> new binary writes at offset 28 of `obligation`, `reserve_sol` and `reserve_usdc`. Calling those
> bytes immaterial would be `KILL-4`, which is why the rescue and the kill are the same door.
>
> **Nothing below is rewritten.** It is kept as the pre-registration the verdict is read against, and
> as a reproducible refutation asset. A verdict ends the phase; it does not start the next one.

## The pre-registration, as committed before measuring


**Status: G0 pre-registration. Written and committed BEFORE any measurement.**
Founder ruling, 2026-08-21. This is the binding gate; `STATUS.md` records the phase.

> **The closed gates are not reopened by this one.**
> H1 `KILLED` ([`docs/GATE.md`](./GATE.md), [`decisions/P1-real-target.md`](./decisions/P1-real-target.md)) ·
> H2 `FROZEN UNEXECUTED` ([`docs/H2-GATE.md`](./H2-GATE.md)) ·
> H3 `KILLED at the design gate` ([`docs/H3-GATE.md`](./H3-GATE.md),
> [`decisions/H3-design-gate-kill.md`](./decisions/H3-design-gate-kill.md)).
> No verdict above is rewritten, resumed, or worked around. H4 starts from H3's refutation as input.

Rules carried forward unchanged: a verdict is a **number or a reproducible experiment**;
**not-proven is a KILL**; pre-registered parameters are not changed after measurement; every number
carries a **slot**; every discrepancy is reported **with its direction**, in both directions; at most
two agents, and **no model reviews its own output**. Forbidden without exception: real funds, mainnet
or devnet deploy, force push, committed secrets.

---

## 1. Forbidden by name — what H3's kill makes off-limits

H3 died because capability semantics could not be reduced to a protocol-independent schema; the
freedom to choose which effect satisfies a field was relocated, not removed. H4 is therefore
**forbidden**, without exception, from:

- a **general-purpose Capability Passport**, or any artifact intended to describe more than the one
  integration named in §3;
- a **protocol-independent schema** — any field definition intended to be filled in for a second
  protocol, and any "adapter" that supplies venue meaning to a generic rule;
- **cross-protocol scoring, grading, comparison or ranking** of any kind;
- reviving H1 artifacts (verifier, attestation, `MandateSpec`, guard, gallery, `attest/`, the agent
  framing) or resuming H2;
- modifying the sibling repo `../solvo` in any way. **Read-only, without exception** (KILL-5).

If closing a gap in H4 requires any of the above, **that is KILL-4** — not a scope change.

## 2. The hypothesis

> **For one named Solana integration, the concrete behaviour that integration depends on can be
> re-executed against the real pre-upgrade and post-upgrade BPF binaries, on identical cloned state
> with identical inputs, and yield a `compatible` / `breaking` / `unknown` verdict that a byte hash
> comparison cannot produce.**

**Why this is not H3, structurally.** H3 needed to know *what a behaviour means* ("cannot pay",
"a promise", "admin-reachable") and every such meaning turned out to be venue-specific. H4 needs no
meaning at all: it holds the input, the state and the case fixed, varies **only the binary**, and
compares the two outputs **for equality**. Equality is not an interpretation. That is the whole
structural claim, and KILL-4 exists to check that it survives contact with a real measurement.

**What a byte hash gives you today:** klend's `code_hash` changed from `8eab9f85…3d1cda` to
`b1344d19…9c22d9`. That difference is true, cheap, and says *nothing* about whether the integration
still works. The hash-only baseline can only answer **"re-verify everything"**. H4 claims to answer
`CONTINUE` or `ISOLATE` instead, from evidence.

## 3. The fixed target — frozen before measurement

**Integration.** One concrete operation `../solvo` depends on: **Kamino klend
`withdraw_obligation_collateral_and_redeem_reserve_collateral_v2`**, invoked **top-level** by the
wallet that owns the obligation, after klend's mandatory same-slot refresh preamble
(`refresh_reserve` × 2, `refresh_obligation`), against the **USDC** reserve of the cloned obligation.
Anchor discriminators are computed as `sha256("global:<snake_case_name>")[0..8]` **in the harness**
and asserted, never hand-copied.

Solvo's Leg A established this exact path executes (`solvo/docs/specs/012-g1-leg-a-kamino.md`); H4
drops its SBF probe because H4 does not ask a CPI-authority question. **No program of our own is
built, compiled or deployed.**

**Old binary.** `../solvo/fixtures/g1/programs/klend.so.gz`, committed there, copied read-only.
Stripped of trailing `0x00`: **2,414,913 bytes**, `sha256`
`8eab9f858da01fee962452ad3838570b3340cd43b80a236de8fe5297063d1cda`. Fixture slot **440,477,781**.

**New binary.** Fetched from mainnet **after this document is committed and pushed**, by
`getAccountInfo` on klend's ProgramData account, recording **ProgramData pubkey, the account's
context slot, `last_deploy_slot`, `upgrade_authority`, `elf_len`, and `code_hash`** (the `sha256` of
`data[45..]` with trailing `0x00` removed — the loader-v3 layout independently confirmed in H3's
review). Expected, and to be re-checked rather than assumed: ProgramData
`9uSbGW1y9H5Av6H5TKxQ1wnFApSq2t3oEpfF2YfjDQGA`, deploy slot 440,486,775, `code_hash` `b1344d19…`.
**If klend has been redeployed again since, the newest binary is the new binary** and the recorded
slot/hash say so.

**State.** One clone set, used byte-identically by both runs: Solvo's committed `fixtures/g1`
accounts at slot 440,477,781, copied read-only into `fixtures/h4/` with a `MANIFEST.json` recording
`pubkey, owner, len, sha256` per account plus the provenance of every file.

**Cases — four, fixed here.** `D` = the cloned obligation's USDC `deposited_amount`, read from the
fixture and **printed by the run**; it is not peeked at while writing this document.

| case | input | class |
|---|---|---|
| **A** | withdraw `100,000,000` collateral atoms | success (the amount Solvo's A2 moved) |
| **B** | withdraw exactly `D` | threshold boundary |
| **C** | withdraw `D + 1` | failure (over the boundary by one atom) |
| **E** | withdraw `u64::MAX` | sentinel value — the case most likely to be special-cased in code |

Each case runs **twice**: once on the old binary, once on the new one, from the same initial state,
with the same instruction bytes and the same signer. Nothing else varies.

**Outputs, per case per binary.** Decisive:

1. **`result`** — `Ok`, or `Err` with the exact `TransactionError` / `InstructionError` (including
   the numeric `Custom` code).
2. **`token_deltas`** — `amount` (SPL Token layout, offset 64, `u64` LE) before and after, for the
   destination token account, the reserve's liquidity supply vault, and the collateral supply vault.
3. **`state_equal`** — whether the **full post-transaction bytes of every account in the fixture**
   are identical between the two runs, with every differing `(account, offset, len)` range listed.

Auxiliary, **never decisive**: program logs, compute units, `return_data`. Logs are printed and are
**never asserted on** — a rule carried from Solvo's harness discipline.

## 4. The verdict rule — fixed before measurement

Per case, comparing old against new:

| verdict | condition |
|---|---|
| **`compatible`** | `result` equal (same `Ok`, or same error variant *and* same code) **and** `token_deltas` equal **and** `state_equal` |
| **`breaking`** | `result` differs, **or** any `token_delta` differs |
| **`unknown`** | `result` and `token_deltas` are equal but `state_equal` is false; **or** the case could not be executed identically on both binaries; **or** two runs of the same case on the same binary are not byte-identical |

**`unknown` on a state-only difference is deliberate and cuts against H4.** Deciding whether a
changed byte matters would require exactly the venue semantics H3 died of (KILL-4). Not-proven is a
KILL, so the honest verdict is `unknown` — and if that dominates, H4 fails on its own rule.

**Integration action**, from the four case verdicts:

| all four | action |
|---|---|
| all `compatible` | **`CONTINUE`** — the integration may proceed on the new binary |
| ≥1 `breaking`, none `unknown` | **`ISOLATE`** — stop the integration until re-audited, with the failing case named |
| ≥1 `unknown` | **`RE-VERIFY`** — H4 could not decide |

## 5. Kill conditions — the founder's five, operationalised

| # | condition | fires when |
|---|---|---|
| **KILL-1** | the two real binaries cannot be run in one reproducible environment | either binary fails to load, or a case cannot be executed on both, or the harness is not offline-reproducible after fixtures are committed, or two runs are not byte-identical |
| **KILL-2** | every case is `unknown` | 4 of 4 `unknown` |
| **KILL-3** | the result adds nothing to the hash difference | the integration action is **`RE-VERIFY`** — the hash-only baseline's only possible answer. H4 must reach `CONTINUE` or `ISOLATE`, with evidence, to have beaten it |
| **KILL-4** | deciding requires a protocol-independent schema or arbitrary adapter semantics | any verdict above needs a rule that generalises beyond this one operation, or needs a meaning assigned to a klend field beyond reading its bytes |
| **KILL-5** | `../solvo` must be changed to do this | any write, of any kind, to that repo |

**Reading of KILL-3, stated because the wording admits two.** Literally, *"the hash difference alone
cannot decide continue/isolate/re-verify"* is H4's premise, not a failure. The reading used here is
the one consistent with the hypothesis: **H4 fails if its output is no more actionable than the hash
difference already was.** Since a differing hash can only ever yield `RE-VERIFY`, KILL-3 fires
exactly when H4 also lands on `RE-VERIFY`. Recorded here so the choice cannot be made after seeing
the result; if the founder meant the other reading, this document is wrong and should be corrected
**before** the measurement runs.

## 6. Harness requirements

Evidence tooling, not product. One crate, `crates/h4-sentinel`, with its own `[workspace]` table and
a root `exclude` entry — a second copy of the unbundled solana crates alongside litesvm breaks the
build (Solvo decision 000 §3.4). Dependency set copied from Solvo's proven one, **including the
supply-chain pin `arrayref = "=0.3.9"`, which is not to be relaxed or removed for looking unused**
(`arrayref 0.3.10` pulls a typosquat whose `build.rs` downloads and executes a binary; it executed on
this machine on 2026-08-20 — Solvo decision 004 §6).

1. **Both binaries are hash-asserted before execution.** The harness recomputes `sha256` of each
   loaded ELF and aborts unless it equals the recorded `code_hash`. The binary tested and the binary
   named in the evidence are then the same bytes by construction.
2. **Identical initial state.** Both runs start from the same fixture bytes. The VM is rebuilt from
   the fixtures for every run; state never carries across cases or binaries.
3. **Disclosed mutations.** Every byte of cloned state the harness writes — repointing the
   obligation's `owner` to the harness signer, creating the destination token account — is printed by
   the run itself: account, offset, len, before, after, and why. **A mutation not printed invalidates
   the run**, and both binaries receive byte-identical mutations.
4. **Assertions read account bytes out of the VM. No assertion may read a log line.**
5. **Determinism.** Two invocations produce byte-identical output, including the emitted JSON.
6. **Offline.** Once fixtures are committed, the run touches no network. Fetching is a separate,
   explicitly invoked step.
7. **Output.** `evidence/h4-sentinel.json` — both binaries' identity blocks, the four cases, every
   decisive output, every disclosed mutation — plus a human-readable table.
8. **No SBF program is written or compiled.** No `cargo build-sbf`, no probe, no deploy.

## 7. Predictions, recorded before measurement

Wrong predictions are **not** a kill; they are the evidence the instrument tells us something we did
not already know. Refusing to record them would be.

| case | prediction (old vs new) | confidence |
|---|---|---|
| A | `compatible` — same `Ok`, same deltas | medium |
| B | **open** — the boundary is where a rewrite most often moves | — |
| C | `compatible` — same `Err`, same code | medium |
| E | **open** — `u64::MAX` is the likeliest special case | — |
| **action** | **`CONTINUE`**, i.e. all four `compatible` | low–medium |

A `CONTINUE` on a *changed hash* would be H4's most useful possible result, and its least dramatic:
it is precisely the answer the hash cannot give. An `ISOLATE` would be more striking and no more
valid. Both beat `RE-VERIFY`.

## 8. Two ambiguities in the brief, resolved and flagged

1. **"Gate 0 では実装を行わないでください" vs "commit/push してから測定してください".** A measurement of two
   BPF binaries cannot happen without executable tooling. Resolution used: **"実装" means the product**
   — on-chain programs, UI, token, deployment, and any artifact intended to outlive this measurement.
   The sentinel harness is **evidence tooling**: one excluded crate, no program of our own, no
   product surface, no deploy. If the founder meant that no code at all may be written, then the
   measurement cannot be performed and H4 stops at this document.
2. **KILL-3's two readings** — resolved in §5, before measuring.

## 9. What G0 does not authorise

`G0` produces **this document and one measurement**, and nothing else. It does not start Gate 1, it
does not authorise implementation, generalisation to a second operation or a second protocol, a
schema, a product, or any deployment. **A `PASS` here ends the phase; it does not start the next
one.** On any outcome: update `STATUS.md`, commit the evidence and the decision record, push, and
hand back to the founder.
