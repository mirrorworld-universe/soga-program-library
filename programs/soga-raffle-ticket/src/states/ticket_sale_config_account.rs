use anchor_lang::prelude::*;

pub const TICKET_CONFIG_ACCOUNT_PREFIX: &str = "TICKET";

#[account]
pub struct TicketConfigAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    /// program main signing authority
    pub signing_authority: Pubkey,

    pub ticket_purchase_enable: bool,

    pub ticket_refund_enable: bool,

    pub total_ticket_purchased: u64,

    pub total_ticket_refunded: u64,

    pub total_winner_ticket: u64,

    pub total_winner_claimed_ticket: u64,

    pub winner_ticket_limit: u64,
}

impl TicketConfigAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 32 // main_signing_authority
            + 1 // ticket_purchase_enable
            + 1 // ticket_refund_enable
            + 8 // total_ticket_purchased
            + 8 // total_ticket_refunded
            + 8 // total_winner_ticket
            + 8 // total_winner_claimed_ticket
            + 8 // winner_ticket_limit
    }
}