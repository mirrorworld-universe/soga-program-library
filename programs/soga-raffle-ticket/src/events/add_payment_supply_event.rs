use anchor_lang::prelude::*;

#[event]
pub struct AddPaymentSupplyEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub token_mint_account: Pubkey,

    pub supply_provider: Pubkey,

    pub amount: u64,
}