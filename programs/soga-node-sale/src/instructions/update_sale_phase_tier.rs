use anchor_lang::prelude::*;

use crate::states::{
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseTierDetailAccount,
};

use crate::events::{UpdateSalePhaseTierEvent};

use crate::utils::{
    check_signing_authority,
    check_value_is_zero,
};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8, sale_phase_name: String, tier_id: String)]
pub struct UpdateSalePhaseTierInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(
    seeds = [
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_name.as_ref(),
    ],
    bump = _sale_phase_detail_bump,
    )]
    pub sale_phase_detail: Box<Account<'info, SogaNodeSalePhaseDetailAccount>>,

    #[account(
    mut,
    seeds = [
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_detail.key().as_ref(),
    tier_id.as_ref()
    ],
    bump = _sale_phase_tier_detail_bump,
    )]
    pub sale_phase_tier_detail: Box<Account<'info, SogaNodeSalePhaseTierDetailAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_update_sale_phase_tier(ctx: Context<UpdateSalePhaseTierInputAccounts>,
                                     _sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8, sale_phase_name: String,
                                     tier_id: String, price: u64, mint_limit: u64,
                                     buy_enable: bool, buy_with_token_enable: bool, airdrop_enable: bool,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;

    // Checks

    check_value_is_zero(price as usize)?;

    check_value_is_zero(mint_limit as usize)?;

    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    let sale_phase_tier_detail: &mut Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &mut ctx.accounts.sale_phase_tier_detail;
    sale_phase_tier_detail.last_block_timestamp = timestamp;
    sale_phase_tier_detail.price = price;
    sale_phase_tier_detail.mint_limit = mint_limit;
    sale_phase_tier_detail.buy_enable = buy_enable;
    sale_phase_tier_detail.airdrop_enable = airdrop_enable;

    // Event
    let event: UpdateSalePhaseTierEvent = UpdateSalePhaseTierEvent {
        timestamp,
        sale_phase_name,
        tier_id,
        price,
        mint_limit,
        buy_enable,
        buy_with_token_enable,
        airdrop_enable,
    };

    emit!(event);

    Ok(())
}