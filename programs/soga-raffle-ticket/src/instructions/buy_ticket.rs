use anchor_lang::prelude::*;

use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use anchor_spl::associated_token::{AssociatedToken};

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
use crate::utils::{check_is_payment_enable, check_is_ticket_purchase_enable, check_signing_authority, check_value_is_zero};

#[derive(Accounts)]
#[instruction(ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8)]
pub struct BuyTicketInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

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
        bump = _payment_config_bump,
    )]
    pub payment_config: Box<Account<'info, PaymentConfigAccount>>,

    #[account(
        init_if_needed,
        payer = fee_and_rent_payer,
        space = UserConfigAccount::space(),
        seeds = [
        USER_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config.key().as_ref(),
        user.key().as_ref(),
        ],
        bump,
    )]
    pub user_config: Box<Account<'info, UserConfigAccount>>,

    #[account(
        init_if_needed,
        payer = fee_and_rent_payer,
        space = UserPaymentConfigAccount::space(),
        seeds = [
        USER_PAYMENT_CONFIG_ACCOUNT_PREFIX.as_ref(),
        user_config.key().as_ref(),
        payment_config.key().as_ref(),
        ],
        bump,
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

pub fn handle_buy_ticket(ctx: Context<BuyTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, quantity: u64) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let ticket_config: &Box<Account<TicketConfigAccount>> = &ctx.accounts.ticket_config;
    let payment_config: &Box<Account<PaymentConfigAccount>>  = &ctx.accounts.payment_config;

    // Checks
    check_is_ticket_purchase_enable(ticket_config.ticket_purchase_enable)?;
    check_is_payment_enable(payment_config.enable)?;
    check_is_ticket_purchase_enable(payment_config.ticket_purchase_enable)?;
    check_value_is_zero(quantity as usize)?;

    let purchase_amount: u64 = quantity * payment_config.ticket_price;

    // Token Transfer
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        mint: ctx.accounts.token_mint_account.to_account_info(),
        to: ctx.accounts.payment_config_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_context, purchase_amount, ctx.accounts.token_mint_account.decimals)?;

    // update
    let ticket_config: &mut Box<Account<TicketConfigAccount>> = &mut ctx.accounts.ticket_config;
    ticket_config.last_block_timestamp = timestamp;
    ticket_config.total_ticket_purchased += quantity;

    let payment_config: &mut Box<Account<PaymentConfigAccount>> = &mut ctx.accounts.payment_config;
    payment_config.last_block_timestamp = timestamp;
    payment_config.current_balance += purchase_amount;
    payment_config.total_buy += purchase_amount;
    payment_config.total_ticket_purchased += quantity;

    let user_config: &mut Box<Account<UserConfigAccount>> = &mut ctx.accounts.user_config;
    user_config.last_block_timestamp = timestamp;
    user_config.total_tickets += quantity;

    let user_payment_config: &mut Box<Account<UserPaymentConfigAccount>> = &mut ctx.accounts.user_payment_config;
    user_payment_config.last_block_timestamp = timestamp;
    user_payment_config.total_tickets += quantity;
    user_payment_config.total_purchase_amount += purchase_amount;

    // TODO: Add Event

    Ok(())
}