use anchor_lang::prelude::*;

pub const SOGA_NODE_SALE_PHASE_PAYMENT_TOKEN_DETAIL_ACCOUNT_PREFIX: &str = "PHASE_PAYMENT_TOKEN";

#[account]
pub struct SogaNodeSalePhasePaymentTokenDetailAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub mint: Pubkey,

    pub price_feed_address: Pubkey,

    pub enable: bool,

    pub decimals: u8,
}

impl SogaNodeSalePhasePaymentTokenDetailAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 32 // mint
            + 32 // price_feed_address
            + 1 // enable
            + 1 // enable
    }
}