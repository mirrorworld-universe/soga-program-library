use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::states::{
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
    PAYMENT_CONFIG_ACCOUNT_PREFIX,
    PaymentConfigAccount,
};
use crate::utils::{check_signing_authority, check_value_is_zero};

#[derive(Accounts)]
#[instruction(ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8)]
pub struct UpdatePaymentConfigInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(
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

    pub token_mint_account: Box<InterfaceAccount<'info, Mint>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_update_payment_config(ctx: Context<UpdatePaymentConfigInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, ticket_price: u64, refund_amount: u64, enable: bool, ticket_purchase_enable: bool, ticket_refund_enable: bool) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let ticket_config: &Box<Account<TicketConfigAccount>> = &ctx.accounts.ticket_config;

    // Checks
    check_signing_authority(ticket_config.signing_authority.key(), ctx.accounts.signing_authority.key())?;
    check_value_is_zero(ticket_price as usize)?;
    check_value_is_zero(refund_amount as usize)?;


    let payment_config: &mut Box<Account<PaymentConfigAccount>> = &mut ctx.accounts.payment_config;
    payment_config.last_block_timestamp = timestamp;
    payment_config.enable = enable;
    payment_config.ticket_price = ticket_price;
    payment_config.refund_amount = refund_amount;
    payment_config.ticket_purchase_enable = ticket_purchase_enable;
    payment_config.ticket_refund_enable = ticket_refund_enable;

    // TODO: Add Event

    Ok(())
}