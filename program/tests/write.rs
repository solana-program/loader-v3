#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{setup, upgradeable_state_account},
    mollusk_svm::result::Check,
    solana_loader_v3_program::{instruction::write, state::UpgradeableLoaderState},
    solana_sdk::{
        account::{Account, WritableAccount},
        instruction::InstructionError,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[test]
fn fail_invalid_buffer_state() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &write(&buffer, &authority, 0, vec![4; 12]),
        &[
            (
                buffer,
                upgradeable_state_account(
                    // Not the correct state.
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: Some(Pubkey::new_unique()),
                    },
                    &[],
                    false,
                ),
            ),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountData)],
    );
}

#[test]
fn fail_immutable_buffer() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &write(&buffer, &authority, 0, vec![4; 12]),
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: None, // Immutable.
                    },
                    &[],
                    false,
                ),
            ),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn fail_incorrect_buffer_authority() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &write(&buffer, &authority, 0, vec![4; 12]),
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(Pubkey::new_unique()), // Incorrect authority.
                    },
                    &[],
                    false,
                ),
            ),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_buffer_authority_not_signer() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let mut instruction = write(&buffer, &authority, 0, vec![4; 12]);
    instruction.accounts[1].is_signer = false; // Authority not signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    &[],
                    false,
                ),
            ),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn fail_buffer_too_small() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &write(&buffer, &authority, 0, vec![4; 12]),
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    &[], // Too small.
                    false,
                ),
            ),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::AccountDataTooSmall)],
    );
}

// #[test]
#[allow(dead_code)]
fn fail_buffer_account_not_owned_by_loader() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = UpgradeableLoaderState::Buffer {
        authority_address: Some(authority),
    };
    let uninitialized_data = &[0; 36];

    let mut buffer_account = upgradeable_state_account(&state, uninitialized_data, false);
    buffer_account.set_owner(Pubkey::new_unique()); // Not owned by the loader.

    mollusk.process_and_validate_instruction(
        &write(&buffer, &authority, 0, vec![4; 12]),
        &[(buffer, buffer_account), (authority, Account::default())],
        &[Check::instruction_err(
            InstructionError::ExternalAccountDataModified,
        )],
    );
}

#[test]
fn success() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = UpgradeableLoaderState::Buffer {
        authority_address: Some(authority),
    };
    let uninitialized_data = &[0; 36];

    let mut check_data = bincode::serialize(&state).unwrap();
    check_data.extend_from_slice(uninitialized_data);

    // Write successfully.
    let offset = 0;
    let bytes = vec![4; 12];
    check_data[UpgradeableLoaderState::size_of_buffer_metadata()
        ..UpgradeableLoaderState::size_of_buffer_metadata().saturating_add(12)]
        .copy_from_slice(&bytes);

    let result = mollusk.process_and_validate_instruction(
        &write(&buffer, &authority, offset, bytes),
        &[
            (
                buffer,
                upgradeable_state_account(&state, uninitialized_data, false),
            ),
            (authority, Account::default()),
        ],
        &[
            Check::success(),
            Check::compute_units(1_854),
            Check::account(&buffer).data(&check_data).build(),
        ],
    );

    // Do it again.
    let offset = 12;
    let bytes = vec![8; 24];
    check_data[UpgradeableLoaderState::size_of_buffer_metadata().saturating_add(12)..]
        .copy_from_slice(&bytes);

    mollusk.process_and_validate_instruction(
        &write(&buffer, &authority, offset, bytes),
        &[
            (buffer, result.get_account(&buffer).unwrap().clone()),
            (authority, Account::default()),
        ],
        &[
            Check::success(),
            Check::compute_units(2_022),
            Check::account(&buffer).data(&check_data).build(),
        ],
    );
}
