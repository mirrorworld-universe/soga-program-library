use anchor_lang::prelude::*;

#[event]
pub struct UpdateSalePhaseEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub price_feed: Pubkey,

    pub price_feed_id: String,

    pub payment_receiver: Pubkey,

    pub buy_enable: bool,

    pub buy_with_token_enable: bool,

    pub airdrop_enable: bool,
}