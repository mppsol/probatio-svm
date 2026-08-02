# Probatio GTM — the re-execution validator for the agent Validation socket

**Thesis (grounded 2026-08):** both dominant agent registries (Solana Agent Registry, ERC-8004) *name*
independent **re-execution validation** as the intended trust model — but the **Solana Validation module
is archived / not yet deployed**, and ERC-8004's is **under active revision**. So the socket isn't
callable yet. That's the opening, not the obstacle: be the **reference re-execution validator** — attest
through the **live, permissionless Reputation Registry today** (Path A below), and build to the ERC-8004
`validationResponse` shape so Probatio is the drop-in when the Validation socket ships (Path B). First to
write real re-exec attestations on-chain, ready before the socket lands. Channel: Colosseum Fall 2026
(opens 2026-09-28, **submit by ~2026-11-02**) + Solana Foundation.

## What we can now show (proof, not slideware)

The fold is live and merged — one authored `MandateSpec`, checked at two stations:

- **certify** (probatio-svm): replays an agent's episode against its mandate on real SVM state; flags
  claim/solvency/phantom/mandate deviations; verdict any third party re-runs (`spec_hash`).
- **screen** (custos): re-simulates the next tx against real mainnet state; `MandateConformance` (M1)
  fires RED when realized token outflow exceeds the **same** authored `max_value_out`.
- **`mandate_demo`**: the *same* 800-token payment is `GREEN` under the default bank and `RED` under an
  authored `max_value_out=500` — "even a tricked agent can't exceed its mandate."

**The pain anchor (dated, real):** the Grok/Bankr prompt-injection drain (~$150–200K, 2026-05) —
"the exploit relied on how the AI interpreted input, not a contract vulnerability." That is exactly the
pre-broadcast + mandate gap the screen station closes. Lead every conversation with this.

## The one-line pitch (Solana Agent Registry)

> You shipped a Validation Registry with hooks for independent validators — I'm the validator: Probatio
> re-executes any registered agent against its mandate on live Solana state, and screens its next
> transaction, then writes a proof anyone can re-run. "Attested" stops meaning *exists* and starts
> meaning *behaved safely, and can't exceed its mandate.*

## Integration — two paths (grounded)

**Path A — shippable TODAY, via Solana's live *permissionless* Reputation Registry.** The Validation
module is archived, but `giveFeedback()`/`appendResponse()` are permissionless, so Probatio can write an
on-chain, agent-identity-tied re-execution verdict now:
- Program (agent-registry-8004) mainnet `8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ` (devnet
  `8oo4J9tBB3Hna1jRQ3rWvJjojqM5DYTDJo5cejUuJy3C`); TS SDK npm `8004-solana`. Agents are Metaplex Core
  NFTs; agentId = asset pubkey.
- Flow: (1) run certify (+screen) on the target agent → pass/fail + a re-runnable receipt (bound to
  `spec_hash`/`traceHash`); pin the receipt (IPFS). (2) `sdk.giveFeedback(targetAgent.asset, { value:
  '100'|'0', tag1: 're-exec', feedbackUri: 'ipfs://<receipt>' })` — `value` is the 0–100 field,
  `feedbackUri` anchors the evidence. (`appendResponse()` is program-level; SDK exposes giveFeedback —
  call the program directly if you need appendResponse.)
- Net: a third-party, on-chain, agent-tied re-exec attestation **today** — filed under Reputation, not
  Validation. This is the Colosseum differentiator: real on-chain attestation, not a mockup.

**Path B — the intended Validation socket, build-ready for when it un-archives.** Mirror ERC-8004's
ledger so Probatio is a drop-in. ERC-8004 (EVM) interface, live and documented:
```solidity
validationRequest(address validator, uint256 agentId, string requestURI, bytes32 requestHash); // by agent owner
validationResponse(bytes32 requestHash, uint8 response /*0-100*/, string responseURI, bytes32 responseHash, bytes32 tag); // by the named validator
getValidationStatus(bytes32 requestHash) -> (validator, agentId, response, tag, lastUpdate);
```
Probatio's contract *is* the named `validator`: owner requests → Probatio re-executes off-chain →
`validationResponse(requestHash, 100|0, responseURI→re-exec receipt, responseHash, tag)`. Any stake/slash
lives in Probatio's own contract, not the registry (the EIP leaves that to the validator). Since Solana's
registry is a direct ERC-8004 port, its future Validation instruction will very likely mirror this
request/response shape — so building to it now = ready on day one. (Precedent validator against this
interface: Reclaim Protocol's ZK-credential 8004 validator.)

## Colosseum Fall 2026 — opens 2026-09-28, submit by ~2026-11-02

Corrected: 09-28 is the hackathon **start**, not the deadline; the Fall competition runs to ~Nov 2. More
runway than assumed. Submission checklist (from Colosseum "How to Win"):
- [ ] Public repo.
- [ ] **Demo video < 3 minutes** (highest-weighted item) — build it on `mandate_demo` Green→Red + a
      certify verdict + the on-chain Path-A attestation.
- [ ] Slide deck: team, product, rationale, market, user-acquisition/monetization.
- [ ] Working MVP/demo (have: the fold; add the Path-A on-chain attestation).
- [ ] Judged on business viability + founder-market fit; **Public Good award** exists as a fallback lane
      (fits a neutral re-exec validator well). ~40 prizes; accelerator (~$250K precedent) for top teams.
- Agent-infra as a *named* Fall track: unconfirmed — but Agent Registry + the Feb 2026 Agent Hackathon
  make agent-trust a live Foundation/Colosseum theme, so the narrative aligns.

**The one build that makes the submission land:** wire Path-A — Probatio runs certify(+screen) on a
registered agent and writes the re-exec verdict on-chain via `giveFeedback(agent, {value, feedbackUri→
receipt})`. That turns "we re-execute" into "we re-execute *and attest on-chain, verifiably, today*" —
the differentiator vs TEE-attestation/signed-receipt incumbents.

**Positioning:** not "another agent-safety scanner" — the *neutral, re-executable* validator the registry
model names but nobody has shipped (uncontested per demand research; nearest analogs use TEE-attestation
or signed-mandate receipts, not re-execution).

## Approach order (from the prospect table) — top 5 before 09-28

1. **Solana Agent Registry (Quantu AI × Foundation)** — same channel; they built the socket. First dollar
   is realistically a grant/hackathon prize. Pitch above.
2. **Giza / Re7 Capital** — live $500K USDC into an autonomous agent; hand-built pre-flight checks today.
   "Turn your self-attested check into an independent, re-runnable safety proof underwritten before every
   allocation." (EVM/Base — worth the exception; strongest demonstrated buyer intent.)
3. **ERC-8004 validation ecosystem** — "your spec names stake-secured re-execution; Probatio is that
   reference validator, cross-VM." Positions Probatio as canonical, not vendor.
4. **Almanak** — largest agent-vault TVL ($132M peak); "one mandate breach or zero-day drainer is
   existential; we screen every tx pre-broadcast and prove each rebalance stayed in mandate."
5. **Catena Labs (ACK)** — best-funded ($18M a16z), regulated agent bank; "ACK proves who an agent is and
   that it can pay; Probatio proves it will *behave* — the compliance-grade behavioural gate you can't
   self-issue."

## Honest risks (from demand research)

- Registry WTP is **strategic/grant-first**, not a standing subscription — treat the registry as
  distribution + credibility, monetize the *allocators* (Giza/Almanak/Catena) who bear real downside.
- Crypto-native agent-payment volume is still thin ("mirage," ~$28K/day real x402) — the durable WTP is
  the **capital allocator** segment, not the rails. Lead there for revenue, use the registry for reach.
- Keep it lane ① (neutral/paid-to-prove). No offensive posture here — that's the walled ③ lane.
