//! Program instruction types.

use {
    serde::{Deserialize, Serialize},
    shank::ShankInstruction,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};

/// Instructions supported by the Solana BPF Loader v3 program.
#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ShankInstruction)]
pub enum LoaderV3Instruction {
    /// Initialize a Buffer account.
    ///
    /// A Buffer account is an intermediary that once fully populated is used
    /// with the `DeployWithMaxDataLen` instruction to populate the program's
    /// ProgramData account.
    ///
    /// The `InitializeBuffer` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party may initialize the account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Source account to initialize.
    /// 1. `[ ]` Buffer authority.
    #[account(
        0,
        writable,
        name = "source_account",
        desc = "Source account to initialize."
    )]
    #[account(
        1,
        name = "buffer_authority",
        desc = "Buffer authority."
    )]
    InitializeBuffer,

    /// Write program data into a Buffer account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Buffer account.
    /// 1. `[s]` Buffer authority.
    #[account(
        0,
        writable,
        name = "buffer_account",
        desc = "Buffer account."
    )]
    #[account(
        1,
        signer,
        name = "buffer_authority",
        desc = "Buffer authority."
    )]
    Write {
        /// Offset at which to write the given bytes.
        offset: u32,
        /// Serialized program data.
        bytes: Vec<u8>,
    },

    /// Deploy an executable program.
    ///
    /// A program consists of a Program and ProgramData account pair.
    ///   - The Program account's address will serve as the program id for any
    ///     instructions that execute this program.
    ///   - The ProgramData account will remain mutable by the loader only and
    ///     holds the program data and authority information.  The ProgramData
    ///     account's address is derived from the Program account's address and
    ///     created by the DeployWithMaxDataLen instruction.
    ///
    /// The ProgramData address is derived from the Program account's address
    /// as follows:
    ///
    /// ```
    /// # use solana_program::pubkey::Pubkey;
    /// # use solana_program::bpf_loader_upgradeable;
    /// # let program_address = &[];
    /// let (program_data_address, _) = Pubkey::find_program_address(
    ///      &[program_address],
    ///      &bpf_loader_upgradeable::id()
    ///  );
    /// ```
    ///
    /// The `DeployWithMaxDataLen` instruction does not require the ProgramData
    /// account be a signer and therefore MUST be included within the same
    /// Transaction as the system program's `CreateAccount` instruction that
    /// creates the Program account. Otherwise another party may initialize the
    /// account.
    ///
    /// Note: The buffer authority must match the program's authority.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w, s]` Payer account that will pay to create the ProgramData
    ///    account.
    /// 1. `[w]` ProgramData account (uninitialized).
    /// 2. `[w]` Program account (uninitialized).
    /// 3. `[w]` Buffer account where the program data has been written.
    /// 4. `[ ]` Rent sysvar.
    /// 5. `[ ]` Clock sysvar.
    /// 6. `[ ]` System program.
    /// 7. `[s]` Authority.
    #[account(
        0,
        writable,
        signer,
        name = "payer_account",
        desc = "Payer account that will pay to create the ProgramData account."
    )]
    #[account(
        1,
        writable,
        name = "program_data_account",
        desc = "ProgramData account (uninitialized)."
    )]
    #[account(
        2,
        writable,
        name = "program_account",
        desc = "Program account (uninitialized)."
    )]
    #[account(
        3,
        writable,
        name = "buffer_account",
        desc = "Buffer account where the program data has been written."
    )]
    #[account(
        4,
        name = "rent_sysvar",
        desc = "Rent sysvar."
    )]
    #[account(
        5,
        name = "clock_sysvar",
        desc = "Clock sysvar."
    )]
    #[account(
        6,
        name = "system_program",
        desc = "System program."
    )]
    #[account(
        7,
        signer,
        name = "authority",
        desc = "Authority."
    )]
    DeployWithMaxDataLen {
        /// Maximum length that the program can be upgraded to.
        max_data_len: usize,
    },

    /// Upgrade a program.
    ///
    /// A program can be upgraded as long as the program's authority has not
    /// been set to `None`.
    ///
    /// The Buffer account must contain sufficient lamports to fund the
    /// ProgramData account to be rent-exempt, any additional lamports left
    /// over will be transferred to the spill account, leaving the Buffer
    /// account balance at zero.
    ///
    /// Note: The buffer authority must match the program's authority.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` ProgramData account.
    /// 1. `[w]` Program account.
    /// 2. `[w]` Buffer account where the new program data has been written.
    /// 3. `[w]` Spill account.
    /// 4. `[ ]` Rent sysvar.
    /// 5. `[ ]` Clock sysvar.
    /// 6. `[s]` Authority.
    #[account(
        0,
        writable,
        name = "program_data_account",
        desc = "ProgramData account."
    )]
    #[account(
        1,
        writable,
        name = "program_account",
        desc = "Program account."
    )]
    #[account(
        2,
        writable,
        name = "buffer_account",
        desc = "Buffer account where the new program data has been written."
    )]
    #[account(
        3,
        writable,
        name = "spill_account",
        desc = "Spill account."
    )]
    #[account(
        4,
        name = "rent_sysvar",
        desc = "Rent sysvar."
    )]
    #[account(
        5,
        name = "clock_sysvar",
        desc = "Clock sysvar."
    )]
    #[account(
        6,
        signer,
        name = "authority",
        desc = "Authority."
    )]
    Upgrade,

    /// Set a new authority that is allowed to write the buffer or upgrade the
    /// program. To permanently make the buffer immutable or disable program
    /// updates, omit the new authority.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Buffer or ProgramData account.
    /// 1. `[s]` Current authority.
    /// 2. `[ ]` New authority (optional).
    #[account(
        0,
        writable,
        name = "buffer_or_program_data_account",
        desc = "Buffer or ProgramData account."
    )]
    #[account(
        1,
        signer,
        name = "current_authority",
        desc = "Current authority."
    )]
    #[account(
        2,
        optional,
        name = "new_authority",
        desc = "New authority (optional)."
    )]
    SetAuthority,

    /// Closes an account owned by the upgradeable loader of all lamports and
    /// withdraws all the lamports.
    ///
    /// Note: The authority is only required to close an initialized account.
    /// Note: The program account is required to close an initialized
    /// ProgramData account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Buffer or ProgramData account to close.
    /// 1. `[w]` Destination account for reclaimed lamports.
    /// 2. `[s]` Authority (optional).
    /// 3. `[w]` Program account (optional).
    #[account(
        0,
        writable,
        name = "buffer_or_program_data_account",
        desc = "Buffer or ProgramData account to close."
    )]
    #[account(
        1,
        writable,
        name = "destination_account",
        desc = "Destination account for reclaimed lamports."
    )]
    #[account(
        2,
        optional,
        signer,
        name = "authority",
        desc = "Authority (optional)."
    )]
    #[account(
        3,
        optional,
        name = "program_account",
        desc = "Program account (optional)."
    )]
    Close,

    /// Extend a program's ProgramData account by the specified number of bytes.
    /// Only upgradeable programs can be extended.
    ///
    /// The payer account must contain sufficient lamports to fund the
    /// ProgramData account to be rent-exempt. If the ProgramData account
    /// balance is already sufficient to cover the rent exemption cost
    /// for the extended bytes, the payer account is not required.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` ProgramData account.
    /// 1. `[w]` Program account.
    /// 2. `[ ]` System program (optional).
    /// 3. `[w, s]` Payer (optional).
    #[account(
        0,
        writable,
        name = "program_data_account",
        desc = "ProgramData account."
    )]
    #[account(
        1,
        writable,
        name = "program_account",
        desc = "Program account."
    )]
    #[account(
        2,
        optional,
        name = "system_program",
        desc = "System program (optional)."
    )]
    #[account(
        3,
        optional,
        writable,
        signer,
        name = "payer",
        desc = "Payer."
    )]
    ExtendProgram {
        /// Number of bytes to extend the program data.
        additional_bytes: u32,
    },

    /// Set a new authority that is allowed to write the buffer or upgrade the
    /// program.
    ///
    /// This instruction differs from SetAuthority in that the new authority is
    /// a required signer.
    ///
    /// Accounts expected by this instruction:
    /// 0. `[w]` Buffer or ProgramData account to change the authority of.
    /// 1. `[s]` Current authority.
    /// 2. `[s]` New authority.
    #[account(
        0,
        writable,
        name = "buffer_or_program_data_account",
        desc = "Buffer or ProgramData account to change the authority of."
    )]
    #[account(
        1,
        signer,
        name = "current_authority",
        desc = "Current authority."
    )]
    #[account(
        2,
        signer,
        name = "new_authority",
        desc = "New authority."
    )]
    SetAuthorityChecked,
}

/// Creates an
/// [InitializeBuffer](enum.LoaderV3Instruction.html)
/// instruction.
pub fn initialize_buffer(source_address: &Pubkey, authority_address: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*source_address, false),
        AccountMeta::new(*authority_address, false),
    ];
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV3Instruction::InitializeBuffer,
        accounts,
    )
}

/// Creates a
/// [Write](enum.LoaderV3Instruction.html)
/// instruction.
pub fn write(
    buffer_address: &Pubkey,
    authority_address: &Pubkey,
    offset: u32,
    bytes: Vec<u8>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*buffer_address, false),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV3Instruction::Write { offset, bytes },
        accounts,
    )
}

/// Creates a
/// [DeployWithMaxDataLen](enum.LoaderV3Instruction.html)
/// instruction.
pub fn deploy_with_max_data_len(
    payer_address: &Pubkey,
    program_data_address: &Pubkey,
    program_address: &Pubkey,
    buffer_address: &Pubkey,
    authority_address: &Pubkey,
    max_data_len: usize,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*payer_address, true),
        AccountMeta::new(*program_data_address, false),
        AccountMeta::new(*program_address, false),
        AccountMeta::new(*buffer_address, false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::clock::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV3Instruction::DeployWithMaxDataLen { max_data_len },
        accounts,
    )
}

/// Creates an
/// [Upgrade](enum.LoaderV3Instruction.html)
/// instruction.
pub fn upgrade(
    program_data_address: &Pubkey,
    program_address: &Pubkey,
    buffer_address: &Pubkey,
    spill_address: &Pubkey,
    authority_address: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*program_data_address, false),
        AccountMeta::new(*program_address, false),
        AccountMeta::new(*buffer_address, false),
        AccountMeta::new(*spill_address, false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::clock::id(), false),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    Instruction::new_with_bincode(crate::id(), &LoaderV3Instruction::Upgrade, accounts)
}

/// Creates a
/// [SetAuthority](enum.LoaderV3Instruction.html)
/// instruction.
pub fn set_authority(
    buffer_or_program_data_address: &Pubkey,
    current_authority_address: &Pubkey,
    new_authority_address: Option<&Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*buffer_or_program_data_address, false),
        AccountMeta::new_readonly(*current_authority_address, true),
    ];
    if let Some(new_authority_address) = new_authority_address {
        accounts.push(AccountMeta::new_readonly(*new_authority_address, false));
    }
    Instruction::new_with_bincode(crate::id(), &LoaderV3Instruction::SetAuthority, accounts)
}

/// Creates a
/// [Close](enum.LoaderV3Instruction.html)
/// instruction.
pub fn close(
    buffer_or_program_data_address: &Pubkey,
    destination_address: &Pubkey,
    authority_address: Option<&Pubkey>,
    program_address: Option<&Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*buffer_or_program_data_address, false),
        AccountMeta::new(*destination_address, false),
    ];
    if let Some(authority_address) = authority_address {
        accounts.push(AccountMeta::new_readonly(*authority_address, true));
    }
    if let Some(program_address) = program_address {
        accounts.push(AccountMeta::new(*program_address, false));
    }
    Instruction::new_with_bincode(crate::id(), &LoaderV3Instruction::Close, accounts)
}

/// Creates an
/// [ExtendProgram](enum.LoaderV3Instruction.html)
/// instruction.
pub fn extend_program(
    program_data_address: &Pubkey,
    program_address: &Pubkey,
    additional_bytes: u32,
    payer_address: Option<&Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*program_data_address, false),
        AccountMeta::new(*program_address, false),
    ];
    if let Some(payer_address) = payer_address {
        accounts.push(AccountMeta::new_readonly(
            solana_program::system_program::id(),
            false,
        ));
        accounts.push(AccountMeta::new(*payer_address, true));
    }
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV3Instruction::ExtendProgram { additional_bytes },
        accounts,
    )
}

/// Creates a
/// [SetAuthorityChecked](enum.LoaderV3Instruction.html)
/// instruction.
pub fn set_authority_checked(
    buffer_or_program_data_address: &Pubkey,
    current_authority_address: &Pubkey,
    new_authority_address: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*buffer_or_program_data_address, false),
        AccountMeta::new_readonly(*current_authority_address, true),
        AccountMeta::new_readonly(*new_authority_address, true),
    ];
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV3Instruction::SetAuthorityChecked,
        accounts,
    )
}
