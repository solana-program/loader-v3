#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{setup, system_account_with_lamports, upgradeable_state_account},
    mollusk_svm::{program::keyed_account_for_system_program, result::Check},
    solana_loader_v3_program::{
        instruction::deploy_with_max_data_len,
        state::{get_program_data_address, UpgradeableLoaderState},
    },
    solana_sdk::{
        account::{Account, ReadableAccount},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction::MAX_PERMITTED_DATA_LENGTH,
    },
};

#[test]
fn fail_program_already_initialized() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                // Already initialized.
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    },
                    elf,
                    true,
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::AccountAlreadyInitialized)],
    );
}

#[test]
fn fail_program_account_too_small() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    100_000_000,
                    UpgradeableLoaderState::size_of_program().saturating_sub(1), // Too small.
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::AccountDataTooSmall)],
    );
}

#[test]
fn fail_program_account_not_rent_exempt() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports.saturating_sub(1), // Not rent exempt.
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::InsufficientFunds)],
    );
}

#[test]
fn fail_invalid_buffer_state() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                // Not the correct state.
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_incorrect_buffer_authority() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(Pubkey::new_unique()), // Incorrect authority.
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_buffer_authority_not_signer() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    let mut instruction = deploy_with_max_data_len(
        &payer,
        &programdata,
        &program,
        &buffer,
        &authority,
        max_data_len,
    );
    instruction.accounts[7].is_signer = false;

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn fail_buffer_account_too_small() {
    // Error is unreachable, since the program checks for the authority, which
    // must be `Some`.
}

#[test]
fn fail_max_data_len_too_small() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 15_000]; // ELF, _greater_ than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::AccountDataTooSmall)],
    );
}

#[test]
fn fail_max_data_len_too_large() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = MAX_PERMITTED_DATA_LENGTH.saturating_add(1) as usize; // Too large.

    let elf = &[7; 5_000];

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_incorrect_programdata_address() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = Pubkey::new_unique(); // Incorrect programdata address.
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_programdata_already_initialized() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (
                programdata,
                // Already initialized.
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::Custom(
            0, // SystemError::AccountAlreadyInUse
        ))],
    );
}

#[test]
fn success() {
    let mollusk = setup();

    let payer = Pubkey::new_unique();
    let program = Pubkey::new_from_array([1; 32]); // Consistent CUs when logging.
    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);
    let max_data_len = 10_000;

    let elf = &[7; 5_000]; // ELF, less than max data len.

    let program_rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_program());

    let result = mollusk.process_and_validate_instruction(
        &deploy_with_max_data_len(
            &payer,
            &programdata,
            &program,
            &buffer,
            &authority,
            max_data_len,
        ),
        &[
            (payer, system_account_with_lamports(100_000_000_000)),
            (programdata, Account::default()),
            (
                program,
                Account::new(
                    program_rent_exempt_lamports,
                    UpgradeableLoaderState::size_of_program(),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            keyed_account_for_system_program(),
            (authority, Account::default()),
        ],
        &[
            Check::success(),
            Check::compute_units(12_052),
            Check::account(&program)
                .data(
                    &bincode::serialize(&UpgradeableLoaderState::Program {
                        programdata_address: programdata,
                    })
                    .unwrap(),
                )
                .build(),
        ],
    );

    let programdata_account = result.get_account(&programdata).unwrap();
    let mut check_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
        slot: mollusk.sysvars.clock.slot,
        upgrade_authority_address: Some(authority),
    })
    .unwrap();
    check_data.extend_from_slice(elf);
    check_data.extend_from_slice(&[0; 5_000]); // Rest of max data length.
    assert_eq!(programdata_account.data(), &check_data)
}
