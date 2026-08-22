<!--
Independent review of docs/H5-GATE.md §8.1 — the PROPOSED, NOT-IN-FORCE freeze of the five items
gate §8 left open. Requested by CC after the founder said to continue; one round.
Reviewer: Codex, run read-only against commit 174fff2:
  /Applications/ChatGPT.app/Contents/Resources/codex exec \
    -C /Users/hiroyusai/src/probatio-svm -s read-only "<prompt>" < /dev/null

CC wrote every line under review, so this is a clean cross-review — Codex wrote none of it. Codex did
write crates/h4-sentinel, whose committed evidence §8.1 cites as input; its answer to Q2 discloses
where it leans on that.

The prompt required an adversarial reading, forbade helping H5 pass, and explicitly invited the
verdict "KILL AT DESIGN GATE" as the cheapest good outcome. The body below is Codex's output,
verbatim and unedited.

VERDICT: KILL AT DESIGN GATE — KILL-1. The §8.1 candidate is dead. Whether H5 ITSELF is dead is a
founder ruling, not this review's to make, and not CC's: §8.1 was never in force, and §8.1 itself
pre-registered that no substitute candidate may be swapped in without a new founder ruling and a new
G0 row. See docs/H5-GATE.md §8.1's REJECTED banner and STATUS.md.
-->

# H5 G0 Agent Release Tests Review

**Verdict: KILL AT DESIGN GATE — KILL-1.** Given the proposed `S-generous` baseline, T2’s returned post-state contains the entire witness for `F`. Applying the fixed agent rule to that state detects the failure in one simulation; three transactions add no detection capability.

## Findings

### P0 — `S-generous` catches `F` at T2; H5 is equivalent to a single simulation

`docs/H5-GATE.md:198-254`

`F` is `P1 && P2`. P1 is the nonzero `obligation.deposits[USDC].deposited_amount`; P2 follows deterministically from the agent rule’s `Ok → believed_remaining := 0`. Both are knowable after a single simulation of T2 from post-T1 state if returned account data is available. The same klend field parser required by H5 can parse the returned obligation bytes.

The proposed escape—that the simulator “does not know intent”—is not structural. The intent is the scripted agent source at `docs/H5-GATE.md:219-226`, available to the same developer invoking simulation. A simulator-side regression assertion can simply state: “after `withdraw(u64::MAX)`, `deposited_amount == 0`; otherwise this agent’s `Ok` branch incorrectly exits.” No third transaction or new oracle is required.

This fires **KILL-1 now**, not after a ritual run that can only reproduce the already known result.

### P1 — `S-generous` is governing in prose but not mechanically specified

`docs/H5-GATE.md:235-254`

The proposal does bar retreating to `S-strict`: it expressly says `S-generous` “governs” and that its detection fires KILL-1. But “allowed to read the post-state accounts it returns and diff them” does not pin:

- which returned accounts must be requested;
- which field/parser is applied;
- the exact predicate that constitutes “catches `F`;
- whether the declared agent rule is supplied to the baseline.

That leaves room to run `S-generous`, observe `1,762,128,091`, then call it non-detection because the baseline was not handed the intent. That is merely withholding the test oracle, not a capability difference.

### P1 — `Pol` is selected after observing a favorable realized amount and does not define an enforceable policy semantics

`docs/H5-GATE.md:171-181`, `docs/H5-GATE.md:238-242`

The 1,000,000,000-atom cap sits above the already-known realized output of 582,271,854, while the call requests `u64::MAX` and purports to exit a 2,248,785,777-atom position. The proposal does not say whether a policy caps requested instruction input, maximum possible output, or post-execution realized output. A pre-sign wallet policy ordinarily acts on requested/decoded transaction data; a reasonable one would reject `u64::MAX`, reject an unbounded sentinel, or require the postcondition for an exit action.

The stated `Pol` can be made to miss only by defining the cap over the known realized output. That is not a fair baseline until its timing, units, and instruction decoding are frozen.

### P1 — T3 is not a concrete transaction and does not witness the failure

`docs/H5-GATE.md:192-196`, `docs/H5-GATE.md:219-231`

T3 is “whatever” the rule emits. In the actual defective branch it emits `EXIT-COMPLETE action`, not a named instruction or transaction. Thus the proposed false branch may not meet A’s “≥3 transactions” requirement at all.

More importantly, T3 does no causal work: P1 already holds after T2, and P2 is already implied by the `Ok` branch. T3 only labels the known error as an exit. The multi-transaction differentiator is therefore theatre for this candidate.

### P1 — The CI “entrypoint” is a placeholder, not a release-process attachment

`docs/H5-GATE.md:256-271`

`<one command> --episode <episode.json>` is an interface sketch, not a command, package, executable, agent-decision-rule input, or episode format. It cannot presently be attached to an agent developer’s pipeline.

This does not independently prove that no concrete entrypoint can ever be defined, so I do not separately call KILL-4. It does mean §8.1 cannot be ratified as the promised concrete freeze. Making it genuinely reusable for “their own agent” would require a defined agent/intention interface; otherwise it is only a hard-coded klend test.

### P1 — `F` is a retrospective demonstration, not a prospectively found failure

`docs/H5-GATE.md:159-167`, `docs/H5-GATE.md:288-298`

Disclosure is good but insufficient. H4 had already exposed the successful `u64::MAX` call and its token deltas before H5 selected it. H5 then constructs an agent whose defect is precisely “treat `Ok` from that call as full exit.” That is post-selection from known evidence, even if it was not “hunted inside H5.”

It should be recorded as a **retrospective replay fixture**, not evidence that H5 found a previously unavailable release failure. In any event, it is now a known counterexample to H5 because `S-generous` sees it.

### P2 — The freeze does not pin one BPF identity or all execution inputs

`docs/H5-GATE.md:185-196`

H5 names fixture slot and says “real `klend` + `farms` BPF,” but does not select `klend_old` versus `klend_new` by hash. Nor does it freeze the required H4 mutations (obligation-owner replacement and deterministic destination account). The committed H4 harness documents these inputs, but §8.1 needs to name them itself before a measurement.

## Direct answers

1. **B falsifiable? — No, not adequately.** The governing language prevents an overt retreat to `S-strict`, but `S-generous` does not freeze its returned accounts or decision predicate. A future run could observe the decisive state and declare it non-detection by withholding the agent rule.

2. **`S-generous` correctly specified? — No. H5 is already dead: KILL-1.** Given the stipulated ability to return post-execution accounts, one simulation of T2 can read `deposited_amount = 1,762,128,091`. Combined with the fixed `Ok → EXITED` rule, it detects F. The identical parser is already present in the H4 evidence tooling. I authored `crates/h4-sentinel`; I rely here only on its committed parser/evidence as disclosed input.

3. **Intent oracle? — Relabelling, not a structural difference.** “Intent” is the line `on result Ok -> believed_remaining := 0`, plus the intended postcondition “remaining collateral is zero.” It can be stated for this one workflow without a protocol-independent schema, but then the simulator test can use it too. If H5 tries to make that intent portable across arbitrary agents and protocols, it needs an interface for effects/postconditions and re-enters H3’s schema problem.

4. **Does F need three transactions? — No; T3 is theatre.** T2 fully witnesses the nonzero position and the erroneous `Ok` belief. T3 is neither needed to detect F nor concretely specified as a transaction in the defective branch.

5. **Provenance of F? — Disclosure is insufficient for the claimed evidentiary role.** Naming known H4 data is legitimate only as a retrospective regression fixture. It cannot demonstrate that H5 newly finds a failure unavailable to its baselines.

6. **Pol and cap? — The cap is chosen to be beatable.** It is above the known 582,271,854 realized flow. A reasonable pre-sign policy would likely flag `u64::MAX`, an unbounded sentinel, or a requested full exit lacking a bounded maximum. A policy literally comparing requested raw amount to a 1,000,000,000 cap rejects `u64::MAX`; if collateral and USDC units differ, that itself proves `Pol` needs an explicit unit/conversion definition.

7. **KILL-4 entrypoint? — Placeholder, not a real entrypoint.** The contract describes desired behavior but supplies neither a runnable command nor the integration boundary for a developer’s own agent. It requires product/interface work before it can be judged.

8. **H1 revival? — No, on the present record.** §8.1 does not create an attestation, certificate, population claim, or use H1 as demand evidence. Its problem is not H1 revival; it is that the purported release-test distinction collapses into a one-transaction assertion.

9. **Offline execution? — Fixture data is offline; the proposed measurement is not executable from evidence alone.** No new fetch or protocol is needed for the named klend fixture. But new code is required for: a three-transaction driver, actual returned-account simulation semantics, the scripted-agent comparison, the baseline implementations, trace output, and any CI executable. The committed H4 evidence records hashes/ranges, not the actual returned post-state account bytes needed to run H5 directly. H5 also fails to pin a BPF hash and mutations.

10. **One change to make a fake pass harder:** require `S-generous` to return the obligation account and apply the exact H5 predicate plus fixed agent rule immediately after T2; explicitly state that this is a KILL-1 if it fires. That change honestly kills this candidate, which is the correct outcome.

## Read-only limits

I did not run a measurement, modify files, access the network, or inspect `../solvo`. I did not independently establish H4’s external mainnet/Solvo provenance; its decision record already identifies that limitation. I accepted the prompt’s stipulated premise that a real `simulateTransaction` can return requested post-execution account data.
