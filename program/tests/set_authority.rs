#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{setup, upgradeable_state_account},
    mollusk_svm::result::Check,
    solana_loader_v3_program::{instruction::set_authority, state::UpgradeableLoaderState},
    solana_sdk::{account::AccountSharedData, program_error::ProgramError, pubkey::Pubkey},
};

#[test]
fn fail_invalid_account_state() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    mollusk.process_and_validate_instruction(
        &set_authority(&buffer, &current_authority, Some(&new_authority)),
        &[
            (
                buffer,
                // Invalid account state (no authority).
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: Pubkey::new_unique(),
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn buffer_fail_authority_not_provided() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    let mut instruction = set_authority(&buffer, &current_authority, Some(&new_authority));
    instruction.accounts.pop(); // Authority not provided.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(current_authority),
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn buffer_fail_buffer_immutable() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    mollusk.process_and_validate_instruction(
        &set_authority(&buffer, &current_authority, Some(&new_authority)),
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: None, // Immutable.
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn buffer_fail_incorrect_authority() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    mollusk.process_and_validate_instruction(
        &set_authority(&buffer, &current_authority, Some(&new_authority)),
        &[
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
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn buffer_fail_authority_not_signer() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    let mut instruction = set_authority(&buffer, &current_authority, Some(&new_authority));
    instruction.accounts[1].is_signer = false; // Not a signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(current_authority),
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn buffer_success() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    let check_data = |authority_address: Option<Pubkey>| {
        let mut data = vec![0; UpgradeableLoaderState::size_of_buffer_metadata()];
        bincode::serialize_into(
            &mut data[..],
            &UpgradeableLoaderState::Buffer { authority_address },
        )
        .unwrap();
        data.extend_from_slice(elf);
        data
    };

    mollusk.process_and_validate_instruction(
        &set_authority(&buffer, &current_authority, Some(&new_authority)),
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(current_authority),
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&buffer)
                .data(
                    &check_data(Some(new_authority)), // Updated.
                )
                .build(),
        ],
    );

    // Can't set to `None`, since buffer authority is not optional.
}

#[test]
fn programdata_fail_not_upgradeable() {
    let mollusk = setup();

    let programdata = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    mollusk.process_and_validate_instruction(
        &set_authority(&programdata, &current_authority, Some(&new_authority)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: None, // Immutable.
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn programdata_fail_incorrect_authority() {
    let mollusk = setup();

    let programdata = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    mollusk.process_and_validate_instruction(
        &set_authority(&programdata, &current_authority, Some(&new_authority)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: Some(Pubkey::new_unique()), // Incorrect authority.
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn programdata_fail_authority_not_signer() {
    let mollusk = setup();

    let programdata = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    let mut instruction = set_authority(&programdata, &current_authority, Some(&new_authority));
    instruction.accounts[1].is_signer = false; // Not a signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: Some(current_authority),
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn programdata_success() {
    let mollusk = setup();

    let programdata = Pubkey::new_unique();
    let current_authority = Pubkey::new_unique();
    let new_authority = Pubkey::new_unique();

    let elf = &[3; 5_000];

    let check_data = |upgrade_authority_address: Option<Pubkey>| {
        let mut data = vec![0; UpgradeableLoaderState::size_of_programdata_metadata()];
        bincode::serialize_into(
            &mut data[..],
            &UpgradeableLoaderState::ProgramData {
                slot: 0,
                upgrade_authority_address,
            },
        )
        .unwrap();
        data.extend_from_slice(elf);
        data
    };

    mollusk.process_and_validate_instruction(
        &set_authority(&programdata, &current_authority, Some(&new_authority)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(current_authority),
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&programdata)
                .data(
                    &check_data(Some(new_authority)), // Updated.
                )
                .build(),
        ],
    );

    // Now set to `None`.
    mollusk.process_and_validate_instruction(
        &set_authority(&programdata, &current_authority, None),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(current_authority),
                    },
                    elf,
                    false,
                ),
            ),
            (current_authority, AccountSharedData::default()),
            (new_authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&programdata)
                .data(
                    &check_data(None), // Updated.
                )
                .build(),
        ],
    );
}
