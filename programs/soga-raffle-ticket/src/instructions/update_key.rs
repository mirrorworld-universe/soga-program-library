use anchor_lang::prelude::*;

use crate::states::{
    SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX,
    SogaRaffleTicketConfigAccount,
    TICKET_CONFIG_ACCOUNT_PREFIX,
    TicketConfigAccount,
};
use crate::utils::{check_main_signing_authority};

#[derive(Accounts)]
#[instruction(_config_bump: u8, _ticket_config_name: String, _ticket_config_bump: u8)]
pub struct UpdateKeyInputAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    pub main_signing_authority: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(
        seeds = [
        SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX.as_ref()
        ],
        bump = _config_bump,
    )]
    pub config: Box<Account<'info, SogaRaffleTicketConfigAccount>>,

    #[account(
        mut,
        seeds = [
        TICKET_CONFIG_ACCOUNT_PREFIX.as_ref(),
        _ticket_config_name.as_ref(),
        ],
        bump = _ticket_config_bump,
    )]
    pub ticket_config: Box<Account<'info, TicketConfigAccount>>,
}

pub fn handle_update_key(ctx: Context<UpdateKeyInputAccounts>, _config_bump: u8, _ticket_config_name: String, _ticket_config_bump: u8) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let config: &Box<Account<SogaRaffleTicketConfigAccount>> = &ctx.accounts.config;

    // Checks
    check_main_signing_authority(config.main_signing_authority.key(), ctx.accounts.main_signing_authority.key())?;

    let ticket_config: &mut Box<Account<TicketConfigAccount>> = &mut ctx.accounts.ticket_config;
    ticket_config.last_block_timestamp = timestamp;
    ticket_config.signing_authority = ctx.accounts.signing_authority.key();

    Ok(())
}