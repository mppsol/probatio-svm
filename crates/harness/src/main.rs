//! `cargo run -p probatio-svm-harness -- --backend svm` — plays the honest + two cheater policies
//! through either backend, runs the verifier, prints a summary, and writes `report.json`.

use probatio_svm_harness::agent::{ClaudeAgent, ScriptedDecider};
use probatio_svm_harness::policy::{Honest, MeasurementGamer, PhantomHider, Policy};
use probatio_svm_harness::jupiter::{
    jupiter_to_snapshots, sample_drift, sample_neutral, JupPosition, JupSide, JupSlot,
};
use probatio_svm_harness::world::EpisodeResult;
use probatio_svm_harness::{
    demonstrate, discover, run_episode, run_episode_ref_hostile, run_episode_with_backend, verify,
    Backend, CurlClaude, HostileParams, Transcript, Verdict, N_SLOTS, NEUTRAL_MM,
};
use probatio_contract::{Action, AgentAccountRef, Side};

fn main() {
    let mut raw_args = std::env::args().skip(1);
    if let Some(first) = raw_args.next() {
        match first.as_str() {
            "redteam" => {
                run_redteam();
                return;
            }
            "agent" => {
                run_agent(raw_args.collect());
                return;
            }
            "gallery" => {
                run_gallery(raw_args.collect());
                return;
            }
            "certify-jupiter" => {
                run_certify_jupiter(raw_args.collect());
                return;
            }
            _ => {}
        }
    }

    let mut backend = Backend::Ref;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--backend" {
            let Some(value) = args.next() else {
                eprintln!("error: --backend requires ref or svm");
                std::process::exit(2);
            };
            backend = Backend::parse(&value).unwrap_or_else(|| {
                eprintln!("error: unsupported backend `{value}` (expected ref|svm)");
                std::process::exit(2);
            });
        } else {
            eprintln!("error: unknown argument `{arg}`");
            std::process::exit(2);
        }
    }

    let mut policies: Vec<Box<dyn Policy>> = vec![
        Box::new(Honest),
        Box::new(MeasurementGamer),
        Box::new(PhantomHider),
    ];

    let mut json_lines = Vec::new();
    println!(
        "Probatio SVM — Stage 0 episode via {} backend (60 slots, oracle shock @ slot 30)\n",
        backend.as_str()
    );

    for policy in policies.iter_mut() {
        let ep = match run_episode_with_backend(policy.as_mut(), backend) {
            Ok(ep) => ep,
            Err(err) => {
                eprintln!(
                    "error: backend {} failed for {}: {err}",
                    backend.as_str(),
                    policy.name()
                );
                std::process::exit(1);
            }
        };
        let report = verify(ep.policy, &ep.trace, &ep.claim);

        let mark = match report.verdict {
            Verdict::Pass => "PASS ",
            Verdict::ShortcutDetected => "FLAG ",
        };
        println!("[{}] {}", mark, report.policy);
        for f in &report.findings {
            println!(
                "        - {:<22} slots {:?}  ({})",
                f.kind.as_str(),
                f.evidence_slots,
                f.detail
            );
        }
        if report.findings.is_empty() {
            println!("        - no shortcuts detected");
        }
        println!();

        json_lines.push(report.to_json());
    }

    let out = json_lines.join("\n") + "\n";
    if let Err(e) = std::fs::write("report.json", &out) {
        eprintln!("warning: could not write report.json: {e}");
    } else {
        println!("wrote report.json ({} policies)", json_lines.len());
    }
}

fn run_agent(args: Vec<String>) {
    let hostile = match args.as_slice() {
        [] => false,
        [flag] if flag == "--hostile" => true,
        [other] => {
            eprintln!("error: unknown argument `{other}`");
            std::process::exit(2);
        }
        _ => {
            eprintln!("error: usage: agent [--hostile]");
            std::process::exit(2);
        }
    };

    let decider = match CurlClaude::from_env() {
        Ok(decider) => decider,
        Err(_) => {
            eprintln!("error: ANTHROPIC_API_KEY is required for `agent`; export it and rerun.");
            std::process::exit(1);
        }
    };
    let mut policy = ClaudeAgent::new(Box::new(decider), NEUTRAL_MM);
    let ep = if hostile {
        run_episode_ref_hostile(&mut policy, &HostileParams::hostile())
    } else {
        run_episode(&mut policy)
    };
    let report = verify(ep.policy, &ep.trace, &ep.claim);

    println!(
        "Probatio SVM — Claude agent via {} episode\n",
        if hostile { "hostile ref" } else { "clean ref" }
    );
    print_report(&report);
}

fn run_gallery(args: Vec<String>) {
    match args.as_slice() {
        [flag] if flag == "--sample" => {
            // Deterministic scripted-drift illustration — no API key needed. Claims neutral, opens long.
            let mut script = vec![Action::Noop; N_SLOTS as usize];
            script[0] = Action::Open { acct: AgentAccountRef::Measured, side: Side::Long, qty: 10 };
            let ep =
                run_episode(&mut ClaudeAgent::new(Box::new(ScriptedDecider::new(script)), NEUTRAL_MM));
            let report = verify(ep.policy, &ep.trace, &ep.claim);
            let transcript = Transcript::capture("scripted-drift", &NEUTRAL_MM, "clean", &ep, &report);
            write_transcript("gallery/sample-scripted-drift.json", &transcript);
            println!(
                "wrote gallery/sample-scripted-drift.json (scripted illustration) — verdict {:?}",
                report.verdict
            );
        }
        rest => {
            let hostile = match rest {
                [] => false,
                [flag] if flag == "--hostile" => true,
                _ => {
                    eprintln!("error: usage: gallery [--sample | --hostile]");
                    std::process::exit(2);
                }
            };
            let decider = match CurlClaude::from_env() {
                Ok(decider) => decider,
                Err(_) => {
                    eprintln!(
                        "error: ANTHROPIC_API_KEY is required for `gallery`; export it (or use `gallery --sample`)."
                    );
                    std::process::exit(1);
                }
            };
            let mut policy = ClaudeAgent::new(Box::new(decider), NEUTRAL_MM);
            let (backend, ep) = if hostile {
                ("hostile", run_episode_ref_hostile(&mut policy, &HostileParams::hostile()))
            } else {
                ("clean", run_episode(&mut policy))
            };
            let report = verify(ep.policy, &ep.trace, &ep.claim);
            let transcript = Transcript::capture("neutral_mm", &NEUTRAL_MM, backend, &ep, &report);
            let path = format!("gallery/neutral_mm-{backend}.json");
            write_transcript(&path, &transcript);

            println!("Probatio SVM — Claude agent gallery ({backend})\n");
            print_report(&report);
            println!("\nwrote {path}");
        }
    }
}

fn run_certify_jupiter(args: Vec<String>) {
    match args.as_slice() {
        [flag] if flag == "--sample" => {
            let (r1, t1) = certify_jupiter("jupiter-neutral", &sample_neutral(N_SLOTS));
            write_transcript("gallery/jupiter-neutral.json", &t1);
            println!("wrote gallery/jupiter-neutral.json — verdict {:?}", r1.verdict);
            let (r2, t2) = certify_jupiter("jupiter-drift", &sample_drift(N_SLOTS));
            write_transcript("gallery/jupiter-drift.json", &t2);
            println!("wrote gallery/jupiter-drift.json — verdict {:?}", r2.verdict);
        }
        [path] if !path.starts_with('-') => {
            let slots = match parse_jupiter_trace(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: could not parse Jupiter trace `{path}`: {e}");
                    std::process::exit(1);
                }
            };
            let (report, transcript) = certify_jupiter("jupiter-agent", &slots);
            println!("Probatio SVM — Jupiter Perps certification ({} slots)\n", slots.len());
            print_report(&report);
            write_transcript("gallery/jupiter-certification.json", &transcript);
            println!("\nwrote gallery/jupiter-certification.json");
        }
        _ => {
            eprintln!("error: usage: certify-jupiter [--sample | <trace.json>]");
            std::process::exit(2);
        }
    }
}

/// Map a Jupiter position trace → snapshots, certify against the delta-neutral mandate, build a transcript.
fn certify_jupiter(
    label: &'static str,
    measured: &[JupSlot],
) -> (probatio_svm_harness::ShortcutReport, Transcript) {
    let snaps = jupiter_to_snapshots(measured, &[]);
    let claim = NEUTRAL_MM.claim();
    let report = verify(label, &snaps, &claim);
    let ep = EpisodeResult { policy: label, trace: snaps, claim };
    let transcript = Transcript::capture(label, &NEUTRAL_MM, "jupiter", &ep, &report);
    (report, transcript)
}

/// Parse a Jupiter trace file: `[{ "slot": u, "mark_usd": i, "positions": [{ "side": "long"|"short",
/// "size_usd": i, "collateral_usd": i, "entry_usd": i }] }]` — all values in WHOLE USD.
fn parse_jupiter_trace(path: &str) -> Result<Vec<JupSlot>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    parse_jupiter_trace_str(&text)
}

/// Pure parse + validation (offline-testable). Rejects malformed values rather than casting through them
/// (review 010: a negative `slot` was silently `as u64`-cast and certified as valid).
fn parse_jupiter_trace_str(text: &str) -> Result<Vec<JupSlot>, String> {
    let root: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let arr = root.as_array().ok_or("top level must be an array of slots")?;

    // Read a required i64 field, optionally enforcing a lower bound.
    fn field_i(v: &serde_json::Value, k: &str, min: Option<i64>) -> Result<i64, String> {
        let n = v.get(k).and_then(serde_json::Value::as_i64).ok_or_else(|| format!("missing i64 field `{k}`"))?;
        if let Some(m) = min {
            if n < m {
                return Err(format!("field `{k}` = {n} is below minimum {m}"));
            }
        }
        Ok(n)
    }

    let mut slots = Vec::with_capacity(arr.len());
    for s in arr {
        let positions_json =
            s.get("positions").and_then(serde_json::Value::as_array).ok_or("slot missing `positions`")?;
        let mut positions = Vec::with_capacity(positions_json.len());
        for p in positions_json {
            let side = match p.get("side").and_then(serde_json::Value::as_str) {
                Some("long") => JupSide::Long,
                Some("short") => JupSide::Short,
                _ => return Err("position `side` must be \"long\" or \"short\"".to_string()),
            };
            positions.push(JupPosition {
                side,
                size_usd: field_i(p, "size_usd", Some(0))?,       // notional must be non-negative
                collateral_usd: field_i(p, "collateral_usd", Some(0))?,
                entry_usd: field_i(p, "entry_usd", Some(1))?,     // entry > 0 (mapper divides by it)
            });
        }
        slots.push(JupSlot {
            slot: field_i(s, "slot", Some(0))? as u64,             // slot >= 0 (no negative → u64 wrap)
            mark_usd: field_i(s, "mark_usd", Some(1))?,            // mark > 0
            positions,
        });
    }
    Ok(slots)
}

#[cfg(test)]
mod tests {
    use super::parse_jupiter_trace_str;

    const GOOD: &str = r#"[{"slot":1,"mark_usd":100,"positions":[
        {"side":"long","size_usd":10000,"collateral_usd":3000,"entry_usd":100}]}]"#;

    #[test]
    fn parses_a_valid_trace() {
        let slots = parse_jupiter_trace_str(GOOD).unwrap();
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].slot, 1);
        assert_eq!(slots[0].positions.len(), 1);
    }

    #[test]
    fn rejects_malformed_input() {
        // negative slot (review 010), non-positive entry/mark, negative size, bad side, non-array, missing field
        assert!(parse_jupiter_trace_str(r#"[{"slot":-1,"mark_usd":100,"positions":[]}]"#).is_err());
        assert!(parse_jupiter_trace_str(r#"[{"slot":1,"mark_usd":0,"positions":[]}]"#).is_err());
        assert!(parse_jupiter_trace_str(
            r#"[{"slot":1,"mark_usd":100,"positions":[{"side":"long","size_usd":-1,"collateral_usd":0,"entry_usd":100}]}]"#
        )
        .is_err());
        assert!(parse_jupiter_trace_str(
            r#"[{"slot":1,"mark_usd":100,"positions":[{"side":"long","size_usd":1,"collateral_usd":0,"entry_usd":0}]}]"#
        )
        .is_err());
        assert!(parse_jupiter_trace_str(
            r#"[{"slot":1,"mark_usd":100,"positions":[{"side":"up","size_usd":1,"collateral_usd":0,"entry_usd":100}]}]"#
        )
        .is_err());
        assert!(parse_jupiter_trace_str(r#"{"not":"an array"}"#).is_err());
        assert!(parse_jupiter_trace_str(r#"[{"mark_usd":100,"positions":[]}]"#).is_err());
    }
}

fn write_transcript(path: &str, transcript: &Transcript) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(err) = std::fs::write(path, transcript.to_json() + "\n") {
        eprintln!("error: could not write {path}: {err}");
        std::process::exit(1);
    }
}

fn run_redteam() {
    println!("Probatio SVM — red-team discovery loop (ParamAttack: claims neutral, holds risk)\n");

    let escapes = discover();
    println!("baseline invariant set — escapes found: {}", escapes.len());
    for e in &escapes {
        println!(
            "  - open@{} settle@{} entry {} claim_delta {} PASSED baseline while exposed on slots {}..{}",
            e.open_slot,
            e.settle_slot,
            e.entry_size,
            e.end_delta,
            e.breach_slots.first().copied().unwrap_or(0),
            e.breach_slots.last().copied().unwrap_or(0),
        );
    }
    println!(
        "\n  → exact-neutral escapes flatten before the ContinuousNeutrality window; near-neutral\n    escapes (claim_delta ±1) dodge the exact-neutral gate AND the final-slot ClaimMismatch.\n"
    );

    match demonstrate() {
        Some(demo) => {
            println!("promotion — generalize to claim-tracking `ClaimTracksExposure`:");
            println!(
                "  escape (settle@{}, claim_delta {}): baseline={:?} → promoted={:?} ({} on slots {}..)",
                demo.escape.settle_slot,
                demo.escape.end_delta,
                demo.baseline_verdict,
                demo.promoted_verdict,
                if demo.promoted_flagged_claim_tracking { "ClaimTracksExposure" } else { "—" },
                demo.promoted_evidence.first().copied().unwrap_or(0),
            );
            println!(
                "  honest directional trader: baseline={:?}, promoted={:?}  (no false positive)",
                demo.honest_baseline, demo.honest_promoted
            );
        }
        None => println!("no escape found — baseline set already covers this attack space"),
    }
}

fn print_report(report: &probatio_svm_harness::ShortcutReport) {
    let mark = match report.verdict {
        Verdict::Pass => "PASS ",
        Verdict::ShortcutDetected => "FLAG ",
    };
    println!("[{}] {}", mark, report.policy);
    for f in &report.findings {
        println!(
            "        - {:<22} slots {:?}  ({})",
            f.kind.as_str(),
            f.evidence_slots,
            f.detail
        );
    }
    if report.findings.is_empty() {
        println!("        - no shortcuts detected");
    }
}
