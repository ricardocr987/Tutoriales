use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateArticle<'info> {
    #[account(mut, has_one = authority)] //@ CustomError::WrongArticleCreator)]
    article_account: Account<'info, ArticleAccount>,
    
    #[account(mut, has_one = authority)] //@ CustomError::WrongArticleCreator)]
    author_account: Account<'info, AuthorAccount>,
    
    #[account(mut)]
    authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateArticle>, 
    category: String,
    paid_or_free: bool 
) -> Result<()> {

    let article = &mut ctx.accounts.article_account;
    //let author = &mut ctx.accounts.author_account;
    
    article.authority = *ctx.accounts.authority.to_account_info().key;
    article.paid_or_free = paid_or_free; // true = paid articles
    article.category = category;

    Ok(())
}