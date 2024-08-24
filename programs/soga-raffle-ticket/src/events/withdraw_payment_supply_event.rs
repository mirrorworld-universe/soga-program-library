use anchor_lang::prelude::*;

#[event]
pub struct WithdrawPaymentSupplyEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub token_mint_account: Pubkey,

    pub receiver: Pubkey,

    pub amount: u64,
}