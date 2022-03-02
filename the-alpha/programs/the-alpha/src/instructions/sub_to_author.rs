/*use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SubToAuthor<'info> {
    #[account(mut, has_one = authority)] //@ CustomError::WrongArticleCreator)]
    author_account: Account<'info, AuthorAccount>,
    
    #[account(mut, has_one = authority)] //@ CustomError::WrongArticleCreator)]
    reader_account: Account<'info, ReaderAccount>,

    #[account(mut)]
    authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<SubToAuthor>, 
    sub_timestamp: i16,
    author_pubkey: Pubkey
) -> Result<()> {

    let reader = &mut ctx.accounts.reader_account;
    let author = &mut ctx.accounts.author_account;

    reader.authority = *ctx.accounts.authority.to_account_info().key;
    reader.time_sub_vector_tuple.push((author_pubkey, sub_timestamp));
    
    author.sub_pubkeys.push(*ctx.accounts.authority.to_account_info().key);

    // Hacer la transacción
    

    Ok(())
}*/