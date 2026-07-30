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
            verdict,
            findings,
            slots,
        }
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
        serde_json::to_string_pretty(&json!({
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
        }))
        .unwrap_or_default()
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
    }

    #[test]
    fn live_card_persists_unsolicited_dd_provenance() {
        // P1-3: the gallery card outlives the console banner, so the "unsolicited DD / mandate declared
        // by us / a FLAG is not an accusation" framing must be IN the serialized card.
        let mut script = vec![Action::Noop; N_SLOTS as usize];
        script[0] = Action::Open { acct: AgentAccountRef::Measured, side: Side::Long, qty: 10 };
        let ep = run_episode(&mut ClaudeAgent::new(Box::new(ScriptedDecider::new(script)), NEUTRAL_MM));
        let report = verify(ep.policy, &ep.trace, &ep.claim);

        // The live path captures with the "jupiter-live" backend.
        let transcript = Transcript::capture("jupiter-live-Ah", &NEUTRAL_MM, "jupiter-live", &ep, &report);
        let value: Value = serde_json::from_str(&transcript.to_json()).unwrap();

        assert_eq!(value["assessment_kind"], "unsolicited_due_diligence");
        assert_eq!(value["mandate_source"], "declared_by_probatio");
        let note = value["provenance_note"].as_str().unwrap();
        assert!(note.contains("unsolicited") || note.contains("on its own initiative"));
        assert!(note.contains("NOT an assertion"), "must disclaim that a FLAG accuses the operator");
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
