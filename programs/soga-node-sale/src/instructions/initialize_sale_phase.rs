use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::states::{
    SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX,
    SogaNodeSaleConfigAccount,
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
};

use crate::events::{
    InitializeSalePhaseEvent
};

use crate::utils::{check_main_signing_authority, check_value_is_zero};

#[derive(Accounts)]
#[instruction(_sale_config_bump: u8, sale_phase_name: String)]
pub struct InitializeSalePhaseInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub main_signing_authority: Signer<'info>,

    pub signing_authority: Signer<'info>,

    pub back_authority: Signer<'info>,

    pub price_feed: Account<'info, PriceUpdateV2>,

    /// CHECK: payment receiver
    pub payment_receiver: AccountInfo<'info>,

    #[account(
    seeds = [
    SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX.as_ref()
    ],
    bump = _sale_config_bump,
    )]
    pub sale_config: Box<Account<'info, SogaNodeSaleConfigAccount>>,

    #[account(
    init,
    payer = payer,
    space = SogaNodeSalePhaseDetailAccount::space(),
    seeds = [
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_name.as_ref(),
    ],
    bump,
    )]
    pub sale_phase_detail: Box<Account<'info, SogaNodeSalePhaseDetailAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_sale_phase(ctx: Context<InitializeSalePhaseInputAccounts>,
                                    _sale_config_bump: u8, sale_phase_name: String,
                                    total_tiers: u32, name: String, symbol: String, metadata_base_uri: String,
                                    price_feed_id: String
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_config: &Box<Account<SogaNodeSaleConfigAccount>> = &ctx.accounts.sale_config;

    // Checks

    check_value_is_zero(total_tiers as usize)?;

    check_main_signing_authority(sale_config.main_signing_authority, ctx.accounts.main_signing_authority.key())?;

    let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;

    sale_phase_detail.last_block_timestamp = timestamp;
    sale_phase_detail.signing_authority = ctx.accounts.signing_authority.key();
    sale_phase_detail.back_authority = ctx.accounts.back_authority.key();
    sale_phase_detail.price_feed_address = ctx.accounts.price_feed.key();
    sale_phase_detail.price_feed_id = price_feed_id.clone();
    sale_phase_detail.payment_receiver = ctx.accounts.payment_receiver.key();

    sale_phase_detail.buy_enable = true;
    sale_phase_detail.buy_with_token_enable = true;
    sale_phase_detail.airdrop_enable = true;

    sale_phase_detail.total_tiers = total_tiers;

    sale_phase_detail.name = name;
    sale_phase_detail.symbol = symbol;
    sale_phase_detail.metadata_base_uri = metadata_base_uri;

    // Event
    let event: InitializeSalePhaseEvent = InitializeSalePhaseEvent {
        timestamp,
        sale_phase_name,
        total_tiers,
        signing_authority: ctx.accounts.signing_authority.key(),
        price_feed: ctx.accounts.price_feed.key(),
        price_feed_id,
        payment_receiver: ctx.accounts.payment_receiver.key(),
    };

    emit!(event);

    Ok(())
}