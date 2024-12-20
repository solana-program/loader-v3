//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct Upgrade {
    /// ProgramData account.
    pub program_data_account: solana_program::pubkey::Pubkey,
    /// Program account.
    pub program_account: solana_program::pubkey::Pubkey,
    /// Buffer account where the new program data has been written.
    pub buffer_account: solana_program::pubkey::Pubkey,
    /// Spill account.
    pub spill_account: solana_program::pubkey::Pubkey,
    /// Rent sysvar.
    pub rent_sysvar: solana_program::pubkey::Pubkey,
    /// Clock sysvar.
    pub clock_sysvar: solana_program::pubkey::Pubkey,
    /// Authority.
    pub authority: solana_program::pubkey::Pubkey,
}

impl Upgrade {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.program_data_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.program_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.buffer_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.spill_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent_sysvar,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.clock_sysvar,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = UpgradeInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::LOADER_V3_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UpgradeInstructionData {
    discriminator: u32,
}

impl UpgradeInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 3 }
    }
}

impl Default for UpgradeInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `Upgrade`.
///
/// ### Accounts:
///
///   0. `[writable]` program_data_account
///   1. `[writable]` program_account
///   2. `[writable]` buffer_account
///   3. `[writable]` spill_account
///   4. `[optional]` rent_sysvar (default to
///      `SysvarRent111111111111111111111111111111111`)
///   5. `[optional]` clock_sysvar (default to
///      `SysvarC1ock11111111111111111111111111111111`)
///   6. `[signer]` authority
#[derive(Clone, Debug, Default)]
pub struct UpgradeBuilder {
    program_data_account: Option<solana_program::pubkey::Pubkey>,
    program_account: Option<solana_program::pubkey::Pubkey>,
    buffer_account: Option<solana_program::pubkey::Pubkey>,
    spill_account: Option<solana_program::pubkey::Pubkey>,
    rent_sysvar: Option<solana_program::pubkey::Pubkey>,
    clock_sysvar: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpgradeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// ProgramData account.
    #[inline(always)]
    pub fn program_data_account(
        &mut self,
        program_data_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.program_data_account = Some(program_data_account);
        self
    }
    /// Program account.
    #[inline(always)]
    pub fn program_account(
        &mut self,
        program_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.program_account = Some(program_account);
        self
    }
    /// Buffer account where the new program data has been written.
    #[inline(always)]
    pub fn buffer_account(&mut self, buffer_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.buffer_account = Some(buffer_account);
        self
    }
    /// Spill account.
    #[inline(always)]
    pub fn spill_account(&mut self, spill_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.spill_account = Some(spill_account);
        self
    }
    /// `[optional account, default to
    /// 'SysvarRent111111111111111111111111111111111']` Rent sysvar.
    #[inline(always)]
    pub fn rent_sysvar(&mut self, rent_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent_sysvar = Some(rent_sysvar);
        self
    }
    /// `[optional account, default to
    /// 'SysvarC1ock11111111111111111111111111111111']` Clock sysvar.
    #[inline(always)]
    pub fn clock_sysvar(&mut self, clock_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.clock_sysvar = Some(clock_sysvar);
        self
    }
    /// Authority.
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
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
        let accounts = Upgrade {
            program_data_account: self
                .program_data_account
                .expect("program_data_account is not set"),
            program_account: self.program_account.expect("program_account is not set"),
            buffer_account: self.buffer_account.expect("buffer_account is not set"),
            spill_account: self.spill_account.expect("spill_account is not set"),
            rent_sysvar: self.rent_sysvar.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            clock_sysvar: self.clock_sysvar.unwrap_or(solana_program::pubkey!(
                "SysvarC1ock11111111111111111111111111111111"
            )),
            authority: self.authority.expect("authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `upgrade` CPI accounts.
pub struct UpgradeCpiAccounts<'a, 'b> {
    /// ProgramData account.
    pub program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program account.
    pub program_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer account where the new program data has been written.
    pub buffer_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Spill account.
    pub spill_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Rent sysvar.
    pub rent_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar.
    pub clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `upgrade` CPI instruction.
pub struct UpgradeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// ProgramData account.
    pub program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program account.
    pub program_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer account where the new program data has been written.
    pub buffer_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Spill account.
    pub spill_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Rent sysvar.
    pub rent_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar.
    pub clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> UpgradeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpgradeCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            program_data_account: accounts.program_data_account,
            program_account: accounts.program_account,
            buffer_account: accounts.buffer_account,
            spill_account: accounts.spill_account,
            rent_sysvar: accounts.rent_sysvar,
            clock_sysvar: accounts.clock_sysvar,
            authority: accounts.authority,
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
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.program_data_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.program_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.buffer_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.spill_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent_sysvar.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.clock_sysvar.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = UpgradeInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LOADER_V3_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.program_data_account.clone());
        account_infos.push(self.program_account.clone());
        account_infos.push(self.buffer_account.clone());
        account_infos.push(self.spill_account.clone());
        account_infos.push(self.rent_sysvar.clone());
        account_infos.push(self.clock_sysvar.clone());
        account_infos.push(self.authority.clone());
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

/// Instruction builder for `Upgrade` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` program_data_account
///   1. `[writable]` program_account
///   2. `[writable]` buffer_account
///   3. `[writable]` spill_account
///   4. `[]` rent_sysvar
///   5. `[]` clock_sysvar
///   6. `[signer]` authority
#[derive(Clone, Debug)]
pub struct UpgradeCpiBuilder<'a, 'b> {
    instruction: Box<UpgradeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpgradeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpgradeCpiBuilderInstruction {
            __program: program,
            program_data_account: None,
            program_account: None,
            buffer_account: None,
            spill_account: None,
            rent_sysvar: None,
            clock_sysvar: None,
            authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// ProgramData account.
    #[inline(always)]
    pub fn program_data_account(
        &mut self,
        program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program_data_account = Some(program_data_account);
        self
    }
    /// Program account.
    #[inline(always)]
    pub fn program_account(
        &mut self,
        program_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program_account = Some(program_account);
        self
    }
    /// Buffer account where the new program data has been written.
    #[inline(always)]
    pub fn buffer_account(
        &mut self,
        buffer_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.buffer_account = Some(buffer_account);
        self
    }
    /// Spill account.
    #[inline(always)]
    pub fn spill_account(
        &mut self,
        spill_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spill_account = Some(spill_account);
        self
    }
    /// Rent sysvar.
    #[inline(always)]
    pub fn rent_sysvar(
        &mut self,
        rent_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.rent_sysvar = Some(rent_sysvar);
        self
    }
    /// Clock sysvar.
    #[inline(always)]
    pub fn clock_sysvar(
        &mut self,
        clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.clock_sysvar = Some(clock_sysvar);
        self
    }
    /// Authority.
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
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
        let instruction = UpgradeCpi {
            __program: self.instruction.__program,

            program_data_account: self
                .instruction
                .program_data_account
                .expect("program_data_account is not set"),

            program_account: self
                .instruction
                .program_account
                .expect("program_account is not set"),

            buffer_account: self
                .instruction
                .buffer_account
                .expect("buffer_account is not set"),

            spill_account: self
                .instruction
                .spill_account
                .expect("spill_account is not set"),

            rent_sysvar: self
                .instruction
                .rent_sysvar
                .expect("rent_sysvar is not set"),

            clock_sysvar: self
                .instruction
                .clock_sysvar
                .expect("clock_sysvar is not set"),

            authority: self.instruction.authority.expect("authority is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpgradeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    program_data_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    buffer_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spill_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    clock_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
