use anchor_lang::prelude::*;

#[event]
pub struct BuyWithTokenEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub tier_id: String,

    pub order_id: String,

    pub user: Pubkey,

    pub user_payer: Pubkey,

    pub price_feed: Pubkey,

    pub payment_receiver: Pubkey,

    pub full_discount_receiver: Pubkey,

    pub half_discount_receiver: Pubkey,

    pub total_price_in_lamport: u64,

    pub full_discount_in_lamport: u64,

    pub half_discount_in_lamport: u64,

    pub user_discount_in_lamport: u64,

    pub pyth_price: u64,

    pub pyth_expo: u64,

    pub allow_full_discount: bool,

    pub full_discount: u16,

    pub allow_half_discount: bool,

    pub half_discount: u16,

    pub allow_user_discount: bool,

    pub user_discount: u16,

    pub total_price_in_usd: u64,

    pub full_discount_in_usd: u64,

    pub half_discount_in_usd: u64,

    pub user_discount_in_usd: u64,

    pub payment_token_mint_account: Pubkey,

    pub payment_token_user_payer_token_account: Pubkey,

    pub payment_token_payment_receiver_token_account: Pubkey,

    pub payment_token_full_discount_receiver_token_account: Pubkey,

    pub payment_token_half_discount_receiver_token_account: Pubkey,

    pub quantity: u64,

    pub is_whitelist: bool,
}