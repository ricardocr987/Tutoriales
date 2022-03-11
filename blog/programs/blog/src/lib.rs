use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;
    pub fn initialize_blog(
        ctx: Context<InitializeBlog>, 
        blog_account_bump: u8
    ) -> Result<()> {

        let blog = &mut ctx.accounts.blog_account;

        blog.bump = blog_account_bump;
        blog.authority = *ctx.accounts.user.to_account_info().key;
        blog.post_count = 0;

        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>, 
        post_account_bump: u8, 
        title: String, 
        body: String
    ) -> Result<()> {

        let blog = &mut ctx.accounts.blog_account;
        let post = &mut ctx.accounts.post_account;

        post.authority = *ctx.accounts.authority.to_account_info().key;
        post.title = title;
        post.body = body;
        post.bump = post_account_bump;
        post.entry = blog.post_count;
       
        blog.post_count += 1;

        Ok(())
    }

    pub fn update_post(
        ctx: Context<UpdatePost>, 
        title: String, 
        body: String
    ) -> Result<()> {

        let post = &mut ctx.accounts.post_account;

        post.title = title;
        post.body = body;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(blog_account_bump: u8)]
pub struct InitializeBlog<'info> {
    #[account(
        init,
        payer = user,
        space = BlogAccount::space(),
        seeds = [
            b"blog".as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    blog_account: Account<'info, BlogAccount>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(post_account_bump: u8, title: String, body: String)]
pub struct CreatePost<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            b"post".as_ref(),
            blog_account.key().as_ref(),
            &[blog_account.post_count as u8].as_ref(),
        ],
        bump,
        space = PostAccount::space(&title, &body),
    )]
    post_account: Account<'info, PostAccount>,
    #[account(mut, has_one = authority)]
    blog_account: Account<'info, BlogAccount>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, body: String)]
pub struct UpdatePost<'info> {
    #[account(mut, has_one = authority)]
    blog_account: Account<'info, BlogAccount>,
    #[account(mut, has_one = authority)]
    post_account: Account<'info, PostAccount>,
    #[account(mut)]
    authority: Signer<'info>,
}


#[account]
pub struct BlogAccount{
    pub authority: Pubkey,
    pub bump: u8,
    pub post_count: u8,
}

impl BlogAccount{
    pub fn space()->usize{
        8 +
        32 +
        1 +
        1
    }
}

#[account]
pub struct PostAccount{
    pub authority: Pubkey,
    pub bump: u8,
    pub entry: u8,
    pub title: String,
    pub body: String,
}

impl PostAccount{
    pub fn space(title: &str, body: &str)->usize{
        8 +
        32 + 
        1 + 
        1 + 
        4 + title.len() + 
        4 + body.len()
    }
}

