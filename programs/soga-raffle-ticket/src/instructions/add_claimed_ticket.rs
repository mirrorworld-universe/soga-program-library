use anchor_lang::prelude::*;

use anchor_spl::token_interface::{TokenInterface};

use crate::states::{
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
    USER_CONFIG_ACCOUNT_PREFIX,
    UserConfigAccount,
};
use crate::utils::{check_signing_authority, check_ticket_claim};

#[derive(Accounts)]
#[instruction(ticket_config_name: String, _ticket_config_bump: u8, _user_config_bump: u8)]
pub struct AddClaimedTicketInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    /// CHECK: receiver
    pub user: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
        TICKET_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config_name.as_ref(),
        ],
        bump = _ticket_config_bump,
    )]
    pub ticket_config: Box<Account<'info, TicketConfigAccount>>,

    #[account(
        mut,
        seeds = [
        USER_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config.key().as_ref(),
        user.key().as_ref(),
        ],
        bump = _user_config_bump,
    )]
    pub user_config: Box<Account<'info, UserConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_add_claimed_ticket(ctx: Context<AddClaimedTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _user_config_bump: u8) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let ticket_config: &Box<Account<TicketConfigAccount>> = &ctx.accounts.ticket_config;

    let user_config: &Box<Account<UserConfigAccount>> = &ctx.accounts.user_config;

    // Checks
    check_signing_authority(ticket_config.signing_authority.key(), ctx.accounts.signing_authority.key())?;

    check_ticket_claim(ticket_config.total_winner_ticket, ticket_config.total_winner_claimed_ticket + 1)?;

    check_ticket_claim(user_config.total_win_tickets, user_config.total_win_claimed_tickets + 1)?;

    // update
    let ticket_config: &mut Box<Account<TicketConfigAccount>> = &mut ctx.accounts.ticket_config;
    ticket_config.last_block_timestamp = timestamp;
    ticket_config.total_winner_claimed_ticket += 1;

    let user_config: &mut Box<Account<UserConfigAccount>> = &mut ctx.accounts.user_config;
    user_config.last_block_timestamp = timestamp;
    user_config.total_win_claimed_tickets += 1;

    // TODO: Add Event

    Ok(())
}