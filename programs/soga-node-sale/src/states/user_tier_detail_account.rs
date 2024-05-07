use anchor_lang::prelude::*;

pub const USER_TIER_DETAIL_ACCOUNT_PREFIX: &str = "USER_TIER_DETAIL";

#[account]
pub struct UserTierDetailAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub total_mint: u64,

    pub total_buy: u64,

    pub total_buy_with_token: u64,

    pub total_airdrop: u64,

    pub total_payment: u64,

    pub total_discount: u64,

    pub total_full_discount_received: u64,

    pub total_half_discount_received: u64,
}

impl UserTierDetailAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 8 // last_block_timestamp
            + 8 // total_mint
            + 8 // total_buy
            + 8 // total_buy_with_token
            + 8 // total_airdrop
            + 8 // total_payment
            + 8 // total_discount
            + 8 // total_full_discount_received
            + 8 // total_half_discount_received
    }
}