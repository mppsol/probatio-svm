//! `cargo run -p probatio-svm-harness` — plays the honest + two cheater policies through the Stage 0a
//! reference world, runs the verifier, prints a summary, and writes `report.json`.

use probatio_svm_harness::policy::{Honest, MeasurementGamer, PhantomHider, Policy};
use probatio_svm_harness::{run_episode, verify, Verdict};

fn main() {
    let mut policies: Vec<Box<dyn Policy>> =
        vec![Box::new(Honest), Box::new(MeasurementGamer), Box::new(PhantomHider)];

    let mut json_lines = Vec::new();
    println!("Probatio SVM — Stage 0a reference episode (60 slots, oracle shock @ slot 30)\n");

    for policy in policies.iter_mut() {
        let ep = run_episode(policy.as_mut());
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
