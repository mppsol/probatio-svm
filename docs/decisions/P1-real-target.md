# P1 — "A real target" — **KILLED**

**Date:** 2026-08-20 · **Chain snapshot:** mainnet slots 440472264–440473521 · **Adjudicated by:** Claude (spec/evidence role)

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
python3 docs/decisions/repro/p1_real_target.py            # ~75s, stdlib only, ~450 MB temp
cargo build --bins
./target/debug/probatio-svm certify-jupiter --live DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw
```

The script reads two populations off mainnet and intersects them. It derives nothing from this
repo's code except the two constants it re-declares from `crates/harness/src/jupiter.rs`
(the `Position` byte offsets and `DELTA_UNIT_USD`), so a third party can re-run it against the
chain without trusting the harness.

---

## Finding 1 — no registered on-chain agent has state this harness can certify

The Solana Agent Registry is the repo's own named channel (`docs/GTM-agent-registry.md`) and the only
on-chain source of truth for "this address is an agent". It is live on mainnet at
`8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ`.

| | slot 440473519 |
|---|---|
| registered agent records | **1,471** |
| distinct pubkeys they reference (owner, authority, asset id, signer) | **2,915** |
| …that have **ever** held a Jupiter Perps `Position` account | **6** |
| …that have an **open** position right now | **0** |

The harness's only live ingestion path is Jupiter Perps `Position` accounts. So the set of
registered on-chain agents this tool can run against is **empty**.

Running the tool on the best of the 6 candidates:

```
$ ./target/debug/probatio-svm certify-jupiter --live DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw
DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw has no open Jupiter positions — nothing to certify.
```

And that address is not an agent either: it is a mass-**registrar** wallet (vanity prefix `Dev`) that
registered ~50 of the 1,471 agents. Its Jupiter history is a human developer trading perps. The other
five are individual registrant wallets, likewise closed.

## Finding 2 — no agent *vault* holds live open interest either

P1 allows an agent **vault** as an alternative target. A vault is capital under program control, so it
is separable on-chain: its position owner is a PDA, not a System-Program wallet. Of the 4,689 addresses
holding an open Jupiter Perps position at slot 440473521:

| holder kind | count | open notional | share |
|---|---:|---:|---:|
| plain wallets (System Program) | 4,350 | $63,154,199 | 96.4% |
| accounts that no longer exist | 334 | $2,307,825 | 3.5% |
| **program-controlled (vault-like PDAs)** | **5** | **$32,423** | **0.05%** |
| total | 4,689 | $65,494,447 | |

All five program-controlled holders are zero-data authority PDAs; none is identifiable as an agent
vault, and the largest holds $29,897. **99.95% of the live open interest this harness can read sits in
plain wallets that are indistinguishable on-chain from human traders.** There is no agent-attributable
population to certify.

## Finding 3 — the run the repo already had is not a P1 run, and its PASS is a rounding artifact

`gallery/jupiter-live-AhUvhrHH.json` was the project's headline live PASS. Re-run today
(slot 440472881) it still passes — but recomputing the verdict **from its definition** rather than
from the card:

```rust
// crates/harness/src/jupiter.rs
pub const DELTA_UNIT_USD: i64 = 100;
fn delta_units(usd: i64) -> i64 { (usd + usd.signum() * (DELTA_UNIT_USD / 2)) / DELTA_UNIT_USD }
```

PASS ⟺ `|net signed notional| < $50`. `AhUvhrHH…`'s entire position is **$16**. It does not pass
because it is delta-neutral; it passes because it is **too small to measure**. Across the whole live
population the same recomputation gives:

| | count | share |
|---|---:|---:|
| PASS | 791 / 4,689 | 16.87% |
| …of which gross position < $50 (below the rounding band) | **779** | **98.5% of the PASS class** |
| …of which genuinely hedged (gross ≥ $50, net < $50) | 12 | 1.5% |
| FLAG | 3,898 / 4,689 | 83.13% |

Applied to the real population the certification separates approximately *"do you have more than $50
at risk?"* — and the repo's one committed PASS is on the dust side of that line. Three further runs
against real, large targets (`GykwUwgC…` $127k, `8Z98LMVG…` $2.27M, `68jj5xZc…` $29.9k, the largest
program-controlled holder) all returned the identical three findings — the FLAG is effectively
unconditional on anything with a real position.

---

## Adversarial recomputation — direction of every discrepancy

`docs/GATE.md`: *"Every numeric error found in this portfolio so far has favoured the project that
produced it."* Each check below was resolved **in the project's favour** and P1 still fails.

| check | result | direction |
|---|---|---|
| Streaming parse of the 450 MB Jupiter snapshot vs. an independent `dataSlice:0` count | 864,474 @ slot 440472439 vs **864,475** @ slot 440472771 (+1 over 332 slots) | independent count is *higher* — the parse did not undercount the search space **against** the project |
| What counts as an "agent" pubkey | all **4** pubkey fields per record (2,915 keys), not just the 1,471 asset ids | widened **for** the project; intersection still 0 open |
| Which positions count | **historical/closed** included, not only open | widened **for** the project; only 6 hits, 0 live |
| Search scope | **every** Position account on mainnet, not a sample | maximal **for** the project |
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

1. ≥1 of the 2,915 registry-referenced pubkeys holding an open Jupiter Perps position — the
   intersection is a single number the script prints, and it is 0.
2. A named agent vault among the 5 program-controlled holders — e.g. one of `FziSMN4a…`,
   `syspv6Qe…`, `H67ehFrM…`, `BUyLRaYE…` shown to be an autonomous-agent vault program rather than
   an ordinary protocol PDA.
3. A PASS/FLAG split that carries information about behaviour rather than position size — i.e. a
   PASS class not 98.5% composed of sub-$50 dust.

Re-running the script on a later slot is the cheapest way to revisit (1) and (3).
