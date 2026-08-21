# H3 — Composability Passport — **KILLED at the design gate**

**Date:** 2026-08-21 · **Verdict by:** founder ruling on the reviewed G0
**Produced by:** Claude (spec/evidence role) · **Independently reviewed by:** Codex —
[r1 `CHANGES`](../../reviews/H3-G0-passport.md) (8 × P0) →
[r2 `CHANGES`](../../reviews/H3-G0-passport-r2.md) (6 × P0, **the same class as r1's**)
**Artifacts:** G0 r1 `4ef5efd` · review r1 `f5cb3ec` · G0 r2 `8d92bf1` · review r2 `91bb712`

> **H3's hypothesis ([`docs/H3-GATE.md`](../H3-GATE.md) §1):** a protocol's claim to be "composable"
> or "CPI-able" decomposes into a small **fixed set of capability fields**, each decided by an
> executable probe against that protocol's real mainnet binary and cloned mainnet state.

**What was pre-registered, exactly:** §13 of the gate stated that H3 **would not go to r3**, and that
if the r2 review returned `CHANGES` with P0s of the same class — deciding semantics still deferred —
that is the finding, it is reported to the founder as-is, and the phase stops. It did. The phase
stopped. **The `KILLED` verdict itself is a founder ruling on that finding, not a pre-registered kill
number**, and this record does not pretend otherwise.

## Verdict: KILLED

**What died is the fixed, protocol-independent capability schema — not the harness idea it was
attached to.** Two independent review rounds established that the capability semantics could not be
reduced to venue-agnostic field definitions. r2 did not remove the implementer's freedom to choose
which effect satisfies a field; it **relocated** that freedom into the adapter manifest, and the
reviewer showed for each relocation how a venue can be made to look better or worse than it is:

| where the freedom moved | what it lets an implementer do |
|---|---|
| `AdapterManifest.scarcity_range` + C3's boundary sweep | prove that *some* gate binds at the request amount — a withdrawal cap or policy limit satisfies all four sweep runs while the venue holds ample liquidity; and a genuine liquidity field whose boundary sits at `r + fee` is wrongly rejected |
| `AdapterManifest.value_out_ix`'s account list (C6's "universe") | the candidate universe is enumerable only *after* a value-out route is chosen, so a small route returns `NOT_FOUND_IN_UNIVERSE` while an admin-controlled alternate route disables the capability |
| the settlement-path selection (C4) and the writer/access-control selection (C6) | neither is a manifest member at all, which contradicts §2's claim that venue knowledge enters "only" through the manifest |

Two further P0s were structural rather than semantic, and point the same way: **C7 is a property of a
*pair* of protocols stored in a *per-venue* Passport**, so nothing in E1–E4 expires it when the
reference venue upgrades; and **G1b — the adopter test — can be satisfied by two courteous form
letters**, which is H1's asserted adopter in written form.

**The consequence, which is the actual finding.** If capability semantics are irreducibly
venue-specific, then the deliverable is a **report per protocol, not a schema** — the exact thing the
brief's item 5 asked H3 to prove it was not. A per-protocol report is not comparable across
protocols, so `agent / router / vault` cannot make an adoption decision from it as a primitive.

**Not rescued by more specification.** Closing the six P0s means pinning per-venue semantics for
every field, for every venue — which is the H2 failure mode (three rounds, 331 lines, zero
measurements) arriving one layer down. `docs/GATE.md` names it: *"a specification reached 3,000 lines
and three review rounds before anyone asked whether the thing it specified could be built at all."*

**Not rescued by running G1a.** The counted-surface item was ready to run (~51,000 RPC reads, ~1 h)
and was **deliberately not run**: a consumer count answers "does anyone compose across venues", which
does not repair a schema that cannot be specified. Running it would have produced a number that
decided nothing.

## What was measured, read-only, and stands

These were gathered to *identify subjects*, not to decide any gate item, and they survive the kill.
Every one carries its slot.

| fact | value |
|---|---|
| venues whose identity was verified at slot 440,578,912 | 7 (program id, loader, ProgramData, deploy slot, upgrade authority, elf length, `code_hash`) |
| of those, immutable | **0 of 7** — every one can be replaced by a single transaction from a single key |
| klend and Kamino Vaults upgrade authority | **identical** (`GzFgdRJXmawPhGeBsyRCDLx4jAKPsvbUqoqitzppkzkW`) |
| `../solvo`'s klend fixture slot | 440,477,781 (`max_response_context_slot` 440,477,778) |
| klend's redeploy after that capture | slot 440,486,775 — **8,994 slots, ≈ 1 hour later** |
| captured ELF vs mainnet ELF | 2,414,913 B / `8eab9f85…3d1cda` vs 2,431,953 B / `b1344d19…9c22d9` |
| Phoenix Eternal transaction rate | **≈ 199.8 sig/s** (klend 1.14, Jupiter Perps 0.76, marginfi 0.38, Kamino Vaults 0.22, Save 0.012, Drift v2 0.0013) |

**Verified by one party only.** Codex's sandbox could not resolve the RPC hostname in **either**
round and correctly declined to assume the identity table held. It re-derived from local files what
it could: the klend fixture slot, the 8,994-slot gap, the 2,414,913-byte / `8eab9f85…` hash, and that
Solvo's Phoenix leg ran on a **freshly initialised localnet fixture, not cloned mainnet state**.

## What was NOT executed

- **No code.** No probe program, no harness, no fixture fetched, no adapter manifest filled.
- **No gate item.** G1a, G1b, G2, G3, G4, G5 — none started. No capability field was ever measured on
  any protocol. **This kill is therefore about the specification, not about klend or Phoenix**, and
  nothing here says anything about how either venue behaves.
- **No third party contacted.** G1b was pre-registered and never performed.
- **`../solvo` was not edited** — read-only reference throughout.
- **No r3.**

## What would have changed this verdict

A field definition that is decided by effects observable in any transaction **and** that an
implementer cannot satisfy with the wrong effect — surviving independent review without relocating
the choice into a per-venue artifact. Concretely, any one of:

- a C3 admissibility test that distinguishes *"cannot pay"* from *"some gate binds at r"* without
  naming a venue-specific field;
- a C4 test that detects a promise regardless of who may redeem it and when, without a per-venue
  settlement state machine;
- a C6 candidate universe that does not depend on which value-out route was chosen.

None was produced in two rounds. **Not-proven is a KILL**, and adding a hypothesis to stay alive is
forbidden.

## What survives, and what is deliberately deferred

**Survives as reproducible refutation assets** (the four commits are kept, not reverted):

- **E1–E4** — identity and expiry bound to real bytes: the ProgramData linkage procedure, loader tags,
  `Option<Pubkey>` semantics, the `code_hash` / `programdata_hash` / `elf_len` triple, and the 12-byte
  `last_deploy_slot` pre-check. Confirmed closed by the reviewer.
- **G4's deployment counting** — only decoded loader deployment instructions count as upgrades;
  `SetAuthority` is a separate event; pagination gaps fail closed.
- **The harness discipline** — mainnet binary hash-asserted before execution, cloned mainnet state
  only, disclosed mutations printed by the run, no assertion on logs, byte-identical reruns.
- **The measured facts above**, and the observation that motivated the whole hypothesis: a carefully
  executed, independently recomputed capability verdict was bound to a binary that had already been
  replaced before the verdict was written down.

**Founder note, recorded as a deferral and NOT a new phase (2026-08-21):** what died is the
*general-purpose, comparable* Passport. The strong remaining asset is re-runnable verification tied
to a real binary, real state and a real upgrade. If anything moves next, it should be a **narrow
evidentiary product that reconstructs a specific failure or incident after the fact**, not a
general-purpose scorecard. **This is not started.** It has no gate, no hypothesis statement, no kill
number, and no authorisation. Per `docs/H3-GATE.md` §12 and the standing rule, a verdict ends a
phase; it does not start the next one. A successor requires its own G0.

**Probatio as a whole is not killed by this record.** H1 is `KILLED`, H2 is `FROZEN UNEXECUTED`, H3 is
`KILLED at the design gate`. No hypothesis is currently live, and nothing may be built until a founder
ruling opens a new gate.

## Reproduce

Nothing to run — no measurement was executed. The refutation is the review trail:

```
git show 4ef5efd    # G0 r1
git show f5cb3ec    # Codex r1 review, verbatim: CHANGES, 8 x P0
git show 8d92bf1    # G0 r2, closing all eight
git show 91bb712    # Codex r2 review, verbatim: CHANGES, 6 x P0, same class
```

The read-only identity facts are re-derivable from `docs/H3-GATE.md` §3, which records the exact
`getAccountInfo` procedure and every fail-closed condition.
