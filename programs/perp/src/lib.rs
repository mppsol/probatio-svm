#![cfg_attr(feature = "bpf-entrypoint", no_std)]

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{pubkey_eq, Pubkey},
    ProgramResult,
};
use probatio_contract::{ContractError, Market, PerpInstruction, Position, Side};

pinocchio_pubkey::declare_id!("GtdambwDgHWrDJdVPBkEHGhCwokqgAoch162teUjJse2");
pub const HARNESS_AUTHORITY: Pubkey =
    pinocchio_pubkey::pubkey!("9Hh9h1ATNtRdkNUT3GBwau2RDn9tyjVf1LDCToXDGhcM");

#[cfg(feature = "bpf-entrypoint")]
pinocchio::program_entrypoint!(entry);
#[cfg(feature = "bpf-entrypoint")]
pinocchio::default_allocator!();
#[cfg(feature = "bpf-entrypoint")]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { pinocchio::syscalls::abort() }
}

#[cfg(feature = "bpf-entrypoint")]
fn entry(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    process_instruction(program_id, accounts, instruction_data)
}

fn map_contract_error(err: ContractError) -> ProgramError {
    ProgramError::Custom(err.to_u32())
}

fn read_market(account: &AccountInfo, program_id: &Pubkey) -> Result<Market, ProgramError> {
    if !account.is_writable() || !account.is_owned_by(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }
    let data = account.try_borrow_data()?;
    Market::decode(&data).map_err(map_contract_error)
}

fn write_market(account: &AccountInfo, market: Market) -> ProgramResult {
    let mut data = account.try_borrow_mut_data()?;
    market.encode(&mut data).map_err(map_contract_error)
}

fn read_position(account: &AccountInfo, program_id: &Pubkey) -> Result<Position, ProgramError> {
    if !account.is_writable() || !account.is_owned_by(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }
    let data = account.try_borrow_data()?;
    Position::decode(&data).map_err(map_contract_error)
}

fn write_position(account: &AccountInfo, position: Position) -> ProgramResult {
    let mut data = account.try_borrow_mut_data()?;
    position.encode(&mut data).map_err(map_contract_error)
}

fn checked_add_u64(value: u64, delta: i64) -> Result<u64, ProgramError> {
    let widened = (value as i128) + (delta as i128);
    if widened < 0 || widened > u64::MAX as i128 {
        return Err(ProgramError::ArithmeticOverflow);
    }
    Ok(widened as u64)
}

fn realized_collateral(collateral: u64, pnl: i64) -> u64 {
    ((collateral as i128) + (pnl as i128)).max(0) as u64
}

fn trade(pos: &mut Position, delta: i64, price: i64) -> Result<(), ProgramError> {
    if delta == 0 {
        return Ok(());
    }
    let new_size = pos
        .size
        .checked_add(delta)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    if pos.size == 0 {
        pos.entry = price;
    } else if (pos.size > 0) == (delta > 0) {
        let total = pos
            .size
            .abs()
            .checked_add(delta.abs())
            .ok_or(ProgramError::ArithmeticOverflow)?;
        let weighted_entry = pos
            .entry
            .checked_mul(pos.size.abs())
            .and_then(|v| price.checked_mul(delta.abs()).and_then(|rhs| v.checked_add(rhs)))
            .ok_or(ProgramError::ArithmeticOverflow)?;
        pos.entry = weighted_entry / total;
    } else {
        let closed = delta.abs().min(pos.size.abs());
        let dir: i64 = if pos.size > 0 { 1 } else { -1 };
        let pnl = dir
            .checked_mul(closed)
            .and_then(|v| price.checked_sub(pos.entry).and_then(|spread| v.checked_mul(spread)))
            .ok_or(ProgramError::ArithmeticOverflow)?;
        pos.collateral = realized_collateral(pos.collateral, pnl);
        if delta.abs() > pos.size.abs() {
            pos.entry = price;
        }
    }
    pos.size = new_size;
    if pos.size == 0 {
        pos.entry = 0;
    }
    Ok(())
}

fn require_position_owner(position: &Position, owner: &AccountInfo) -> ProgramResult {
    if !owner.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !pubkey_eq(&position.owner, owner.key()) {
        return Err(ProgramError::IllegalOwner);
    }
    Ok(())
}

fn require_harness(authority: &AccountInfo) -> ProgramResult {
    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !pubkey_eq(authority.key(), &HARNESS_AUTHORITY) {
        return Err(ProgramError::IncorrectAuthority);
    }
    Ok(())
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PerpInstruction::decode(instruction_data).map_err(map_contract_error)?;
    match instruction {
        PerpInstruction::Deposit { amount } => {
            let [market_acc, position_acc, owner_acc] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            let _market = read_market(market_acc, program_id)?;
            let mut position = read_position(position_acc, program_id)?;
            require_position_owner(&position, owner_acc)?;
            position.collateral = position
                .collateral
                .checked_add(amount)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            write_position(position_acc, position)
        }
        PerpInstruction::Open { side, qty } => {
            let [market_acc, position_acc, owner_acc] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            let market = read_market(market_acc, program_id)?;
            let mut position = read_position(position_acc, program_id)?;
            require_position_owner(&position, owner_acc)?;
            let delta = match side {
                Side::Long => qty as i64,
                Side::Short => -(qty as i64),
            };
            trade(&mut position, delta, market.mark)?;
            write_position(position_acc, position)
        }
        PerpInstruction::Hedge { target_delta } => {
            let [market_acc, position_acc, owner_acc] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            let market = read_market(market_acc, program_id)?;
            let mut position = read_position(position_acc, program_id)?;
            require_position_owner(&position, owner_acc)?;
            let delta = target_delta
                .checked_sub(position.size)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            trade(&mut position, delta, market.mark)?;
            write_position(position_acc, position)
        }
        PerpInstruction::Close => {
            let [market_acc, position_acc, owner_acc] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            let market = read_market(market_acc, program_id)?;
            let mut position = read_position(position_acc, program_id)?;
            require_position_owner(&position, owner_acc)?;
            let delta = -position.size;
            trade(&mut position, delta, market.mark)?;
            write_position(position_acc, position)
        }
        PerpInstruction::CrankOracle { mark } => {
            let [market_acc, authority_acc] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            require_harness(authority_acc)?;
            let mut market = read_market(market_acc, program_id)?;
            market.mark = mark;
            write_market(market_acc, market)
        }
        PerpInstruction::SettleFunding => {
            let [market_acc, position_acc] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            let market = read_market(market_acc, program_id)?;
            let mut position = read_position(position_acc, program_id)?;
            let funding_delta = market
                .funding_index
                .checked_sub(position.funding_entry)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            let payment = position
                .size
                .checked_mul(funding_delta)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            position.collateral = checked_add_u64(position.collateral, -payment)?;
            position.funding_entry = market.funding_index;
            write_position(position_acc, position)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(size: i64, collateral: u64, entry: i64) -> Position {
        Position { owner: [0; 32], size, collateral, entry, funding_entry: 0, instrument: 0 }
    }

    #[test]
    fn trade_matches_reference_reduce_and_flip_paths() {
        let mut a = pos(10, 1_000, 100);
        trade(&mut a, -15, 120).unwrap();
        assert_eq!(a.size, -5);
        assert_eq!(a.entry, 120);
        assert_eq!(a.collateral, 1_200);

        let mut b = pos(-10, 1_000, 100);
        trade(&mut b, 4, 80).unwrap();
        assert_eq!(b.size, -6);
        assert_eq!(b.entry, 100);
        assert_eq!(b.collateral, 1_080);
    }
}
