#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{setup, upgradeable_state_account},
    mollusk_svm::result::Check,
    solana_loader_v3_program::{
        instruction::close,
        state::{get_program_data_address, UpgradeableLoaderState},
    },
    solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[test]
fn fail_recipient_same_as_close_account() {
    let mollusk = setup();

    let target = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &close(&target, &target, None, None),
        &[
            (target, AccountSharedData::default()),
            (target, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_invalid_account_state() {
    let mollusk = setup();

    let target = Pubkey::new_unique();
    let destination = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &close(&target, &destination, None, None),
        &[
            (
                target,
                // Invalid account state (can't be closed individually).
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: Pubkey::new_unique(),
                    },
                    &[],
                    true,
                ),
            ),
            (destination, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn uninitialized_success() {
    let mollusk = setup();

    let target = Pubkey::new_unique();
    let destination = Pubkey::new_unique();

    let reclaimed_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(UpgradeableLoaderState::size_of_uninitialized());

    mollusk.process_and_validate_instruction(
        &close(&target, &destination, None, None),
        &[
            (
                target,
                upgradeable_state_account(&UpgradeableLoaderState::Uninitialized, &[], false),
            ),
            (destination, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            // Closed, but still owned by the loader.
            Check::account(&target)
                .data(&[0, 0, 0, 0]) // Size of Uninitialized.
                .lamports(0)
                .owner(&solana_loader_v3_program::id())
                .build(),
            Check::account(&destination)
                .lamports(reclaimed_lamports)
                .build(),
        ],
    );
}

#[test]
fn buffer_fail_buffer_immutable() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &close(&buffer, &destination, Some(&authority), None),
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: None, // Buffer is immutable.
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn buffer_fail_incorrect_authority() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &close(&buffer, &destination, Some(&authority), None),
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(Pubkey::new_unique()), // Incorrect authority.
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn buffer_fail_authority_not_signer() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let mut instruction = close(&buffer, &destination, Some(&authority), None);
    instruction.accounts[2].is_signer = false;

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn buffer_success() {
    let mollusk = setup();

    let buffer = Pubkey::new_unique();
    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let elf = &[3; 5_000];
    let reclaimed_lamports = mollusk.sysvars.rent.minimum_balance(
        UpgradeableLoaderState::size_of_buffer_metadata().saturating_add(elf.len()),
    );

    mollusk.process_and_validate_instruction(
        &close(&buffer, &destination, Some(&authority), None),
        &[
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
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            // Closed, but still owned by the loader.
            Check::account(&buffer)
                .data(&[0, 0, 0, 0]) // Size of Uninitialized.
                .lamports(0)
                .owner(&solana_loader_v3_program::id())
                .build(),
            Check::account(&destination)
                .lamports(reclaimed_lamports)
                .build(),
        ],
    );
}

#[test]
fn programdata_fail_program_not_writable() {
    let mollusk = setup();

    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let mut instruction = close(&programdata, &destination, Some(&authority), Some(&program));
    instruction.accounts[0].is_writable = false;

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
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
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn programdata_fail_program_not_owned_by_loader() {
    let mollusk = setup();

    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let mut program_account = upgradeable_state_account(
        &UpgradeableLoaderState::Program {
            programdata_address: programdata,
        },
        &[],
        true,
    );
    program_account.set_owner(Pubkey::new_unique()); // Not owned by the loader.

    mollusk.process_and_validate_instruction(
        &close(&programdata, &destination, Some(&authority), Some(&program)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
            (program, program_account),
        ],
        &[Check::err(ProgramError::IncorrectProgramId)],
    );
}

#[test]
fn programdata_fail_program_deployed_in_slot() {
    let mollusk = setup();
    // No slot warp...

    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    mollusk.process_and_validate_instruction(
        &close(&programdata, &destination, Some(&authority), Some(&program)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
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
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn programdata_fail_not_upgradeable() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    mollusk.process_and_validate_instruction(
        &close(&programdata, &destination, Some(&authority), Some(&program)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: None, // Not upgradeable.
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
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
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn programdata_fail_incorrect_authority() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    mollusk.process_and_validate_instruction(
        &close(&programdata, &destination, Some(&authority), Some(&program)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(Pubkey::new_unique()), // Incorrect authority.
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
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
        ],
        &[
            Check::err(ProgramError::IncorrectAuthority),
        ],
    );
}

#[test]
fn programdata_fail_authority_not_signer() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let mut instruction = close(&programdata, &destination, Some(&authority), Some(&program));
    instruction.accounts[2].is_signer = false;

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    &[3; 5_000],
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
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
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn programdata_success() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let destination = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let elf = &[3; 5_000];
    let reclaimed_lamports = mollusk.sysvars.rent.minimum_balance(
        UpgradeableLoaderState::size_of_programdata_metadata().saturating_add(elf.len()),
    );

    mollusk.process_and_validate_instruction(
        &close(&programdata, &destination, Some(&authority), Some(&program)),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    elf,
                    false,
                ),
            ),
            (destination, AccountSharedData::default()),
            (authority, AccountSharedData::default()),
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
        ],
        &[
            Check::success(),
            // Closed, but still owned by the loader.
            Check::account(&programdata)
                .data(&[0, 0, 0, 0]) // Size of Uninitialized.
                .lamports(0)
                .owner(&solana_loader_v3_program::id())
                .build(),
            Check::account(&destination)
                .lamports(reclaimed_lamports)
                .build(),
        ],
    );
}
