use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenInterface},
};


use crate::states::{
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
    SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhasePaymentTokenDetailAccount,
};

use crate::events::{
    InitializeSalePhasePaymentTokenEvent
};

use crate::utils::{
    check_signing_authority,
};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, sale_phase_name: String)]
pub struct InitializeSalePhasePaymentTokenInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    /// CHECK: pyth price feed
    pub price_feed: AccountInfo<'info>,

    #[account(
    seeds = [
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_name.as_ref(),
    ],
    bump = _sale_phase_detail_bump,
    )]
    pub sale_phase_detail: Box<Account<'info, SogaNodeSalePhaseDetailAccount>>,

    #[account(
    init,
    payer = payer,
    space = SogaNodeSalePhasePaymentTokenDetailAccount::space(),
    seeds = [
    SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_detail.key().as_ref(),
    payment_token_mint_account.key().as_ref()
    ],
    bump,
    )]
    pub sale_phase_payment_token_detail: Box<Account<'info, SogaNodeSalePhasePaymentTokenDetailAccount>>,

    #[account(
    mint::token_program = payment_token_program,
    )]
    pub payment_token_mint_account: Box<InterfaceAccount<'info, Mint>>,

    pub payment_token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_sale_phase_token_payment(ctx: Context<InitializeSalePhasePaymentTokenInputAccounts>,
                                                  _sale_phase_detail_bump: u8, sale_phase_name: String,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;

    // Checks
    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    let sale_phase_payment_token_detail: &mut Box<Account<SogaNodeSalePhasePaymentTokenDetailAccount>> = &mut ctx.accounts.sale_phase_payment_token_detail;
    sale_phase_payment_token_detail.last_block_timestamp = timestamp;
    sale_phase_payment_token_detail.price_feed_address = ctx.accounts.price_feed.key();
    sale_phase_payment_token_detail.mint = ctx.accounts.payment_token_mint_account.key();
    sale_phase_payment_token_detail.decimals = ctx.accounts.payment_token_mint_account.decimals;
    sale_phase_payment_token_detail.enable = true;

    let event: InitializeSalePhasePaymentTokenEvent = InitializeSalePhasePaymentTokenEvent {
        timestamp,
        sale_phase_name,
        price_feed: ctx.accounts.price_feed.key(),
        mint: ctx.accounts.payment_token_mint_account.key(),
    };

    emit!(event);

    Ok(())
}