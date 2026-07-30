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
/// Anchor account discriminator for a Jupiter `Position` (`sha256("account:Position")[..8]`). Extracted
/// from the committed mainnet fixture. Verified both as a `getProgramAccounts` `memcmp` filter and at
/// decode time, so a same-sized *non-Position* account cannot be certified through the fixed offsets.
pub const POSITION_DISCRIMINATOR: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
/// Base58 of `POSITION_DISCRIMINATOR` for the RPC `memcmp` filter (default `bytes` encoding is base58).
const POSITION_DISCRIMINATOR_B58: &str = "VZMoMoKgZQb";

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

/// Decode one `Position` account into a `JupPosition`, in WHOLE USD.
///
/// The two "no position" cases are kept distinct so the ingestion boundary never conflates *trusted
/// absence* with *untrusted data*:
/// - `Ok(None)` — a structurally valid but **closed slot** (correct discriminator, `sizeUsd == 0`;
///   Jupiter pre-allocates up to nine position slots per owner and leaves closed ones zero-sized).
/// - `Err(..)` — an **untrusted/invalid** account (wrong length, wrong discriminator, or an open slot
///   with an impossible `side`). The caller must surface this, never silently drop it.
pub fn decode_position(data: &[u8]) -> Result<Option<JupPosition>, JupFetchError> {
    if data.len() != POSITION_ACCOUNT_LEN {
        return Err(JupFetchError::InvalidAccountData(format!(
            "expected {POSITION_ACCOUNT_LEN}-byte Position account, got {}",
            data.len()
        )));
    }
    if data[0..8] != POSITION_DISCRIMINATOR {
        return Err(JupFetchError::InvalidAccountData(
            "account discriminator is not Jupiter Position".into(),
        ));
    }
    let size_atomic = read_u64_le(data, OFF_SIZE_USD)
        .ok_or_else(|| JupFetchError::InvalidAccountData("unreadable sizeUsd".into()))?;
    if size_atomic == 0 {
        return Ok(None); // structurally valid, pre-allocated closed slot
    }
    let side = match data[OFF_SIDE] {
        1 => JupSide::Long,
        2 => JupSide::Short,
        other => {
            return Err(JupFetchError::InvalidAccountData(format!(
                "open Position has invalid side byte {other}"
            )))
        }
    };
    let collateral_atomic = read_u64_le(data, OFF_COLLATERAL_USD)
        .ok_or_else(|| JupFetchError::InvalidAccountData("unreadable collateralUsd".into()))?;
    let entry_atomic = read_u64_le(data, OFF_PRICE)
        .ok_or_else(|| JupFetchError::InvalidAccountData("unreadable price".into()))?;
    Ok(Some(JupPosition {
        side,
        size_usd: (size_atomic / USD_ATOMIC) as i64,
        collateral_usd: (collateral_atomic / USD_ATOMIC) as i64,
        entry_usd: (entry_atomic / USD_ATOMIC) as i64,
    }))
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
    let bytes = s.as_bytes();
    // Canonical base64 is a positive multiple of four characters; a truncated account would otherwise
    // decode "best effort" to a wrong length and then vanish as if it were a closed slot (see P1-1).
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return Err(JupFetchError::InvalidAccountData(format!(
            "base64 length {} is not a positive multiple of 4",
            bytes.len()
        )));
    }
    // `=` padding may only be the final one or two characters.
    let pad = bytes.iter().rev().take_while(|&&c| c == b'=').count();
    if pad > 2 {
        return Err(JupFetchError::InvalidAccountData("more than two base64 pad chars".into()));
    }
    let body = &bytes[..bytes.len() - pad];
    let mut out = Vec::with_capacity(body.len() * 3 / 4);
    let mut acc: u32 = 0;
    let mut nbits = 0;
    for &c in body {
        let v = val(c).ok_or_else(|| JupFetchError::InvalidAccountData(format!("bad base64 char {c:#x}")))?;
        acc = (acc << 6) | v as u32;
        nbits += 6;
        if nbits >= 8 {
            nbits -= 8;
            out.push((acc >> nbits) as u8);
        }
    }
    // A canonical encoder zeroes the leftover padding bits; reject anything else as corrupt.
    if nbits > 0 && (acc & ((1 << nbits) - 1)) != 0 {
        return Err(JupFetchError::InvalidAccountData("non-zero base64 padding bits".into()));
    }
    Ok(out)
}

/// One live on-chain snapshot: the open positions recovered plus the Solana slot the RPC observed them
/// at (present only when the response carries context — see `withContext`). The slot lets a point-in-time
/// due-diligence card identify exactly which chain snapshot it certified.
#[derive(Clone, Debug)]
pub struct GpaSnapshot {
    pub positions: Vec<JupPosition>,
    pub slot: Option<u64>,
}

/// Parse a `getProgramAccounts` JSON-RPC response into open positions + snapshot slot (the pure,
/// offline-testable half of the live path). Surfaces an RPC `error` object rather than silently
/// returning an empty set. Accepts **both** response shapes so an old bare-array fixture still parses:
/// - bare:        `result: [ …accounts… ]`                       (no context ⇒ `slot: None`)
/// - withContext: `result: { context: { slot }, value: [ … ] }`  (⇒ `slot: Some(..)`)
pub fn parse_gpa_response(json_text: &str) -> Result<GpaSnapshot, JupFetchError> {
    let root: serde_json::Value =
        serde_json::from_str(json_text).map_err(|e| JupFetchError::InvalidJson(e.to_string()))?;
    if let Some(err) = root.get("error") {
        return Err(JupFetchError::RpcError(err.to_string()));
    }
    let result = root
        .get("result")
        .ok_or_else(|| JupFetchError::InvalidJson("missing `result`".into()))?;
    let (accounts, slot) = if let Some(arr) = result.as_array() {
        (arr, None)
    } else {
        let value = result
            .get("value")
            .and_then(|v| v.as_array())
            .ok_or_else(|| {
                JupFetchError::InvalidJson("`result` is neither an array nor a withContext object".into())
            })?;
        let slot = result.get("context").and_then(|c| c.get("slot")).and_then(|s| s.as_u64());
        (value, slot)
    };
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
        // `?` surfaces an untrusted/invalid account; `None` is a validated closed slot we skip.
        if let Some(p) = decode_position(&bytes)? {
            positions.push(p);
        }
    }
    Ok(GpaSnapshot { positions, slot })
}

/// Reduce an RPC URL to its **host only** for a persisted card — the path/query/userinfo may carry an
/// API key, and a due-diligence card outlives the console, so it must never embed the credential.
pub fn rpc_host_only(url: &str) -> String {
    let after_scheme = url.split_once("://").map(|(_, r)| r).unwrap_or(url);
    let after_userinfo = after_scheme.split_once('@').map(|(_, r)| r).unwrap_or(after_scheme);
    let host = after_userinfo.split(['/', '?', ':']).next().unwrap_or("");
    if host.is_empty() {
        "<redacted>".to_string()
    } else {
        host.to_string()
    }
}

/// Fetch every OPEN Jupiter position owned by `owner` from `rpc_url` via `curl`, in one snapshot.
/// Uses a `dataSize` + owner `memcmp` filter so the scan returns only that owner's ≤9 position
/// accounts. Not unit-tested (network); the decode/parse it delegates to IS tested offline.
pub fn fetch_owner_positions(rpc_url: &str, owner: &str) -> Result<GpaSnapshot, JupFetchError> {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getProgramAccounts",
        "params": [JUP_PERPS_PROGRAM, {
            "encoding": "base64",
            "withContext": true,
            "filters": [
                { "dataSize": POSITION_ACCOUNT_LEN },
                { "memcmp": { "offset": 0, "bytes": POSITION_DISCRIMINATOR_B58 } },
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
pub fn live_slot(positions: Vec<JupPosition>, mark_override: Option<i64>, chain_slot: u64) -> JupSlot {
    let mark = mark_override.unwrap_or_else(|| size_weighted_entry(&positions));
    JupSlot { slot: chain_slot, mark_usd: mark, positions }
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

    /// A real `getProgramAccounts` **withContext** response, one open **SHORT** on a **different custody**
    /// than the SOL long fixture (custody `5Pv3gM9…` vs `7xS2gz2…`; entry ≈ $63k ⇒ a BTC-class market),
    /// captured from mainnet at slot 436117349 on 2026-07-30 (trimmed to the single account of interest;
    /// bytes verbatim). Proves the fixed offsets recover a real short + a second custody from live bytes —
    /// not just synthetic bytes written at the same offsets (task 018 P2-1). Being a withContext response,
    /// it also exercises the context-shape parse against real bytes.
    const GPA_FIXTURE_SHORT: &str = include_str!("testdata/jupiter_gpa_short.json");

    #[test]
    fn base64_roundtrip_and_padding() {
        assert_eq!(base64_decode("AAAA").unwrap(), vec![0, 0, 0]);
        assert_eq!(base64_decode("Zg==").unwrap(), b"f"); // 1-byte, double pad
        assert_eq!(base64_decode("Zm8=").unwrap(), b"fo"); // 2-byte, single pad
        assert_eq!(base64_decode("Zm9v").unwrap(), b"foo");
        assert!(base64_decode("not base64!").is_err());
        // Strictness (P1-1): a non-quartet length (dropped/missing padding) is rejected outright,
        // never best-effort decoded to a wrong length that would then vanish as a "closed slot".
        assert!(base64_decode("Zg").is_err()); // missing padding
        assert!(base64_decode("Zm9").is_err()); // one char short of a quartet
        assert!(base64_decode("Zg===").is_err()); // over-padded
        assert!(base64_decode("Z=g=").is_err()); // misplaced padding
    }

    #[test]
    fn decode_real_mainnet_position() {
        let snap = parse_gpa_response(GPA_FIXTURE).expect("parse real gPA response");
        assert_eq!(snap.positions.len(), 1, "fixture owner had exactly one open position");
        let p = snap.positions[0];
        assert_eq!(p.side, JupSide::Long);
        assert_eq!(p.entry_usd, 143); // $143.49 → whole USD
        assert_eq!(p.size_usd, 16); // $16.75 leveraged notional
        assert_eq!(p.collateral_usd, 15); // $15.22 collateral
    }

    #[test]
    fn decode_real_mainnet_short_distinct_custody() {
        // A REAL short on a different custody (P2-1): the fixed offsets must recover a short + a second
        // market from live bytes, not just round-trip synthetic bytes at the same offsets.
        let snap = parse_gpa_response(GPA_FIXTURE_SHORT).expect("parse real short gPA response");
        assert_eq!(snap.positions.len(), 1);
        let p = snap.positions[0];
        assert_eq!(p.side, JupSide::Short);
        assert_eq!(p.size_usd, 26_499); // leveraged notional (whole USD)
        assert_eq!(p.collateral_usd, 2_632);
        assert_eq!(p.entry_usd, 63_263); // ≈ BTC-class entry ⇒ clearly a different market than the SOL long
        assert_eq!(p.signed_notional(), -26_499, "short ⇒ negative signed notional");
        // Being a withContext response, the real slot is recovered too.
        assert_eq!(snap.slot, Some(436_117_349));
    }

    /// Build a structurally valid 216-byte open Position with the correct discriminator.
    fn synth_open(side: u8, size: u64, collateral: u64, entry: u64) -> Vec<u8> {
        let mut data = vec![0u8; POSITION_ACCOUNT_LEN];
        data[0..8].copy_from_slice(&POSITION_DISCRIMINATOR);
        data[OFF_SIDE] = side;
        data[OFF_PRICE..OFF_PRICE + 8].copy_from_slice(&entry.to_le_bytes());
        data[OFF_SIZE_USD..OFF_SIZE_USD + 8].copy_from_slice(&size.to_le_bytes());
        data[OFF_COLLATERAL_USD..OFF_COLLATERAL_USD + 8].copy_from_slice(&collateral.to_le_bytes());
        data
    }

    #[test]
    fn closed_slot_and_short_side_decode() {
        // Wrong length ⇒ Err (untrusted), never silently dropped.
        assert!(matches!(decode_position(&[0u8; 10]), Err(JupFetchError::InvalidAccountData(_))));
        // Correct discriminator but sizeUsd == 0 ⇒ structurally valid closed slot ⇒ Ok(None).
        let mut data = synth_open(1, 0, 0, 0);
        assert!(decode_position(&data).unwrap().is_none());
        // Give it a Short side and nonzero size/collateral/entry ⇒ Ok(Some).
        data = synth_open(2, 5_000_000_000, 2_000_000_000, 100_000_000);
        let p = decode_position(&data).unwrap().expect("open short decodes");
        assert_eq!(p.side, JupSide::Short);
        assert_eq!(p.signed_notional(), -5000);
        assert_eq!(p.collateral_usd, 2000);
    }

    #[test]
    fn wrong_discriminator_is_rejected_not_dropped() {
        // A same-sized account with the wrong discriminator must Err (type confusion, P1-2), not be
        // skipped like a closed slot.
        let mut data = synth_open(1, 5_000_000_000, 2_000_000_000, 100_000_000);
        assert!(decode_position(&data).unwrap().is_some(), "sanity: valid Position decodes");
        data[0] ^= 0xFF;
        assert!(matches!(decode_position(&data), Err(JupFetchError::InvalidAccountData(_))));
    }

    #[test]
    fn truncated_account_in_gpa_array_errs_not_partial() {
        // A GPA array with one truncated account must return Err — never a partial vector that
        // certifies a live verdict over an incomplete set (P1-1).
        let short_b64 = "A".repeat(284); // 284 base64 chars ⇒ 213 bytes (one quartet short of 216)
        let json = format!(
            r#"{{"jsonrpc":"2.0","id":1,"result":[{{"account":{{"data":["{short_b64}","base64"]}}}}]}}"#
        );
        assert!(matches!(
            parse_gpa_response(&json),
            Err(JupFetchError::InvalidAccountData(_))
        ));
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
        let a = jupiter_to_snapshots(&[live_slot(positions.clone(), Some(100), 42)], &[]);
        let b = jupiter_to_snapshots(&[live_slot(positions, Some(50), 42)], &[]);
        assert_eq!(a[0].measured_delta, b[0].measured_delta);
        assert_eq!(a[0].measured_delta, 100); // net long $10k ⇒ 100 units
    }

    #[test]
    fn live_slot_carries_chain_slot() {
        let positions =
            vec![JupPosition { side: JupSide::Long, size_usd: 10_000, collateral_usd: 4_000, entry_usd: 100 }];
        assert_eq!(live_slot(positions, Some(100), 305_123_456).slot, 305_123_456);
    }

    #[test]
    fn parse_accepts_both_shapes_and_extracts_slot() {
        // A single open Long account, wrapped once as a bare array and once as a withContext object.
        let acc = format!(
            r#"{{"account":{{"data":["{}","base64"]}}}}"#,
            {
                // reuse the real fixture's account so the bytes are a valid Position
                let v: serde_json::Value = serde_json::from_str(GPA_FIXTURE).unwrap();
                v["result"][0]["account"]["data"][0].as_str().unwrap().to_string()
            }
        );
        let bare = format!(r#"{{"jsonrpc":"2.0","id":1,"result":[{acc}]}}"#);
        let ctx = format!(
            r#"{{"jsonrpc":"2.0","id":1,"result":{{"context":{{"slot":305123456}},"value":[{acc}]}}}}"#
        );

        let sb = parse_gpa_response(&bare).expect("bare array parses");
        let sc = parse_gpa_response(&ctx).expect("withContext parses");
        // Identical positions from both shapes…
        assert_eq!(sb.positions.len(), 1);
        assert_eq!(sc.positions.len(), 1);
        assert_eq!(sb.positions[0].signed_notional(), sc.positions[0].signed_notional());
        // …but only the withContext shape yields the snapshot slot.
        assert_eq!(sb.slot, None);
        assert_eq!(sc.slot, Some(305_123_456));
    }

    #[test]
    fn rpc_host_only_redacts_credentials() {
        // The path/query/userinfo may carry an API key — a persisted card must keep only the host.
        assert_eq!(rpc_host_only("https://mainnet.helius-rpc.com/?api-key=SECRET"), "mainnet.helius-rpc.com");
        assert_eq!(rpc_host_only("https://user:pass@rpc.example.com:8899/path"), "rpc.example.com");
        assert_eq!(rpc_host_only("https://api.mainnet-beta.solana.com"), "api.mainnet-beta.solana.com");
        assert_eq!(rpc_host_only(""), "<redacted>");
    }
}
