use anchor_lang::prelude::*;

pub const USER_CONFIG_ACCOUNT_PREFIX: &str = "USER";

#[account]
pub struct UserConfigAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub total_tickets: u64,

    pub total_win_tickets: u64,

    pub total_win_claimed_tickets: u64,

    pub total_refunded_tickets: u64,
}

impl UserConfigAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 8 // total_tickets
            + 8 // total_win_tickets
            + 8 // total_win_claimed_tickets
            + 8 // total_refunded_tickets
    }
}