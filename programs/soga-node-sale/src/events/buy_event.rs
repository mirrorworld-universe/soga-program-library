use anchor_lang::prelude::*;

#[event]
pub struct BuyEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub tier_id: String,

    pub order_id: String,

    pub user: Pubkey,

    pub price_feed: Pubkey,

    pub payment_receiver: Pubkey,

    pub full_discount_receiver: Pubkey,

    pub half_discount_receiver: Pubkey,

    pub total_price_in_lamport: u64,

    pub full_discount_in_lamport: u64,

    pub half_discount_in_lamport: u64,

    pub pyth_price: u64,

    pub pyth_expo: u64,

    pub allow_full_discount: bool,

    pub full_discount: u64,

    pub allow_half_discount: bool,

    pub half_discount: u64,

    pub total_price_in_usd: u64,

    pub full_discount_in_usd: u64,

    pub half_discount_in_usd: u64,

    pub quantity: u64,
}