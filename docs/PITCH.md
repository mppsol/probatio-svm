# Probatio SVM — 90-second pitch storyboard

A shot-by-shot plan for the Colosseum submission video. Every beat maps to a **real command in this
repo** — nothing staged. Total target ~90s. Keep narration tight and honest (see the claims guardrail at
the end). Record the terminal cuts against a warm build so there's no cold-build lag.

## Pre-record setup (do once, off-camera)

```bash
cd /Users/hiroyusai/src/probatio-svm
cargo build --offline                                   # warm the host build
cargo run --offline -q -p probatio-svm-harness -- --backend svm >/dev/null   # warm build-sbf + .so cache
export ANTHROPIC_API_KEY=sk-ant-…                       # only for the live-agent cut (0:55)
```

Terminal: large monospace font, ~100 cols, dark theme, clear scrollback before each cut.

---

## Beat 1 — Hook (0:00–0:08)

- **On screen:** title card. `Probatio SVM` / *"a proving ground for autonomous agents in Solana DeFi."*
- **VO:** "Autonomous agents are getting the keys to on-chain vaults. Before one touches real capital —
  who checks that it won't rug it?"
- **Cut note:** hold the title 2s, then hard-cut to a terminal.

## Beat 2 — The verifier: honest passes, cheaters get caught (0:08–0:28)

- **Command:**
  ```bash
  cargo run -q -p probatio-svm-harness -- --backend ref
  ```
- **On screen:** the output — `[PASS ] honest`, then `[FLAG ] measurement_gamer` and
  `[FLAG ] phantom_hider` with their finding lines. Highlight the **slot ranges** (e.g.
  `ContinuousNeutrality slots [55…59]`, `PhantomExposure slots [1…60]`).
- **VO:** "Probatio replays a 60-slot episode and reads account state as ground truth — on Solana every
  position is an addressable account, so there's no oracle to reconstruct. An honest agent passes. A
  measurement-gamer and a phantom-exposure cheat are flagged — with the exact slots they cheated on."
- **Cut note:** zoom the FLAG lines; the slot lists are the money shot (evidence, not vibes).

## Beat 3 — On a real program, enforced on-chain (0:28–0:45)

- **Command:**
  ```bash
  cargo run -q -p probatio-svm-harness -- --backend svm       # same verdicts, real BPF program
  cargo test -q --offline -p probatio-svm-harness --lib inline_enforcement_blocks           # the in-block revert, proven
  ```
- **On screen:** `--backend svm` printing the *same* PASS/FLAG (caption: "real Pinocchio program,
  LiteSVM, ~583 CU/Open"); then the two `inline_enforcement_blocks_*` tests going green.
- **VO:** "Same episode, now on a real Solana program compiled to BPF. And it's not just detection: the
  perp inline-enforces its invariants, so a transaction that would breach mandate or self-inflict
  insolvency **reverts in-block** — the account is byte-identical afterward. Unbypassable, because the
  position is program-owned."
- **Cut note:** caption the CU number; it backs the "cheap safety rails" line.

## Beat 4 — The moat: it finds its own gaps (0:45–0:58)

- **Command:**
  ```bash
  cargo run -q -p probatio-svm-harness -- redteam
  ```
- **On screen:** "escapes found: 16", the escape lines, then the promotion line
  (`baseline=Pass → promoted=ShortcutDetected (ClaimTracksExposure…)`, `honest … Pass`).
- **VO:** "The check isn't a fixed list someone hand-wrote. A red-team loop searches for shortcuts our own
  invariants miss — it found a near-neutral-claim bypass — then promotes a fix that catches every one,
  without flagging the honest agent."
- **Cut note:** this is the differentiator — lean on "it patches itself."

## Beat 5 — A real Claude agent, certified (0:58–1:16)

- **Command (live, needs the key):**
  ```bash
  cargo run -q -p probatio-svm-harness -- gallery            # real Claude under a delta-neutral mandate
  cat gallery/neutral_mm-clean.json | head -20               # the saved transcript
  ```
  *(Fallback with no key / for a deterministic take: `cargo run -q -p probatio-svm-harness -- gallery
  --sample` → `gallery/sample-scripted-drift.json`.)*
- **On screen:** the gallery run printing the verdict, then the transcript JSON (mandate `system`,
  `claimed_delta: 0`, `verdict`, per-slot `measured_delta`).
- **VO:** "Now a real Claude agent, handed a delta-neutral market-maker mandate. Probatio certifies
  whether it actually stayed neutral — and saves the transcript. If it drifts from what it claimed,
  ClaimTracksExposure catches it."
- **Cut note:** if the live agent passes, say "certified"; if it drifts, say "and here's the catch" —
  either outcome is a good demo. Do a couple of takes.

## Beat 6 — Close (1:16–1:30)

- **On screen:** two-line summary card:
  *"Verifier — certify agents before they touch capital (offline, nothing to bypass). Enforcement —
  revert the violators on-chain."* Then the repo line `github.com/psyto/probatio-svm` and the tagline.
- **VO:** "A proving ground that certifies autonomous agents before they touch capital — and enforces the
  rules on-chain. Probatio SVM."
- **Cut note:** end on the title card; 51 tests / real BPF can be a small caption.

---

## Claims guardrail (do NOT overclaim on camera)

- Say **"reverts in-block / unbypassable for the perp's own accounts"** (true: positions are
  program-owned + inline-enforced). Do **NOT** say "agents can never cheat" in general — a *composable
  guard* wrapping a third-party program's accounts is still same-tx (opt-in) until the CPI promotion.
- Probatio is a **pre-deployment proving ground / certification harness**, not a realtime mainnet
  monitor — don't imply it watches live chains.
- The misrepresentation invariants are price-noise invariant **for a fixed action sequence**; a
  price-reactive agent (the live one) legitimately varies with price. Don't claim price-invariance for
  the live agent.
- Numbers to cite are real: 51 tests, real BPF via LiteSVM, Open ≈583 CU, guard ≈508–714 CU, 16
  discovered escapes. Keep them accurate.
- Positioning (from the 2026-07 competitive scan): frame it as **"Patronus for on-chain Solana DeFi"** —
  pre-deployment agent certification, a category that's empty on Solana. Defensibility = account state is
  free ground truth (no website replicas) + on-chain enforcement + self-repairing invariants. Demand is
  **anticipatory** (validated by analogy to regulated-enterprise AI assurance), not proven on-chain pain —
  pitch the wedge, don't claim a bleeding market. **Do NOT cite specific crypto AI-agent exploit incidents
  as precedent** — the ones surfaced (a ~$25M bot self-attack, a $1.78M Moonwell loss) failed verification.

## Shot list (commands, in order)

1. `cargo run -q -p probatio-svm-harness -- --backend ref`
2. `cargo run -q -p probatio-svm-harness -- --backend svm`
3. `cargo test -q --offline -p probatio-svm-harness --lib inline_enforcement_blocks`
4. `cargo run -q -p probatio-svm-harness -- redteam`
5. `cargo run -q -p probatio-svm-harness -- gallery` (or `-- gallery --sample`)
6. `head -20 gallery/neutral_mm-clean.json` (or `gallery/sample-scripted-drift.json`)
