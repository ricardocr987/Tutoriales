// https://medium.com/@pirosb3/using-pdas-and-spl-token-in-anchor-and-solana-df05c57ccd04
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod safe_token_transfer_app {
    use super::*;
    pub fn initialize_new_grant(ctx: Context<InitializeNewGrant>, application_idx: u64, state_bump: u8, _wallet_bump: u8, amount: u64) -> ProgramResult {
        let state = &mut ctx.accounts.application_state;
        state.idx = application_idx;
        state.user_sending = ctx.accounts.user_sending.key().clone();
        state.user_receiving = ctx.accounts.user_receiving.key().clone();
        state.mint_of_token_being_sent = ctx.accounts.mint_of_token_being_sent.key().clone();
        state.escrow_wallet = ctx.accounts.escrow_wallet_state.key().clone();
        state.amount_tokens = amount;

        msg!("Initialized new Safe Transfer instance for {}", amount);

        let bump_vector = state_bump.to_le_bytes();
        let mint_of_token_being_sent_pk = ctx.accounts.mint_of_token_being_sent.key().clone();
        let application_idx_bytes = application_idx.to_le_bytes();
        let inner = vec![
            b"state".as_ref(),
            ctx.accounts.user_sending.key.as_ref(),
            ctx.accounts.user_receiving.key.as_ref(),
            mint_of_token_being_sent_pk.as_ref(), 
            application_idx_bytes.as_ref(),
            bump_vector.as_ref(),
        ];

        let outer = vec![inner.as_slice()];
        
        let transfer_instruction = Transfer{
            from: ctx.accounts.wallet_to_withdraw_from.to_account_info(),
            to: ctx.accounts.escrow_wallet_state.to_account_info(),
            authority: ctx.accounts.user_sending.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        anchor_spl::token::transfer(cpi_ctx, state.amount_tokens)?;

        // Mark stage as deposited.
        state.stage = Stage::FundsDeposited.to_code();

        Ok(())
    }
    // 
    /// A small utility function that allows us to transfer funds out of the Escrow.
    ///
    /// # Arguments
    ///
    /// * `user_sending` - Alice's account
    /// * `user_sending` - Bob's account
    /// * `mint_of_token_being_sent` - The mint of the token being held in escrow
    /// * `escrow_wallet` - The escrow Token account
    /// * `application_idx` - The primary key (timestamp) of the instance
    /// * `state` - the application state public key (PDA)
    /// * `state_bump` - the application state public key (PDA) bump
    /// * `token_program` - the token program address
    /// * `destination_wallet` - The public key of the destination address (where to send funds)
    /// * `amount` - the amount of `mint_of_token_being_sent` that is sent from `escrow_wallet` to `destination_wallet`
    ///
    fn transfer_escrow_out<'info>(
        user_sending: AccountInfo<'info>,
        user_receiving: AccountInfo<'info>,
        mint_of_token_being_sent: AccountInfo<'info>,
        escrow_wallet: &mut Account<'info, TokenAccount>,
        application_idx: u64,
        state: AccountInfo<'info>,
        state_bump: u8,
        token_program: AccountInfo<'info>,
        destination_wallet: AccountInfo<'info>,
        amount: u64
    ) -> ProgramResult {

    // Nothing interesting here! just boilerplate to compute our signer seeds for
    // signing on behalf of our PDA.
    let bump_vector = state_bump.to_le_bytes();
    let mint_of_token_being_sent_pk = mint_of_token_being_sent.key().clone();
    let application_idx_bytes = application_idx.to_le_bytes();
    let inner = vec![
        b"state".as_ref(),
        user_sending.key.as_ref(),
        user_receiving.key.as_ref(),
        mint_of_token_being_sent_pk.as_ref(), 
        application_idx_bytes.as_ref(),
        bump_vector.as_ref(),
    ];
    let outer = vec![inner.as_slice()];

    // Perform the actual transfer
    let transfer_instruction = Transfer{
        from: escrow_wallet.to_account_info(),
        to: destination_wallet,
        authority: state.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );
    anchor_spl::token::transfer(cpi_ctx, amount)?;


    // Use the `reload()` function on an account to reload it's state. Since we performed the
    // transfer, we are expecting the `amount` field to have changed.
    let should_close = {
        escrow_wallet.reload()?;
        escrow_wallet.amount == 0
    };

    // If token account has no more tokens, it should be wiped out since it has no other use case.
    if should_close {
        let ca = CloseAccount{
            account: escrow_wallet.to_account_info(),
            destination: user_sending.to_account_info(),
            authority: state.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            ca,
            outer.as_slice(),
        );
        anchor_spl::token::close_account(cpi_ctx)?;
    }
    

    pub fn complete_grant(ctx: Context<CompleteGrant>, application_idx: u64, state_bump: u8, _wallet_bump: u8) -> ProgramResult {
        if Stage::from(ctx.accounts.application_state.stage)? != Stage::FundsDeposited {
            msg!("Stage is invalid, state stage is {}", ctx.accounts.application_state.stage);
            return Err(ErrorCode::StageInvalid.into());
        }

        transfer_escrow_out(
            ctx.accounts.user_sending.to_account_info(),
            ctx.accounts.user_receiving.to_account_info(),
            ctx.accounts.mint_of_token_being_sent.to_account_info(),
            &mut ctx.accounts.escrow_wallet_state,
            application_idx,
            ctx.accounts.application_state.to_account_info(),
            state_bump,
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.wallet_to_deposit_to.to_account_info(),
            ctx.accounts.application_state.amount_tokens
        )?;

        let state = &mut ctx.accounts.application_state;
        state.stage = Stage::EscrowComplete.to_code();
        Ok(())
    }


    pub fn pull_back(ctx: Context<PullBackInstruction>, application_idx: u64, state_bump: u8, _wallet_bump: u8) -> ProgramResult {
        let current_stage = Stage::from(ctx.accounts.application_state.stage)?;
        let is_valid_stage = current_stage == Stage::FundsDeposited || current_stage == Stage::PullBackComplete;
        if !is_valid_stage {
            msg!("Stage is invalid, state stage is {}", ctx.accounts.application_state.stage);
            return Err(ErrorCode::StageInvalid.into());
        }

        let wallet_amount = ctx.accounts.escrow_wallet_state.amount;
        transfer_escrow_out(
            ctx.accounts.user_sending.to_account_info(),
            ctx.accounts.user_receiving.to_account_info(),
            ctx.accounts.mint_of_token_being_sent.to_account_info(),
            &mut ctx.accounts.escrow_wallet_state,
            application_idx,
            ctx.accounts.application_state.to_account_info(),
            state_bump,
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.refund_wallet.to_account_info(),
            wallet_amount,
        )?;
        let state = &mut ctx.accounts.application_state;
        state.stage = Stage::PullBackComplete.to_code();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(application_idx: u64, state_bump: u8, wallet_bump: u8)]
pub struct InitializeNewGrant<'info> {
    #[account(
        init,
        payer = user_sending,
        seeds = [
            b"state".as_ref(), 
            user_sending.key().as_ref(), 
            user_receiving.key().as_ref(),
            mint_of_token_being_sent.key().as_ref(),
            application_idx.to_le_bytes().as_ref()
        ],
        bump = state_bump,
    )]

    #[account(
        mut,
        constraint = wallet_to_withdrw_from.owner == user_sending.key(),
        constraint = wallet_to_withdraw_from.mint == mint_of_token_being_sent.key()
    )]
    wallet_to_withdraw_from: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(application_idx: u64, state_bump: u8, wallet_bump: u8)]
pub struct CompleteGrant<'info> {
    #[account(
        mut,
        seeds=[b"state".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref(), mint_of_token_being_sent.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump = state_bump,
        has_one = user_sending,
        has_one = user_receiving,
        has_one = mint_of_token_being_sent,
    )]
    application_state: Account<'info, State>,
    #[account(
        mut,
        seeds=[b"wallet".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref(), mint_of_token_being_sent.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump = wallet_bump,
    )]
    escrow_wallet_state: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user_receiving,
        associated_token::mint = mint_of_token_being_sent,
        associated_token::authority = user_receiving,
    )]
    wallet_to_deposit_to: Account<'info, TokenAccount>,   // Bob's USDC wallet (will be initialized if it did not exist)

    // Users and accounts in the system
    #[account(mut)]
    user_sending: AccountInfo<'info>,                     // Alice
    #[account(mut)]
    user_receiving: Signer<'info>,                        // Bob
    mint_of_token_being_sent: Account<'info, Mint>,       // USDC

    // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
#[instruction(application_idx: u64, state_bump: u8, wallet_bump: u8)]
pub struct PullBackInstruction<'info> {
    #[account(
        mut,
        seeds=[b"state".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref(), mint_of_token_being_sent.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump = state_bump,
        has_one = user_sending,
        has_one = user_receiving,
        has_one = mint_of_token_being_sent,
    )]
    application_state: Account<'info, State>,
    #[account(
        mut,
        seeds=[b"wallet".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref(), mint_of_token_being_sent.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump = wallet_bump,
    )]
    escrow_wallet_state: Account<'info, TokenAccount>,    
    // Users and accounts in the system
    #[account(mut)]
    user_sending: Signer<'info>,
    user_receiving: AccountInfo<'info>,
    mint_of_token_being_sent: Account<'info, Mint>,

    // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,

    // Wallet to deposit to
    #[account(
        mut,
        constraint=refund_wallet.owner == user_sending.key(),
        constraint=refund_wallet.mint == mint_of_token_being_sent.key()
    )]
    refund_wallet: Account<'info, TokenAccount>,
}

#[account]
#[derive(Default)]
pub struct State {
    idx: u64,
    user_sending: Pubkey,
    user_receiving: Pubkey,
    mint_of_token_being_sent: Pubkey,
    escrow_wallet: Pubkey,
    amount_tokens: u64,
    stage: u8,
}
