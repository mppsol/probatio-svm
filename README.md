# Probatio SVM

**A falsification record.** This repository ran five hypotheses about verifying autonomous agents on
Solana through a kill-gate discipline. **All five are closed. None survived. Nothing is live.**

What is here is the evidence: pre-registrations written before measurement, the measurements that
were run, independent cross-reviews, and the verdicts — including the ones that killed the work. The
code is real and the numbers are reproducible; **the product claims are not, and have been withdrawn.**

Sibling of [Probatio](https://github.com/psyto/probatio) (the Reth/revm proving ground) and of the
read-only `../solvo`. Built by **Claude Code + Codex** in cross-review — see [`AGENTS.md`](./AGENTS.md).

> **Read [`STATUS.md`](./STATUS.md) first.** It is the current state of record.
> Standing rules: **a verdict is a number or a reproducible experiment**, never an assessment or a
> plan; **not-proven is a KILL**, not a pending; **adding a hypothesis to stay alive is forbidden**.

## The five verdicts

| | hypothesis | verdict | record |
|---|---|---|---|
| **H1** | certify autonomous agents before capital is trusted to them | **`KILLED`** (2026-08-20) | [gate](./docs/GATE.md) · [decision](./docs/decisions/P1-real-target.md) |
| **H2** | a countable population bearing a reconstructible loss it does not control | **`FROZEN UNEXECUTED`** (2026-08-21) | [gate](./docs/H2-GATE.md) |
| **H3** | a protocol-independent composability capability schema | **`KILLED` at the design gate** (2026-08-21) | [gate](./docs/H3-GATE.md) · [decision](./docs/decisions/H3-design-gate-kill.md) |
| **H4** | an upgrade behaviour sentinel — same input, same state, only the binary varies | **`KILLED` at G0** (2026-08-22) | [gate](./docs/H4-GATE.md) · [decision](./docs/decisions/H4-sentinel-kill.md) |
| **H5** | pre-release regression tests that find what a simulation or wallet policy cannot | **`CLOSED`** (2026-08-22) | [gate](./docs/H5-GATE.md) · [decision](./docs/decisions/H5-closed.md) |

**No verdict above is rewritten, revived, or worked around.** Each gate document is kept intact as the
pre-registration its verdict is read against.

## What each one actually refuted

- **H1** died at its first gate item: **no real target existed.** The premise assumed an on-chain agent
  population to certify; that population could not be produced. Everything downstream — attestations,
  the certification market, the registry go-to-market — rests on that premise and falls with it.
- **H2** was **never executed.** Its pre-registration entered a specification spiral — three rounds,
  331 lines, zero measurements — and was frozen rather than given a fourth round. It is **not** a
  refutation of H2's content; nothing was measured.
- **H3** died because **capability semantics could not be made protocol-independent.** Two review
  rounds showed the freedom to decide what satisfies a field was **relocated** into an adapter
  manifest, not removed. The deliverable would have been a report per protocol, not a comparable
  primitive — the exact thing it had to prove it was not.
- **H4** was **measured** and killed by its own pre-registered numbers. Four cases, real pre- and
  post-upgrade klend binaries, identical cloned state: **all four returned `unknown`**, so the action
  was `RE-VERIFY` — the only answer the differing code hash could already give. Deciding otherwise
  would have required exactly the venue semantics H3 died of.
- **H5** never reached a measurement. **Four authorised candidates, four design-gate failures**, and
  the last one — duplicate execution after a lost confirmation — **could not be differentiated from a
  runtime wallet policy at all**, because a cumulative spend cap set at the agent's own intent total
  catches a duplicate spend by definition. **H5 is `unproven`, not `refuted`**: nothing here says
  sequence-only failures do not exist on Solana. That was never tested, and is not claimed.

## What the code is

The engineering is real, offline, and reproducible. It is **evidence tooling and refutation assets** —
**not a product, and not a foundation for a new hypothesis.** Reusing the mechanics as fixtures is
fine; **reusing a conclusion as evidence is not.**

- **A real-BPF episode driver.** Compiles Pinocchio programs with `cargo build-sbf`, loads the `.so`
  into [`LiteSVM`](https://github.com/LiteSVM/litesvm), and executes transactions with real
  compute-unit accounting. Account state is read as ground truth; **no assertion reads a log line.**
- **A shared account-layout contract** (`crates/contract`) read by the perp program, the guard program,
  and the off-chain verifier — one definition, three consumers.
- **Inline, unbypassable enforcement.** Because `Position` accounts are owned by the perp program and
  only the owning program can mutate an account, a transaction that omits any external guard **still
  reverts**.
- **The H4 sentinel** (`crates/h4-sentinel`) — two real mainnet klend binaries, 17 cloned mainnet
  accounts at slot 440,477,781, disclosed mutations, deterministic offline replay, no network.

### Measured results that remain valid

These are properties of the code, and they still hold. **They were never the thing in doubt** — the
hypotheses were.

**Verifier**, identical across the `ref` and `svm` backends:

| policy | verdict | findings |
|---|---|---|
| `honest` | PASS | — |
| `measurement_gamer` | FLAG | `ContinuousNeutrality`[55–59] + `IntraEpisodeInsolvency`[30–59] |
| `phantom_hider` | FLAG | `PhantomExposure`[1–60] + `IntraEpisodeInsolvency`[30–60] |

**Enforcement** — a perp `Open` sent alone, with no guard instruction, still reverts atomically
(proven by reading the account back: `before == after`):

| solo perp tx, no guard ix | outcome |
|---|---|
| honest `Open` | Ok, position mutated |
| out-of-mandate `Open` (qty=101) | reverted `Custom(10)` `MandateDeviation` |
| self-inflicted insolvency `Open` (collateral=10) | reverted `Custom(11)` `SelfInflictedInsolvency` |

Perp instruction CU: `Open`=583, `Hedge`=758, `SettleFunding`=356 — far under the 200k budget.
**91 tests green offline, 0 failed**, re-run 2026-08-22 (`cargo test --offline`): harness 77 lib + 2
binary, contract 7, `reexec-spec` 3, perp and guard 1 each. *(An earlier version of this file said 87;
that count was stale, and the number above is the one this run produced.)*

**H4's measurement**, byte-identical across three consecutive runs
(`evidence/h4-sentinel.json`, `f05c0ea6…8f0cf0`): both binaries agreed on every `result` and every
token delta; the sole divergence was **4 bytes the new binary writes at offset 28** of `obligation`,
`reserve_sol` and `reserve_usdc`. Requesting `u64::MAX` collateral returned **`Ok`** and moved
**486,657,686** of **2,248,785,777**; requesting exactly the full deposit **failed** with
`Custom(6011)`.

## Quickstart

```bash
# Off-chain verifier over the pure-Rust reference model:
cargo run --offline -p probatio-svm-harness -- --backend ref

# The same episode through the real Pinocchio program on LiteSVM
# (builds the BPF .so on first run via `cargo build-sbf`):
cargo run --offline -p probatio-svm-harness -- --backend svm

# All tests (ref+svm parity, unbypassable-enforcement reverts, atomicity, CU):
cargo test --offline

# H4's sentinel — two real klend binaries, one cloned state set, offline, deterministic.
# Rewrites evidence/h4-sentinel.json; two runs are byte-identical.
cd crates/h4-sentinel && cargo run --release
```

Requires the Rust toolchain (pinned in `rust-toolchain.toml`) and the Solana SBF toolchain
(`cargo build-sbf`) for the `svm` backend. The `h4-sentinel` crate is a **separate workspace** by
necessity — mixing LiteSVM's unbundled Solana crates with the root graph produces duplicate
incompatible crates.

## Layout

```
STATUS.md           the state of record — read this first
docs/*-GATE.md      the five pre-registrations, kept intact; each carries its closing verdict
docs/decisions/     the verdicts and their reasoning
reviews/            independent cross-review verdicts, verbatim
evidence/           measured output (H4's sentinel run)
fixtures/h4/        17 cloned mainnet accounts at slot 440,477,781 + two real klend binaries
crates/reexec-spec  the authored, hashable MandateSpec — #![no_std]        (H1 artifact, refuted)
crates/contract     shared account layout + codecs + check_position()      (H1 artifact, refuted)
crates/harness      episode driver (ref + LiteSVM), policies, verifier     (H1 artifact, refuted)
crates/h4-sentinel  the H4 upgrade sentinel — evidence tooling             (H4 artifact, refuted)
programs/perp       Pinocchio perp; inline-enforces on every mutating ix   (H1 artifact, refuted)
programs/guard      Pinocchio composable guard                             (H1 artifact, refuted)
docs/tasks          task briefs (the CC↔Codex handoff surface)
```

**"Refuted" marks the hypothesis, not the code.** Each crate does what it says and its tests pass. What
was refuted is the claim that it was worth building for the reason it was built.

## Honest limitations

- **No product claim is made or supported by anything here.** The market positioning, the demand
  argument, the registry go-to-market and the roadmap that earlier versions of this file carried were
  **H1's**, and H1 is `KILLED`. They are removed rather than softened, and the documents that still
  contain them are marked as historical.
- **H4's measurement has one recorded gap**, disclosed in its decision record: `post_state` records
  per-account ranges differing **from the fixture** per side, so it proves the two post-states differ
  but not that offset 28 is the *only* place they differ. The harness was deliberately **not** changed
  after measurement.
- **Nothing in `../solvo` is ever written from here.** It is a read-only reference.
- `cargo build-sbf` emits one benign `sol_memcpy_` post-processing warning. `vendor/hermit-abi` is a
  no-op offline-build shim, not a real dependency ([details](./vendor/hermit-abi/README.md)).

## Roadmap

**None.** No hypothesis is live and **nothing is authorised** — not a new fixture, not a candidate
search, not implementation, measurement, UI, token, or deploy.

**Opening anything requires a founder ruling.** Reopening the H5 line additionally requires a **new,
independent hypothesis** — not a repair or rewording of a closed one — and a **new pre-registration**
that names the comparator **the hypothesis itself names, at full strength**. Choosing a weaker
comparator is the specific mistake that ended H5.

## What this repository is actually good for

Five hypotheses died cheaply, and the record of *how* is the durable output:

- **The design gate is the cheapest place to fail.** H3, H5's four candidates, and most of H4's cost
  were paid before implementation. H5 produced **zero lines of product code**.
- **The cross-review loop caught errors in both directions.** One round returned `DISSENT` and
  overturned a proposal to kill a candidate early; another killed a candidate the author believed in.
  **No model reviews its own output**, and that rule earned its keep.
- **A comparator chosen for winnability is worse than a losing comparison** — it produces a result that
  cannot be defended.
- **Pre-registering the rule that kills you is what makes a kill honest.** H4 wrote down, before
  measuring, that a state-only difference would be `unknown` and that `unknown` would end it. It was,
  and it did.

## Built with cross-review

Two agents that cross-review each other: **Claude Code** (frame-thin — architecture, gates, decision
records, the shared contract, the reference model) and **Codex** (frame-thick — the Pinocchio programs,
the LiteSVM driver, adversarial audits and independent reviews). **Whoever implements a change does not
review it**, and at most two agents work at once. See [`AGENTS.md`](./AGENTS.md).

## License

Licensed under either of [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at your option.
