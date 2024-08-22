use anchor_lang::prelude::*;

pub const SOGA_RAFFLE_TICKET_CONFIG_ACCOUNT_PREFIX: &str = "CONFIG";

#[account]
pub struct SogaRaffleTicketConfigAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    /// program main signing authority
    pub main_signing_authority: Pubkey,
}

impl SogaRaffleTicketConfigAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 32 // main_signing_authority
    }
}