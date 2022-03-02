use anchor_lang::prelude::*;
use instructions::*;

declare_id!("YAe2MnPSv1oRLFEfeRXeVuPbVUPLCxBL4fCon9RvpJS");

pub mod state;
pub mod instructions;


#[program]
pub mod the_alpha {
    use super::*;

    pub fn initialize_author(
        ctx: Context<InitializeAuthor>, 
        author_account_bump: u8, 
        vector_capacity: u16, 
        name: String,
        price_sub: u8, 
        paid_or_free: bool,
        /*nft: Pubkey*/
    ) -> Result<()> {
        instructions::init_author::handler(
            ctx, 
            author_account_bump, 
            vector_capacity, 
            name, 
            price_sub, 
            paid_or_free,
        )
    }
/*
    pub fn initialize_reader(
        ctx: Context<InitializeReader>, 
        author_account_bump: u8, 
        vector_capacity: u16, 
        name: String,
        /*nft: Pubkey*/

    ) -> Result<()> {
        instructions::init_reader::handler(
            ctx, 
            author_account_bump, 
            vector_capacity, 
            name
        )
    }*/

    pub fn initialize_article(
        ctx: Context<InitializeArticle>, 
        article_account_bump: u8, 
        category: String,
        paid_or_free: bool 
    ) -> Result<()> {
        instructions::init_article::handler(
            ctx, 
            article_account_bump, 
            category, 
            paid_or_free
        )
    }

    pub fn update_article(
        ctx: Context<UpdateArticle>, 
        category: String,
        paid_or_free: bool 
    ) -> Result<()> {
        instructions::update_article::handler(ctx, category, paid_or_free)
    }

    pub fn update_author(
        ctx: Context<UpdateAuthor>, 
        name: String,
        paid_or_free: bool,
        price_sub: u8,
    ) -> Result<()> {
        instructions::update_author::handler(ctx, name, paid_or_free, price_sub)
    }
}

#[error_code]
pub enum CustomError {
    #[msg("two amounts that are supposed to be equal are not")]
    AmountMismatch,

    #[msg("This subscription is full")]
    SubVectorFull,

    #[msg("Bounty must be enough to mark account rent-exempt")]
    BountyTooSmall,

    #[msg("Only the list owner or item creator may cancel an item")]
    CancelPermissions,

    #[msg("Only the list owner or item creator may finish an item")]
    FinishPermissions,

    #[msg("Item does not belong to this todo list")]
    ArticleNotFound,

    #[msg("Specified owner does not match the pubkey in the list")]
    WrongAuthorOwner,

    #[msg("Specified article creator does not match the pubkey in the item")]
    WrongArticleCreator,
}
