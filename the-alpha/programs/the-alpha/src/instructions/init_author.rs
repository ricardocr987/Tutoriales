use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(author_account_bump: u8, vector_capacity: u16, name: String)]
pub struct InitializeAuthor<'info> {
    #[account(
        init, 
        seeds = [ 
            b"author".as_ref(),
            user.key().as_ref(),
        ], 
        bump, 
        payer = user,
        space = AuthorAccount::space(vector_capacity, &name)
    )]
    author_account: Account<'info, AuthorAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeAuthor>, 
    author_account_bump: u8, 
    vector_capacity: u16, 
    name: String,
    price_sub: u8, 
    paid_or_free: bool,
    /*nft: Pubkey*/
) -> Result<()> {

    let author = &mut ctx.accounts.author_account;

    //author.nft_owned = nft;
    author.authority = *ctx.accounts.user.to_account_info().key;
    author.price_sub = price_sub;
    author.name = name;
    author.paid_or_free = paid_or_free; // true = paid articles
    author.article_count = 0;
    author.bump = author_account_bump;
    author.vector_capacity = vector_capacity;
    author.sub_pubkeys = vec![];

    Ok(())
}