use anchor_lang::prelude::*;

use crate::states::{
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
};

use crate::events::{
    UpdateSalePhaseEvent
};

use crate::utils::{check_signing_authority};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, sale_phase_name: String)]
pub struct UpdateSalePhaseInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    /// CHECK: pyth price feed
    pub price_feed: AccountInfo<'info>,

    /// CHECK: payment receiver
    pub payment_receiver: AccountInfo<'info>,

    #[account(
    mut,
    seeds = [
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_name.as_ref(),
    ],
    bump = _sale_phase_detail_bump,
    )]
    pub sale_phase_detail: Box<Account<'info, SogaNodeSalePhaseDetailAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_update_sale_phase(ctx: Context<UpdateSalePhaseInputAccounts>,
                                _sale_phase_detail_bump: u8, sale_phase_name: String,
                                name: String, symbol: String, metadata_base_uri: String,
                                buy_enable: bool, buy_with_token_enable: bool, airdrop_enable: bool,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;

    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    sale_phase_detail.last_block_timestamp = timestamp;
    sale_phase_detail.price_feed_address = ctx.accounts.price_feed.key();
    sale_phase_detail.payment_receiver = ctx.accounts.payment_receiver.key();

    sale_phase_detail.buy_enable = buy_enable;
    sale_phase_detail.airdrop_enable = airdrop_enable;

    sale_phase_detail.name = name;
    sale_phase_detail.symbol = symbol;
    sale_phase_detail.metadata_base_uri = metadata_base_uri;

    let event: UpdateSalePhaseEvent = UpdateSalePhaseEvent {
        timestamp,
        sale_phase_name,
        price_feed: ctx.accounts.price_feed.key(),
        payment_receiver: ctx.accounts.payment_receiver.key(),
        buy_enable,
        buy_with_token_enable,
        airdrop_enable,
    };

    emit!(event);

    Ok(())
}