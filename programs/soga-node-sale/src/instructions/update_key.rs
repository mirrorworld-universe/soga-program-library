use anchor_lang::prelude::*;

use crate::states::{
    SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX,
    SogaNodeSaleConfigAccount,
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
};

use crate::utils::{check_main_signing_authority};

#[derive(Accounts)]
#[instruction(_sale_config_bump: u8, _sale_phase_name: String, _sale_phase_detail_bump: u8)]
pub struct UpdateKeyInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub main_signing_authority: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(
        seeds = [
        SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX.as_ref()
        ],
        bump = _sale_config_bump,
    )]
    pub sale_config: Box<Account<'info, SogaNodeSaleConfigAccount>>,

    #[account(
        mut,
        seeds = [
        SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX.as_ref(),
        _sale_phase_name.as_ref(),
        ],
        bump = _sale_phase_detail_bump,
    )]
    pub sale_phase_detail: Box<Account<'info, SogaNodeSalePhaseDetailAccount>>,

}

pub fn handle_update_key(ctx: Context<UpdateKeyInputAccounts>, _sale_config_bump: u8, _sale_phase_name: String, _sale_phase_detail_bump: u8) -> Result<()> {
    // let timestamp = Clock::get().unwrap().unix_timestamp;
    //
    // let sale_config: &Box<Account<SogaNodeSaleConfigAccount>> = &ctx.accounts.sale_config;
    //
    // // Checks
    // check_main_signing_authority(sale_config.main_signing_authority.key(), ctx.accounts.main_signing_authority.key())?;
    //
    // let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;
    // sale_phase_detail.last_block_timestamp = timestamp;
    // sale_phase_detail.signing_authority = ctx.accounts.signing_authority.key();

    Ok(())
}