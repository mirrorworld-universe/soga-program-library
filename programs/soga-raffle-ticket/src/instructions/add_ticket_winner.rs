use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint};

use crate::states::{
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
    PAYMENT_CONFIG_ACCOUNT_PREFIX,
    PaymentConfigAccount,
    USER_CONFIG_ACCOUNT_PREFIX,
    UserConfigAccount,
    USER_PAYMENT_CONFIG_ACCOUNT_PREFIX,
    UserPaymentConfigAccount
};
use crate::utils::{check_exceed_ticket_winner_limit, check_signing_authority, check_user_ticket_quantity, check_value_is_zero};

use crate::events::AddTicketWinnerEvent;

#[derive(Accounts)]
#[instruction(ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, _user_config_bump: u8, _user_payment_config_bump: u8)]
pub struct AddWinnerTicketInputAccounts<'info> {
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
        PAYMENT_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config.key().as_ref(),
        token_mint_account.key().as_ref(),
        ],
        bump = _payment_config_bump,
    )]
    pub payment_config: Box<Account<'info, PaymentConfigAccount>>,

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

    #[account(
        mut,
        seeds = [
        USER_PAYMENT_CONFIG_ACCOUNT_PREFIX.as_ref(),
        user_config.key().as_ref(),
        payment_config.key().as_ref(),
        ],
        bump = _user_payment_config_bump,
    )]
    pub user_payment_config: Box<Account<'info, UserPaymentConfigAccount>>,

    pub token_mint_account: Box<InterfaceAccount<'info, Mint>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_add_ticket_winner(ctx: Context<AddWinnerTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, _user_config_bump: u8, _user_payment_config_bump: u8, quantity: u64) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let ticket_config: &Box<Account<TicketConfigAccount>> = &ctx.accounts.ticket_config;

    let user_config: &Box<Account<UserConfigAccount>>  = &ctx.accounts.user_config;
    let user_payment_config: &Box<Account<UserPaymentConfigAccount>>  = &ctx.accounts.user_payment_config;

    // Checks
    check_signing_authority(ticket_config.signing_authority.key(), ctx.accounts.signing_authority.key())?;
    check_value_is_zero(quantity as usize)?;

    check_exceed_ticket_winner_limit(ticket_config.winner_ticket_limit, quantity)?;
    check_exceed_ticket_winner_limit(ticket_config.winner_ticket_limit,ticket_config.total_winner_ticket + quantity)?;

    check_user_ticket_quantity(user_config.total_tickets, user_config.total_win_tickets + quantity)?;
    check_user_ticket_quantity(user_payment_config.total_tickets, user_payment_config.total_win_tickets + quantity)?;


    // update
    let ticket_config: &mut Box<Account<TicketConfigAccount>> = &mut ctx.accounts.ticket_config;
    ticket_config.last_block_timestamp = timestamp;
    ticket_config.total_winner_ticket += quantity;

    let payment_config: &mut Box<Account<PaymentConfigAccount>> = &mut ctx.accounts.payment_config;
    payment_config.last_block_timestamp = timestamp;
    payment_config.total_winner_ticket += quantity;

    let user_config: &mut Box<Account<UserConfigAccount>> = &mut ctx.accounts.user_config;
    user_config.last_block_timestamp = timestamp;
    user_config.total_win_tickets += quantity;

    let user_payment_config: &mut Box<Account<UserPaymentConfigAccount>> = &mut ctx.accounts.user_payment_config;
    user_payment_config.last_block_timestamp = timestamp;
    user_payment_config.total_win_tickets += quantity;

    // Event
    let event: AddTicketWinnerEvent = AddTicketWinnerEvent {
        timestamp,
        ticket_config_name,
        token_mint_account: ctx.accounts.token_mint_account.key(),
        user: ctx.accounts.user.key(),
        quantity,
    };

    emit!(event);

    Ok(())
}