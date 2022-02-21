//https://medium.com/@Arrivant_/how-to-verify-nfts-in-an-anchor-program-a051299acde8

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod verify_nft {
    use super::*;
    pub fn initialize(ctx: Context<VerifyNFT>) -> ProgramResult {
        
        let nft_token_account = &ctx.accounts.nft_token_account;
        let user = &ctx.accounts.user;
        let nft_mint_account = &ctx.accounts.nft_mint;

        assert_eq!(nft_token_account.owner, user.key());
        assert_eq!(nft_token_account.mint, nft_mint_Account.key());
        assert_eq!(nft_token_account.amount, 1);

        let master_eddition_seed = &[
            PREFIX.as_bytes(),
            ctx.accounts.token_metadata_program.key.as_ref(),
            user_token_account.mint.as_ref(),
            EDITION.as_bytes()
        ];

        let (master_edition_key, master_edition_seed) = 
            Pubkey::find_program_address(master_edition_seed, ctx.accounts.token_metadata_program.key);

        assert_eq!(master_edition_key, ctx.accounts.creature_edition.key());

        if(ctx.accounts.creature_edition.data_is_empty()){
            return Err(ErrorCode::NotInitialized.into());
        }

        let nft_metadata_account = &ctx.accounts.nft_metadata_accounts;

        let nft_mint_account_pubkey = ctx.accounts.nft_mint.key();

        let metadata_seed = &[
            "metadata".as_bytes(),
            ctx.accounts.token_metadata_program.key.as_ref,
            nft_mint_account_pubkey.as_ref(),
        ]
        
        let (metadata_derived_key, _bump_seed) = 
            Pubkey::find_program_Address(
                metadata_seed,
                ctx.accounts.token_metadata_program.key
            );

            assert_eq!(metadata_derived_key, nft_metadata_account.key());

            if ctx.accounts.nft_metadata_Account.data_is_empty(){
                return;
            }

        let metadata_full_account = 
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
            
        let full_metadata_clone = metadata_full_account.clone();

        let expected_creator = 
            Pubkey::from_str("AQUÍ LA DIRECCIÓN DEL CREADOR").unwrap();

        assert_eq!(
            full_metadata_clone.data.creators.as_ref().unwrap()[0].address,
            expected_creator
        );

        if(!full_metadata_clone.data.creators.unwrap()[0].verified){
            return Err(ErrorCode::AlredyVerified.into());
        }

        Ok(())
    }
    pub fn tutorial_nft(ctx: Context<VerifyNFT>) -> ProgramResult {
        
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyNFT {
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    pub nft_token_account: Account<'info, TokenAccount>,
    pub nft_metadata_account: AccountInfo<'info>,
}

#[account(address = metadata_program_id)]
pub token_metadata_program: AccountInfo<'info>,
