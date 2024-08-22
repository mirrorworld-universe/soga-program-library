use anchor_lang::prelude::*;

pub const PAYMENT_CONFIG_ACCOUNT_PREFIX: &str = "PAYMENT";

#[account]
pub struct PaymentConfigAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub ticket_price: u64,

    pub refund_amount: u64,

    pub enable: bool,

    pub mint: Pubkey,

    pub current_balance: u64,

    pub total_buy: u64,

    pub total_refund: u64,

    pub total_added_supply: u64,

    pub total_withdraw_supply: u64,

    pub ticket_purchase_enable: bool,

    pub ticket_refund_enable: bool,

    pub total_ticket_purchased: u64,

    pub total_ticket_refunded: u64,

    pub total_winner_ticket: u64,
}

impl PaymentConfigAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 8 // ticket_price
            + 8 // refund_amount
            + 1 // enable
            + 32 // mint
            + 8 // current_balance
            + 8 // total_buy
            + 8 // total_refund
            + 8 // total_added_supply
            + 8 // total_withdraw_supply
            + 1 // ticket_purchase_enable
            + 1 // ticket_refund_enable
            + 8 // total_ticket_purchased
            + 8 // total_ticket_refunded
            + 8 // total_winner_ticket
    }
}