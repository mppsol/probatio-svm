//! Deterministic, offline attestation receipts for the Solana Agent Registry reputation path.

use solana_address::Address;

use crate::{spec_hash, ShortcutReport, Verdict};
use probatio_contract::MandateSpec;

/// Enough information for a third party to re-run the certified episode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reproduce {
    pub policy: String,
    pub backend: String,
    pub n_slots: u64,
}

/// The exact offline-prepared arguments for Reputation Registry `giveFeedback`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeedbackCall {
    pub agent_asset: [u8; 32],
    pub value: u8,
    pub tag: String,
    pub feedback_uri: String,
}

impl FeedbackCall {
    /// Canonical JSON for the on-chain call arguments. Sending remains a later, explicit step.
    pub fn to_json(&self) -> String {
        format!(
            "{{\"agent\":\"{}\",\"value\":{},\"tag\":\"{}\",\"feedback_uri\":\"{}\"}}",
            Address::from(self.agent_asset),
            self.value,
            json_escape(&self.tag),
            json_escape(&self.feedback_uri),
        )
    }
}

/// A canonical receipt plus the call arguments that reference its future pinned location.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attestation {
    pub receipt_json: String,
    pub call: FeedbackCall,
}

/// Turn an offline certification verdict into a deterministic, re-runnable receipt.
///
/// `captured_at` is supplied by the caller; this function never reads a clock, accesses the network,
/// signs, or submits a transaction. The receipt is Probatio's assessment, not a claim by the agent
/// operator.
pub fn attest(
    agent_asset: [u8; 32],
    spec: &MandateSpec,
    report: &ShortcutReport,
    reproduce: Reproduce,
    feedback_uri: String,
    captured_at: u64,
) -> Attestation {
    let (value, verdict) = match report.verdict {
        Verdict::Pass => (100, "PASS"),
        Verdict::ShortcutDetected => (0, "FLAG"),
    };
    let call = FeedbackCall {
        agent_asset,
        value,
        tag: "re-exec".to_string(),
        feedback_uri,
    };
    let findings = report
        .findings
        .iter()
        .map(|finding| {
            let slots = finding
                .evidence_slots
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"kind\":\"{}\",\"evidence_slots\":[{}]}}",
                finding.kind.as_str(),
                slots,
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let receipt_json = format!(
        concat!(
            "{{\"schema\":\"probatio.attestation.v1\",",
            "\"attester\":\"probatio\",",
            "\"agent\":\"{}\",",
            "\"mandate_spec_hash\":\"{}\",",
            "\"reproduce\":{{\"policy\":\"{}\",\"backend\":\"{}\",\"n_slots\":{}}},",
            "\"verdict\":\"{}\",",
            "\"findings\":[{}],",
            "\"report_hash\":\"{}\",",
            "\"captured_at\":{}",
            "}}"
        ),
        Address::from(agent_asset),
        hex(&spec_hash(spec)),
        json_escape(&reproduce.policy),
        json_escape(&reproduce.backend),
        reproduce.n_slots,
        verdict,
        findings,
        hex(&report_hash(report)),
        captured_at,
    );

    Attestation { receipt_json, call }
}

/// Stage-0 identity tag over the exact bytes of `ShortcutReport::to_json()`.
///
/// This is FNV-1a-64 zero-extended to 32 bytes, not a security hash.
// TODO(reexec-core): replace with keccak256 when the shared engine is extracted (Reckn already ships it).
pub fn report_hash(report: &ShortcutReport) -> [u8; 32] {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in report.to_json().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let mut out = [0u8; 32];
    out[..8].copy_from_slice(&hash.to_le_bytes());
    out
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn json_escape(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => {
                use core::fmt::Write;
                let _ = write!(out, "\\u{:04x}", ch as u32);
            }
            ch => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Finding, FindingKind};
    use serde_json::Value;

    fn reproduce() -> Reproduce {
        Reproduce {
            policy: "honest".to_string(),
            backend: "ref".to_string(),
            n_slots: 60,
        }
    }

    fn pass_report() -> ShortcutReport {
        ShortcutReport {
            policy: "honest".to_string(),
            verdict: Verdict::Pass,
            findings: vec![],
        }
    }

    fn flag_report() -> ShortcutReport {
        ShortcutReport {
            policy: "cheater".to_string(),
            verdict: Verdict::ShortcutDetected,
            findings: vec![Finding {
                kind: FindingKind::MandateDeviation,
                detail: "outside mandate".to_string(),
                evidence_slots: vec![4, 5],
            }],
        }
    }

    #[test]
    fn pass_receipt_maps_to_full_feedback_and_canonical_identifiers() {
        let agent = [7u8; 32];
        let spec = MandateSpec::stage0_default();
        let attestation = attest(
            agent,
            &spec,
            &pass_report(),
            reproduce(),
            "ipfs://receipt".to_string(),
            123,
        );
        assert_eq!(attestation.call.value, 100);
        assert!(attestation.receipt_json.contains("\"verdict\":\"PASS\""));
        assert!(attestation.receipt_json.contains(&format!("\"agent\":\"{}\"", Address::from(agent))));
        assert!(attestation.receipt_json.contains(&hex(&spec_hash(&spec))));
        let receipt: Value = serde_json::from_str(&attestation.receipt_json).unwrap();
        assert_eq!(receipt["schema"], "probatio.attestation.v1");
        assert_eq!(receipt["captured_at"], 123u64);
    }

    #[test]
    fn flag_receipt_maps_to_zero_feedback_and_serializes_findings() {
        let attestation = attest(
            [8u8; 32],
            &MandateSpec::stage0_default(),
            &flag_report(),
            reproduce(),
            "https://example.invalid/receipt".to_string(),
            123,
        );
        assert_eq!(attestation.call.value, 0);
        assert!(attestation.receipt_json.contains("\"verdict\":\"FLAG\""));
        assert!(attestation.receipt_json.contains("\"kind\":\"MandateDeviation\""));
        assert!(attestation.receipt_json.contains("\"evidence_slots\":[4,5]"));
    }

    #[test]
    fn receipt_is_byte_identical_for_identical_inputs() {
        let report = flag_report();
        let spec = MandateSpec::stage0_default();
        let first = attest(
            [9u8; 32],
            &spec,
            &report,
            reproduce(),
            "ipfs://receipt".to_string(),
            456,
        );
        let second = attest(
            [9u8; 32],
            &spec,
            &report,
            reproduce(),
            "ipfs://receipt".to_string(),
            456,
        );
        assert_eq!(first, second);
    }

    #[test]
    fn report_hash_is_stable_and_sensitive_to_report_contents() {
        let pass = pass_report();
        assert_eq!(report_hash(&pass), report_hash(&pass));
        assert_ne!(report_hash(&pass), report_hash(&flag_report()));
    }
}
