use anchor_lang::prelude::*;

use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};
use anchor_spl::associated_token::{AssociatedToken};

use crate::states::{
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
    PAYMENT_CONFIG_ACCOUNT_PREFIX,
    PaymentConfigAccount,
};
use crate::utils::{check_signing_authority, check_value_is_zero};

#[derive(Accounts)]
#[instruction(ticket_config_name: String, _ticket_config_bump: u8)]
pub struct CreatePaymentConfigInputAccounts<'info> {
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

    #[account(
        init,
        payer = fee_and_rent_payer,
        space = PaymentConfigAccount::space(),
        seeds = [
        PAYMENT_CONFIG_ACCOUNT_PREFIX.as_ref(),
        ticket_config.key().as_ref(),
        token_mint_account.key().as_ref(),
        ],
        bump,
    )]
    pub payment_config: Box<Account<'info, PaymentConfigAccount>>,

    #[account(
        init,
        payer = fee_and_rent_payer,
        associated_token::mint = token_mint_account,
        associated_token::authority = payment_config,
        associated_token::token_program = token_program,
    )]
    pub payment_config_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_mint_account: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_payment_config(ctx: Context<CreatePaymentConfigInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, ticket_price: u64, refund_amount: u64) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let ticket_config: &Box<Account<TicketConfigAccount>>  = &ctx.accounts.ticket_config;

    // Checks
    check_signing_authority(ticket_config.signing_authority.key(), ctx.accounts.signing_authority.key())?;
    check_value_is_zero(ticket_price as usize)?;
    check_value_is_zero(refund_amount as usize)?;


    let payment_config: &mut Box<Account<PaymentConfigAccount>>  = &mut ctx.accounts.payment_config;
    payment_config.last_block_timestamp = timestamp;
    payment_config.mint = ctx.accounts.token_mint_account.key();
    payment_config.enable = true;
    payment_config.ticket_price = ticket_price;
    payment_config.refund_amount = refund_amount;
    payment_config.ticket_purchase_enable = true;
    payment_config.ticket_refund_enable = true;

    // TODO: Add Event

    Ok(())
}