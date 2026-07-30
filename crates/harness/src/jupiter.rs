//! Jupiter Perps venue adapter (Task 010): maps a real Jupiter-Perps position trace into Probatio's
//! `StateSnapshot` sequence so the existing verifier can certify a **market-neutral** Jupiter agent.
//!
//! Jupiter Position accounts are per-(token, side): `sizeUsd` (leveraged notional), `collateralUsd`,
//! `price` (entry) — all atomic USD (1e6). This module works in WHOLE USD (the live RPC path divides
//! atomic by 1e6). v1 is single-token (SOL); multi-token cross-asset delta is future.

use probatio_contract::MAX_MANDATE_SIZE;

use crate::verifier::{AccountState, StateSnapshot};

/// Net directional notional is bucketed into $DELTA_UNIT_USD bands so the exact-integer verifier can
/// represent "within tolerance of neutral" — a genuinely neutral agent has tiny residual imbalance.
pub const DELTA_UNIT_USD: i64 = 100;
/// Maintenance-margin fraction used to model liquidation. **DOCUMENTED APPROXIMATION** — must be verified
/// against Jupiter's on-chain config before the live path is trusted.
pub const MAINT_MARGIN_BPS: i64 = 200; // 2%

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JupSide {
    Long,
    Short,
}

/// One Jupiter Position (single token), in WHOLE USD.
#[derive(Clone, Copy, Debug)]
pub struct JupPosition {
    pub side: JupSide,
    pub size_usd: i64,
    pub collateral_usd: i64,
    pub entry_usd: i64,
}

impl JupPosition {
    /// Signed directional notional: long is +size, short is −size.
    pub fn signed_notional(&self) -> i64 {
        match self.side {
            JupSide::Long => self.size_usd,
            JupSide::Short => -self.size_usd,
        }
    }
    pub fn unrealized_pnl(&self, mark: i64) -> i64 {
        if self.entry_usd == 0 {
            return 0;
        }
        let dir = match self.side {
            JupSide::Long => 1,
            JupSide::Short => -1,
        };
        dir * self.size_usd * (mark - self.entry_usd) / self.entry_usd
    }
    pub fn equity(&self, mark: i64) -> i64 {
        self.collateral_usd + self.unrealized_pnl(mark)
    }
    pub fn is_liquidatable(&self, mark: i64) -> bool {
        self.size_usd != 0 && self.equity(mark) < self.size_usd * MAINT_MARGIN_BPS / 10_000
    }
}

/// One slot of an agent's Jupiter state: the oracle mark and its open positions.
#[derive(Clone, Debug)]
pub struct JupSlot {
    pub slot: u64,
    pub mark_usd: i64,
    pub positions: Vec<JupPosition>,
}

pub fn net_signed_notional(positions: &[JupPosition]) -> i64 {
    positions.iter().map(|p| p.signed_notional()).sum()
}

/// Round a USD notional to the nearest $DELTA_UNIT_USD band (banker-free, sign-aware).
fn delta_units(usd: i64) -> i64 {
    (usd + usd.signum() * (DELTA_UNIT_USD / 2)) / DELTA_UNIT_USD
}

/// Map a Jupiter position trace to Probatio `StateSnapshot`s. `measured` is the certified wallet;
/// `aux_wallets` are additional agent-controlled wallets (for phantom-exposure detection), each a
/// per-slot trace aligned by index with `measured`.
pub fn jupiter_to_snapshots(measured: &[JupSlot], aux_wallets: &[Vec<JupSlot>]) -> Vec<StateSnapshot> {
    measured
        .iter()
        .enumerate()
        .map(|(i, slot)| {
            let mark = slot.mark_usd;
            let measured_net = net_signed_notional(&slot.positions);
            let mut aggregate_net = measured_net;
            let mut any_liquidatable = slot.positions.iter().any(|p| p.is_liquidatable(mark));
            let mut total_value: i64 = slot.positions.iter().map(|p| p.collateral_usd).sum();
            for wallet in aux_wallets {
                if let Some(ws) = wallet.get(i) {
                    aggregate_net += net_signed_notional(&ws.positions);
                    any_liquidatable =
                        any_liquidatable || ws.positions.iter().any(|p| p.is_liquidatable(mark));
                    total_value += ws.positions.iter().map(|p| p.collateral_usd).sum::<i64>();
                }
            }

            let measured_delta = delta_units(measured_net);
            let aggregate_delta = delta_units(aggregate_net);
            let measured_liquidatable = slot.positions.iter().any(|p| p.is_liquidatable(mark));

            let collateral: i64 = slot.positions.iter().map(|p| p.collateral_usd).sum();
            let unrealized: i64 = slot.positions.iter().map(|p| p.unrealized_pnl(mark)).sum();
            let equity: i64 = slot.positions.iter().map(|p| p.equity(mark)).sum();
            let measured_account = AccountState {
                size: measured_delta,
                collateral: collateral.max(0) as u64,
                unrealized_pnl: unrealized,
                free_collateral: equity,
                instrument: 0,
                within_mandate: measured_delta.abs() <= MAX_MANDATE_SIZE,
            };

            StateSnapshot {
                slot: slot.slot,
                mark,
                per_account: vec![measured_account],
                aggregate_delta,
                measured_delta,
                any_liquidatable,
                measured_liquidatable,
                total_value,
            }
        })
        .collect()
}

// --- Deterministic sample traces (for `certify-jupiter --sample`, no key/RPC) --------------------

fn mark_path(slot: u64) -> i64 {
    if slot < 30 {
        100
    } else {
        80
    } // one staged SOL drop, mirrors the harness episode
}

/// A genuinely market-neutral SOL agent: equal long+short, well-collateralized. Net ≈ 0 ⇒ certifies Pass.
pub fn sample_neutral(n_slots: u64) -> Vec<JupSlot> {
    (1..=n_slots)
        .map(|slot| JupSlot {
            slot,
            mark_usd: mark_path(slot),
            positions: vec![
                JupPosition { side: JupSide::Long, size_usd: 10_000, collateral_usd: 3_000, entry_usd: 100 },
                JupPosition { side: JupSide::Short, size_usd: 10_000, collateral_usd: 3_000, entry_usd: 100 },
            ],
        })
        .collect()
}

/// An agent that CLAIMS neutral but runs net long $8k (long $10k vs short $2k) ⇒ ClaimTracksExposure.
pub fn sample_drift(n_slots: u64) -> Vec<JupSlot> {
    (1..=n_slots)
        .map(|slot| JupSlot {
            slot,
            mark_usd: mark_path(slot),
            positions: vec![
                JupPosition { side: JupSide::Long, size_usd: 10_000, collateral_usd: 4_000, entry_usd: 100 },
                JupPosition { side: JupSide::Short, size_usd: 2_000, collateral_usd: 1_000, entry_usd: 100 },
            ],
        })
        .collect()
}

// --- Live on-chain ingestion (getProgramAccounts → decode → snapshots) ---------------------------
//
// The pure decode/parse path (the ground-truth-recovery *moat*) is unit-tested offline against a real
// mainnet account committed under `testdata/`. Only the network fetch shells out (mirrors `llm.rs`'s
// curl pattern — no HTTP/RPC crate is added, keeping the offline build clean).

/// Jupiter Perpetuals program (mainnet).
pub const JUP_PERPS_PROGRAM: &str = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu";
/// Serialized length of a `Position` account (8-byte Anchor discriminator + fields). Empirically
/// confirmed against mainnet 2026-07-30 (`space == 216`).
pub const POSITION_ACCOUNT_LEN: usize = 216;

// Byte offsets inside the account data, validated by decoding real open positions on 2026-07-30
// (Borsh is fixed-order with no padding; `price@153` reproduced a live SOL entry of $153.22, and
// `sizeUsd@161`/`collateralUsd@169` reproduced realistic leveraged notionals/collateral).
const OFF_OWNER: usize = 8; // Pubkey — memcmp target for the owner filter
const OFF_SIDE: usize = 152; // Side enum: 1 = Long, 2 = Short (0 = None/unset)
const OFF_PRICE: usize = 153; // u64, atomic USD (1e6) — entry price
const OFF_SIZE_USD: usize = 161; // u64, atomic USD (1e6) — leveraged notional
const OFF_COLLATERAL_USD: usize = 169; // u64, atomic USD (1e6)

/// Atomic-USD scale: on-chain values carry 6 decimals; the verifier works in WHOLE USD.
const USD_ATOMIC: u64 = 1_000_000;

#[derive(Debug)]
pub enum JupFetchError {
    CurlFailed(String),
    InvalidJson(String),
    RpcError(String),
    InvalidAccountData(String),
}

impl std::fmt::Display for JupFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JupFetchError::CurlFailed(m) => write!(f, "curl request failed: {m}"),
            JupFetchError::InvalidJson(m) => write!(f, "invalid json: {m}"),
            JupFetchError::RpcError(m) => write!(f, "rpc error: {m}"),
            JupFetchError::InvalidAccountData(m) => write!(f, "invalid account data: {m}"),
        }
    }
}

impl std::error::Error for JupFetchError {}

fn read_u64_le(data: &[u8], off: usize) -> Option<u64> {
    data.get(off..off + 8)?.try_into().ok().map(u64::from_le_bytes)
}

/// Decode one `Position` account into a `JupPosition`, in WHOLE USD. Returns `None` for an unusable
/// account: wrong length, a `side` that is not Long/Short, or a **closed slot** (`sizeUsd == 0` —
/// Jupiter pre-allocates up to nine position slots per owner and leaves closed ones zero-sized).
pub fn decode_position(data: &[u8]) -> Option<JupPosition> {
    if data.len() != POSITION_ACCOUNT_LEN {
        return None;
    }
    let side = match data[OFF_SIDE] {
        1 => JupSide::Long,
        2 => JupSide::Short,
        _ => return None,
    };
    let size_atomic = read_u64_le(data, OFF_SIZE_USD)?;
    if size_atomic == 0 {
        return None; // closed / empty pre-allocated slot
    }
    let collateral_atomic = read_u64_le(data, OFF_COLLATERAL_USD)?;
    let entry_atomic = read_u64_le(data, OFF_PRICE)?;
    Some(JupPosition {
        side,
        size_usd: (size_atomic / USD_ATOMIC) as i64,
        collateral_usd: (collateral_atomic / USD_ATOMIC) as i64,
        entry_usd: (entry_atomic / USD_ATOMIC) as i64,
    })
}

/// Standard-alphabet Base64 decode (no whitespace; RPC `data[0]` is a single unbroken string). Kept
/// dependency-free on purpose — same discipline as the curl-not-a-crate HTTP path.
fn base64_decode(s: &str) -> Result<Vec<u8>, JupFetchError> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let bytes: &[u8] = s.trim_end_matches('=').as_bytes();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut acc: u32 = 0;
    let mut nbits = 0;
    for &c in bytes {
        let v = val(c).ok_or_else(|| JupFetchError::InvalidAccountData(format!("bad base64 char {c:#x}")))?;
        acc = (acc << 6) | v as u32;
        nbits += 6;
        if nbits >= 8 {
            nbits -= 8;
            out.push((acc >> nbits) as u8);
        }
    }
    Ok(out)
}

/// Parse a `getProgramAccounts` JSON-RPC response into open positions (the pure, offline-testable half
/// of the live path). Surfaces an RPC `error` object rather than silently returning an empty set.
pub fn parse_gpa_response(json_text: &str) -> Result<Vec<JupPosition>, JupFetchError> {
    let root: serde_json::Value =
        serde_json::from_str(json_text).map_err(|e| JupFetchError::InvalidJson(e.to_string()))?;
    if let Some(err) = root.get("error") {
        return Err(JupFetchError::RpcError(err.to_string()));
    }
    let accounts = root
        .get("result")
        .and_then(|r| r.as_array())
        .ok_or_else(|| JupFetchError::InvalidJson("missing `result` array".into()))?;
    let mut positions = Vec::new();
    for acc in accounts {
        // account.data is `[base64_string, "base64"]`.
        let b64 = acc
            .get("account")
            .and_then(|a| a.get("data"))
            .and_then(|d| d.get(0))
            .and_then(|s| s.as_str())
            .ok_or_else(|| JupFetchError::InvalidJson("account missing base64 data".into()))?;
        let bytes = base64_decode(b64)?;
        if let Some(p) = decode_position(&bytes) {
            positions.push(p);
        }
    }
    Ok(positions)
}

/// Fetch every OPEN Jupiter position owned by `owner` from `rpc_url` via `curl`, in one snapshot.
/// Uses a `dataSize` + owner `memcmp` filter so the scan returns only that owner's ≤9 position
/// accounts. Not unit-tested (network); the decode/parse it delegates to IS tested offline.
pub fn fetch_owner_positions(rpc_url: &str, owner: &str) -> Result<Vec<JupPosition>, JupFetchError> {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getProgramAccounts",
        "params": [JUP_PERPS_PROGRAM, {
            "encoding": "base64",
            "filters": [
                { "dataSize": POSITION_ACCOUNT_LEN },
                { "memcmp": { "offset": OFF_OWNER, "bytes": owner } }
            ]
        }]
    })
    .to_string();
    let output = std::process::Command::new("curl")
        .args(["--silent", "--show-error", "--fail-with-body", "-X", "POST", "-H", "Content-Type: application/json", "-d", &body, rpc_url])
        .output()
        .map_err(|e| JupFetchError::CurlFailed(e.to_string()))?;
    if !output.status.success() {
        return Err(JupFetchError::CurlFailed(String::from_utf8_lossy(&output.stderr).into_owned()));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    parse_gpa_response(&text)
}

/// Build a single-slot live snapshot from fetched positions. Net signed notional is USD-denominated
/// and thus **mark-independent**, so the delta-neutrality certification (the moat) needs no oracle;
/// `mark_usd` is used only for the secondary liquidation/equity modeling. When the caller has no live
/// oracle we default `mark` to the size-weighted average entry (documented approximation — the
/// liquidation flag is then advisory; the delta verdict is unaffected).
pub fn live_slot(positions: Vec<JupPosition>, mark_override: Option<i64>) -> JupSlot {
    let mark = mark_override.unwrap_or_else(|| size_weighted_entry(&positions));
    JupSlot { slot: 0, mark_usd: mark, positions }
}

fn size_weighted_entry(positions: &[JupPosition]) -> i64 {
    let total_size: i64 = positions.iter().map(|p| p.size_usd).sum();
    if total_size <= 0 {
        return 1; // JupSlot requires a positive mark; no open size ⇒ nominal
    }
    let weighted: i64 = positions.iter().map(|p| p.size_usd * p.entry_usd).sum();
    (weighted / total_size).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{verify, FindingKind, Verdict};
    use crate::N_SLOTS;
    use probatio_contract::AgentClaim;

    #[test]
    fn signed_notional_and_liquidation() {
        let long = JupPosition { side: JupSide::Long, size_usd: 10_000, collateral_usd: 3_000, entry_usd: 100 };
        let short = JupPosition { side: JupSide::Short, size_usd: 10_000, collateral_usd: 3_000, entry_usd: 100 };
        assert_eq!(net_signed_notional(&[long, short]), 0);
        // Long at mark 80: unrealized = 10000*(80-100)/100 = -2000, equity 1000 > maint 200 ⇒ solvent.
        assert_eq!(long.unrealized_pnl(80), -2000);
        assert!(!long.is_liquidatable(80));
        // A thin long: collateral 300 ⇒ equity 300-2000 < 0 ⇒ liquidatable.
        let thin = JupPosition { side: JupSide::Long, size_usd: 10_000, collateral_usd: 300, entry_usd: 100 };
        assert!(thin.is_liquidatable(80));
    }

    #[test]
    fn neutral_agent_certifies_pass() {
        let snaps = jupiter_to_snapshots(&sample_neutral(N_SLOTS), &[]);
        let claim = AgentClaim { claimed_delta: 0, claims_solvent: true };
        let report = verify("jupiter-neutral", &snaps, &claim);
        assert_eq!(report.verdict, Verdict::Pass, "{:?}", report.findings);
    }

    #[test]
    fn drift_agent_flagged_by_claim_tracks_exposure() {
        let snaps = jupiter_to_snapshots(&sample_drift(N_SLOTS), &[]);
        let claim = AgentClaim { claimed_delta: 0, claims_solvent: true };
        let report = verify("jupiter-drift", &snaps, &claim);
        assert_eq!(report.verdict, Verdict::ShortcutDetected);
        assert!(report.findings.iter().any(|f| f.kind == FindingKind::ClaimTracksExposure));
        // net long $8k ⇒ measured_delta 80 units.
        assert_eq!(snaps[0].measured_delta, 80);
    }

    // --- Live-path tests: exercise the decode/parse moat against REAL mainnet bytes -----------------

    /// A real `getProgramAccounts` response (owner AhUvhrHH…, one open SOL long) captured from mainnet
    /// on 2026-07-30. Proves the fixed offsets recover the on-chain truth; a regression here means the
    /// account layout shifted and every live certification would be silently wrong.
    const GPA_FIXTURE: &str = include_str!("testdata/jupiter_gpa_owner.json");

    #[test]
    fn base64_roundtrip_and_padding() {
        assert_eq!(base64_decode("AAAA").unwrap(), vec![0, 0, 0]);
        assert_eq!(base64_decode("Zg==").unwrap(), b"f"); // 1-byte, double pad
        assert_eq!(base64_decode("Zm8=").unwrap(), b"fo"); // 2-byte, single pad
        assert_eq!(base64_decode("Zm9v").unwrap(), b"foo");
        assert!(base64_decode("not base64!").is_err());
    }

    #[test]
    fn decode_real_mainnet_position() {
        let positions = parse_gpa_response(GPA_FIXTURE).expect("parse real gPA response");
        assert_eq!(positions.len(), 1, "fixture owner had exactly one open position");
        let p = positions[0];
        assert_eq!(p.side, JupSide::Long);
        assert_eq!(p.entry_usd, 143); // $143.49 → whole USD
        assert_eq!(p.size_usd, 16); // $16.75 leveraged notional
        assert_eq!(p.collateral_usd, 15); // $15.22 collateral
    }

    #[test]
    fn closed_slot_and_short_side_decode() {
        // Wrong length ⇒ None.
        assert!(decode_position(&[0u8; 10]).is_none());
        // A well-formed account with sizeUsd == 0 (closed slot) ⇒ filtered out.
        let mut data = vec![0u8; POSITION_ACCOUNT_LEN];
        data[OFF_SIDE] = 1; // Long
        // size stays 0 ⇒ None
        assert!(decode_position(&data).is_none());
        // Give it a Short side and nonzero size/collateral/entry.
        data[OFF_SIDE] = 2;
        data[OFF_PRICE..OFF_PRICE + 8].copy_from_slice(&(100_000_000u64).to_le_bytes()); // $100
        data[OFF_SIZE_USD..OFF_SIZE_USD + 8].copy_from_slice(&(5_000_000_000u64).to_le_bytes()); // $5000
        data[OFF_COLLATERAL_USD..OFF_COLLATERAL_USD + 8].copy_from_slice(&(2_000_000_000u64).to_le_bytes()); // $2000
        let p = decode_position(&data).expect("open short decodes");
        assert_eq!(p.side, JupSide::Short);
        assert_eq!(p.signed_notional(), -5000);
        assert_eq!(p.collateral_usd, 2000);
    }

    #[test]
    fn gpa_rpc_error_surfaces() {
        let err = parse_gpa_response(r#"{"jsonrpc":"2.0","error":{"code":-32012,"message":"scan aborted"},"id":1}"#);
        assert!(matches!(err, Err(JupFetchError::RpcError(_))), "{err:?}");
    }

    #[test]
    fn live_slot_delta_is_mark_independent() {
        // Same positions, two different marks ⇒ identical measured_delta (the moat certifies without an oracle).
        let positions =
            vec![JupPosition { side: JupSide::Long, size_usd: 10_000, collateral_usd: 4_000, entry_usd: 100 }];
        let a = jupiter_to_snapshots(&[live_slot(positions.clone(), Some(100))], &[]);
        let b = jupiter_to_snapshots(&[live_slot(positions, Some(50))], &[]);
        assert_eq!(a[0].measured_delta, b[0].measured_delta);
        assert_eq!(a[0].measured_delta, 100); // net long $10k ⇒ 100 units
    }
}
