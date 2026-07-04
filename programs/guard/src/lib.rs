#![cfg_attr(feature = "bpf-entrypoint", no_std)]

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use probatio_contract::{ContractError, GuardInstruction, Market, Position};

pinocchio_pubkey::declare_id!("1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM");

#[cfg(feature = "bpf-entrypoint")]
pinocchio::program_entrypoint!(entry);
#[cfg(feature = "bpf-entrypoint")]
pinocchio::default_allocator!();
#[cfg(feature = "bpf-entrypoint")]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(feature = "bpf-entrypoint")]
fn entry(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    process_instruction(program_id, accounts, instruction_data)
}

#[repr(u32)]
enum GuardError {
    MandateDeviation = 10,
    SelfInflictedInsolvency = 11,
}

fn map_contract_error(err: ContractError) -> ProgramError {
    ProgramError::Custom(err.to_u32())
}

fn read_market(account: &AccountInfo) -> Result<Market, ProgramError> {
    let data = account.try_borrow_data()?;
    Market::decode(&data).map_err(map_contract_error)
}

fn read_position(account: &AccountInfo) -> Result<Position, ProgramError> {
    let data = account.try_borrow_data()?;
    Position::decode(&data).map_err(map_contract_error)
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match GuardInstruction::decode(instruction_data).map_err(map_contract_error)? {
        GuardInstruction::CheckPosition => {
            let [market_acc, position_acc] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            let market = read_market(market_acc)?;
            let position = read_position(position_acc)?;
            if !position.within_mandate() {
                return Err(ProgramError::Custom(GuardError::MandateDeviation as u32));
            }
            if position.is_liquidatable(market.mark) {
                return Err(ProgramError::Custom(GuardError::SelfInflictedInsolvency as u32));
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_instruction_decodes() {
        assert_eq!(
            GuardInstruction::decode(&[0]).unwrap(),
            GuardInstruction::CheckPosition
        );
    }
}
