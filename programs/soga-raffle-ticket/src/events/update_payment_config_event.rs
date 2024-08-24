use anchor_lang::prelude::*;

#[event]
pub struct UpdatePaymentConfigEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub token_mint_account: Pubkey,

    pub ticket_price: u64,

    pub refund_amount: u64,

    pub enable: bool,

    pub ticket_purchase_enable: bool,

    pub ticket_refund_enable: bool,
}