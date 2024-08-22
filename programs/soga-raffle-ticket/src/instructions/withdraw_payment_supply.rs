use anchor_lang::prelude::*;

use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use anchor_spl::associated_token::{AssociatedToken};

use crate::states::{
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
    PAYMENT_CONFIG_ACCOUNT_PREFIX,
    PaymentConfigAccount,
};
use crate::utils::{check_is_payment_enable, check_payment_supply, check_signing_authority, check_value_is_zero};

#[derive(Accounts)]
#[instruction(ticket_config_name: String, _ticket_config_bump: u8, payment_config_bump: u8)]
pub struct WithdrawPaymentSupplyInputAccounts<'info> {
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
        bump = payment_config_bump,
    )]
    pub payment_config: Box<Account<'info, PaymentConfigAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint_account,
        associated_token::authority = payment_config,
        associated_token::token_program = token_program,
    )]
    pub payment_config_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: receiver
    pub receiver: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = fee_and_rent_payer,
        associated_token::mint = token_mint_account,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_mint_account: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_withdraw_payment_supply(ctx: Context<WithdrawPaymentSupplyInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, payment_config_bump: u8, amount: u64) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let payment_config: &Box<Account<PaymentConfigAccount>>  = &ctx.accounts.payment_config;

    let ticket_config: &Box<Account<TicketConfigAccount>>  = &ctx.accounts.ticket_config;

    // Checks
    check_signing_authority(ticket_config.signing_authority.key(), ctx.accounts.signing_authority.key())?;
    check_is_payment_enable(payment_config.enable)?;
    check_value_is_zero(amount as usize)?;
    check_payment_supply(payment_config.current_balance, amount)?;

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
        to: ctx.accounts.receiver_token_account.to_account_info(),
        authority: ctx.accounts.payment_config.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    transfer_checked(cpi_context, amount, ctx.accounts.token_mint_account.decimals)?;

    let payment_config: &mut Box<Account<PaymentConfigAccount>>  = &mut ctx.accounts.payment_config;
    payment_config.last_block_timestamp = timestamp;
    payment_config.current_balance -= amount;
    payment_config.total_withdraw_supply += amount;

    // TODO: Add Event

    Ok(())
}