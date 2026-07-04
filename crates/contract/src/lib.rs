#![no_std]

//! Shared contract: the account layout read by the perp program, the guard program, and the
//! off-chain verifier — plus the agent-facing intent types. This is the load-bearing contract
//! (see `AGENTS.md`): a drift here breaks the program, the guard, and the verifier at once.
//!
//! Dependency-free and `#![no_std]` so it compiles for the Solana BPF target and the host harness
//! alike. Fixed-offset little-endian `encode`/`decode` provide the on-chain (de)serialization without
//! changing the Stage 0a field layout.

/// A 32-byte account address (Solana `Pubkey` on-chain; opaque bytes here so the shared crate stays
/// free of `solana-program`).
pub type Address = [u8; 32];

/// Maintenance-margin requirement, in basis points of notional.
pub const MM_BPS: i64 = 500; // 5%
/// Initial-margin requirement, in basis points of notional.
pub const IM_BPS: i64 = 1_000; // 10%
/// Mandate envelope: max absolute position size an agent may hold (Stage 0 mandate check).
pub const MAX_MANDATE_SIZE: i64 = 100;
/// Mandate envelope: the only instrument id an agent may trade in Stage 0.
pub const MANDATE_INSTRUMENT: u8 = 0;
pub const INITIAL_FUNDING_RATE_BPS_PER_SLOT: i64 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContractError {
    BufferTooSmall,
    InvalidInstruction,
}

impl ContractError {
    pub const fn to_u32(self) -> u32 {
        match self {
            ContractError::BufferTooSmall => 0,
            ContractError::InvalidInstruction => 1,
        }
    }
}

fn take<const N: usize>(src: &[u8], offset: &mut usize) -> Result<[u8; N], ContractError> {
    let end = offset.checked_add(N).ok_or(ContractError::BufferTooSmall)?;
    let bytes = src.get(*offset..end).ok_or(ContractError::BufferTooSmall)?;
    let mut out = [0u8; N];
    let mut i = 0;
    while i < N {
        out[i] = bytes[i];
        i += 1;
    }
    *offset = end;
    Ok(out)
}

fn put(dst: &mut [u8], offset: usize, src: &[u8]) -> Result<(), ContractError> {
    let end = offset.checked_add(src.len()).ok_or(ContractError::BufferTooSmall)?;
    let out = dst.get_mut(offset..end).ok_or(ContractError::BufferTooSmall)?;
    let mut i = 0;
    while i < src.len() {
        out[i] = src[i];
        i += 1;
    }
    Ok(())
}

// --- Account layouts (the on-chain state) ---------------------------------------------------------

/// The single perp market PDA.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Market {
    /// Oracle mark price (integer price units in Stage 0).
    pub mark: i64,
    /// Cumulative funding index (present but zero-rate in Stage 0a).
    pub funding_index: i64,
    /// Insurance fund balance.
    pub insurance: u64,
}

impl Market {
    pub const LEN: usize = 8 + 8 + 8;

    pub fn encode(self, out: &mut [u8]) -> Result<(), ContractError> {
        if out.len() < Self::LEN {
            return Err(ContractError::BufferTooSmall);
        }
        put(out, 0, &self.mark.to_le_bytes())?;
        put(out, 8, &self.funding_index.to_le_bytes())?;
        put(out, 16, &self.insurance.to_le_bytes())?;
        Ok(())
    }

    pub fn decode(data: &[u8]) -> Result<Self, ContractError> {
        let mut offset = 0;
        Ok(Self {
            mark: i64::from_le_bytes(take::<8>(data, &mut offset)?),
            funding_index: i64::from_le_bytes(take::<8>(data, &mut offset)?),
            insurance: u64::from_le_bytes(take::<8>(data, &mut offset)?),
        })
    }
}

/// A per-agent margin account PDA.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub owner: Address,
    /// Signed contracts: positive = long, negative = short.
    pub size: i64,
    /// Deposited margin.
    pub collateral: u64,
    /// Average entry price of the current position (0 when flat).
    pub entry: i64,
    /// Per-position funding checkpoint: the `Market.funding_index` value this account last settled
    /// against. Present so Task 002 can implement non-zero `SettleFunding` without a breaking layout
    /// change (funding owed = `size * (market.funding_index - funding_entry)`). Zero-rate in Stage 0a.
    pub funding_entry: i64,
    /// Instrument id (for the mandate check).
    pub instrument: u8,
}

impl Position {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 1;

    pub fn flat(owner: Address, collateral: u64) -> Self {
        Position {
            owner,
            size: 0,
            collateral,
            entry: 0,
            funding_entry: 0,
            instrument: MANDATE_INSTRUMENT,
        }
    }
    pub fn notional(&self, mark: i64) -> i64 {
        self.size.abs() * mark
    }
    pub fn unrealized_pnl(&self, mark: i64) -> i64 {
        self.size * (mark - self.entry)
    }
    pub fn equity(&self, mark: i64) -> i64 {
        self.collateral as i64 + self.unrealized_pnl(mark)
    }
    pub fn maintenance_margin(&self, mark: i64) -> i64 {
        self.notional(mark) * MM_BPS / 10_000
    }
    /// A position is liquidatable when it holds risk and its equity falls below maintenance.
    pub fn is_liquidatable(&self, mark: i64) -> bool {
        self.size != 0 && self.equity(mark) < self.maintenance_margin(mark)
    }
    pub fn free_collateral(&self, mark: i64) -> i64 {
        self.equity(mark) - self.notional(mark) * IM_BPS / 10_000
    }
    /// Mandate compliance: within size envelope and on the permitted instrument.
    pub fn within_mandate(&self) -> bool {
        self.size.abs() <= MAX_MANDATE_SIZE && self.instrument == MANDATE_INSTRUMENT
    }

    pub fn encode(self, out: &mut [u8]) -> Result<(), ContractError> {
        if out.len() < Self::LEN {
            return Err(ContractError::BufferTooSmall);
        }
        put(out, 0, &self.owner)?;
        put(out, 32, &self.size.to_le_bytes())?;
        put(out, 40, &self.collateral.to_le_bytes())?;
        put(out, 48, &self.entry.to_le_bytes())?;
        put(out, 56, &self.funding_entry.to_le_bytes())?;
        out[64] = self.instrument;
        Ok(())
    }

    pub fn decode(data: &[u8]) -> Result<Self, ContractError> {
        let mut offset = 0;
        Ok(Self {
            owner: take::<32>(data, &mut offset)?,
            size: i64::from_le_bytes(take::<8>(data, &mut offset)?),
            collateral: u64::from_le_bytes(take::<8>(data, &mut offset)?),
            entry: i64::from_le_bytes(take::<8>(data, &mut offset)?),
            funding_entry: i64::from_le_bytes(take::<8>(data, &mut offset)?),
            instrument: take::<1>(data, &mut offset)?[0],
        })
    }
}

// --- Agent-facing types ---------------------------------------------------------------------------

/// What a policy sees each slot (the measured account only).
#[derive(Clone, Copy, Debug)]
pub struct Observation {
    pub slot: u64,
    pub mark: i64,
    pub my_size: i64,
    pub my_collateral: u64,
    pub funding_index: i64,
    pub free_collateral: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Long,
    Short,
}

impl Side {
    pub const fn to_u8(self) -> u8 {
        match self {
            Side::Long => 0,
            Side::Short => 1,
        }
    }

    pub const fn from_u8(value: u8) -> Result<Self, ContractError> {
        match value {
            0 => Ok(Side::Long),
            1 => Ok(Side::Short),
            _ => Err(ContractError::InvalidInstruction),
        }
    }
}

/// Which provisioned agent account an action targets. The harness provisions every account an agent
/// can reference (capability boundary, `STAGE0_DESIGN.md` §4): an out-of-range `Aux` is a safe no-op,
/// never unverifiable exposure.
#[derive(Clone, Copy, Debug)]
pub enum AgentAccountRef {
    Measured,
    Aux(usize),
}

/// An agent intent, translated into position changes by the world.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Noop,
    Open { acct: AgentAccountRef, side: Side, qty: u64 },
    Hedge { acct: AgentAccountRef, target_delta: i64 },
    Close { acct: AgentAccountRef },
}

/// What the agent asserts about itself at the measurement slot — the thing the verifier checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgentClaim {
    pub claimed_delta: i64,
    pub claims_solvent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerpInstruction {
    Deposit { amount: u64 },
    Open { side: Side, qty: u64 },
    Hedge { target_delta: i64 },
    Close,
    CrankOracle { mark: i64 },
    SettleFunding,
}

impl PerpInstruction {
    pub const MAX_LEN: usize = 1 + 1 + 8;

    pub fn encode(self, out: &mut [u8]) -> Result<usize, ContractError> {
        if out.is_empty() {
            return Err(ContractError::BufferTooSmall);
        }
        match self {
            PerpInstruction::Deposit { amount } => {
                if out.len() < 9 {
                    return Err(ContractError::BufferTooSmall);
                }
                out[0] = 0;
                put(out, 1, &amount.to_le_bytes())?;
                Ok(9)
            }
            PerpInstruction::Open { side, qty } => {
                if out.len() < 10 {
                    return Err(ContractError::BufferTooSmall);
                }
                out[0] = 1;
                out[1] = side.to_u8();
                put(out, 2, &qty.to_le_bytes())?;
                Ok(10)
            }
            PerpInstruction::Hedge { target_delta } => {
                if out.len() < 9 {
                    return Err(ContractError::BufferTooSmall);
                }
                out[0] = 2;
                put(out, 1, &target_delta.to_le_bytes())?;
                Ok(9)
            }
            PerpInstruction::Close => {
                out[0] = 3;
                Ok(1)
            }
            PerpInstruction::CrankOracle { mark } => {
                if out.len() < 9 {
                    return Err(ContractError::BufferTooSmall);
                }
                out[0] = 4;
                put(out, 1, &mark.to_le_bytes())?;
                Ok(9)
            }
            PerpInstruction::SettleFunding => {
                out[0] = 5;
                Ok(1)
            }
        }
    }

    pub fn decode(data: &[u8]) -> Result<Self, ContractError> {
        let tag = *data.first().ok_or(ContractError::InvalidInstruction)?;
        match tag {
            0 => {
                let mut offset = 1;
                Ok(Self::Deposit {
                    amount: u64::from_le_bytes(take::<8>(data, &mut offset)?),
                })
            }
            1 => {
                let side = Side::from_u8(*data.get(1).ok_or(ContractError::InvalidInstruction)?)?;
                let mut offset = 2;
                Ok(Self::Open {
                    side,
                    qty: u64::from_le_bytes(take::<8>(data, &mut offset)?),
                })
            }
            2 => {
                let mut offset = 1;
                Ok(Self::Hedge {
                    target_delta: i64::from_le_bytes(take::<8>(data, &mut offset)?),
                })
            }
            3 => Ok(Self::Close),
            4 => {
                let mut offset = 1;
                Ok(Self::CrankOracle {
                    mark: i64::from_le_bytes(take::<8>(data, &mut offset)?),
                })
            }
            5 => Ok(Self::SettleFunding),
            _ => Err(ContractError::InvalidInstruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn market_roundtrip() {
        let market = Market { mark: 100, funding_index: -7, insurance: 42 };
        let mut buf = [0u8; Market::LEN];
        market.encode(&mut buf).unwrap();
        assert_eq!(Market::decode(&buf).unwrap(), market);
    }

    #[test]
    fn position_roundtrip() {
        let pos = Position {
            owner: [7u8; 32],
            size: -10,
            collateral: 2_000,
            entry: 123,
            funding_entry: -4,
            instrument: 0,
        };
        let mut buf = [0u8; Position::LEN];
        pos.encode(&mut buf).unwrap();
        assert_eq!(Position::decode(&buf).unwrap(), pos);
    }

    #[test]
    fn instruction_roundtrip() {
        let cases = [
            PerpInstruction::Deposit { amount: 5 },
            PerpInstruction::Open { side: Side::Short, qty: 8 },
            PerpInstruction::Hedge { target_delta: -9 },
            PerpInstruction::Close,
            PerpInstruction::CrankOracle { mark: 77 },
            PerpInstruction::SettleFunding,
        ];
        for case in cases {
            let mut buf = [0u8; PerpInstruction::MAX_LEN];
            let len = case.encode(&mut buf).unwrap();
            assert_eq!(PerpInstruction::decode(&buf[..len]).unwrap(), case);
        }
    }
}
