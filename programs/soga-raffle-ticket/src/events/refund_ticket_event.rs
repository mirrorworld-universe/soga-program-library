use anchor_lang::prelude::*;

#[event]
pub struct RefundTicketEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub token_mint_account: Pubkey,

    pub user: Pubkey,

    pub refund_tickets_quantity: u64,

    pub ticket_refund_amount: u64,

    pub total_ticket_refund_amount: u64,
}