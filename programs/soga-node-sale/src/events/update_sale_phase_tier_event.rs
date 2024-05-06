use anchor_lang::prelude::*;

#[event]
pub struct UpdateSalePhaseTierEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub tier_id: String,

    pub price: u64,

    pub mint_limit: u64,

    pub buy_enable: bool,

    pub airdrop_enable: bool,
}