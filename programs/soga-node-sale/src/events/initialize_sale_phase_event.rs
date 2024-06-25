use anchor_lang::prelude::*;

#[event]
pub struct InitializeSalePhaseEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub total_tiers: u32,

    pub signing_authority: Pubkey,

    pub price_feed: Pubkey,

    pub payment_receiver: Pubkey,

    pub price_feed_id: String,
}