use anchor_lang::prelude::*;

pub const SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX: &str = "PHASE";

#[account]
pub struct SogaNodeSalePhaseDetailAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub signing_authority: Pubkey,

    pub back_authority: Pubkey,

    pub price_feed_address: Pubkey,

    pub price_feed_id: String,

    pub payment_receiver: Pubkey,

    pub total_payment: u64,

    pub total_discount: u64,

    pub total_tiers: u32,

    pub total_initialize_tiers: u32,

    pub total_completed_tiers: u32,

    pub buy_enable: bool,

    pub buy_with_token_enable: bool,

    pub airdrop_enable: bool,

    pub total_mint: u64,

    pub total_buy: u64,

    pub total_buy_with_token: u64,

    pub total_airdrop: u64,

    pub name: String,

    pub symbol: String,

    pub metadata_base_uri: String,

    pub total_whitelist_mint: u64,
}

impl SogaNodeSalePhaseDetailAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 32 // signing_authority
            + 32 // back_authority
            + 32 // price_feed_address
            + 66 // price_feed_id
            + 32 // payment_receiver
            + 8 // total_payment
            + 8 // total_discount
            + 4 // total_tiers
            + 4 // total_initialize_tiers
            + 4 // total_completed_tiers
            + 1 // buy_enable
            + 1 // buy_with_token_enable
            + 1 // airdrop_enable
            + 8 // total_mint
            + 8 // total_buy
            + 8 // total_buy_with_token
            + 8 // total_airdrop
            + 20 // name
            + 20 // symbol
            + 100 // metadata_base_uri
            + 8 // total_whitelist_mint
    }
}