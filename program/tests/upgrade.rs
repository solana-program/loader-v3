#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{setup, upgradeable_state_account},
    mollusk_svm::result::Check,
    solana_loader_v3_program::{
        instruction::upgrade,
        state::{get_program_data_address, UpgradeableLoaderState},
    },
    solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[test]
fn fail_program_not_executable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
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
                    false, // Not executable.
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountData)],
    );
}

#[test]
fn fail_program_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    let mut instruction = upgrade(&programdata, &program, &buffer, &spill, &authority);
    instruction.accounts[1].is_writable = false; // Program not writable.

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
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_program_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    let mut program_account = upgradeable_state_account(
        &UpgradeableLoaderState::Program {
            programdata_address: programdata,
        },
        &[],
        true,
    );
    program_account.set_owner(Pubkey::new_unique()); // Not owned by the loader.

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
                    false,
                ),
            ),
            (program, program_account),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectProgramId)],
    );
}

#[test]
fn fail_invalid_program_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
                    false,
                ),
            ),
            (
                program,
                // Not the correct state.
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: Some(authority),
                    },
                    &[],
                    true,
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountData)],
    );
}

#[test]
fn fail_incorrect_programdata_address() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
                    false,
                ),
            ),
            (
                program,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Program {
                        programdata_address: Pubkey::new_unique(), // Incorrect address.
                    },
                    &[],
                    true,
                ),
            ),
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_invalid_buffer_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
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
            (
                buffer,
                // Not the correct state.
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 2,
                        upgrade_authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_incorrect_buffer_authority() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(Pubkey::new_unique()), // Incorrect authority.
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_buffer_authority_not_signer() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    let mut instruction = upgrade(&programdata, &program, &buffer, &spill, &authority);
    instruction.accounts[6].is_signer = false; // Authority not signer.

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
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
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
fn fail_programdata_account_too_small() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 6_000]; // Larger.

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::AccountDataTooSmall)],
    );
}

#[test]
fn fail_buffer_not_enough_lamports() {
    // Error is unreachable, since the buffer has to have enough lamports to
    // be rent-exempt for the contained ELF.
}

#[test]
fn fail_invalid_programdata_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_020];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                // Not the correct state.
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountData)],
    );
}

#[test]
fn fail_program_deployed_in_slot() {
    let mollusk = setup();
    // No slot warp...

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_program_not_upgradeable() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: None, // Not upgradeable.
                    },
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn fail_incorrect_upgrade_authority() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(Pubkey::new_unique()), // Incorrect authority.
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_upgrade_authority_not_signer() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let program = Pubkey::new_unique();
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    let mut instruction = upgrade(&programdata, &program, &buffer, &spill, &authority);
    instruction.accounts[6].is_signer = false; // Authority not signer.

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
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn success() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(2); // To move past deployment slot.

    let program = Pubkey::new_from_array([1; 32]); // Consistent CUs when logging.
    let buffer = Pubkey::new_unique();
    let spill = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let programdata = get_program_data_address(&program);

    let original_elf = &[3; 5_000];
    let new_elf = &[7; 5_000];

    let result = mollusk.process_and_validate_instruction(
        &upgrade(&programdata, &program, &buffer, &spill, &authority),
        &[
            (
                programdata,
                upgradeable_state_account(
                    &UpgradeableLoaderState::ProgramData {
                        slot: 0,
                        upgrade_authority_address: Some(authority),
                    },
                    original_elf,
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
            (
                buffer,
                upgradeable_state_account(
                    &UpgradeableLoaderState::Buffer {
                        authority_address: Some(authority),
                    },
                    new_elf,
                    false,
                ),
            ),
            (spill, AccountSharedData::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::compute_units(16_663),
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
    check_data.extend_from_slice(new_elf);
    assert_eq!(programdata_account.data(), &check_data)
}
