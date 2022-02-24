//https://hackmd.io/@ironaddicteddog/solana-anchor-escrow
use anchor_lang::prelude::*;

declare_id!("4NpAm9u4XrXYdKK7Lma7AA3UVSDzDntmQxmeDen8mBx5");

#[program]
pub mod escrow_anchor {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    //the inputs accounts are assigned to EscrowAccount fields one by one. Then, a PDA
    //is derived to be going to become new authority of initializer_deposit_token_account
    pub fn initialize(
        ctx: Context<Initialize>,
        _vault_account_bump: u8,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> ProgramResult {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts.escrow_account.initializer_deposit_token_account = *ctx.accounts.escrow_account.initializer_deposit_token_account.to_account_info().key;
        ctx.accounts.escrow_account.initializer_receive_token_account = *ctx.accounts.escrow_account.initializer_receive_token_account.to_account_info().key;
        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;
        
        let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts_account.initializer_amount,
        )?;

        Ok(())
    }

    //reset the authority from PDA back to the initializer
    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        let (_vault_authority, _vault_account_bump) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &_vault_authority_bump];

        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context().with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::close_account(
            ctx.accounts.into_close_context().with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }

    //3 things happer:
    //-Token A gets transfered from pda_deposit_token_account to taker_receive_token_account
    //-Token B gets transfered from taker_token_account to initializer_receive_token_account
    //-The authority of pda_deposit_token_account gets set back to the initializer
    pub fn exchange(ctx: Context<Exchange>) -> ProgramResult {
        let (_vault_authority, _vault_account_bump) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
    
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &_vault_authority_bump];

        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context(),
            ctx.accounts.escrow_account.taker_amount,
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_taker_context().with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::close_account(
            ctx.accounts.into_close_context().with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }
}

//atributes:
//    #[account(signer)] --> Checks the given account signed the transaction
//    #[account(mut)] --> Marks the account as mutable and persists the state transition
//    #[account(constraints = <expresion>)] --> Executes the given code as a constraint. 
//                                              The expression should evaluate to a boolean
//    #[account(close = <target>)] --> Marks the account as being closed at the end of the instruction execution, sending the rend 
//                                     exemption lamprots to the specified
#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>, //signer of initialize instruction, to be stored on Escrow
    pub mint:Account<'info, Mint>, 
    #[account(
        init,
        seed = [b"token-seed".as_ref()],
        bump = vault_account_bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>, //the account of Vault, which is created bu Anchor via constraints
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount >= initializer_amount
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>, //the account of token account for token exchange, to be stored on EscrowAccount
    pub initializer_receive_token_account: Account<'info, TokenAccount>, //the account of token account for token exchange, to be stored on EscrowAccount
    #[account(zero)]
    pub escrow_account: Box<Account<'info, EscrowAccount>>, //the account of EscrowAccount
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<Rent>,
    pub token_program: AccountInfo<'info>, //the account of token program
}
//the difference between Account and AccountInfo, is that using Account you use it to Anchor gives you deserialized it
//using account --> ctx.accounts.vault_account.mint;

#[derive(Accounts)]
pub struct Cancel {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>, //signer of initialize instruction, to be stored on Escrow
    #[account(mut)]
    pub mint:Account<'info, Mint>, 
    pub vault_account: Account<'info, TokenAccount>, //the account of Vault, which is created bu Anchor via constraints
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>, //the account of token account for token exchange, to be stored on EscrowAccount
    pub initializer_receive_token_account: Account<'info, TokenAccount>, //the account of token account for token exchange, to be stored on EscrowAccount
    #[account(
        mut,
        constraint = escrow_account.initializer_key == *initializer.key,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        close = initializer,
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>, //the account of EscrowAccount
    pub token_program: AccountInfo<'info>, //the account of token program
}

#[derive(Accounts)]
pub struct Exchange {
    #[account(signer)]
    pub taker: AccountInfo<'info>, //signer of exchange instruction
    #[account(mut)]
    pub taker_depositor_token_account: Account<'info, TokenAccount>, 
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer: AccountInfo<'info>, //to be used in constraints
    #[account(
        mut,
        constraint = escrow_account.taker_amount <= taker_deposit_token_account.amount,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        constraint = escrow_account.initializer_receive_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_account.initializer_key == *initializer.key,
        close = initializer
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>, //the addess of EscrowAccount, have to check if the EscrowAccount follow certain constraints
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>, //PDA
    pub vault_authority: AccountInfo<'info>, //PDA
    pub token_program: AccountInfo<'info>, //PDA
}

#[derive(Accounts)]
#[instruction(token_bump: u8)]
pub struct TestTokenSeedsInit<'info> {
    #[account(
        init,
        seeds = [b"my-token-seed".as_ref()],
        bump = token_bump,
        payer = authority,
        token::mint = mint,
        token::authority = authority,
    )]
    pub my_pda: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

//Utils
//There are some util functions used for wrapping the data to be passed in tokens::transfer, token::close_account and token::set_authority. It might look 
//a bit overwhelmed in the first place. However, the purpose behind these functions are clear and simple:

impl<'info> Initialize<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.initializer_deposit_token_account.to_account_info().clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority{
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone(),
        }
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Cancel<'info>{
    fn into_transfer_to_initializer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer{
            from: self.vault_account.to_account_info().clone(),
            to: self.initializer_deposit_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount{
            account: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Exchange<'info>{
    fn into_transfer_to_initializer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let cpi_accounts = Transfer {
            from: self.taker_deposit_token_account.to_account_info().clone(),
            to: self.initializer_receive_token_account.to_account_info().clone(),
            authority: self.taker.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer{
            to: self.vault_account.to_account_info().clone(),
            from: self.taker_receive_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_account = CloseAccount{
            to: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(&self.token_program.clone(), cpi_account)
    }
}

#[account]
pub struct EscrowAccount{
    pub initializer_key: Pubkey, //to authorize the actions
    pub initializer_deposit_token_account: Pubkey, //to record the deposit account
    pub initializer_receive_token_account: Pubkey, //to record the receiving account of initializer
    pub initializer_amount: u64,//to record how much token should the initializer transfer to taker
    pub taker_amount: u64, //to record how much token should the initializer receive from the taker
}
