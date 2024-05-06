use anchor_lang::prelude::*;

#[event]
pub struct InitializeSalePhaseTierEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub tier_id: String,

    pub collection_mint_address: Pubkey,

    pub price: u64,

    pub quantity: u64,

    pub mint_limit: u64,
}