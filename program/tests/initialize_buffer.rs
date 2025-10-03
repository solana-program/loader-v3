#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{setup, upgradeable_state_account},
    mollusk_svm::result::Check,
    solana_loader_v3_program::{instruction::initialize_buffer, state::UpgradeableLoaderState},
    solana_sdk::{
        account::Account, instruction::InstructionError, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[test]
fn fail_buffer_already_initialized() {
    let mollusk = setup();

    let source = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &initialize_buffer(&source, &authority),
        &[
            (
                source,
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
        &[Check::err(ProgramError::AccountAlreadyInitialized)],
    );
}

#[test]
fn fail_buffer_account_too_small() {
    let mollusk = setup();

    let source = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &initialize_buffer(&source, &authority),
        &[
            (
                source,
                Account::new(
                    100_000_000,
                    0, // Too small.
                    &solana_loader_v3_program::id(),
                ),
            ),
            (authority, Account::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountData)],
    );
}

// #[test]
#[allow(dead_code)]
fn fail_buffer_account_not_owned_by_loader() {
    let mollusk = setup();

    let source = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &initialize_buffer(&source, &authority),
        &[
            (
                source,
                Account::new(
                    100_000_000,
                    UpgradeableLoaderState::size_of_buffer(0),
                    &Pubkey::new_unique(), // Not the loader.
                ),
            ),
            (authority, Account::default()),
        ],
        &[Check::instruction_err(
            InstructionError::ExternalAccountDataModified,
        )],
    );
}

#[test]
fn success() {
    let mollusk = setup();

    let source = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &initialize_buffer(&source, &authority),
        &[
            (
                source,
                Account::new(
                    100_000_000,
                    UpgradeableLoaderState::size_of_buffer(0),
                    &solana_loader_v3_program::id(),
                ),
            ),
            (authority, Account::default()),
        ],
        &[
            Check::success(),
            Check::compute_units(1_763),
            Check::account(&source)
                .lamports(100_000_000)
                .owner(&solana_loader_v3_program::id())
                .data(
                    &bincode::serialize(&UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    })
                    .unwrap(),
                )
                .build(),
        ],
    );
}
