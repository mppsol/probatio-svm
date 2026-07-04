VERDICT: CHANGES

P1
- `crates/harness/src/hostile.rs:1-3` and `crates/harness/src/hostile.rs:155-176` overclaim that the delta-based findings are generally "price-noise invariant". The proof only covers `MeasurementGamer` and `PhantomHider`, which ignore price and act on slot alone. The public `Policy` contract does not have that restriction: `crates/harness/src/policy.rs:14-19` gives `act()` the full `Observation`, and `crates/harness/src/world.rs:332-341` passes `mark` and `free_collateral` every slot. A mark-reactive policy can therefore change its size timeline under hostility, which changes `ClaimTracksExposure` / `ContinuousNeutrality` evidence slots and can even remove the finding entirely for the same policy code. Concrete counterexample: a policy that opens only when `obs.slot == 30 && obs.mark == 40`, claims `delta = 0`, and closes at slot 60 is flagged under the clean path but passes under `HostileParams::hostile()` because slot 30 is mark 72 there. That means the branch demonstrates invariance for the current slot-scripted demos, not for the allowed policy surface or a future LLM agent. Suggested fix: narrow the docs/comments/tests to "for slot-scripted policies" or add a mark-reactive regression test that makes this boundary explicit.

P2
- `crates/harness/src/hostile.rs:66-71` is not robust for arbitrary public `HostileParams`: `let span = (2 * amp + 1) as u64;` overflows in `i64` before the cast when `noise_amp` is large. In debug builds that panics, so the advertised "deterministic bounded wiggle" is not actually guaranteed for all accepted inputs. Suggested fix: validate or clamp `noise_amp`, switch it to an unsigned bounded type, or use checked/saturating arithmetic and reject unsupported amplitudes.

Notes
- `cargo test --offline` is green. The harness crate reports 38 tests, and I did not find a clean-path verdict regression in the shipped policies.
