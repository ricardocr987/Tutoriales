use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        _vault_account_bump: u8,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts
            .escrow_account
            .initializer_deposit_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;
        ctx.accounts
            .escrow_account
            .initializer_receive_token_account = *ctx
            .accounts
            .initializer_receive_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;

        let (vault_authority, _vault_authority_vault) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.escrow_account.initializer_amount,
        )?;
        
        Ok(())
    }
    pub fn cancel_escrow(
        ctx: Context<CancelEscrow>
    ) -> Result<()> {
        let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;
        Ok(())
    }

    pub fn exchange(
        ctx: Context<Exchange>
    ) -> Result<()> {
        // Transferring from initializer to taker
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context(),
            ctx.accounts.escrow_account.taker_amount,
        )?;

        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }
}

//Initializer can send a transaction to the escrow program to initialize the Vault
//In this transaction, two new accounts: Vault and EscrowAccount, will be created and
//tokens to be exchange will be transfered from Initializer to Vault
#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct InitializeEscrow<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    #[account(    
        init,
        seeds = [b"token-seed".as_ref()],
        bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount >= initializer_amount
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

//Taker can send a transaction to the escrow to exhange token B for token A
//First, tokens B will transfered from Taker to Initializer. Afterward, the
//tokens A kept in the Vault will be transfered to Taker. Finally,
//both Vault and Escrow accounts will be closed
#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.taker_amount <= taker_deposit_token_account.amount,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_account.initializer_receive_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_account.initializer_key == *initializer.key,
        close = initializer,
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

//Initializer can also send a transaction to the escrow program to cancel the demand
//of escrow. The tokens will be transfered back to the Initializer and both Vault
//and EscrowAccount will be closed in this case
#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.initializer_key == *initializer.key,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        close = initializer,
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}

impl EscrowAccount{
    pub fn space() -> usize{
        8 +
        32 +
        32 +
        32 +
        8 +
        8
    }
}

impl<'info> InitializeEscrow<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .initializer_deposit_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> CancelEscrow<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self
                .initializer_deposit_token_account
                .to_account_info()
                .clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.taker_deposit_token_account.to_account_info().clone(),
            to: self
                .initializer_receive_token_account
                .to_account_info()
                .clone(),
            authority: self.taker.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.taker_receive_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}
