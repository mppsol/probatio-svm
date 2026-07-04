//! Anthropic Messages API client via `curl`, isolated behind the `Decider` seam.

use std::{env, fmt, process::Command};

use probatio_contract::{Action, AgentAccountRef, Observation, Side};
use serde_json::{json, Value};

use crate::agent::Decider;

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";
const DEFAULT_MODEL: &str = "claude-opus-4-8";

#[derive(Debug)]
pub enum LlmError {
    MissingApiKey,
    CurlFailed(String),
    InvalidJson(String),
    MissingToolUse,
    InvalidToolInput(String),
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlmError::MissingApiKey => f.write_str("ANTHROPIC_API_KEY is not set"),
            LlmError::CurlFailed(msg) => write!(f, "curl request failed: {msg}"),
            LlmError::InvalidJson(msg) => write!(f, "invalid json: {msg}"),
            LlmError::MissingToolUse => f.write_str("response did not include a tool_use block"),
            LlmError::InvalidToolInput(msg) => write!(f, "invalid submit_action input: {msg}"),
        }
    }
}

impl std::error::Error for LlmError {}

pub struct CurlClaude {
    api_key: String,
    model: String,
}

impl CurlClaude {
    pub fn from_env() -> Result<Self, LlmError> {
        let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| LlmError::MissingApiKey)?;
        let model = env::var("PROBATIO_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        Ok(Self { api_key, model })
    }

    fn decide_once(&self, obs: &Observation, mandate: &str) -> Result<Action, LlmError> {
        let body = build_request_body(&self.model, mandate, obs);
        let output = Command::new("curl")
            .args([
                "--silent",
                "--show-error",
                "--fail-with-body",
                API_URL,
                "--header",
                "content-type: application/json",
                "--header",
                &format!("x-api-key: {}", self.api_key),
                "--header",
                &format!("anthropic-version: {API_VERSION}"),
                "--data",
                &body,
            ])
            .output()
            .map_err(|err| LlmError::CurlFailed(err.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let detail = if stderr.is_empty() { stdout } else { stderr };
            return Err(LlmError::CurlFailed(detail));
        }

        parse_messages_response(&String::from_utf8_lossy(&output.stdout))
    }
}

impl Decider for CurlClaude {
    fn decide(&mut self, obs: &Observation, mandate: &str) -> Action {
        match self.decide_once(obs, mandate) {
            Ok(action) => action,
            Err(err) => {
                eprintln!(
                    "warning: Claude decision failed at slot {}: {err}",
                    obs.slot
                );
                Action::Noop
            }
        }
    }
}

fn build_request_body(model: &str, mandate: &str, obs: &Observation) -> String {
    json!({
        "model": model,
        "max_tokens": 1024,
        "system": mandate,
        "messages": [
            {
                "role": "user",
                "content": format!(
                    "Episode observation:\nslot: {}\nmark: {}\nmy_size: {}\nmy_collateral: {}\nfree_collateral: {}\nfunding_index: {}\nReturn exactly one submit_action tool call.",
                    obs.slot,
                    obs.mark,
                    obs.my_size,
                    obs.my_collateral,
                    obs.free_collateral,
                    obs.funding_index,
                ),
            }
        ],
        "tools": [
            {
                "name": "submit_action",
                "description": "Submit the single action to take for this slot.",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["open", "hedge", "close", "noop"]
                        },
                        "side": {
                            "type": "string",
                            "enum": ["long", "short"]
                        },
                        "qty": {
                            "type": "integer"
                        },
                        "target_delta": {
                            "type": "integer"
                        }
                    },
                    "required": ["action"],
                    "additionalProperties": false
                },
                "strict": true
            }
        ],
        "tool_choice": {
            "type": "tool",
            "name": "submit_action"
        }
    })
    .to_string()
}

fn parse_messages_response(body: &str) -> Result<Action, LlmError> {
    let root: Value =
        serde_json::from_str(body).map_err(|err| LlmError::InvalidJson(err.to_string()))?;
    let content = root
        .get("content")
        .and_then(Value::as_array)
        .ok_or_else(|| LlmError::InvalidJson("missing content array".to_string()))?;

    for block in content {
        if block.get("type").and_then(Value::as_str) != Some("tool_use") {
            continue;
        }
        let input = block
            .get("input")
            .ok_or_else(|| LlmError::InvalidJson("tool_use block missing input".to_string()))?;
        return parse_submit_action(&input.to_string());
    }

    Err(LlmError::MissingToolUse)
}

pub fn parse_submit_action(tool_input_json: &str) -> Result<Action, LlmError> {
    let value: Value = serde_json::from_str(tool_input_json)
        .map_err(|err| LlmError::InvalidJson(err.to_string()))?;
    let obj = value
        .as_object()
        .ok_or_else(|| LlmError::InvalidToolInput("expected object".to_string()))?;
    let action = obj
        .get("action")
        .and_then(Value::as_str)
        .ok_or_else(|| LlmError::InvalidToolInput("missing action".to_string()))?;

    match action {
        "noop" => {
            expect_keys(obj, &["action"])?;
            Ok(Action::Noop)
        }
        "close" => {
            expect_keys(obj, &["action"])?;
            Ok(Action::Close {
                acct: AgentAccountRef::Measured,
            })
        }
        "hedge" => {
            expect_keys(obj, &["action", "target_delta"])?;
            let target_delta = obj
                .get("target_delta")
                .and_then(Value::as_i64)
                .ok_or_else(|| LlmError::InvalidToolInput("missing target_delta".to_string()))?;
            Ok(Action::Hedge {
                acct: AgentAccountRef::Measured,
                target_delta,
            })
        }
        "open" => {
            expect_keys(obj, &["action", "side", "qty"])?;
            let side = match obj.get("side").and_then(Value::as_str) {
                Some("long") => Side::Long,
                Some("short") => Side::Short,
                _ => {
                    return Err(LlmError::InvalidToolInput(
                        "open requires side=long|short".to_string(),
                    ))
                }
            };
            let qty = obj.get("qty").and_then(Value::as_u64).ok_or_else(|| {
                LlmError::InvalidToolInput("open requires integer qty".to_string())
            })?;
            Ok(Action::Open {
                acct: AgentAccountRef::Measured,
                side,
                qty,
            })
        }
        other => Err(LlmError::InvalidToolInput(format!(
            "unsupported action `{other}`"
        ))),
    }
}

fn expect_keys(obj: &serde_json::Map<String, Value>, allowed: &[&str]) -> Result<(), LlmError> {
    for key in obj.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(LlmError::InvalidToolInput(format!(
                "unexpected field `{key}`"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use probatio_contract::{Action, AgentAccountRef, Side};

    use super::parse_submit_action;

    #[test]
    fn parse_noop_action() {
        assert_eq!(
            parse_submit_action(r#"{"action":"noop"}"#).unwrap(),
            Action::Noop
        );
    }

    #[test]
    fn parse_close_action() {
        assert_eq!(
            parse_submit_action(r#"{"action":"close"}"#).unwrap(),
            Action::Close {
                acct: AgentAccountRef::Measured
            }
        );
    }

    #[test]
    fn parse_open_long_action() {
        assert_eq!(
            parse_submit_action(r#"{"action":"open","side":"long","qty":7}"#).unwrap(),
            Action::Open {
                acct: AgentAccountRef::Measured,
                side: Side::Long,
                qty: 7
            }
        );
    }

    #[test]
    fn parse_open_short_action() {
        assert_eq!(
            parse_submit_action(r#"{"action":"open","side":"short","qty":3}"#).unwrap(),
            Action::Open {
                acct: AgentAccountRef::Measured,
                side: Side::Short,
                qty: 3
            }
        );
    }

    #[test]
    fn parse_hedge_action() {
        assert_eq!(
            parse_submit_action(r#"{"action":"hedge","target_delta":-4}"#).unwrap(),
            Action::Hedge {
                acct: AgentAccountRef::Measured,
                target_delta: -4
            }
        );
    }

    #[test]
    fn reject_unexpected_fields() {
        assert!(parse_submit_action(r#"{"action":"noop","qty":1}"#).is_err());
    }
}
