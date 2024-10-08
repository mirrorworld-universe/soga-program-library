use anchor_lang::prelude::*;

#[event]
pub struct UpdateTicketConfigEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub winner_ticket_limit: u64,

    pub ticket_purchase_enable: bool,

    pub ticket_refund_enable: bool,
}