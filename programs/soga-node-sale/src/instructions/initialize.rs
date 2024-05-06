use anchor_lang::prelude::*;

use crate::states::{
    SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX,
    SogaNodeSaleConfigAccount,
};

#[derive(Accounts)]
pub struct InitializeInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub main_signing_authority: Signer<'info>,

    #[account(
    init,
    payer = payer,
    space = SogaNodeSaleConfigAccount::space(),
    seeds = [
    SOGA_NODE_SALE_CONFIG_ACCOUNT_PREFIX.as_ref()
    ],
    bump,
    )]
    pub sale_config: Box<Account<'info, SogaNodeSaleConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize(ctx: Context<InitializeInputAccounts>) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_config: &mut Box<Account<SogaNodeSaleConfigAccount>> = &mut ctx.accounts.sale_config;
    sale_config.main_signing_authority = ctx.accounts.main_signing_authority.key();
    sale_config.last_block_timestamp = timestamp;

    Ok(())
}