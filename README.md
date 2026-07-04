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
the 200k/instruction budget. **25 tests green offline.**

## Quickstart

```bash
# Off-chain verifier over the pure-Rust reference model:
cargo run --offline -p probatio-svm-harness -- --backend ref

# Same episode driven through the real Pinocchio program on LiteSVM
# (builds the BPF .so on first run via `cargo build-sbf`):
cargo run --offline -p probatio-svm-harness -- --backend svm

# All tests (ref+svm parity, unbypassable-enforcement reverts, atomicity, CU):
cargo test --offline
```

Requires the Rust toolchain (pinned in `rust-toolchain.toml`) and the Solana SBF toolchain
(`cargo build-sbf`) for the `svm` backend.

## Layout

```
crates/contract   shared account layout (Market, Position) + instruction codecs + check_position()
                  enforcement predicate — the load-bearing contract, read by the perp, the guard, AND
                  the verifier (#![no_std])
crates/harness    episode driver (ref + LiteSVM backends), scripted policies, invariant-set verifier
programs/perp     Pinocchio perp; inline-enforces check_position() on every mutating instruction
programs/guard    Pinocchio composable guard for wrapping third-party-owned accounts
docs/tasks        task briefs (the CC↔Codex handoff surface)
reviews           cross-review verdicts
STAGE0_DESIGN.md  the design + honest scope notes + roadmap
```

## Honest limitations

- **Coverage, not mechanism, is the open frontier.** The scripted `measurement_gamer` / `phantom_hider`
  policies prove the machinery works; they do **not** prove the invariant set catches *unknown* economic
  exploits. Generality comes from the **red-team discovery loop** (roadmap, not yet built), which is the
  real moat.
- **Episodes are still clean.** One deterministic oracle drop, no slippage / MEV / adversarial ordering.
  Certifying robustness against realistic mainnet conditions needs "hostile episodes" (roadmap).
- **Third-party enforcement needs CPI.** The perp enforces *its own* accounts unbypassably (inline). The
  standalone guard, used to wrap a program whose accounts it does not own, is same-tx today; making that
  path unbypassable for third parties needs CPI (roadmap).
- `cargo build-sbf` emits one benign `sol_memcpy_` post-processing warning; the programs build, load, and
  run correctly. `vendor/hermit-abi` is a no-op offline-build shim, not a real dependency
  ([details](./vendor/hermit-abi/README.md)).

## Roadmap

- **Red-team discovery loop** — search the shortcut space, promote newly-found classes into invariants
  (the coverage moat; [[solinv]] DNA).
- **Hostile episodes** — inject slippage, oracle lag, adversarial ordering to test robustness.
- **CPI guard promotion** — unbypassable enforcement for third-party-owned accounts.
- LLM agent behind the `Policy` trait; pitch video (certify PASS / catch FLAG / enforce revert).

## Built with cross-review

Two agents that cross-review each other: **Claude Code** (frame-thin — architecture, the shared
contract, the reference model, verifier soundness) and **Codex** (frame-thick — the Pinocchio programs,
the LiteSVM driver, adversarial audits). Whoever implements a change does not review it. See
[`AGENTS.md`](./AGENTS.md).

## License

Licensed under either of [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at your option.
