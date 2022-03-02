use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(reader_account_bump: u8, subscription_vector_capacity: u16, timestamp_vector_capacity: u16, name: String)]
pub struct InitializeReader<'info> {
    #[account(
        init, 
        seeds = [ 
            b"reader".as_ref(),
            user.key().as_ref(),
        ], 
        bump, 
        payer = user,
        space = ReaderAccount::space(&name, timestamp_vector_capacity, subscription_vector_capacity)
    )]
    reader_account: Account<'info, ReaderAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeReader>, 
    reader_account_bump: u8, 
    name: String,
    subscription_vector_capacity: u16,
    timestamp_vector_capacity: u16,
    /*nft: Pubkey*/
) -> Result<()> {

    let reader = &mut ctx.accounts.reader_account;

    reader.authority = *ctx.accounts.user.to_account_info().key;
    reader.bump = reader_account_bump;
    reader.name = name;
    reader.subscription_vector_capacity = subscription_vector_capacity;
    reader.subscription_pubkeys_vector = Vec::new();
    reader.timestamp_vector_capacity = timestamp_vector_capacity;
    reader.timestamp_vector = Vec::new();

    Ok(())
}