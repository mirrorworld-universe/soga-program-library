use anchor_lang::prelude::*;

use crate::states::{
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
};
use crate::utils::{check_signing_authority, check_valid_ticket_winner_limit, check_value_is_zero};

use crate::events::UpdateTicketConfigEvent;

#[derive(Accounts)]
#[instruction(ticket_config_name: String, _ticket_config_bump: u8)]
pub struct UpdateTicketConfigInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
        TICKET_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config_name.as_ref(),
        ],
        bump = _ticket_config_bump,
    )]
    pub ticket_config: Box<Account<'info, TicketConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_update_ticket_config(ctx: Context<UpdateTicketConfigInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, ticket_purchase_enable: bool, ticket_refund_enable: bool, winner_ticket_limit: u64) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let ticket_config: &Box<Account<TicketConfigAccount>> = &ctx.accounts.ticket_config;

    // Checks
    check_signing_authority(ticket_config.signing_authority.key(), ctx.accounts.signing_authority.key())?;
    check_value_is_zero(winner_ticket_limit as usize)?;
    check_valid_ticket_winner_limit(ticket_config.total_winner_ticket, winner_ticket_limit)?;

    let ticket_config: &mut Box<Account<TicketConfigAccount>> = &mut ctx.accounts.ticket_config;
    ticket_config.last_block_timestamp = timestamp;
    ticket_config.ticket_purchase_enable = ticket_purchase_enable;
    ticket_config.ticket_refund_enable = ticket_refund_enable;
    ticket_config.winner_ticket_limit = winner_ticket_limit;

    // Event
    let event: UpdateTicketConfigEvent = UpdateTicketConfigEvent {
        timestamp,
        ticket_config_name,
        winner_ticket_limit,
        ticket_purchase_enable,
        ticket_refund_enable,
    };

    emit!(event);

    Ok(())
}