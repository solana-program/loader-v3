#![allow(dead_code)]
#![cfg(feature = "test-sbf")]

#[allow(deprecated)]
use solana_sdk::system_program;
use {
    mollusk_svm::Mollusk,
    solana_loader_v3_program::state::UpgradeableLoaderState,
    solana_sdk::{account::Account, rent::Rent, system_program},
};

pub fn setup() -> Mollusk {
    Mollusk::new(&solana_loader_v3_program::id(), "solana_loader_v3_program")
}

pub fn system_account_with_lamports(lamports: u64) -> Account {
    Account::new(lamports, 0, &system_program::id())
}

pub fn upgradeable_state_account(
    state: &UpgradeableLoaderState,
    additional_bytes: &[u8],
    executable: bool,
) -> Account {
    // Annoying, but necessary because of the program's layout expectations.
    let data_size = match state {
        UpgradeableLoaderState::Uninitialized => UpgradeableLoaderState::size_of_uninitialized(),
        UpgradeableLoaderState::Buffer { .. } => UpgradeableLoaderState::size_of_buffer_metadata(),
        UpgradeableLoaderState::Program { .. } => UpgradeableLoaderState::size_of_program(),
        UpgradeableLoaderState::ProgramData { .. } => {
            UpgradeableLoaderState::size_of_programdata_metadata()
        }
    };

    let mut data = vec![0; data_size];
    bincode::serialize_into(&mut data[..], state).unwrap();
    data.extend_from_slice(additional_bytes);

    let space = data.len();
    let lamports = Rent::default().minimum_balance(space);

    Account {
        lamports,
        owner: solana_loader_v3_program::id(),
        data,
        executable,
        ..Account::default()
    }
}
