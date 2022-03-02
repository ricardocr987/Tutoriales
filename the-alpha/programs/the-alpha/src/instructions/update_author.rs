use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateAuthor<'info> {
    #[account(mut, has_one = authority)] //@ CustomError::WrongArticleCreator)]
    author_account: Account<'info, AuthorAccount>,
    
    #[account(mut)]
    authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateAuthor>, 
    name: String,
    paid_or_free: bool,
    price_sub: u8,
) -> Result<()> {

    let author = &mut ctx.accounts.author_account;
    //let author = &mut ctx.accounts.author_account;
    
    author.authority = *ctx.accounts.authority.to_account_info().key;
    author.paid_or_free = paid_or_free; // true = paid articles
    author.name = name;
    author.price_sub = price_sub;

    Ok(())
}