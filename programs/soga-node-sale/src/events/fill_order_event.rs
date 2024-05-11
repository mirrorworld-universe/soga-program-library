use anchor_lang::prelude::*;

#[event]
pub struct FillOrderEvent {
    pub timestamp: i64,

    pub sale_phase_name: String,

    pub tier_id: String,

    pub order_id: String,

    pub token_id: String,

    pub user: Pubkey,

    pub collection_mint_account: Pubkey,

    pub node_mint_account: Pubkey,

    pub is_completed: bool,
}