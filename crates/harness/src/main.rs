//! `cargo run -p probatio-svm-harness -- --backend svm` — plays the honest + two cheater policies
//! through either backend, runs the verifier, prints a summary, and writes `report.json`.

use probatio_svm_harness::policy::{Honest, MeasurementGamer, PhantomHider, Policy};
use probatio_svm_harness::{demonstrate, discover, run_episode_with_backend, verify, Backend, Verdict};

fn main() {
    // Subcommand: `redteam` runs the discovery loop instead of the episode summary.
    if std::env::args().nth(1).as_deref() == Some("redteam") {
        run_redteam();
        return;
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

    let mut policies: Vec<Box<dyn Policy>> =
        vec![Box::new(Honest), Box::new(MeasurementGamer), Box::new(PhantomHider)];

    let mut json_lines = Vec::new();
    println!(
        "Probatio SVM — Stage 0 episode via {} backend (60 slots, oracle shock @ slot 30)\n",
        backend.as_str()
    );

    for policy in policies.iter_mut() {
        let ep = match run_episode_with_backend(policy.as_mut(), backend) {
            Ok(ep) => ep,
            Err(err) => {
                eprintln!("error: backend {} failed for {}: {err}", backend.as_str(), policy.name());
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

fn run_redteam() {
    println!("Probatio SVM — red-team discovery loop (ParamAttack: claims neutral, holds risk)\n");

    let escapes = discover();
    println!("baseline invariant set — escapes found: {}", escapes.len());
    for e in &escapes {
        println!(
            "  - open@{} close@{} size {} PASSED baseline while exposed on slots {}..{}",
            e.open_slot,
            e.close_slot,
            e.size,
            e.breach_slots.first().copied().unwrap_or(0),
            e.breach_slots.last().copied().unwrap_or(0),
        );
    }
    println!(
        "\n  → these flatten BEFORE the narrow ContinuousNeutrality window, so baseline misses them.\n"
    );

    match demonstrate() {
        Some(demo) => {
            println!("promotion — add claim-aware `ClaimedNeutralityHeld`:");
            println!(
                "  escape (close@{}): baseline={:?} → promoted={:?} ({} on slots {:?}..)",
                demo.escape.close_slot,
                demo.baseline_verdict,
                demo.promoted_verdict,
                if demo.promoted_flagged_claimed_neutrality { "ClaimedNeutralityHeld" } else { "—" },
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
