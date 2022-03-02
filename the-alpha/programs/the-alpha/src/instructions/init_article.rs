use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

use crate::state::*;

#[derive(Accounts)]
#[instruction(article_account_bump: u8, category: String)]
pub struct InitializeArticle<'info> {
    #[account(
        init, 
        seeds = [ 
            b"article".as_ref(), 
            author_account.key().as_ref(),
            &[author_account.article_count as u8].as_ref()
        ], 
        bump, 
        payer = user,
        space = ArticleAccount::space(&category),
    )]
    article_account: Account<'info, ArticleAccount>,
    
    #[account(mut)]
    author_account: Account<'info, AuthorAccount>,
    
    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeArticle>, 
    article_account_bump: u8, 
    category: String,
    paid_or_free: bool 
) -> Result<()> {

    let article = &mut ctx.accounts.article_account;
    let author = &mut ctx.accounts.author_account;
    
    author.article_count += 1;

    article.paid_or_free = paid_or_free; // true = paid articles
    article.authority = *ctx.accounts.user.to_account_info().key;
    article.bump = article_account_bump;
    article.article_id = author.article_count;
    article.likes_count = 0;
    article.published_timestamp = Clock::get().unwrap().unix_timestamp;
    article.category= category;

    Ok(())
}