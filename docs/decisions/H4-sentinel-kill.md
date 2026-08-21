# H4 — Upgrade Behavior Sentinel — **KILLED at G0, KILL-2 + KILL-3**

**Date:** 2026-08-22 · **Verdict by:** the pre-registered kill conditions, fired by the measurement
**Measurement produced by:** Codex (harness implementation) · **run and adjudicated by:** Claude
**Independently reviewed by:** Codex — [`reviews/H4-G0-sentinel.md`](../../reviews/H4-G0-sentinel.md),
one round, verdict **`MEASUREMENT SOUND`**, no P0
**Artifacts:** gate `35f8d0e` · fixtures + brief `2d7b5c3` · harness `75f3cb9` · loading correction
`7efbb37` · this commit

> **H4's hypothesis ([`docs/H4-GATE.md`](../H4-GATE.md) §2):** for one named Solana integration, the
> concrete behaviour it depends on can be re-executed against the real pre- and post-upgrade BPF
> binaries, on identical cloned state with identical inputs, and yield a
> `compatible` / `breaking` / `unknown` verdict **that a byte hash comparison cannot produce**.

**This is not a founder ruling on a judgement call.** Unlike H3, H4 pre-registered numeric kill
conditions and then produced the numbers. KILL-2 and KILL-3 are read off the measurement.

## Verdict: KILLED — KILL-2 and KILL-3

The measurement ran. Both real binaries loaded, all eight executions completed, and the run is
deterministic. **All four cases returned `unknown`**, which is KILL-2 exactly; the integration action
that follows from the pre-registered table is therefore **`RE-VERIFY`**, which is KILL-3 exactly.

`RE-VERIFY` is the only answer the differing code hash could already give. **H4 spent a real
measurement to arrive back at its own baseline.**

## The numbers

Fixture slot **440,477,781** · fixture manifest `sha256`
`b5db5e51244b23aedacfc689d71102a842c0e6badaeb9855f63136c42694fc6a` ·
`D` (the cloned obligation's USDC `deposited_amount`, read out of the fixture bytes by the run) =
**2,248,785,777**.

| binary | stored | `elf_len` | `code_hash` |
|---|---:|---:|---|
| `klend_old` (pre-upgrade) | 10,485,715 | 2,414,913 | `8eab9f85…3d1cda` |
| `klend_new` (post-upgrade, ProgramData `9uSbGW1y…DQGA`, deploy slot 440,486,775) | 10,485,715 | 2,431,953 | `b1344d19…9c22d9` |

| case | `collateral_amount` | old `result` | new `result` | `token_deltas` | `state_equal` | **verdict** |
|---|---:|---|---|---|---|---|
| A | 100,000,000 | `Ok` | `Ok` | equal | **false** | `unknown` |
| B | 2,248,785,777 (`D`) | `Err InstructionError(1, Custom(6011))` | same | equal | **false** | `unknown` |
| C | 2,248,785,778 (`D + 1`) | `Err InstructionError(1, Custom(6011))` | same | equal | **false** | `unknown` |
| E | 18,446,744,073,709,551,615 (`u64::MAX`) | `Ok` | `Ok` | equal | **false** | `unknown` |

**4 of 4 `unknown` → KILL-2.** **Action `RE-VERIFY` → KILL-3.**

## Why every case is `unknown`, and why that is the honest answer

In all four cases the two binaries agree on **everything H4 declared decisive except full post-state
byte equality**: same `result` (same `Ok`, or the same error variant *and* the same `Custom(6011)`),
and identical `token_deltas` on all three tracked token accounts — the destination, the USDC
liquidity supply vault, and the USDC collateral supply vault. Case A moves the same
119,647,109 USDC atoms to the destination under both binaries.

The single divergence, in every case, is that the **new binary additionally writes 4 bytes at
offset 28** of `obligation`, `reserve_sol` and `reserve_usdc`, which the old binary leaves at their
fixture value of `00000000`. Nothing else in any of the 17 accounts differs from the fixture in a
range the two binaries do not share.

Gate §4 pre-registered that `result` and `token_deltas` equal with `state_equal` false is `unknown`,
and pre-registered **why**: deciding whether a changed byte *matters* would require exactly the
venue-specific semantics H3 died of, which is KILL-4. That rule was written before the measurement
precisely so it could not be softened after seeing this result — and it is what fires KILL-2 here.

**So H4 could have been rescued only by doing the forbidden thing.** To turn these four `unknown`s
into `compatible` we would have to assert that those 4 bytes are a padding/reserved field whose
change is immaterial. That assertion is a klend-specific semantic judgement about a klend field —
KILL-4 — and generalising it to a rule ("ignore reserved ranges") is a protocol-independent schema,
also KILL-4. **The rescue and the kill are the same door.** H4 fails on its own pre-registered rule,
by the mechanism the rule anticipated.

## The two kills, stated against their pre-registered wording

| # | pre-registered condition | fired? | evidence |
|---|---|---|---|
| **KILL-1** | binaries cannot be run in one reproducible environment, or reruns are not byte-identical | **no** | both ELFs loaded; 8/8 executions `executed: true`; three consecutive runs produced a byte-identical `evidence/h4-sentinel.json` |
| **KILL-2** | every case is `unknown` | **YES** | A, B, C, E all `unknown` |
| **KILL-3** | the integration action is `RE-VERIFY` | **YES** | ≥1 `unknown` → `RE-VERIFY` per gate §4 |
| **KILL-4** | deciding requires a protocol-independent schema or arbitrary adapter semantics | not reached — but it is what *blocks the rescue*, as above | — |
| **KILL-5** | `../solvo` must be changed | **no** | see below |

**KILL-1 is explicitly recorded as not firing.** An earlier run (`75f3cb9`) could load neither
binary: the brief had instructed that the ELFs be stripped of trailing `0x00` before loading, and
both section-header tables end **15 bytes past the last non-zero byte** (old: `e_shoff` 2,414,288 +
10 × 64 = 2,414,928 required against 2,414,913 stripped; new: 2,431,328 + 640 = 2,431,968 against
2,431,953). That was a harness defect, corrected in `7efbb37` and in the loader change committed
here: the **untrimmed** stored payload is loaded, and the trailing-zero-stripped `sha256` is kept as
an **identity only**. The correction is applied identically to both binaries and cannot bias the
old-versus-new comparison in either direction. The completed measurement, not the defective run, is
what KILL-1 is judged on.

**KILL-5 did not fire.** `fixtures/h4/programs/klend_old.so.gz` is byte-identical to
`../solvo/fixtures/g1/programs/klend.so.gz` (`sha256`
`adc2b55ba0dc7001b6ac893e05ff2773087fffbf1faccaa5926adec25df9cb93` on both), i.e. copied, not
edited. `git -C ../solvo status --porcelain` is **not** empty — it reports one untracked file,
`docs/H3-GATE.md`, mtime 2026-08-22 08:26. That file is **Solvo's own** H3 gate ("H3 Gate — Solvo
Contract Tests", citing Solvo decisions 005–009); it was written by a separate Claude Code session
whose working directory is `/Users/hiroyusai/src/solvo`, and no Codex session in this repo
references it. **No write to `../solvo` originated from H4.** It is recorded here rather than
omitted because the brief's acceptance criterion 5 asked for that command's output to be empty and
it is not.

## The review, and what it could not check

One round, as ruled. Codex returned **`MEASUREMENT SOUND`**, concurred with KILL-2 + KILL-3, found
**no P0**, and independently recomputed the binary hashes, the manifest hash, all 17 account hashes,
the evidence hash, and the fixture bytes at offset 28.

**Disclosed conflict:** Codex wrote `crates/h4-sentinel`, including the loader correction committed
here, so this review is partly self-review. The review prompt said so and required independent
re-derivation from the fixtures rather than certification of the source; the review's own disclosure
section names which parts are its own code. CC's mitigation was to re-derive the load-bearing
numbers separately — the three `code_hash`es, the three `e_shoff` computations, and the old
binary's identity with Solvo's committed copy — all of which match.

Two gaps the review could not close read-only, both left open rather than argued away:

1. **P1 — the evidence cannot prove offset 28 is the *only* old/new difference, nor what the new
   bytes are.** `post_state` records, per account, a `sha256` plus the ranges differing **from the
   fixture**, separately for old and new; equal range *lists* do not prove equal range *contents*.
   The harness has deliberately **not** been changed to add a direct old-versus-new byte diff: the
   instrument is frozen after measurement, and the gap cannot affect the verdict — `state_equal` is
   false by the `sha256` mismatch whichever bytes cause it, so every case is `unknown` either way.
   What is not proven is the *characterisation* of the divergence, not the divergence.
2. **P2 — provenance is attested, not reproducible from inside a read-only review.** The reviewer was
   forbidden `../solvo` and the network. CC closed the `../solvo` half directly (the hash equality
   above). The mainnet half — that `klend_new` is the ProgramData fetch it claims to be — rests on
   `fixtures/h4/MANIFEST.json` and `klend_new.identity.json`, plus the fact that the gate named the
   expected hash `b1344d19…` **before** the fetch and the committed bytes match it.

## What this does and does not settle

**Settled:** re-executing one fixed operation against two real binaries on identical cloned state is
**buildable, offline, and deterministic** — that part worked. It is the *verdict* that failed: with
byte-equality as the only interpretation-free comparison available, any real upgrade that touches
reserved bytes lands on `unknown`, and `unknown` is worth exactly what the hash was worth.

**Not settled, and not to be claimed:** nothing here says an upgrade sentinel is impossible. It says
**this** one, decided **this** way, is not more actionable than a hash. Reaching `CONTINUE` needs a
rule for which state differences are ignorable, and every such rule available to us is KILL-4.

**Not-proven is a KILL.** H4 is closed. Per gate §9, a verdict ends the phase and does not start the
next one.

## Carried forward as input, not as a live hypothesis

- Byte-equality across two binaries is **decidable but not actionable**. A useful verdict needs a
  notion of which differences matter, and that notion is venue-specific — the same wall H3 hit,
  reached from the opposite direction. H3 needed meaning up front; H4 avoided meaning entirely and
  found it needed meaning at the end.
- The fixture/harness *mechanics* are sound and reusable as fixtures: real BPF, cloned mainnet state,
  disclosed mutations, deterministic replay, no network. **Reusing the mechanics is not reusing the
  hypothesis**, and nothing in H4 may be revived as a foundation on the grounds that it exists.
