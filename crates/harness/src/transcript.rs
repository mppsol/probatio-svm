//! Certification transcript (Task 009): a serializable record of what an agent did over an episode +
//! Probatio's verdict. Pure and offline-testable; the live gallery writes these to `gallery/`.

use serde_json::{json, Value};

use crate::agent::Mandate;
use crate::verifier::{ShortcutReport, Verdict};
use crate::world::EpisodeResult;

/// One slot of the agent's actual state (from the ground-truth trace).
pub struct SlotRecord {
    pub slot: u64,
    pub mark: i64,
    pub measured_delta: i64,
    pub aggregate_delta: i64,
    pub any_liquidatable: bool,
}

/// A full certification transcript: the mandate the agent was given, its per-slot exposure, and the
/// verifier's verdict + findings.
pub struct Transcript {
    pub label: String,
    pub system: String,
    pub claimed_delta: i64,
    pub claims_solvent: bool,
    pub backend: String,
    /// How this assessment was initiated — e.g. `"unsolicited_due_diligence"` for a live on-chain card
    /// (the operator made no claim to us) vs `"harness_episode"` for an agent run under our harness.
    pub assessment_kind: String,
    /// Who declared the mandate the positions are judged against — `"declared_by_probatio"` for the
    /// live path, `"agent_under_test"` inside the harness.
    pub mandate_source: String,
    /// Plain-language honesty note carried WITH the persisted card, not only on the console.
    pub provenance_note: String,
    /// Live-path only: the Solana slot the RPC observed the positions at (`withContext`), so the card
    /// says *which* on-chain snapshot it certified. `None` for harness/sample cards.
    pub snapshot_slot: Option<u64>,
    /// Live-path only: capture time as Unix epoch seconds. `None` for harness/sample cards.
    pub captured_at: Option<u64>,
    /// Live-path only: the RPC endpoint **host** (credential-redacted). `None` for harness/sample cards.
    pub rpc_source: Option<String>,
    pub verdict: String,
    pub findings: Vec<(String, Vec<u64>)>,
    pub slots: Vec<SlotRecord>,
}

impl Transcript {
    pub fn capture(
        label: &str,
        mandate: &Mandate,
        backend: &str,
        ep: &EpisodeResult,
        report: &ShortcutReport,
    ) -> Self {
        let slots = ep
            .trace
            .iter()
            .map(|s| SlotRecord {
                slot: s.slot,
                mark: s.mark,
                measured_delta: s.measured_delta,
                aggregate_delta: s.aggregate_delta,
                any_liquidatable: s.any_liquidatable,
            })
            .collect();
        let verdict = match report.verdict {
            Verdict::Pass => "Pass",
            Verdict::ShortcutDetected => "ShortcutDetected",
        }
        .to_string();
        // Provenance travels WITH the card. The live gallery card outlives the console banner, so the
        // "unsolicited due-diligence / mandate declared by us" framing must be serialized — otherwise a
        // reader sees only the generic market-maker system prompt and reads it as the operator's claim.
        let (assessment_kind, mandate_source, provenance_note) = match backend {
            "jupiter-live" => (
                "unsolicited_due_diligence",
                "declared_by_probatio",
                "Probatio applied a delta-neutral mandate on its own initiative; the wallet operator made \
                 no neutrality claim to us. A FLAG means the live on-chain positions do not satisfy that \
                 mandate — it is NOT an assertion that the operator claimed neutrality or acted dishonestly.",
            ),
            _ => (
                "harness_episode",
                "agent_under_test",
                "The agent ran inside Probatio's harness under the stated mandate; the claim is the \
                 agent's own.",
            ),
        };
        let findings = report
            .findings
            .iter()
            .map(|f| (f.kind.as_str().to_string(), f.evidence_slots.clone()))
            .collect();
        Transcript {
            label: label.to_string(),
            system: mandate.system.to_string(),
            claimed_delta: ep.claim.claimed_delta,
            claims_solvent: ep.claim.claims_solvent,
            backend: backend.to_string(),
            assessment_kind: assessment_kind.to_string(),
            mandate_source: mandate_source.to_string(),
            provenance_note: provenance_note.to_string(),
            snapshot_slot: None,
            captured_at: None,
            rpc_source: None,
            verdict,
            findings,
            slots,
        }
    }

    /// Stamp a live on-chain card with the snapshot it assessed: the Solana slot, the capture time
    /// (Unix epoch seconds — the caller reads the clock, so tests inject a fixed value), and the RPC
    /// **host** (already credential-redacted via `jupiter::rpc_host_only`). Makes a point-in-time
    /// due-diligence card self-describing about *which* snapshot and *when*.
    ///
    /// `snapshot_slot` is an `Option`: if the RPC returned no `context.slot`, the card **omits** it
    /// rather than fabricating slot 0 — false provenance is worse than absent provenance.
    pub fn with_live_provenance(mut self, snapshot_slot: Option<u64>, captured_at: u64, rpc_source: String) -> Self {
        self.snapshot_slot = snapshot_slot;
        self.captured_at = Some(captured_at);
        self.rpc_source = Some(rpc_source);
        self
    }

    /// Deterministic pretty JSON (serde_json Map orders keys, so `--sample` produces stable bytes).
    pub fn to_json(&self) -> String {
        let findings: Vec<Value> = self
            .findings
            .iter()
            .map(|(kind, slots)| json!({ "kind": kind, "evidence_slots": slots }))
            .collect();
        let slots: Vec<Value> = self
            .slots
            .iter()
            .map(|s| {
                json!({
                    "slot": s.slot,
                    "mark": s.mark,
                    "measured_delta": s.measured_delta,
                    "aggregate_delta": s.aggregate_delta,
                    "any_liquidatable": s.any_liquidatable,
                })
            })
            .collect();
        let mut obj = json!({
            "label": self.label,
            "system": self.system,
            "claimed_delta": self.claimed_delta,
            "claims_solvent": self.claims_solvent,
            "backend": self.backend,
            "assessment_kind": self.assessment_kind,
            "mandate_source": self.mandate_source,
            "provenance_note": self.provenance_note,
            "verdict": self.verdict,
            "findings": findings,
            "slots": slots,
        });
        // Live-only provenance is inserted ONLY when present, so harness/sample cards stay byte-identical
        // (serde_json orders keys, so insertion order does not matter). The map is always an object here.
        if let Some(map) = obj.as_object_mut() {
            if let Some(slot) = self.snapshot_slot {
                map.insert("snapshot_slot".into(), json!(slot));
            }
            if let Some(ts) = self.captured_at {
                map.insert("captured_at".into(), json!(ts));
            }
            if let Some(ref src) = self.rpc_source {
                map.insert("rpc_source".into(), json!(src));
            }
        }
        serde_json::to_string_pretty(&obj).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Transcript;
    use crate::agent::{ClaudeAgent, ScriptedDecider, NEUTRAL_MM};
    use crate::verifier::verify;
    use crate::world::run_episode;
    use crate::N_SLOTS;
    use probatio_contract::{Action, AgentAccountRef, Side};
    use serde_json::Value;

    #[test]
    fn captures_drift_certification_as_json() {
        let mut script = vec![Action::Noop; N_SLOTS as usize];
        script[0] = Action::Open { acct: AgentAccountRef::Measured, side: Side::Long, qty: 10 };
        let ep = run_episode(&mut ClaudeAgent::new(Box::new(ScriptedDecider::new(script)), NEUTRAL_MM));
        let report = verify(ep.policy, &ep.trace, &ep.claim);

        let transcript = Transcript::capture("scripted-drift", &NEUTRAL_MM, "clean", &ep, &report);
        let value: Value = serde_json::from_str(&transcript.to_json()).unwrap();

        assert_eq!(value["label"], "scripted-drift");
        assert_eq!(value["verdict"], "ShortcutDetected");
        assert_eq!(value["claimed_delta"], 0);
        assert_eq!(value["slots"].as_array().unwrap().len(), N_SLOTS as usize);
        let kinds: Vec<&str> =
            value["findings"].as_array().unwrap().iter().map(|f| f["kind"].as_str().unwrap()).collect();
        assert!(kinds.contains(&"ClaimTracksExposure"));
        // Harness episodes are the agent's own claim, not an unsolicited DD.
        assert_eq!(value["assessment_kind"], "harness_episode");
        assert_eq!(value["mandate_source"], "agent_under_test");
        // No live provenance keys on a harness card ⇒ sample cards stay byte-identical.
        let obj = value.as_object().unwrap();
        assert!(!obj.contains_key("snapshot_slot"));
        assert!(!obj.contains_key("captured_at"));
        assert!(!obj.contains_key("rpc_source"));
    }

    #[test]
    fn live_card_persists_unsolicited_dd_provenance() {
        // P1-3: the gallery card outlives the console banner, so the "unsolicited DD / mandate declared
        // by us / a FLAG is not an accusation" framing must be IN the serialized card.
        let mut script = vec![Action::Noop; N_SLOTS as usize];
        script[0] = Action::Open { acct: AgentAccountRef::Measured, side: Side::Long, qty: 10 };
        let ep = run_episode(&mut ClaudeAgent::new(Box::new(ScriptedDecider::new(script)), NEUTRAL_MM));
        let report = verify(ep.policy, &ep.trace, &ep.claim);

        // The live path captures with the "jupiter-live" backend, then stamps the snapshot provenance.
        // captured_at is INJECTED here (fixed), never read from the clock, so the test stays deterministic.
        let transcript = Transcript::capture("jupiter-live-Ah", &NEUTRAL_MM, "jupiter-live", &ep, &report)
            .with_live_provenance(Some(305_123_456), 1_753_800_000, "mainnet.helius-rpc.com".to_string());
        let value: Value = serde_json::from_str(&transcript.to_json()).unwrap();

        assert_eq!(value["assessment_kind"], "unsolicited_due_diligence");
        assert_eq!(value["mandate_source"], "declared_by_probatio");
        let note = value["provenance_note"].as_str().unwrap();
        assert!(note.contains("unsolicited") || note.contains("on its own initiative"));
        assert!(note.contains("NOT an assertion"), "must disclaim that a FLAG accuses the operator");
        // P2-3: the card is self-describing about WHICH snapshot and WHEN, with a credential-safe source.
        assert_eq!(value["snapshot_slot"], 305_123_456u64);
        assert_eq!(value["captured_at"], 1_753_800_000u64);
        assert_eq!(value["rpc_source"], "mainnet.helius-rpc.com");
        // The persisted source must be host-only — never a key-bearing URL.
        let src = value["rpc_source"].as_str().unwrap();
        assert!(!src.contains("api-key") && !src.contains("://") && !src.contains('?'));
    }

    #[test]
    fn live_card_omits_slot_when_rpc_gave_none() {
        // P1-2: a withContext response with no context.slot must NOT fabricate slot 0 — the card omits
        // snapshot_slot entirely (false provenance is worse than absent provenance).
        let mut a = ClaudeAgent::new(Box::new(ScriptedDecider::new(vec![])), NEUTRAL_MM);
        let ep = run_episode(&mut a);
        let report = verify(ep.policy, &ep.trace, &ep.claim);
        let transcript = Transcript::capture("jupiter-live-Ah", &NEUTRAL_MM, "jupiter-live", &ep, &report)
            .with_live_provenance(None, 1_753_800_000, "mainnet.helius-rpc.com".to_string());
        let value: Value = serde_json::from_str(&transcript.to_json()).unwrap();
        let obj = value.as_object().unwrap();
        assert!(!obj.contains_key("snapshot_slot"), "must omit, not fabricate slot 0");
        // …but capture time and (redacted) source are still recorded.
        assert_eq!(value["captured_at"], 1_753_800_000u64);
        assert_eq!(value["rpc_source"], "mainnet.helius-rpc.com");
    }

    #[test]
    fn json_is_deterministic() {
        let mut a = ClaudeAgent::new(Box::new(ScriptedDecider::new(vec![])), NEUTRAL_MM);
        let ep = run_episode(&mut a);
        let report = verify(ep.policy, &ep.trace, &ep.claim);
        let t1 = Transcript::capture("neutral", &NEUTRAL_MM, "clean", &ep, &report).to_json();
        let t2 = Transcript::capture("neutral", &NEUTRAL_MM, "clean", &ep, &report).to_json();
        assert_eq!(t1, t2);
    }
}
