use anchor_lang::prelude::*;

use crate::states::{
    SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX,
    SogaRaffleTicketConfigAccount,
};

#[derive(Accounts)]
pub struct InitializeInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    pub main_signing_authority: Signer<'info>,

    #[account(
        init,
        payer = fee_and_rent_payer,
        space = SogaRaffleTicketConfigAccount::space(),
        seeds = [
        SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX.as_ref()
        ],
        bump,
    )]
    pub config: Box<Account<'info, SogaRaffleTicketConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize(ctx: Context<InitializeInputAccounts>) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let config: &mut Box<Account<SogaRaffleTicketConfigAccount>> = &mut ctx.accounts.config;
    config.main_signing_authority = ctx.accounts.main_signing_authority.key();
    config.last_block_timestamp = timestamp;

    Ok(())
}