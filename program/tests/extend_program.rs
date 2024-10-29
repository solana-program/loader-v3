#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{setup, system_account_with_lamports, upgradeable_state_account},
    mollusk_svm::{program::keyed_account_for_system_program, result::Check},
    solana_loader_v3_program::{
        instruction::extend_program,
        state::{get_program_data_address, UpgradeableLoaderState},
    },
    solana_sdk::{
        account::WritableAccount, entrypoint::MAX_PERMITTED_DATA_INCREASE,
        program_error::ProgramError, pubkey::Pubkey, system_instruction::MAX_PERMITTED_DATA_LENGTH,
    },
};

#[test]
fn fail_programdata_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    let mut programdata_account = upgradeable_state_account(
        &UpgradeableLoaderState::ProgramData {
            slot: 0,
            upgrade_authority_address: Some(Pubkey::new_unique()),
        },
        elf,
        false,
    );
    programdata_account.set_owner(Pubkey::new_unique()); // Not owned by the loader.

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (programdata, programdata_account),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_programdata_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    let mut instruction = extend_program(
        &programdata,
        &program,
        additional_bytes as u32,
        Some(&payer),
    );
    instruction.accounts[0].is_writable = false; // Not writable.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_program_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    let mut instruction = extend_program(
        &programdata,
        &program,
        additional_bytes as u32,
        Some(&payer),
    );
    instruction.accounts[1].is_writable = false; // Not writable.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_program_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    let mut program_account = upgradeable_state_account(
        &UpgradeableLoaderState::Program {
            programdata_address: programdata,
        },
        &[],
        true,
    );
    program_account.set_owner(Pubkey::new_unique()); // Not owned by the loader.

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (program, program_account),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_invalid_program_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                // Not the correct state.
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidAccountData)],
    );
}

#[test]
fn fail_program_programdata_mismatch() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: Pubkey::new_unique(), // Mismatch.
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_max_data_len_exceeded() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = elf.len().saturating_add(MAX_PERMITTED_DATA_LENGTH as usize);

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidRealloc)],
    );
}

#[test]
fn fail_invalid_programdata_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                // Not the correct state.
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidAccountData)],
    );
}

#[test]
fn fail_program_extended_in_slot() {
    let mollusk = setup();
    // No slot warp...

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_program_not_upgradeable() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let program = Pubkey::new_unique();
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let additional_bytes: usize = 1_500;

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: None, // Not upgradeable.
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(
                    mollusk.sysvars.rent.minimum_balance(additional_bytes),
                ),
            ),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn success() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let program = Pubkey::new_from_array([1; 32]); // Consistent CUs when logging.
    let payer = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];

    let original_size =
        UpgradeableLoaderState::size_of_programdata_metadata().saturating_add(elf.len());
    let original_rent_exemption = mollusk.sysvars.rent.minimum_balance(original_size);

    let additional_bytes = MAX_PERMITTED_DATA_INCREASE;
    let new_size = original_size.saturating_add(additional_bytes);
    let new_rent_exemption = mollusk.sysvars.rent.minimum_balance(new_size);

    let rent_for_additional_bytes = new_rent_exemption.saturating_sub(original_rent_exemption);

    mollusk.process_and_validate_instruction(
        &extend_program(
            &programdata,
            &program,
            additional_bytes as u32,
            Some(&payer),
        ),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    &[],
                    true,
                ),
            ),
            keyed_account_for_system_program(),
            (
                payer,
                system_account_with_lamports(rent_for_additional_bytes),
            ),
        ],
        &[
            Check::success(),
            Check::compute_units(7_005),
            Check::account(&programdata)
                .lamports(new_rent_exemption)
                .space(new_size)
                .build(),
            Check::account(&payer).lamports(0).build(),
        ],
    );
}
