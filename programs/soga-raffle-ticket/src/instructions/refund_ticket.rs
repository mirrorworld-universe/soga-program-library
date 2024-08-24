use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

use crate::states::{
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
    PAYMENT_CONFIG_ACCOUNT_PREFIX,
    PaymentConfigAccount,
    USER_CONFIG_ACCOUNT_PREFIX,
    UserConfigAccount,
    USER_PAYMENT_CONFIG_ACCOUNT_PREFIX,
    UserPaymentConfigAccount,
};
use crate::utils::{check_is_payment_enable, check_is_payment_ticket_refund_enable, check_is_ticket_refund_enable, check_payment_supply, check_value_is_zero};

use crate::events::RefundTicketEvent;

#[derive(Accounts)]
#[instruction(
    ticket_config_name: String, _ticket_config_bump: u8, payment_config_bump: u8, _user_config_bump: u8, _user_payment_config_bump: u8
)]
pub struct RefundTicketInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    pub user: Signer<'info>,

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
        bump = payment_config_bump,
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

    #[account(
        mut,
        associated_token::mint = token_mint_account,
        associated_token::authority = payment_config,
        associated_token::token_program = token_program,
    )]
    pub payment_config_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = token_mint_account,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_mint_account: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_refund_ticket(ctx: Context<RefundTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, payment_config_bump: u8, _user_config_bump: u8, _user_payment_config_bump: u8) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let ticket_config: &Box<Account<TicketConfigAccount>> = &ctx.accounts.ticket_config;
    let payment_config: &Box<Account<PaymentConfigAccount>> = &ctx.accounts.payment_config;
    let user_payment_config: &Box<Account<UserPaymentConfigAccount>> = &ctx.accounts.user_payment_config;

    // Checks
    check_is_ticket_refund_enable(ticket_config.ticket_refund_enable)?;
    check_is_payment_enable(payment_config.enable)?;
    check_is_payment_ticket_refund_enable(payment_config.ticket_refund_enable)?;
    let mut refund_ticket_quantity: u64 = user_payment_config.total_tickets - user_payment_config.total_win_tickets;
    refund_ticket_quantity -= user_payment_config.total_refunded_tickets;

    check_value_is_zero(refund_ticket_quantity as usize)?;

    let refund_amount: u64 = refund_ticket_quantity * payment_config.refund_amount;

    check_payment_supply(payment_config.current_balance, refund_amount)?;

    check_value_is_zero(refund_amount as usize)?;

    // Token Transfer
    let ticket_config_key: Pubkey = ctx.accounts.ticket_config.key();
    let token_mint_account_key: Pubkey = ctx.accounts.token_mint_account.key();

    let signer_seeds = &[
        PAYMENT_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config_key.as_ref(),
        token_mint_account_key.as_ref(),
        &[payment_config_bump],
    ];

    let signer = &[&signer_seeds[..]];

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.payment_config_token_account.to_account_info(),
        mint: ctx.accounts.token_mint_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.payment_config.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    transfer_checked(cpi_context, refund_amount, ctx.accounts.token_mint_account.decimals)?;

    // update
    let ticket_config: &mut Box<Account<TicketConfigAccount>> = &mut ctx.accounts.ticket_config;
    ticket_config.last_block_timestamp = timestamp;
    ticket_config.total_ticket_refunded += refund_ticket_quantity;

    let payment_config: &mut Box<Account<PaymentConfigAccount>> = &mut ctx.accounts.payment_config;
    payment_config.last_block_timestamp = timestamp;
    payment_config.current_balance -= refund_amount;
    payment_config.total_refund += refund_amount;
    payment_config.total_ticket_refunded += refund_ticket_quantity;

    let user_config: &mut Box<Account<UserConfigAccount>> = &mut ctx.accounts.user_config;
    user_config.last_block_timestamp = timestamp;
    user_config.total_refunded_tickets += refund_ticket_quantity;

    let user_payment_config: &mut Box<Account<UserPaymentConfigAccount>> = &mut ctx.accounts.user_payment_config;
    user_payment_config.last_block_timestamp = timestamp;
    user_payment_config.total_refunded_tickets += refund_ticket_quantity;
    user_payment_config.total_refund_amount += refund_amount;

    // Event
    let event: RefundTicketEvent = RefundTicketEvent {
        timestamp,
        ticket_config_name,
        token_mint_account: ctx.accounts.token_mint_account.key(),
        user: ctx.accounts.user.key(),
        refund_tickets_quantity: refund_ticket_quantity,
        ticket_refund_amount: payment_config.refund_amount,
        total_ticket_refund_amount: refund_amount,
    };

    emit!(event);

    Ok(())
}