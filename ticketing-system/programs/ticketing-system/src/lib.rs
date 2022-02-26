//https://dev.to/fndomendez/building-a-simple-on-chain-point-of-sale-with-solana-anchor-and-react-859
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ticketing_system {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, tickets: Vec<u32>) -> Result<()> {
        let ticketing_system = &mut ctx.accounts.ticketing_system;
        let owner = ticketing_system.to_account_info().key;

        for (idx, ticket) in tickets.iter().enumerate() {
            ticketing_system.tickets[idx] = Ticket {
                owner: *owner,
                id: *ticket,
                available: true,
                idx: idx as u32,
            };
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user)]
    pub ticketing_system: Account<'info, TicketingSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseTicket<'info> {
    #[account(mut)]
    pub ticketing_system: Account<'info, TicketingSystem>,
    pub user: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct TicketingSystem{
    pub tickets: [Ticket; 3],
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct Ticket {
    pub owner: Pubkey,
    pub id: u32,
    pub available: bool,
    pub idx: u32,
}

