use anchor_lang::prelude::*;

#[event]
pub struct AirdropEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub tier_id: String,

    pub token_id: String,

    pub collection_mint_account: Pubkey,

    pub node_mint_account: Pubkey,

    pub user: Pubkey,
}