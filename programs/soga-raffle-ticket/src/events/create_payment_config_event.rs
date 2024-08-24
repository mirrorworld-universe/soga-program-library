use anchor_lang::prelude::*;

#[event]
pub struct CreatePaymentConfigEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub token_mint_account: Pubkey,

    pub ticket_price: u64,

    pub refund_amount: u64,
}