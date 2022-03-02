/*use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(reader_account_bump: u8, vector_capacity: u16, name: String)]
pub struct InitializeReader<'info> {
    #[account(
        init, 
        seeds = [ 
            b"reader".as_ref(),
            user.key().as_ref(),
        ], 
        bump, 
        payer = user,
        space = ReaderAccount::space(&name, vector_capacity)
    )]
    reader_account: Account<'info, ReaderAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeReader>, 
    reader_account_bump: u8, 
    vector_capacity: u16, 
    name: String,
    //subscription_vector_capacity: u16,
    //timestamp_vector_capacity: u16,
    //timestamp_vector: Vec<i16>,

    /*nft: Pubkey*/
) -> Result<()> {

    let reader = &mut ctx.accounts.reader_account;

    reader.authority = *ctx.accounts.user.to_account_info().key;
    reader.name = name;
    reader.bump = reader_account_bump;
    reader.vector_capacity = vector_capacity;
    reader.time_sub_vector_tuple = vec![];

    Ok(())
}*/