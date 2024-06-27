use anchor_lang::prelude::*;

pub const SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX: &str = "PHASE_TIER";

#[account]
pub struct SogaNodeSalePhaseTierDetailAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub collection_mint_address: Pubkey,

    pub price: u64,

    pub quantity: u64,


    pub mint_limit: u64,

    pub is_completed: bool,

    pub total_mint: u64,


    pub total_buy: u64,

    pub total_buy_with_token: u64,

    pub total_airdrop: u64,

    pub total_payment: u64,

    pub total_discount: u64,

    pub buy_enable: bool,

    pub buy_with_token_enable: bool,

    pub airdrop_enable: bool,

    pub whitelist_quantity: u64,

    pub total_whitelist_mint: u64,
}

impl SogaNodeSalePhaseTierDetailAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 32 // collection_mint_address
            + 8 // price
            + 8 // quantity
            + 8 // mint_limit
            + 1 // is_completed
            + 8 // total_mint
            + 8 // total_buy
            + 8 // total_buy_with_token
            + 8 // total_airdrop
            + 8 // total_payment
            + 8 // total_discount
            + 1 // buy_enable
            + 1 // buy_with_token_enable
            + 1 // airdrop_enable
            + 8 // whitelist_quantity
            + 8 // total_whitelist_mint
    }
}