VERDICT: CHANGES

P1
- `crates/harness/src/hostile.rs:1-3` and `crates/harness/src/hostile.rs:155-176` overclaim that the delta-based findings are generally "price-noise invariant". The proof only covers `MeasurementGamer` and `PhantomHider`, which ignore price and act on slot alone. The public `Policy` contract does not have that restriction: `crates/harness/src/policy.rs:14-19` gives `act()` the full `Observation`, and `crates/harness/src/world.rs:332-341` passes `mark` and `free_collateral` every slot. A mark-reactive policy can therefore change its size timeline under hostility, which changes `ClaimTracksExposure` / `ContinuousNeutrality` evidence slots and can even remove the finding entirely for the same policy code. Concrete counterexample: a policy that opens only when `obs.slot == 30 && obs.mark == 40`, claims `delta = 0`, and closes at slot 60 is flagged under the clean path but passes under `HostileParams::hostile()` because slot 30 is mark 72 there. That means the branch demonstrates invariance for the current slot-scripted demos, not for the allowed policy surface or a future LLM agent. Suggested fix: narrow the docs/comments/tests to "for slot-scripted policies" or add a mark-reactive regression test that makes this boundary explicit.

P2
- `crates/harness/src/hostile.rs:66-71` is not robust for arbitrary public `HostileParams`: `let span = (2 * amp + 1) as u64;` overflows in `i64` before the cast when `noise_amp` is large. In debug builds that panics, so the advertised "deterministic bounded wiggle" is not actually guaranteed for all accepted inputs. Suggested fix: validate or clamp `noise_amp`, switch it to an unsigned bounded type, or use checked/saturating arithmetic and reject unsupported amplitudes.

Notes
- `cargo test --offline` is green. The harness crate reports 38 tests, and I did not find a clean-path verdict regression in the shipped policies.

## Round 2

VERDICT: CHANGES

P1 status: PARTIALLY RESOLVED
- Resolved in code/tests: `crates/harness/src/hostile.rs:4-12` now scopes the invariance claim to a fixed action sequence / slot-scripted policies, `misrepresentation_is_price_noise_invariant_for_slot_scripted_policies` is correctly renamed and narrowly framed at `crates/harness/src/hostile.rs:167-191`, and the new `MarkReactiveGamer` at `crates/harness/src/policy.rs:157-182` is genuinely price-reactive because its open condition depends on `obs.mark < 45` rather than slot alone. The boundary test `price_reactive_policy_is_not_price_invariant` at `crates/harness/src/hostile.rs:193-202` does make the intended boundary explicit by asserting the clean/hostile `measured_delta` sequences differ.
- Still overclaimed in the task brief: `docs/tasks/006-hostile-episodes.md:13-17` and `docs/tasks/006-hostile-episodes.md:58-59` still state the misrepresentation invariants are generally price-noise invariant because "Policies act on slots, not on the mark", which is false for the public `Policy` surface and now contradicted by `MarkReactiveGamer`. The narrowed scope needs to be reflected there too.
- Re-check on correctness: I re-confirmed no hostility-induced false positive on honest (`crates/harness/src/hostile.rs:222-227` keeps `Honest` at `Pass` under hostility), and `StressBoundary` being flagged only under hostile remains a genuine stress-relative solvency result, not a verifier bug (`crates/harness/src/policy.rs:133-155`, `crates/harness/src/hostile.rs:204-219`).

P2 status: NOT RESOLVED
- `crates/harness/src/hostile.rs:75-82` fixes the original `2 * amp + 1` overflow, but `noise()` still has an overflow/sign bug for large public `noise_amp`. With `amp = i64::MAX`, `span` saturates to `u64::MAX`, `(slot.wrapping_mul(..) % span) as i64` can wrap negative for sufficiently large `slot`, and the final `- amp` then overflows in debug builds. I reproduced the panic with the current implementation using `slot=1_000_000_000_000` and `amp=i64::MAX`. So the function is still not safe for the full accepted input range.

Round 2 notes
- `cargo test --offline` is green: 39 harness tests passed, plus the rest of the workspace tests.
- I did not find any verdict regression in existing policies beyond the intended hostile-only `StressBoundary` insolvency case.
