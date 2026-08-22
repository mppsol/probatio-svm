<!--
Independent review of docs/H5-CANDIDATES.md — the H5 G0 candidate search that proposed branch A
(H5 KILLED). Run once, by founder ruling 2026-08-22, from the payload committed at
reviews/H5-G0-candidates.codex-prompt.md (d8eb10a).

Reviewer: Codex, read-only:
  /Applications/ChatGPT.app/Contents/Resources/codex exec \
    -C /Users/hiroyusai/src/probatio-svm -s read-only \
    "$(cat reviews/H5-G0-candidates.codex-prompt.md)" < /dev/null

Clean cross-review: CC wrote every line under review. Codex wrote crates/h4-sentinel, whose frozen
evidence is cited as input.

VERDICT: DISSENT — a candidate survives (C3). Two P0s: C3 was killed on a wrong premise, and C1's
KILL-1 silently granted the baseline a state-carrying executor, which is H5's own machinery.

CONSEQUENCE, per the founder's instruction: branch A is WITHDRAWN and NO KILL COMMIT IS MADE.
H5 is not killed. CC's assessment of this review, and the one thing in it the founder must rule on
before anything else, are in docs/H5-CANDIDATES.md §8.
-->

# H5 G0 Candidate Search Review

**Verdict: DISSENT — a candidate survives: C3.** A deterministic lost-confirmation/timeout scenario is a valid agent-release-test input; real BPF still executes both capital transitions on cloned state. C3 clears condition 2 because neither individual transaction nor a destination/cap policy detects the duplicate cumulative withdrawal.

## Findings

### P0 — C3 is killed on the wrong premise

`docs/H5-CANDIDATES.md:167-181`

A dropped confirmation is not a fabricated chain fact; it is an explicit client/transport fault input to the agent under test. The two withdrawals and their state diffs remain real BPF execution on cloned state. This is exactly an agent release-test concern: after a timeout, the agent must resolve transaction status before retrying.

A fixed episode can specify: T2 lands `Ok`; the agent-facing confirmation result is deterministically `Timeout`; the agent retries at T4. Each transaction is individually admissible, while the episode’s cumulative vault delta is wrong. A single `simulateTransaction` does not persist T2, and an allowlist/cap policy permits both transactions. Therefore KILL-2 does not fire.

### P0 — C1’s construction barrier cannot be dismissed by inspecting an unavailable T4

`docs/H5-CANDIDATES.md:103-117`

Under the stated pre-release CI framing, T4 does not exist until the earlier episode has executed. Saying its eventual post-state witnesses the bug silently grants the baseline a state-carrying local executor. That is H5’s distinguishing machinery, not one RPC `simulateTransaction`.

C1 is not sufficiently frozen to select branch B as-is—the exact `R` observation and the erroneous exit rule need specification—but its KILL-1 is unsupported.

### P1 — The “single maximal withdrawal” bound is expressly unproven but reused as decisive

`docs/H5-CANDIDATES.md:79-85, 195-206`

The record correctly says smaller-withdrawal reachability is not established read-only, then says the fixture rules out cumulative-walk candidates. H4 case E proves one sentinel call moved 486,657,686 collateral atoms; it does not, from committed post-state bytes alone, prove path-independence for smaller sequential calls.

### P2 — The fixture-reproduction snippet does not print its claimed derivation

`docs/H5-CANDIDATES.md:36-42`

The provided Python snippet only decodes bytes and defines helpers. It does not enumerate slots, decode pubkeys, or print values.

## Direct answers

1. **Fixture bound: mostly correct.** I independently found one nonzero deposit slot and one nonzero borrow slot:

   - Deposit slot 0: `D6q6…gJ59`, amount `2,248,785,777`; slots 1–7 are zero.
   - Borrow slot 0: `d4A2…Bc4Q`; slots 1–4 are zero.

   Thus the one-collateral/one-borrow layout holds. It excludes a two-collateral ordering case, but not C3’s duplicate-execution case.

2. **Second bound: not established.** Case E establishes `u64::MAX → 486,657,686` collateral moved, not that smaller sequential withdrawals cannot reach a distinct state. The document admits this. It cannot support branch A.

3. **C1: KILL-1 is not justified.** If T4 cannot be constructed until prior transactions execute, a bare single-transaction simulation cannot inspect its post-state in pre-release CI. This is a candidate construction barrier, not merely a missing assertion. C1 needs a complete fixed rule before it could proceed.

4. **C2: the stated KILL-1 assumes the disputed capability.** A production agent can simulate T4 against actual post-T3 chain state. Pre-release CI cannot produce that counterfactual state through RPC simulation alone. If the baseline is allowed to maintain a cloned evolving state, it is functionally H5; if not, C2 is not defeated by one simulation. The gate must choose explicitly.

5. **C3: KILL-2 is wrong.** A confirmation timeout is legitimately part of an agent release test. The harness must disclose and deterministically inject it, but that does not make the BPF state transition fabricated. It exercises a real client defect against real BPF state.

6. **A single RPC simulation may not be chained.** `simulateTransaction` does not commit or advance mainnet state. A baseline that persists post-state across simulations is a local-fork episode executor—the machinery H5 proposes. The §8.1 grant of a “post-T1 state” was therefore not a valid bare-single-simulation baseline and does not settle H5 generally.

7. **No fourth candidate is needed to reject branch A.** C3 already expresses the founder’s retry shape on this fixture, and C1 remains wrongly killed. C2 covers stale/slot advance. The search’s defect is adjudication, not necessarily its count.

8. **Branch A is incorrectly selected.** The appropriate outcome under the founder’s stated branching rule is not a kill proposal. C3 should be frozen completely for a founder ruling; C1/C2 also require the baseline definition to be resolved. The §7 resume condition is too narrow: C3 needs neither a second capital leg nor a partial fill. §§6–7 are not 延命; they are an overbroad closure argument.

9. **Numbers.** The four `_sf / 2^60` calculations are arithmetically correct:

   - `1192`: `2690.344408520984`
   - `2208`: `1343.591508013852`
   - `2240`: `2152.275526816787`
   - `2256`: `2421.309967668886`

   The 80% and 90% ratios follow. The $ labels are corroborated by H4’s frozen output, but bare obligation bytes alone do not establish the currency unit. The claim that case E reaches the protocol’s ultimate solvency boundary is not supported by committed post-state bytes.

10. **One change:** define the baseline operationally: either it is one non-persistent RPC simulation, or it may carry cloned post-state across transactions. Then specify C3’s deterministic timeout injection, exact agent retry rule, and cumulative state-diff predicate before any founder ruling.

## Independent derivation

Command used:

```sh
python3 -c $'import base64,json\nob=base64.b64decode(json.load(open("fixtures/h4/accounts/obligation.json"))["data_b64"])\nu64=lambda o:int.from_bytes(ob[o:o+8],"little")\nu128=lambda o:int.from_bytes(ob[o:o+16],"little")\nprint([(i,any(ob[96+i*136:96+(i+1)*136]),u64(96+i*136+32)) for i in range(8)])\nprint([(i,any(ob[1208+i*200:1208+(i+1)*200])) for i in range(5)])\nprint([(o,u128(o)/(1<<60)) for o in (1192,2208,2240,2256)])'
```

Output confirms exactly one live deposit slot with `2,248,785,777`, exactly one live borrow slot, and the four values above. I also decoded the nonzero reserve keys directly from the same bytes; they match the USDC and SOL reserves claimed.

## Read-only limits

I did not run a harness, simulate transactions, access the network, or inspect `../solvo`. I could not prove klend’s sequential-withdrawal path-independence, nor independently establish fixture provenance beyond its committed manifest and H4 evidence.
