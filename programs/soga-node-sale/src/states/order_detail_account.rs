use anchor_lang::prelude::*;

pub const ORDER_DETAIL_ACCOUNT_PREFIX: &str = "ORDER";

#[account]
pub struct OrderDetailAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub tier_id: u32,

    pub is_completed: bool,

    pub token_ids: Vec<u64>,

    pub is_token_ids_minted: Vec<bool>,

    pub quantity: u64,

    pub total_payment_in_usd: u64,

    pub total_discount_in_usd: u64,

    pub total_user_discount_in_usd: u64,

    pub total_payment: u64,

    pub total_discount: u64,

    pub total_user_discount: u64,

    pub payment_token_mint_account: Option<Pubkey>,

    pub is_whitelist: bool,
}

impl OrderDetailAccount {
    pub fn space(quantity: u64) -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 4 // tier_id
            + 1 // is_completed
            + 8 + (quantity as usize * 8)  // token_ids
            + 8 + (quantity as usize * 1)  // is_token_ids_minted
            + 8 // quantity
            + 8 // total_payment_in_usd
            + 8 // total_discount_in_usd
            + 8 // total_user_discount_in_usd
            + 8 // total_payment
            + 8 // total_discount
            + 8 // total_user_discount
            + 33 // payment_token_mint_account
    }
}