//! Program processor.

use {
    crate::{
        instruction::LoaderV3Instruction,
        state::{get_program_data_address_and_bump_seed, UpgradeableLoaderState},
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        instruction::AccountMeta,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction::{self, MAX_PERMITTED_DATA_LENGTH},
        sysvar::Sysvar,
    },
};

// [Core BPF]: Locally-implemented
// `solana_sdk::program_utils::limited_deserialize`.
fn limited_deserialize<T>(input: &[u8]) -> Result<T, ProgramError>
where
    T: serde::de::DeserializeOwned,
{
    solana_program::program_utils::limited_deserialize(
        input, 1232, // [Core BPF]: See `solana_sdk::packet::PACKET_DATA_SIZE`
    )
    .map_err(|_| ProgramError::InvalidInstructionData)
}

/// Processes an
/// [InitializeBuffer](enum.LoaderV3Instruction.html)
/// instruction.
fn process_initialize_buffer(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let source_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    // Ensure the buffer has not already been initialized.
    {
        let buffer_data = source_info.try_borrow_data()?;
        if UpgradeableLoaderState::Uninitialized
            != UpgradeableLoaderState::deserialize(&buffer_data)?
        {
            msg!("Buffer account already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    }

    let mut buffer_data = source_info.try_borrow_mut_data()?;
    bincode::serialize_into(
        &mut buffer_data[..],
        &UpgradeableLoaderState::Buffer {
            authority_address: Some(*authority_info.key),
        },
    )
    .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(())
}

/// Processes a
/// [Write](enum.LoaderV3Instruction.html)
/// instruction.
fn process_write(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    offset: u32,
    bytes: Vec<u8>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    // Run checks on the authority.
    {
        let buffer_data = buffer_info.try_borrow_data()?;
        if let UpgradeableLoaderState::Buffer { authority_address } =
            UpgradeableLoaderState::deserialize(&buffer_data)?
        {
            if authority_address.is_none() {
                msg!("Buffer is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*authority_info.key) {
                msg!("Incorrect buffer authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !authority_info.is_signer {
                msg!("Buffer authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
        } else {
            msg!("Invalid Buffer account");
            return Err(ProgramError::InvalidAccountData);
        }
    }

    // Ensure the buffer account is large enough.
    let programdata_offset =
        UpgradeableLoaderState::size_of_buffer_metadata().saturating_add(offset as usize);
    let end_offset = programdata_offset.saturating_add(bytes.len());
    if buffer_info.data_len() < end_offset {
        msg!(
            "Write overflow: {} < {}",
            buffer_info.data_len(),
            end_offset
        );
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Write the data.
    buffer_info
        .try_borrow_mut_data()?
        .get_mut(programdata_offset..end_offset)
        .ok_or(ProgramError::AccountDataTooSmall)?
        .copy_from_slice(&bytes);

    Ok(())
}

/// Processes a
/// [DeployWithMaxDataLen](enum.LoaderV3Instruction.html)
/// instruction.
fn process_deploy_with_max_data_len(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    max_data_len: usize,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_info = next_account_info(accounts_iter)?;
    let programdata_info = next_account_info(accounts_iter)?;
    let program_info = next_account_info(accounts_iter)?;
    let buffer_info = next_account_info(accounts_iter)?;
    let rent_info = next_account_info(accounts_iter)?;
    let clock_info = next_account_info(accounts_iter)?;
    let system_program_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    let rent = <Rent as Sysvar>::from_account_info(rent_info)?;
    let clock = <Clock as Sysvar>::from_account_info(clock_info)?;

    // Verify Program account.
    {
        let program_data = program_info.try_borrow_data()?;
        if UpgradeableLoaderState::Uninitialized
            != UpgradeableLoaderState::deserialize(&program_data)?
        {
            msg!("Program account already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    }
    if program_info.data_len() < UpgradeableLoaderState::size_of_program() {
        msg!("Program account too small");
        return Err(ProgramError::AccountDataTooSmall);
    }
    if program_info.lamports() < rent.minimum_balance(program_info.data_len()) {
        msg!("Program account not rent-exempt");
        return Err(ProgramError::InsufficientFunds); // [CORE BPF]: Error code changed.
    }

    // Verify Buffer account.
    {
        let buffer_data = buffer_info.try_borrow_data()?;
        if let UpgradeableLoaderState::Buffer { authority_address } =
            UpgradeableLoaderState::deserialize(&buffer_data)?
        {
            if authority_address != Some(*authority_info.key) {
                msg!("Buffer and upgrade authority don't match");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !authority_info.is_signer {
                msg!("Upgrade authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
        } else {
            msg!("Invalid Buffer account");
            return Err(ProgramError::InvalidArgument);
        }
    }
    let buffer_data_offset = UpgradeableLoaderState::size_of_buffer_metadata();
    let buffer_data_len = buffer_info.data_len().saturating_sub(buffer_data_offset);
    let programdata_data_offset = UpgradeableLoaderState::size_of_programdata_metadata();
    let programdata_len = UpgradeableLoaderState::size_of_programdata(max_data_len);
    if buffer_info.data_len() < UpgradeableLoaderState::size_of_buffer_metadata()
        || buffer_data_len == 0
    {
        msg!("Buffer account too small");
        return Err(ProgramError::InvalidAccountData);
    }
    if max_data_len < buffer_data_len {
        msg!("Max data length is too small to hold Buffer data");
        return Err(ProgramError::AccountDataTooSmall);
    }
    if programdata_len > MAX_PERMITTED_DATA_LENGTH as usize {
        msg!("Max data length is too large");
        return Err(ProgramError::InvalidArgument);
    }

    // Create ProgramData account.
    let (derived_address, bump_seed) = get_program_data_address_and_bump_seed(program_info.key);
    if derived_address != *programdata_info.key {
        msg!("ProgramData address is not derived");
        return Err(ProgramError::InvalidArgument);
    }

    // Drain the Buffer account to the payer before paying for the ProgramData
    // account.
    {
        let new_payer_lamports = payer_info
            .lamports()
            .checked_add(buffer_info.lamports())
            .ok_or::<ProgramError>(ProgramError::ArithmeticOverflow)?;

        **buffer_info.try_borrow_mut_lamports()? = 0;
        **payer_info.try_borrow_mut_lamports()? = new_payer_lamports;
    }

    // Pass an extra account to avoid the overly strict UnbalancedInstruction
    // error.
    let mut instruction = system_instruction::create_account(
        payer_info.key,
        programdata_info.key,
        1.max(rent.minimum_balance(programdata_len)),
        programdata_len as u64,
        program_id,
    );
    instruction
        .accounts
        .push(AccountMeta::new(*buffer_info.key, false));

    invoke_signed(
        &instruction,
        &[
            payer_info.clone(),
            programdata_info.clone(),
            system_program_info.clone(),
            buffer_info.clone(),
        ],
        &[&[program_info.key.as_ref(), &[bump_seed]]],
    )?;

    // Load and verify the program bits.
    // [CORE BPF]: We'll see what happens with on-chain verification...
    // Something like this would be nice:
    // invoke(
    //     &solana_bpf_verify_program::instruction::verify(buffer_info.key),
    //     &[buffer_info.clone()],
    // )?;

    // Update the ProgramData account and record the program bits.
    {
        let mut programdata_data = programdata_info.try_borrow_mut_data()?;

        // First serialize the ProgramData header.
        bincode::serialize_into(
            &mut programdata_data[..],
            &UpgradeableLoaderState::ProgramData {
                slot: clock.slot,
                upgrade_authority_address: Some(*authority_info.key),
            },
        )
        .map_err(|_| ProgramError::InvalidAccountData)?;

        // Then copy the program bits.
        {
            let buffer_data = buffer_info.try_borrow_data()?;
            let elf_bits = buffer_data
                .get(buffer_data_offset..)
                .ok_or(ProgramError::AccountDataTooSmall)?;
            let programdata_elf_end = UpgradeableLoaderState::size_of_programdata(elf_bits.len());
            programdata_data
                .get_mut(programdata_data_offset..programdata_elf_end)
                .ok_or(ProgramError::AccountDataTooSmall)?
                .copy_from_slice(elf_bits);
        }

        // Clear the buffer.
        buffer_info.realloc(UpgradeableLoaderState::size_of_buffer(0), false)?;
    }

    // Update the Program account.
    {
        let mut program_data = program_info.try_borrow_mut_data()?;
        bincode::serialize_into(
            &mut program_data[..],
            &UpgradeableLoaderState::Program {
                programdata_address: *programdata_info.key,
            },
        )
        .map_err(|_| ProgramError::InvalidAccountData)?;

        // [CORE BPF]: Set `executable` flag!!
    }

    // [CORE BPF]: Store modified entry in program cache.

    msg!("Deployed program: {}", program_info.key);

    Ok(())
}

/// Processes an
/// [Upgrade](enum.LoaderV3Instruction.html)
/// instruction.
fn process_upgrade(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [SetAuthority](enum.LoaderV3Instruction.html)
/// instruction.
fn process_set_authority(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [Close](enum.LoaderV3Instruction.html)
/// instruction.
fn process_close(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes an
/// [ExtendProgram](enum.LoaderV3Instruction.html)
/// instruction.
fn process_extend_program(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _additional_bytes: u32,
) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [SetAuthorityChecked](enum.LoaderV3Instruction.html)
/// instruction.
fn process_set_authority_checked(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [LoaderV3Instruction](enum.LoaderV3Instruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    match limited_deserialize(input)? {
        LoaderV3Instruction::InitializeBuffer => {
            msg!("Instruction: InitializeBuffer");
            process_initialize_buffer(program_id, accounts)
        }
        LoaderV3Instruction::Write { offset, bytes } => {
            msg!("Instruction: Write");
            process_write(program_id, accounts, offset, bytes)
        }
        LoaderV3Instruction::DeployWithMaxDataLen { max_data_len } => {
            msg!("Instruction: DeployWithMaxDataLen");
            process_deploy_with_max_data_len(program_id, accounts, max_data_len)
        }
        LoaderV3Instruction::Upgrade => {
            msg!("Instruction: Upgrade");
            process_upgrade(program_id, accounts)
        }
        LoaderV3Instruction::SetAuthority => {
            msg!("Instruction: SetAuthority");
            process_set_authority(program_id, accounts)
        }
        LoaderV3Instruction::Close => {
            msg!("Instruction: Close");
            process_close(program_id, accounts)
        }
        LoaderV3Instruction::ExtendProgram { additional_bytes } => {
            msg!("Instruction: ExtendProgram");
            process_extend_program(program_id, accounts, additional_bytes)
        }
        LoaderV3Instruction::SetAuthorityChecked => {
            msg!("Instruction: SetAuthorityChecked");
            process_set_authority_checked(program_id, accounts)
        }
    }
}
