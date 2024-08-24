use anchor_lang::prelude::*;

#[event]
pub struct CreateTicketConfigEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub signing_authority: Pubkey,

    pub winner_ticket_limit: u64,
}