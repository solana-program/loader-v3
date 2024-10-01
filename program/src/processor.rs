//! Program processor.

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

/// Processes a
/// [LoaderV3Instruction](enum.LoaderV3Instruction.html).
pub fn process(_program_id: &Pubkey, _accounts: &[AccountInfo], _input: &[u8]) -> ProgramResult {
    Ok(())
}
