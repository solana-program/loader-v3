//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct InitializeBuffer {
    /// Source account to initialize.
    pub source_account: solana_program::pubkey::Pubkey,
    /// Buffer authority.
    pub buffer_authority: solana_program::pubkey::Pubkey,
}

impl InitializeBuffer {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.source_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.buffer_authority,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = InitializeBufferInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::LOADER_V3_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeBufferInstructionData {
    discriminator: u32,
}

impl InitializeBufferInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 0 }
    }
}

impl Default for InitializeBufferInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `InitializeBuffer`.
///
/// ### Accounts:
///
///   0. `[writable]` source_account
///   1. `[]` buffer_authority
#[derive(Clone, Debug, Default)]
pub struct InitializeBufferBuilder {
    source_account: Option<solana_program::pubkey::Pubkey>,
    buffer_authority: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeBufferBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Source account to initialize.
    #[inline(always)]
    pub fn source_account(&mut self, source_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.source_account = Some(source_account);
        self
    }
    /// Buffer authority.
    #[inline(always)]
    pub fn buffer_authority(
        &mut self,
        buffer_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.buffer_authority = Some(buffer_authority);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = InitializeBuffer {
            source_account: self.source_account.expect("source_account is not set"),
            buffer_authority: self.buffer_authority.expect("buffer_authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `initialize_buffer` CPI accounts.
pub struct InitializeBufferCpiAccounts<'a, 'b> {
    /// Source account to initialize.
    pub source_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer authority.
    pub buffer_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_buffer` CPI instruction.
pub struct InitializeBufferCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Source account to initialize.
    pub source_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer authority.
    pub buffer_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitializeBufferCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeBufferCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            source_account: accounts.source_account,
            buffer_authority: accounts.buffer_authority,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.source_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.buffer_authority.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = InitializeBufferInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LOADER_V3_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.source_account.clone());
        account_infos.push(self.buffer_authority.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `InitializeBuffer` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` source_account
///   1. `[]` buffer_authority
#[derive(Clone, Debug)]
pub struct InitializeBufferCpiBuilder<'a, 'b> {
    instruction: Box<InitializeBufferCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeBufferCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeBufferCpiBuilderInstruction {
            __program: program,
            source_account: None,
            buffer_authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Source account to initialize.
    #[inline(always)]
    pub fn source_account(
        &mut self,
        source_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.source_account = Some(source_account);
        self
    }
    /// Buffer authority.
    #[inline(always)]
    pub fn buffer_authority(
        &mut self,
        buffer_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.buffer_authority = Some(buffer_authority);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool`
    /// indicating whether the account is writable or not, and a `bool`
    /// indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = InitializeBufferCpi {
            __program: self.instruction.__program,

            source_account: self
                .instruction
                .source_account
                .expect("source_account is not set"),

            buffer_authority: self
                .instruction
                .buffer_authority
                .expect("buffer_authority is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeBufferCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    source_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    buffer_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
