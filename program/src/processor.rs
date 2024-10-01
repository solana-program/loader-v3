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
        program::{invoke, invoke_signed},
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
fn process_upgrade(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let programdata_info = next_account_info(accounts_iter)?;
    let program_info = next_account_info(accounts_iter)?;
    let buffer_info = next_account_info(accounts_iter)?;
    let spill_info = next_account_info(accounts_iter)?;
    let rent_info = next_account_info(accounts_iter)?;
    let clock_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    let rent = <Rent as Sysvar>::from_account_info(rent_info)?;
    let clock = <Clock as Sysvar>::from_account_info(clock_info)?;

    // Verify Program account.
    if !program_info.executable {
        msg!("Program account not executable");
        return Err(ProgramError::InvalidAccountData); // [CORE BPF]: Error code changed.
    }
    if !program_info.is_writable {
        msg!("Program account not writable");
        return Err(ProgramError::InvalidArgument);
    }
    if program_info.owner != program_id {
        msg!("Program account not owned by loader");
        return Err(ProgramError::IncorrectProgramId);
    }
    {
        let program_data = program_info.try_borrow_data()?;
        if let UpgradeableLoaderState::Program {
            programdata_address,
        } = UpgradeableLoaderState::deserialize(&program_data)?
        {
            if programdata_address != *programdata_info.key {
                msg!("Program and ProgramData account mismatch");
                return Err(ProgramError::InvalidArgument);
            }
        } else {
            msg!("Invalid Program account");
            return Err(ProgramError::InvalidAccountData);
        }
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
    let buffer_lamports = buffer_info.lamports();
    let buffer_data_offset = UpgradeableLoaderState::size_of_buffer_metadata();
    let buffer_data_len = buffer_info.data_len().saturating_sub(buffer_data_offset);
    if buffer_info.data_len() < UpgradeableLoaderState::size_of_buffer_metadata()
        || buffer_data_len == 0
    {
        msg!("Buffer account too small");
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify ProgramData account.
    let programdata_data_offset = UpgradeableLoaderState::size_of_programdata_metadata();
    let programdata_balance_required = 1.max(rent.minimum_balance(programdata_info.data_len()));
    if programdata_info.data_len() < UpgradeableLoaderState::size_of_programdata(buffer_data_len) {
        msg!("ProgramData account not large enough");
        return Err(ProgramError::AccountDataTooSmall);
    }
    if programdata_info.lamports().saturating_add(buffer_lamports) < programdata_balance_required {
        msg!("Buffer account balance too low to fund upgrade");
        return Err(ProgramError::InsufficientFunds);
    }
    {
        let programdata_data = programdata_info.try_borrow_data()?;
        if let UpgradeableLoaderState::ProgramData {
            slot,
            upgrade_authority_address,
        } = UpgradeableLoaderState::deserialize(&programdata_data)?
        {
            if clock.slot == slot {
                msg!("Program was deployed in this block already");
                return Err(ProgramError::InvalidArgument);
            }
            if upgrade_authority_address.is_none() {
                msg!("Program not upgradeable");
                return Err(ProgramError::Immutable);
            }
            if upgrade_authority_address != Some(*authority_info.key) {
                msg!("Incorrect upgrade authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !authority_info.is_signer {
                msg!("Upgrade authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
        } else {
            msg!("Invalid ProgramData account");
            return Err(ProgramError::InvalidAccountData);
        }
    }

    // Load and verify the program bits.
    // [CORE BPF]: We'll see what happens with on-chain verification...
    // Something like this would be nice:
    // invoke(
    //     &solana_bpf_verify_program::instruction::verify(buffer_info.key),
    //     &[buffer_info.clone()],
    // )?;

    // Update the ProgramData account, record the upgraded data, and zero the
    // rest.
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
            programdata_data
                .get_mut(programdata_data_offset..)
                .ok_or(ProgramError::AccountDataTooSmall)?
                .copy_from_slice(elf_bits);
        }

        // Zero the rest.
        programdata_data
            .get_mut(programdata_data_offset.saturating_add(buffer_data_len)..)
            .ok_or(ProgramError::AccountDataTooSmall)?
            .fill(0);

        // Clear the buffer.
        buffer_info.realloc(UpgradeableLoaderState::size_of_buffer(0), false)?;
    }

    // Fund ProgramData to rent-exemption, spill the rest.
    {
        let new_spill_lamports = spill_info
            .lamports()
            .checked_add(
                programdata_info
                    .lamports()
                    .saturating_add(buffer_lamports)
                    .saturating_sub(programdata_balance_required),
            )
            .ok_or::<ProgramError>(ProgramError::ArithmeticOverflow)?;

        **buffer_info.try_borrow_mut_lamports()? = 0;
        **spill_info.try_borrow_mut_lamports()? = new_spill_lamports;
        **programdata_info.try_borrow_mut_lamports()? = programdata_balance_required;
    }

    // [CORE BPF]: Store modified entry in program cache.

    msg!("Upgraded program: {}", program_info.key);

    Ok(())
}

/// Processes a
/// [SetAuthority](enum.LoaderV3Instruction.html)
/// instruction.
fn process_set_authority(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_or_programdata_info = next_account_info(accounts_iter)?;
    let current_authority_info = next_account_info(accounts_iter)?;
    let new_authority_info = next_account_info(accounts_iter).ok();

    // Need this to avoid a double-borrow on account data.
    enum AuthorityType {
        Buffer,
        ProgramData { slot: u64 },
    }

    let authority_type = match UpgradeableLoaderState::deserialize(
        &buffer_or_programdata_info.try_borrow_data()?,
    )? {
        UpgradeableLoaderState::Buffer { authority_address } => {
            if new_authority_info.is_none() {
                msg!("Buffer authority is not optional");
                return Err(ProgramError::IncorrectAuthority);
            }
            if authority_address.is_none() {
                msg!("Buffer is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect buffer authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Buffer authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::Buffer
        }
        UpgradeableLoaderState::ProgramData {
            upgrade_authority_address,
            slot,
        } => {
            if upgrade_authority_address.is_none() {
                msg!("ProgramData is not upgradeable");
                return Err(ProgramError::Immutable);
            }
            if upgrade_authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect upgrade authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Upgrade authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::ProgramData { slot }
        }
        _ => {
            msg!("Account does not support authorities");
            return Err(ProgramError::InvalidArgument);
        }
    };

    match authority_type {
        AuthorityType::Buffer => {
            // This looks silly, but `bincode::serialize_into` will serialize 4
            // bytes for `None` and then stop, leaving the rest of the buffer
            // untouched.
            let mut buffer_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            let buffer_metadata_offset = UpgradeableLoaderState::size_of_buffer_metadata();
            buffer_data
                .get_mut(..buffer_metadata_offset)
                .ok_or(ProgramError::AccountDataTooSmall)?
                .fill(0);
            bincode::serialize_into(
                &mut buffer_data[..],
                &UpgradeableLoaderState::Buffer {
                    authority_address: new_authority_info.map(|info| *info.key),
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
        AuthorityType::ProgramData { slot } => {
            // This looks silly, but `bincode::serialize_into` will serialize 4
            // bytes for `None` and then stop, leaving the rest of the buffer
            // untouched.
            let mut programdata_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            let programdata_metadata_offset =
                UpgradeableLoaderState::size_of_programdata_metadata();
            programdata_data
                .get_mut(..programdata_metadata_offset)
                .ok_or(ProgramError::AccountDataTooSmall)?
                .fill(0);
            bincode::serialize_into(
                &mut programdata_data[..],
                &UpgradeableLoaderState::ProgramData {
                    upgrade_authority_address: new_authority_info.map(|info| *info.key),
                    slot,
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
    }

    msg!(
        "New authority: {:?}",
        new_authority_info.map(|info| info.key)
    );

    Ok(())
}

/// Processes a
/// [Close](enum.LoaderV3Instruction.html)
/// instruction.
fn process_close(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_or_programdata_info = next_account_info(accounts_iter)?;
    let destination_info = next_account_info(accounts_iter)?;

    if buffer_or_programdata_info.key == destination_info.key {
        msg!("Recipient is the same as the account being closed");
        return Err(ProgramError::InvalidArgument);
    }

    // Need this to avoid a double-borrow on account data.
    // I guess it's only for a log message at this point?
    enum AuthorityType {
        Uninitialized,
        Buffer,
        ProgramData,
    }

    let authority_type = match UpgradeableLoaderState::deserialize(
        &buffer_or_programdata_info.try_borrow_data()?,
    )? {
        UpgradeableLoaderState::Uninitialized => AuthorityType::Uninitialized,
        UpgradeableLoaderState::Buffer { authority_address } => {
            let authority_info = next_account_info(accounts_iter)?;
            if authority_address.is_none() {
                msg!("Account is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*authority_info.key) {
                msg!("Incorrect authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !authority_info.is_signer {
                msg!("Authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }

            AuthorityType::Buffer
        }
        UpgradeableLoaderState::ProgramData {
            slot,
            upgrade_authority_address,
        } => {
            let authority_info = next_account_info(accounts_iter)?;
            let program_info = next_account_info(accounts_iter)?;

            if !program_info.is_writable {
                msg!("Program account is not writable");
                return Err(ProgramError::InvalidArgument);
            }
            if program_info.owner != program_id {
                msg!("Program account not owned by loader");
                return Err(ProgramError::IncorrectProgramId);
            }

            let clock = <Clock as Sysvar>::get()?;
            if clock.slot == slot {
                msg!("Program was deployed in this block already");
                return Err(ProgramError::InvalidArgument);
            }

            match UpgradeableLoaderState::deserialize(&program_info.try_borrow_data()?)? {
                UpgradeableLoaderState::Program {
                    programdata_address,
                } => {
                    if programdata_address != *buffer_or_programdata_info.key {
                        msg!("ProgramData account does not match ProgramData account");
                        return Err(ProgramError::InvalidArgument);
                    }

                    if upgrade_authority_address.is_none() {
                        msg!("Account is immutable");
                        return Err(ProgramError::Immutable);
                    }
                    if upgrade_authority_address != Some(*authority_info.key) {
                        msg!("Incorrect authority provided");
                        return Err(ProgramError::IncorrectAuthority);
                    }
                    if !authority_info.is_signer {
                        msg!("Authority did not sign");
                        return Err(ProgramError::MissingRequiredSignature);
                    }
                }
                _ => {
                    msg!("Invalid Program account");
                    return Err(ProgramError::InvalidArgument);
                }
            }

            AuthorityType::ProgramData
        }
        _ => {
            msg!("Account does not support closing");
            return Err(ProgramError::InvalidArgument);
        }
    };

    {
        let new_destination_lamports = destination_info
            .lamports()
            .checked_add(buffer_or_programdata_info.lamports())
            .ok_or::<ProgramError>(ProgramError::ArithmeticOverflow)?;

        **buffer_or_programdata_info.try_borrow_mut_lamports()? = 0;
        **destination_info.try_borrow_mut_lamports()? = new_destination_lamports;
    }

    buffer_or_programdata_info.realloc(UpgradeableLoaderState::size_of_uninitialized(), true)?;

    let mut buffer_or_programdata_data = buffer_or_programdata_info.try_borrow_mut_data()?;
    bincode::serialize_into(
        &mut buffer_or_programdata_data[..],
        &UpgradeableLoaderState::Uninitialized,
    )
    .map_err(|_| ProgramError::InvalidAccountData)?;

    // [CORE BPF]: Store modified entry in program cache.

    match authority_type {
        AuthorityType::Uninitialized => {
            msg!("Closed Uninitialized {}", buffer_or_programdata_info.key);
        }
        AuthorityType::Buffer => {
            msg!("Closed Buffer {}", buffer_or_programdata_info.key);
        }
        AuthorityType::ProgramData => {
            msg!("Closed Program {}", buffer_or_programdata_info.key);
        }
    }

    Ok(())
}

/// Processes an
/// [ExtendProgram](enum.LoaderV3Instruction.html)
/// instruction.
fn process_extend_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    additional_bytes: u32,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let programdata_info = next_account_info(accounts_iter)?;
    let program_info = next_account_info(accounts_iter)?;

    if programdata_info.owner != program_id {
        msg!("ProgramData owner is invalid");
        return Err(ProgramError::InvalidAccountOwner);
    }
    if !programdata_info.is_writable {
        msg!("ProgramData is not writable");
        return Err(ProgramError::InvalidArgument);
    }

    if !program_info.is_writable {
        msg!("Program account is not writable");
        return Err(ProgramError::InvalidArgument);
    }
    if program_info.owner != program_id {
        msg!("Program account not owned by loader");
        return Err(ProgramError::InvalidAccountOwner);
    }

    match UpgradeableLoaderState::deserialize(&program_info.try_borrow_data()?)? {
        UpgradeableLoaderState::Program {
            programdata_address,
        } => {
            if programdata_address != *programdata_info.key {
                msg!("Program account does not match ProgramData account");
                return Err(ProgramError::InvalidArgument);
            }
        }
        _ => {
            msg!("Invalid Program account");
            return Err(ProgramError::InvalidAccountData);
        }
    }

    let old_len = programdata_info.data_len();
    let new_len = old_len.saturating_add(additional_bytes as usize);
    if new_len > MAX_PERMITTED_DATA_LENGTH as usize {
        msg!(
            "Extended ProgramData length of {} bytes exceeds max account data length of {} bytes",
            new_len,
            MAX_PERMITTED_DATA_LENGTH
        );
        return Err(ProgramError::InvalidRealloc);
    }

    let clock = <Clock as Sysvar>::get()?;
    let clock_slot = clock.slot;

    let upgrade_authority_address = if let UpgradeableLoaderState::ProgramData {
        slot,
        upgrade_authority_address,
    } =
        UpgradeableLoaderState::deserialize(&programdata_info.try_borrow_data()?)?
    {
        if clock_slot == slot {
            msg!("Program was extended in this block already");
            return Err(ProgramError::InvalidArgument);
        }

        if upgrade_authority_address.is_none() {
            msg!("Cannot extend ProgramData accounts that are not upgradeable");
            return Err(ProgramError::Immutable);
        }
        upgrade_authority_address
    } else {
        msg!("ProgramData state is invalid");
        return Err(ProgramError::InvalidAccountData);
    };

    let required_payment = {
        let balance = programdata_info.lamports();
        let rent = <Rent as Sysvar>::get()?;
        let min_balance = rent.minimum_balance(new_len).max(1);
        min_balance.saturating_sub(balance)
    };

    if required_payment > 0 {
        let system_program_info = next_account_info(accounts_iter)?;
        let payer_info = next_account_info(accounts_iter)?;
        invoke(
            &system_instruction::transfer(payer_info.key, programdata_info.key, required_payment),
            &[
                payer_info.clone(),
                programdata_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    // [CORE BPF]: BPF programs can only reallocate a maximum of 10_240 bytes.
    // See https://github.com/anza-xyz/agave/blob/ed51e70c2e6528f602ad4f8fde718f60d7da2d0c/sdk/account-info/src/lib.rs#L16-L17
    programdata_info.realloc(new_len, true)?;

    // [CORE BPF]: We'll see what happens with on-chain verification...
    // Something like this would be nice:
    // invoke(
    //     &solana_bpf_verify_program::instruction::verify(programdata_info.key),
    //     &[buffer_info.clone()],
    // )?;

    let mut programdata_data = programdata_info.try_borrow_mut_data()?;
    bincode::serialize_into(
        &mut programdata_data[..],
        &UpgradeableLoaderState::ProgramData {
            slot: clock_slot,
            upgrade_authority_address,
        },
    )
    .map_err(|_| ProgramError::InvalidAccountData)?;

    // [CORE BPF]: Store modified entry in program cache.

    msg!("Extended ProgramData account by {} bytes", additional_bytes);

    Ok(())
}

/// Processes a
/// [SetAuthorityChecked](enum.LoaderV3Instruction.html)
/// instruction.
fn process_set_authority_checked(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_or_programdata_info = next_account_info(accounts_iter)?;
    let current_authority_info = next_account_info(accounts_iter)?;
    let new_authority_info = next_account_info(accounts_iter)?;

    // Need this to avoid a double-borrow on account data.
    enum AuthorityType {
        Buffer,
        ProgramData { slot: u64 },
    }

    let authority_type = match UpgradeableLoaderState::deserialize(
        &buffer_or_programdata_info.try_borrow_data()?,
    )? {
        UpgradeableLoaderState::Buffer { authority_address } => {
            if authority_address.is_none() {
                msg!("Buffer is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect buffer authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Buffer authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            if !new_authority_info.is_signer {
                msg!("New authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::Buffer
        }
        UpgradeableLoaderState::ProgramData {
            upgrade_authority_address,
            slot,
        } => {
            if upgrade_authority_address.is_none() {
                msg!("Program not upgradeable");
                return Err(ProgramError::Immutable);
            }
            if upgrade_authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect upgrade authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Upgrade authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            if !new_authority_info.is_signer {
                msg!("New authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::ProgramData { slot }
        }
        _ => {
            msg!("Account does not support authorities");
            return Err(ProgramError::InvalidArgument);
        }
    };

    match authority_type {
        AuthorityType::Buffer => {
            // No need to zero anything, since this instruction requires the
            // new authority to sign (cannot be `None`).
            let mut buffer_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            bincode::serialize_into(
                &mut buffer_data[..],
                &UpgradeableLoaderState::Buffer {
                    authority_address: Some(*new_authority_info.key),
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
        AuthorityType::ProgramData { slot } => {
            // No need to zero anything, since this instruction requires the
            // new authority to sign (cannot be `None`).
            let mut programdata_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            bincode::serialize_into(
                &mut programdata_data[..],
                &UpgradeableLoaderState::ProgramData {
                    upgrade_authority_address: Some(*new_authority_info.key),
                    slot,
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
    }

    msg!("New authority: {:?}", new_authority_info.key);

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
