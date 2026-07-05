APPROVE

- `crates/harness/src/transcript.rs:29-94` `Transcript::capture()` is assembling the transcript from the expected sources: per-slot fields come from `ep.trace`, claim fields come from `ep.claim`, verdict/findings come from `report`, and finding kinds are serialized through `FindingKind::as_str()`. `to_json()` is deterministic in practice here: field insertion order is fixed in the `json!` literals, and `cargo run --offline -q -p probatio-svm-harness -- gallery --sample` produced byte-identical output on two consecutive runs.
- `crates/harness/src/main.rs:129-191` the `gallery` subcommand behavior matches the brief. `gallery --sample` succeeds without a key, writes `gallery/sample-scripted-drift.json`, and exits 0; plain `gallery` with `ANTHROPIC_API_KEY` unset prints the key-required message and exits 1; unknown args exit 2; `--hostile` selects the hostile ref path and names the output `gallery/neutral_mm-hostile.json`. `write_transcript()` creates the parent directory and writes a trailing newline.
- `gallery/sample-scripted-drift.json` matches the generated sample exactly. I verified the committed file hash before and after two `gallery --sample` runs (`bc5cfba2147a53c83a688d8da210ed21c6108cb7` both times). The artifact is format-accurate, has `verdict = "ShortcutDetected"`, includes a `ClaimTracksExposure` finding, and contains 60 slot records.
- `.gitignore:9-10` is set correctly: live `gallery/neutral_mm-*.json` transcripts are ignored, while the committed sample and `gallery/README.md` remain trackable.

Verification:
- `cargo test --offline` is green: 51 passed.
- `cargo run --offline -q -p probatio-svm-harness -- gallery --sample` succeeded twice with identical bytes.
- `env -u ANTHROPIC_API_KEY cargo run --offline -q -p probatio-svm-harness -- gallery` exited 1 with the expected guidance message.
- `env -u ANTHROPIC_API_KEY cargo run --offline -q -p probatio-svm-harness -- gallery --bogus` exited 2 with the expected usage message.

Residual note:
- `cargo test --offline` still prints the pre-existing `cargo_build_sbf::post_processing` warning about `sol_memcpy_`; I did not find any new warning introduced by this task.
