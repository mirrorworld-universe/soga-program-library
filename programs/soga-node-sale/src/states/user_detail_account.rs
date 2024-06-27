use anchor_lang::prelude::*;

pub const USER_DETAIL_ACCOUNT_PREFIX: &str = "USER";

#[account]
pub struct UserDetailAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub total_mint: u64,

    pub total_buy: u64,

    pub total_buy_with_token: u64,

    pub total_airdrop: u64,

    pub total_payment: u64,

    pub total_discount: u64,

    pub total_orders: u64,

    pub total_whitelist_mint: u64,
}

impl UserDetailAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 8 // total_mint
            + 8 // total_buy
            + 8 // total_buy_with_token
            + 8 // total_airdrop
            + 8 // total_payment
            + 8 // total_discount
            + 8 // total_orders
            + 8 // total_whitelist_mint
    }
}