//! Program state types.

use {
    serde::{Deserialize, Serialize},
    solana_program::{program_error::ProgramError, pubkey::Pubkey},
};

/// Returns the program data address for a program ID
pub fn get_program_data_address(program_address: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[program_address.as_ref()], &crate::id()).0
}

/// Upgradeable loader account states.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UpgradeableLoaderState {
    /// Account is not initialized.
    Uninitialized,

    /// A Buffer account.
    Buffer {
        /// Authority address
        authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },

    /// A Program account.
    Program {
        /// Address of the ProgramData account.
        programdata_address: Pubkey,
    },

    // A ProgramData account.
    ProgramData {
        /// Slot that the program was last modified.
        slot: u64,
        /// Address of the Program's upgrade authority.
        upgrade_authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },
}

impl UpgradeableLoaderState {
    /// Size of a serialized program account.
    pub const fn size_of_uninitialized() -> usize {
        4 // see test_state_size_of_uninitialized
    }

    /// Size of a buffer account's serialized metadata.
    pub const fn size_of_buffer_metadata() -> usize {
        37 // see test_state_size_of_buffer_metadata
    }

    /// Size of a programdata account's serialized metadata.
    pub const fn size_of_programdata_metadata() -> usize {
        45 // see test_state_size_of_programdata_metadata
    }

    /// Size of a serialized program account.
    pub const fn size_of_program() -> usize {
        36 // see test_state_size_of_program
    }

    /// Size of a serialized buffer account.
    pub const fn size_of_buffer(program_len: usize) -> usize {
        Self::size_of_buffer_metadata().saturating_add(program_len)
    }

    /// Size of a serialized programdata account.
    pub const fn size_of_programdata(program_len: usize) -> usize {
        Self::size_of_programdata_metadata().saturating_add(program_len)
    }

    /// Deserialize an `UpgradeableLoaderState` from a buffer.
    /// Wraps `bincode::deserialize` to return a `ProgramError`.
    pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
        bincode::deserialize(input).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, bincode::serialized_size};

    #[test]
    fn test_state_size_of_uninitialized() {
        let buffer_state = UpgradeableLoaderState::Uninitialized;
        let size = serialized_size(&buffer_state).unwrap();
        assert_eq!(UpgradeableLoaderState::size_of_uninitialized() as u64, size);
    }

    #[test]
    fn test_state_size_of_buffer_metadata() {
        let buffer_state = UpgradeableLoaderState::Buffer {
            authority_address: Some(Pubkey::default()),
        };
        let size = serialized_size(&buffer_state).unwrap();
        assert_eq!(
            UpgradeableLoaderState::size_of_buffer_metadata() as u64,
            size
        );
    }

    #[test]
    fn test_state_size_of_programdata_metadata() {
        let programdata_state = UpgradeableLoaderState::ProgramData {
            upgrade_authority_address: Some(Pubkey::default()),
            slot: 0,
        };
        let size = serialized_size(&programdata_state).unwrap();
        assert_eq!(
            UpgradeableLoaderState::size_of_programdata_metadata() as u64,
            size
        );
    }

    #[test]
    fn test_state_size_of_program() {
        let program_state = UpgradeableLoaderState::Program {
            programdata_address: Pubkey::default(),
        };
        let size = serialized_size(&program_state).unwrap();
        assert_eq!(UpgradeableLoaderState::size_of_program() as u64, size);
    }
}
