use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[account]
pub struct AuthorAccount {
    //pub nft_owned: Pubkey,

    pub authority: Pubkey,
    pub price_sub: u8,
    pub paid_or_free: bool,
    pub article_count: u64,
    pub bump: u8,
    pub name: String,
    pub vector_capacity: u16,
    pub sub_pubkeys: Vec<Pubkey>,
}

impl AuthorAccount{
    pub fn space(vector_capacity: u16, name: &str) -> usize {
        // discriminator
        8 +
        // pubkey
        32 +
        // u8
        1 +
        // bool
        8 +
        // u64
        8 +
        // u8
        1 +
        // String
        4 + name.len() +
        // u16
        2 +
        // vector Pubkeys 
        4 + (vector_capacity as usize) * std::mem::size_of::<Pubkey>()
    }
}

#[account]
pub struct ArticleAccount {
    //pub nft_owned: Pubkey,

    pub authority: Pubkey, // writter
    pub bump: u8, 
    pub paid_or_free: bool,
    pub article_id: u64,
    pub likes_count: u64,
    pub published_timestamp: i64,
    pub category: String, 
}

impl ArticleAccount{
    pub fn space(category: &str) -> usize {
        // discriminator
        8 +
        // pubkey
        32 +
        // u8
        1 +
        // bool
        8 +
        // u64
        8 +
        // u64
        8 +
        // i64
        8 +
        // String
        4 + category.len()
    }
}

#[account]
pub struct ReaderAccount {
    //pub nft_owned: Pubkey,

    pub authority: Pubkey,
    pub bump: u8, 
    pub name: String,
    pub subscription_vector_capacity: u16,
    pub subscription_pubkeys_vector: Vec<Pubkey>,
    pub timestamp_vector_capacity: u16,
    pub timestamp_vector: Vec<i16>,

    //pub time_sub_vector_tuple: Vec::<(Pubkey,i64)>
}

impl ReaderAccount{
    pub fn space(name: &str, timestamp_vector_capacity: u16, subscription_vector_capacity: u16) -> usize {
        // discriminator
        8 +
        // pubkey
        32 +
        // u8
        1 +
        // String
        4 + name.len() +
        // u16
        2 +
        // vector Timestamp
        4 + (subscription_vector_capacity as usize) * std::mem::size_of::<Pubkey>() +
        // u16
        2 +
        // vector Pubkeys 
        4 + (timestamp_vector_capacity as usize) * std::mem::size_of::<i16>() 
    }
}