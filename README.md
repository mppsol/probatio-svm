# Probatio SVM

**A proving ground for autonomous agents in Solana DeFi — and a runtime circuit-breaker that stops the
ones that cheat.**

The ecosystem is racing to hand autonomous agents real capital. Probatio SVM is the layer that answers
*"will this agent rug the vault before it does?"* — and, uniquely on Solana, can **revert the cheating
transaction inside the block** instead of just flagging it after the fact.

Two primitives:

1. **Verifier (off-chain).** Replays a seedable episode on a real Solana program via `LiteSVM`, reads
   **account state as ground truth** (no oracle to build — on Solana every piece of state is an
   addressable account), and emits a `ShortcutReport` flagging shortcut classes with **slot-level
   evidence**. Invariant-set driven, with a red-team discovery loop that promotes newly-found shortcuts
   into invariants ([[solinv]] DNA).
2. **Guard (on-chain).** A **Pinocchio** CPI-guard program that compiles the invariant set and returns
   `Err` — reverting the whole transaction — when a violation is about to be committed. Low-CU;
   deployable safety rails.

Sibling of [Probatio](../probatio) (the Reth/revm proving ground). Built by **Claude Code + Codex** in
cross-review — see [`AGENTS.md`](./AGENTS.md). Roadmap in [`STAGE0_DESIGN.md`](./STAGE0_DESIGN.md).

Targeting the next Colosseum hackathon (2026-09-28 → 11-02).

## Status

Stage 0 in progress. See `STAGE0_DESIGN.md` §11 for the definition of done.

## License

MIT OR Apache-2.0.
