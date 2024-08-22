use anchor_lang::prelude::*;

use crate::states::{
    SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX,
    SogaRaffleTicketConfigAccount,
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
};
use crate::utils::{check_main_signing_authority, check_value_is_zero};

#[derive(Accounts)]
#[instruction(_config_bump: u8, ticket_config_name: String)]
pub struct CreateTicketInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    pub main_signing_authority: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(
        seeds = [
        SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX.as_ref()
        ],
        bump = _config_bump,
    )]
    pub config: Box<Account<'info, SogaRaffleTicketConfigAccount>>,

    #[account(
        init,
        payer = fee_and_rent_payer,
        space = TicketConfigAccount::space(),
        seeds = [
        TICKET_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config_name.as_ref(),
        ],
        bump,
    )]
    pub ticket_config: Box<Account<'info, TicketConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_ticket_config(ctx: Context<CreateTicketInputAccounts>, _config_bump: u8, ticket_config_name: String, winner_ticket_limit: u64) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let config: &Box<Account<SogaRaffleTicketConfigAccount>> = &ctx.accounts.config;

    // Checks
    check_main_signing_authority(config.main_signing_authority.key(), ctx.accounts.main_signing_authority.key())?;
    check_value_is_zero(winner_ticket_limit as usize)?;

    let ticket_config: &mut Box<Account<TicketConfigAccount>>  = &mut ctx.accounts.ticket_config;
    ticket_config.last_block_timestamp = timestamp;
    ticket_config.signing_authority = ctx.accounts.signing_authority.key();
    ticket_config.ticket_purchase_enable = true;
    ticket_config.ticket_refund_enable = false;
    ticket_config.winner_ticket_limit = winner_ticket_limit;

    // TODO: Add Event

    Ok(())
}