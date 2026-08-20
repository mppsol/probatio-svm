# P1 — "A real target" — **KILLED**

**Date:** 2026-08-20 · **Chain snapshot:** mainnet registry slot 440479435 / Jupiter slot 440479436
**Produced by:** Claude (spec/evidence role) · **Independently reviewed by:** Codex —
[`reviews/P1-real-target.md`](../../reviews/P1-real-target.md) (r1 `CHANGES`, one P0 fixed below)

> **P1 (docs/GATE.md):** *A real target — an actual on-chain agent or agent vault, not a fixture we
> wrote.* Verdict must come from **its address, and a run against it**.

**Kill condition stated before the run:** if no address on Solana can be named that is an actual
autonomous agent or agent vault **and** whose on-chain state this harness can certify — i.e. every
target we can run against is either a fixture we wrote or a wallet we cannot show is agent-operated
— then P1 is not proven, and per `docs/GATE.md` **not-proven is a KILL**.

That is what the chain says.

---

## Reproduce

```
python3 docs/decisions/repro/p1_real_target.py            # ~60s, stdlib only, ~450 MB temp
cargo build --bins
./target/debug/probatio-svm certify-jupiter --live DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw
```

The script reads two populations off mainnet and intersects them. It derives nothing from this repo's
code except the constants it re-declares from `crates/harness/src/jupiter.rs` (the `Position` byte
offsets and `DELTA_UNIT_USD`), so a third party can re-run it against the chain without trusting the
harness. The `AgentAccount` layout it walks comes from the registry's own published schema
(`8004-solana@0.8.3`, `dist/core/borsh-schemas.js`), not from our reading of the bytes.

---

## Finding 1 — no registered on-chain agent has state this harness can certify

The Solana Agent Registry is the repo's own named channel (`docs/GTM-agent-registry.md`) and the only
on-chain source of truth for "this address is an agent". It is live on mainnet at
`8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`.

| | slot 440479435 |
|---|---|
| program account sizes | `{748: 1471, 332: 36, 73: 2}` |
| `AgentAccount` records (748 B) | **1,471** |
| …carrying an operational `agent_wallet` | 50 |
| distinct agent identities (`creator`, `owner`, `asset`, `agent_wallet`, `parent_asset`) | **1,767** |
| …that have **ever** held a Jupiter Perps `Position` account | **6** |
| …that have an **open** position right now | **0** |

The 36 × 332-byte accounts are metadata entries (asset pubkey + tagged strings, no operator address);
the 2 × 73-byte accounts are registry config (base collection + authority). Adding both config
pubkeys to the intersection also yields **0**.

The harness's only live ingestion path is Jupiter Perps `Position` accounts (`fetch_owner_positions`;
every other `certify-jupiter` input is a sample or a supplied trace). So the set of registered on-chain
agents this tool can run against is **empty**.

Running the tool on the best of the 6 candidates:

```
$ ./target/debug/probatio-svm certify-jupiter --live DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw
DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw has no open Jupiter positions — nothing to certify.
```

And that address is not an agent either: it is a mass-**registrar** wallet (vanity prefix `Dev`) that
registered ~50 of the 1,471 agents. Its Jupiter history is a human developer trading perps. The other
five are individual registrant wallets, likewise closed.

## Finding 2 — no agent *vault* holds live open interest either

P1 allows an agent **vault** as an alternative target. Of the 4,708 addresses holding an open Jupiter
Perps position at slot 440479436:

| holder kind | count | open notional | share |
|---|---:|---:|---:|
| plain wallets (System Program) | 4,366 | $62,159,087 | 96.4% |
| accounts that no longer exist | 337 | $2,313,740 | 3.6% |
| **program-controlled** | **5** | **$32,423** | **0.05%** |
| total | 4,708 | $64,505,250 | |

The 337 nonexistent accounts are the interesting case: a rent-collected vault PDA would look exactly
like this. It is decidable without the account, because **a PDA is by construction off the Ed25519
curve**. Decompressing all 337 keys: **0 are off-curve.** Every one is an ordinary wallet whose account
has been emptied — not a single vault PDA among them. (The check is in the script; it accepts 50.4% of
random 32-byte inputs and correctly rejects a real Jupiter `Position` PDA.)

That leaves 5 program-controlled holders, all zero-data authority accounts, none linkable to the
registry or identifiable as an agent-vault program, the largest holding $29,897. **99.95% of the live
open interest this harness can read sits in plain wallets indistinguishable on-chain from human
traders.** There is no agent-attributable population to certify.

## Finding 3 — the run the repo already had is not a P1 run, and its PASS is a rounding artifact

`gallery/jupiter-live-AhUvhrHH.json` was the project's headline live PASS. Recomputing the verdict
**from its definition** rather than from the card:

```rust
// crates/harness/src/jupiter.rs
pub const DELTA_UNIT_USD: i64 = 100;
fn delta_units(usd: i64) -> i64 { (usd + usd.signum() * (DELTA_UNIT_USD / 2)) / DELTA_UNIT_USD }
```

Rust integer division truncates toward zero, so this is zero exactly when `|usd| < 50`: PASS ⟺
**`|net signed notional| < $50`**. `AhUvhrHHZXh7Huu8AtCfEkTvgUdTw4ZwL1j1c6Fu5dfq`'s entire position is
**$16** (fixture bytes and live query agree: `side@152=1`, `price@153=$143`, `sizeUsd@161=$16`,
`collateralUsd@169=$15`). It does not pass because it is delta-neutral; it passes because it is **too
small to measure**. Across the whole live population:

| | count | share |
|---|---:|---:|
| PASS | 793 / 4,708 | 16.84% |
| …of which gross position < $50 (below the rounding band) | **780** | **98.4% of the PASS class** |
| …of which genuinely hedged (gross ≥ $50, net < $50) | 13 | 1.6% |
| FLAG | 3,915 / 4,708 | 83.16% |

Applied to the real population the certification separates approximately *"do you have more than $50
at risk?"* — and the repo's one committed PASS is on the dust side of that line. Three further runs
against real, large targets (`GykwUwgC…` $127k, `8Z98LMVG…` $2.27M, `68jj5xZc…` $29.9k, the largest
program-controlled holder) all returned the identical three findings — the FLAG is effectively
unconditional on anything with a real position.

---

## Adversarial recomputation — direction of every discrepancy

`docs/GATE.md`: *"Every numeric error found in this portfolio so far has favoured the project that
produced it."*

**One numeric error was found in this document, by Codex, and it did not run that way.** Revision r1
decoded `AgentAccount` at fixed offsets `40/72/104/136`. The registry's published Borsh schema has two
`Option<Pubkey>` fields that shift every later field, so the record cannot be read at fixed offsets at
all. The consequence was **non-monotone**, and must not be described as a conservative widening:

- offset `136` was not a pubkey — it read `bump`, `atom_enabled` and 30 bytes of digest as one. This
  injected 1,182 non-identity byte strings into the search set, which **favoured the project**.
- the real `agent_wallet` — the *operational* wallet, 50 records, 41 distinct — was **omitted**. That
  is precisely the field most likely to hold positions, and dropping it **favoured the KILL**.

The corrected walk (schema-driven, fails closed on a bad option tag, validated by all three trailing
Borsh strings decoding as UTF-8 in all 1,471 records) gives **1,767** identities, not 2,915. Both
error directions are now resolved and the deciding experiment is unchanged: **6 ever, 0 live.**

| check | result | direction |
|---|---|---|
| `AgentAccount` layout (**P0, fixed**) | 2,915 → **1,767** identities; `agent_wallet` recovered | **non-monotone** — bogus field favoured the project, omitted `agent_wallet` favoured the kill; corrected, verdict unchanged |
| Streaming parse vs. an independent `dataSlice:0` count | 864,474 @ 440472439 vs **864,475** @ 440472771 (+1 over 332 slots) | independent count is *higher* — the parse did not undercount the search space **against** the project |
| Positions with an unreadable `side` byte | **0** of 5,663 open (script now fails closed rather than skipping) | no live position was silently dropped |
| Which positions count | **historical/closed** included, not only open | widened **for** the project; 6 hits, 0 live |
| Search scope | **every** Position account on mainnet, not a sample | maximal **for** the project |
| "Nonexistent account ⇒ a wallet, not a vault" | assumed in r1; now **proved**: 0 of 337 are off-curve | r1's assumption happened to hold, but was unproven and favoured the **kill** |
| The one number that favours the project (the `AhUvhrHH` PASS) | recomputed from `delta_units` — a $16 position below the $50 band | **against** the project |

## Verdict: KILLED

P1 asks for an address that is an agent or agent vault, plus a run against it. We can produce runs
(the tool works, reproducibly, on real mainnet state at a named slot) and we can produce addresses —
but **not both in the same address.** Every target the harness can certify is a plain wallet we cannot
show is agent-operated; every registered agent has nothing for the harness to read.

This is not `BLOCKED`. `BLOCKED` requires an external dependency that *a stated action can resolve on
a stated date*, and there is none: "wait until registered agents trade Jupiter Perps" names no action
and no date. Nor does a new ingestion adapter rescue it — `docs/GATE.md` is explicit that *"it would
work if we also had X" is a KILL, not a new scope.*

It also pre-empts **P4**. The gate's KILLED clause reads *"the differential cannot be shown, or the
adopter is vague."* Here the adopter population is not vague, it is **empirically 0** at this snapshot:
zero registered on-chain agents with certifiable state, and 0.05% of readable open interest under
program control.

### What would have changed this verdict

Any **one** of:

1. ≥1 of the 1,767 registry identities holding an open Jupiter Perps position — the intersection is a
   single number the script prints, and it is 0.
2. A named agent vault among the 5 program-controlled holders — e.g. one of `FziSMN4a…`,
   `syspv6Qe…`, `H67ehFrM…`, `BUyLRaYE…` shown to be an autonomous-agent vault program rather than an
   ordinary protocol account.
3. A PASS/FLAG split that carries information about behaviour rather than position size — i.e. a PASS
   class not 98.4% composed of sub-$50 dust.

Re-running the script on a later slot is the cheapest way to revisit (1) and (3).
