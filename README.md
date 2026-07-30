# Probatio SVM

**A proving ground that certifies autonomous agents in Solana DeFi before you trust them with capital —
and enforces the rules they must not break, unbypassably, on-chain.**

The ecosystem is racing to hand autonomous agents real money. Probatio SVM is the layer that answers
*"will this agent rug the vault before it does?"* — as a **pre-deployment audit** (off-chain, replayable,
nothing to bypass) — and backs it with **on-chain enforcement** that reverts a violating transaction
inside the block.

Sibling of [Probatio](https://github.com/psyto/probatio) (the Reth/revm proving ground). Built by
**Claude Code + Codex** in cross-review — see [`AGENTS.md`](./AGENTS.md). Targeting the next Colosseum
hackathon (2026-09-28 → 11-02).

## What it is (and isn't)

Probatio SVM is a **proving ground / certification harness**: it replays a seedable episode against a
real Solana program and judges the agent's behavior. It is **not** a realtime mainnet monitor — the
verifier runs offline over a replay, like a fuzzer or CI, so it never has to "keep up" with block times
or MEV. Certify first, deploy second.

The same verifier can also read **live on-chain state** for a one-shot audit: `certify-jupiter --live`
fetches a real wallet's current Jupiter Perps positions via `getProgramAccounts` and certifies that
snapshot against a delta-neutral mandate as **unsolicited due-diligence**. This is a **point-in-time
attestation**, still not a streaming monitor — it reads the chain once and judges what it finds.

## Where it sits

Pre-deployment **certification of autonomous agents** is a recognized, unsolved problem — but the work
so far is either off-chain or adjacent:

- **Off-chain agent-eval** (e.g. Patronus AI, ~$70M raised) builds replay "world models" that stress-test
  agents and detect shortcuts — the same moat, but for enterprise SWE/finance, **not on-chain**. Probatio
  is *"that, for Solana DeFi"* — and it's more defensible on-chain: Patronus must *replicate* websites to
  build a world model; here **account state IS the world, so the ground truth is free**.
- **Runtime guardrails** (Autonex, Blockaid) constrain or screen an agent's transactions *live*, against
  hand-written policies. Probatio is **pre-capital**: it certifies whether an agent honored its *mandate*,
  and its invariant set **self-repairs** via the red-team loop rather than being a fixed policy list.
- **Solana's Agent Registry** is an identity/reputation trust layer, not a certifier — but its *Validation
  Registry* is a hook for attestations, so Probatio can be **the certifier that feeds it**.
- Among 2026 Colosseum agentic-finance projects, the verification/safety layer is essentially empty —
  everyone is building agents; almost no one is building the thing that checks them.

Honest framing: this is an **emerging, anticipatory** category — demand is validated by analogy (regulated
enterprise AI needs pre-deployment assurance) more than by proven on-chain pain today. Probatio is a
first-mover category bet, not a land-grab in a crowded market.

## The two layers

### 1. Verifier (off-chain) — the primary value

Replays a 60-slot episode on a real Solana program via [`LiteSVM`](https://github.com/LiteSVM/litesvm),
reads **account state as ground truth** (on Solana every piece of state is an addressable account — there
is no oracle to reconstruct), and emits a `ShortcutReport` flagging shortcut classes with **slot-level
evidence**. It is an offline audit — **there is nothing for a cheater to switch off.** Invariant-set
driven; a red-team discovery loop (roadmap) promotes newly-found shortcuts into invariants.

### 2. Enforcement (on-chain) — unbypassable, in-block

The perp program **inline-enforces** its invariants at the end of every mutating instruction
(`Open`/`Hedge`/`Close`), via the shared `check_position()` predicate. Because `Position` accounts are
**owned by the perp program**, and only the owning program can mutate an account, **there is no path to
change a position that skips the check** — a transaction that omits any external "guard" still reverts.
A separate composable `programs/guard` reuses the same `check_position()` for the different job of
**wrapping accounts owned by a third-party program** (same-tx today; CPI on the roadmap).

The two layers are complementary: **enforcement blocks bad *actions* in-block; the verifier catches bad
*states/behaviors*** over the episode (measurement gaming, phantom exposure, and passive oracle-driven
insolvency that no single tx causes and no guard can revert).

## Certifying real on-chain positions (live path)

Because on Solana **account state IS the world**, the verifier needs no replay to judge a real position —
it can read the chain directly. `certify-jupiter --live <owner>` fetches every open Jupiter Perps
`Position` account owned by a wallet in one `getProgramAccounts` snapshot, decodes it against the
committed account layout, and certifies net signed notional against a delta-neutral mandate. The delta
verdict is **oracle-free** (signed notional is USD-denominated, so it is mark-independent; `--mark` only
feeds the *advisory* liquidation model).

The ingestion boundary is deliberately strict — this is a **ground-truth recovery** path, so it refuses
to certify over anything it cannot fully trust:

- Accounts are matched by the Jupiter program owner, a `dataSize` filter, **and the Anchor `Position`
  discriminator** (a `memcmp` filter at offset 0, re-checked at decode) — a same-sized account of another
  type is rejected, not certified through the fixed offsets.
- A truncated / malformed account is an **error**, never a silently-dropped slot; the decoder separates
  *untrusted* data (`Err`) from a *validated-closed* slot (`Ok(None)`), so a partial fetch can never look
  like a complete, clean book.
- The fetch uses `withContext`, and the path is **fail-closed on a missing snapshot slot**: a
  point-in-time card is only meaningful if it can name the chain snapshot it judged, so if the RPC returns
  no `context.slot` the CLI exits without writing a card rather than stamp a synthetic slot `0`.

**Honesty:** this is **unsolicited due-diligence** — the wallet operator made no claim to us. A FLAG means
"these live positions do not satisfy a delta-neutral mandate declared by Probatio", **never** "the operator
lied". The card is self-describing about *what*, *which snapshot*, and *when*, all serialized *into* the
gallery card so they survive the console banner:

- `assessment_kind` / `mandate_source` / a plain-language note — the unsolicited-DD framing.
- `snapshot_slot` + `captured_at` — the exact Solana slot and capture time the positions were read at.
- `rpc_source` — the endpoint **host only**, credential-redacted (a DD card that outlives the console must
  never embed an API key in the URL).

The decode/parse boundary is proven against **real committed mainnet fixtures** — an open SOL long and an
open short on a *different custody* (a BTC-class market) — so the fixed offsets are shown to recover a real
short and a second market from live bytes, not just round-trip synthetic bytes.

## Status — Stage 0 complete + unbypassable enforcement ✅

Built on a **real compiled BPF program**, not a mock: the harness runs `cargo build-sbf`, loads the
`.so` into LiteSVM, and executes transactions with real compute-unit accounting.

**Verifier results** (identical across the `ref` and `svm` backends):

| Policy | Verdict | Findings |
|---|---|---|
| `honest` | PASS | — |
| `measurement_gamer` | FLAG | `ContinuousNeutrality`[55–59] + `IntraEpisodeInsolvency`[30–59] |
| `phantom_hider` | FLAG | `PhantomExposure`[1–60] + `IntraEpisodeInsolvency`[30–60] |

**Enforcement results** — a perp `Open` sent **alone, with no guard instruction** (the bypass a naive
same-tx guard would allow) still reverts, atomically (proven by reading the account back — `before ==
after`):

| Scenario (solo perp tx, no guard ix) | Outcome |
|---|---|
| honest `Open` | Ok, position mutated |
| out-of-mandate `Open` (qty=101) | reverted `Custom(10)` MandateDeviation |
| self-inflicted insolvency `Open` (collateral=10) | reverted `Custom(11)` SelfInflictedInsolvency |

Perp instruction CU (with inline enforcement): `Open`=583, `Hedge`=758, `SettleFunding`=356 — far under
the 200k/instruction budget. **78 tests green offline** across the workspace (harness alone: 69 lib + 2
binary, covering the live-ingestion decode/parse boundary against real mainnet long *and* short fixtures,
withContext slot recovery, and credential redaction).

## Quickstart

```bash
# Off-chain verifier over the pure-Rust reference model:
cargo run --offline -p probatio-svm-harness -- --backend ref

# Same episode driven through the real Pinocchio program on LiteSVM
# (builds the BPF .so on first run via `cargo build-sbf`):
cargo run --offline -p probatio-svm-harness -- --backend svm

# All tests (ref+svm parity, unbypassable-enforcement reverts, atomicity, CU):
cargo test --offline

# Certify a Jupiter Perps agent — deterministic sample cards (neutral vs drift), no key/RPC:
cargo run --offline -p probatio-svm-harness -- certify-jupiter --sample

# Unsolicited due-diligence on a REAL wallet's live positions (one on-chain snapshot).
# --rpc defaults to mainnet-beta (or set PROBATIO_RPC_URL); --mark is an advisory liquidation input.
cargo run -p probatio-svm-harness -- certify-jupiter --live <owner_pubkey> [--rpc <url>] [--mark <usd>]
```

Requires the Rust toolchain (pinned in `rust-toolchain.toml`) and the Solana SBF toolchain
(`cargo build-sbf`) for the `svm` backend.

## Layout

```
crates/contract   shared account layout (Market, Position) + instruction codecs + check_position()
                  enforcement predicate — the load-bearing contract, read by the perp, the guard, AND
                  the verifier (#![no_std])
crates/harness    episode driver (ref + LiteSVM backends), scripted policies, invariant-set verifier,
                  Jupiter live on-chain ingestion (getProgramAccounts → decode → certify)
programs/perp     Pinocchio perp; inline-enforces check_position() on every mutating instruction
programs/guard    Pinocchio composable guard for wrapping third-party-owned accounts
gallery           serialized certification cards (sample cards tracked; jupiter-live-*.json gitignored)
docs/tasks        task briefs (the CC↔Codex handoff surface)
reviews           cross-review verdicts
STAGE0_DESIGN.md  the design + honest scope notes + roadmap
```

## Honest limitations

- **Coverage.** The scripted policies prove the machinery works; a **red-team discovery loop** (shipped)
  mechanically searches for shortcuts the invariant set misses and promotes fixes — it already found and
  closed a near-neutral claim bypass. Exhaustive coverage of *unknown* economic exploits remains open.
- **Hostile-episode audit (shipped).** Episodes can now carry slippage, a lagged multi-shock oracle path,
  and deterministic noise. Finding: the misrepresentation invariants are **price-noise invariant for a
  fixed action sequence** (delta is position size, not price) — but a **price-reactive** policy (and a
  future LLM agent) changes its actions with price, so that invariance does not extend to it; that is the
  explicit boundary, and why price-reactive agents need per-episode certification. Solvency is
  **stress-relative**: the episode must declare the stress it certifies against.
- **Not a realtime monitor.** Probatio is a pre-deployment proving ground (offline replay), so verifier
  latency under mainnet block times / MEV is out of frame by design.
- **Third-party enforcement needs CPI.** The perp enforces *its own* accounts unbypassably (inline). The
  standalone guard, used to wrap a program whose accounts it does not own, is same-tx today; making that
  path unbypassable for third parties needs CPI (roadmap).
- `cargo build-sbf` emits one benign `sol_memcpy_` post-processing warning; the programs build, load, and
  run correctly. `vendor/hermit-abi` is a no-op offline-build shim, not a real dependency
  ([details](./vendor/hermit-abi/README.md)).

## Roadmap

- ✅ **Red-team discovery loop** — searches the shortcut space, promotes newly-found classes into
  invariants (the coverage moat; [[solinv]] DNA).
- ✅ **Hostile episodes** — slippage, lagged multi-shock oracle, deterministic noise; verifier robustness
  audit.
- ✅ **LLM agent** behind the `Policy` trait — a real (price-reactive) agent certified per-episode; the
  natural next step from the hostile-episode boundary.
- ✅ **Jupiter Perps adapter + live on-chain ingestion** — map a real venue's positions into the verifier,
  and certify a real wallet's live positions as unsolicited due-diligence (`certify-jupiter --live`).
- ✅ **Real short/multi-custody fixture + snapshot-slot provenance** — a real *short* Position fixture on a
  different custody, plus `withContext` slot / capture time / credential-redacted host stamped into the
  card, fail-closed when the snapshot slot is missing.
- **CPI guard promotion** — unbypassable enforcement for third-party-owned accounts.
- **Live cert web card + multi-slot polling + on-chain Custody mark** — dashboard surface for a live cert,
  a time-series live trace, and an oracle mark read from the Custody account (today `--mark`).
- Pitch video (certify PASS / catch FLAG / enforce revert).

## Built with cross-review

Two agents that cross-review each other: **Claude Code** (frame-thin — architecture, the shared
contract, the reference model, verifier soundness) and **Codex** (frame-thick — the Pinocchio programs,
the LiteSVM driver, adversarial audits). Whoever implements a change does not review it. See
[`AGENTS.md`](./AGENTS.md).

## License

Licensed under either of [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at your option.
