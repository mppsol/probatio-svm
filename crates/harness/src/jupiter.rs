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
}
