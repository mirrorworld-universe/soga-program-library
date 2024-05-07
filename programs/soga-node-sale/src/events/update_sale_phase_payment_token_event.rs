use anchor_lang::prelude::*;

#[event]
pub struct UpdateSalePhasePaymentTokenEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub price_feed: Pubkey,

    pub enable: bool,
}