//! Program entrypoint.

use {
    crate::{error::LoaderV3Error, processor},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
};

solana_program::entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process(program_id, accounts, input) {
        error.print::<LoaderV3Error>();
        return Err(error);
    }
    Ok(())
}
