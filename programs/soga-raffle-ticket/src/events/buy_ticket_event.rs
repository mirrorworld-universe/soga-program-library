use anchor_lang::prelude::*;

#[event]
pub struct BuyTicketEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub token_mint_account: Pubkey,

    pub user: Pubkey,

    pub quantity: u64,

    pub ticket_price: u64,

    pub purchase_amount: u64,
}