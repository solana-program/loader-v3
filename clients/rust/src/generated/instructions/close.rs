//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct Close {
    /// Buffer or ProgramData account to close.
    pub buffer_or_program_data_account: solana_program::pubkey::Pubkey,
    /// Destination account for reclaimed lamports.
    pub destination_account: solana_program::pubkey::Pubkey,
    /// Authority (optional).
    pub authority: Option<solana_program::pubkey::Pubkey>,
    /// Program account (optional).
    pub program_account: Option<solana_program::pubkey::Pubkey>,
}

impl Close {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.buffer_or_program_data_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.destination_account,
            false,
        ));
        if let Some(authority) = self.authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                authority, true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LOADER_V3_ID,
                false,
            ));
        }
        if let Some(program_account) = self.program_account {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                program_account,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LOADER_V3_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let data = CloseInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::LOADER_V3_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CloseInstructionData {
    discriminator: u32,
}

impl CloseInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 5 }
    }
}

impl Default for CloseInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `Close`.
///
/// ### Accounts:
///
///   0. `[writable]` buffer_or_program_data_account
///   1. `[writable]` destination_account
///   2. `[signer, optional]` authority
///   3. `[optional]` program_account
#[derive(Clone, Debug, Default)]
pub struct CloseBuilder {
    buffer_or_program_data_account: Option<solana_program::pubkey::Pubkey>,
    destination_account: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    program_account: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CloseBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Buffer or ProgramData account to close.
    #[inline(always)]
    pub fn buffer_or_program_data_account(
        &mut self,
        buffer_or_program_data_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.buffer_or_program_data_account = Some(buffer_or_program_data_account);
        self
    }
    /// Destination account for reclaimed lamports.
    #[inline(always)]
    pub fn destination_account(
        &mut self,
        destination_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.destination_account = Some(destination_account);
        self
    }
    /// `[optional account]`
    /// Authority (optional).
    #[inline(always)]
    pub fn authority(&mut self, authority: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.authority = authority;
        self
    }
    /// `[optional account]`
    /// Program account (optional).
    #[inline(always)]
    pub fn program_account(
        &mut self,
        program_account: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.program_account = program_account;
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
        let accounts = Close {
            buffer_or_program_data_account: self
                .buffer_or_program_data_account
                .expect("buffer_or_program_data_account is not set"),
            destination_account: self
                .destination_account
                .expect("destination_account is not set"),
            authority: self.authority,
            program_account: self.program_account,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `close` CPI accounts.
pub struct CloseCpiAccounts<'a, 'b> {
    /// Buffer or ProgramData account to close.
    pub buffer_or_program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination account for reclaimed lamports.
    pub destination_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority (optional).
    pub authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Program account (optional).
    pub program_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `close` CPI instruction.
pub struct CloseCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer or ProgramData account to close.
    pub buffer_or_program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination account for reclaimed lamports.
    pub destination_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority (optional).
    pub authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Program account (optional).
    pub program_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

impl<'a, 'b> CloseCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CloseCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            buffer_or_program_data_account: accounts.buffer_or_program_data_account,
            destination_account: accounts.destination_account,
            authority: accounts.authority,
            program_account: accounts.program_account,
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
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.buffer_or_program_data_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.destination_account.key,
            false,
        ));
        if let Some(authority) = self.authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *authority.key,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LOADER_V3_ID,
                false,
            ));
        }
        if let Some(program_account) = self.program_account {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *program_account.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LOADER_V3_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = CloseInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LOADER_V3_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.buffer_or_program_data_account.clone());
        account_infos.push(self.destination_account.clone());
        if let Some(authority) = self.authority {
            account_infos.push(authority.clone());
        }
        if let Some(program_account) = self.program_account {
            account_infos.push(program_account.clone());
        }
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

/// Instruction builder for `Close` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` buffer_or_program_data_account
///   1. `[writable]` destination_account
///   2. `[signer, optional]` authority
///   3. `[optional]` program_account
#[derive(Clone, Debug)]
pub struct CloseCpiBuilder<'a, 'b> {
    instruction: Box<CloseCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CloseCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CloseCpiBuilderInstruction {
            __program: program,
            buffer_or_program_data_account: None,
            destination_account: None,
            authority: None,
            program_account: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Buffer or ProgramData account to close.
    #[inline(always)]
    pub fn buffer_or_program_data_account(
        &mut self,
        buffer_or_program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.buffer_or_program_data_account = Some(buffer_or_program_data_account);
        self
    }
    /// Destination account for reclaimed lamports.
    #[inline(always)]
    pub fn destination_account(
        &mut self,
        destination_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.destination_account = Some(destination_account);
        self
    }
    /// `[optional account]`
    /// Authority (optional).
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.authority = authority;
        self
    }
    /// `[optional account]`
    /// Program account (optional).
    #[inline(always)]
    pub fn program_account(
        &mut self,
        program_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.program_account = program_account;
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
        let instruction = CloseCpi {
            __program: self.instruction.__program,

            buffer_or_program_data_account: self
                .instruction
                .buffer_or_program_data_account
                .expect("buffer_or_program_data_account is not set"),

            destination_account: self
                .instruction
                .destination_account
                .expect("destination_account is not set"),

            authority: self.instruction.authority,

            program_account: self.instruction.program_account,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CloseCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    buffer_or_program_data_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
