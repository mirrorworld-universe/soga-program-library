use anchor_lang::prelude::*;

#[event]
pub struct CreateOrderReceiptEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub tier_id: String,

    pub order_id: String,

    pub user: Pubkey,

    pub quantity: u64,
}