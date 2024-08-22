use anchor_lang::prelude::*;

pub const USER_PAYMENT_CONFIG_ACCOUNT_PREFIX: &str = "USER_PAYMENT";

#[account]
pub struct UserPaymentConfigAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub total_tickets: u64,

    pub total_win_tickets: u64,

    pub total_refunded_tickets: u64,

    pub total_purchase_amount: u64,

    pub total_refund_amount: u64,
}

impl UserPaymentConfigAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 8 // total_tickets
            + 8 // total_win_tickets
            + 8 // total_refunded_tickets
            + 8 // total_purchase_amount
            + 8 // total_refund_amount
    }
}