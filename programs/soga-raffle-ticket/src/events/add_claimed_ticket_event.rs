use anchor_lang::prelude::*;

#[event]
pub struct AddClaimedWinnerEvent {
    pub timestamp: i64,

    pub ticket_config_name: String,

    pub user: Pubkey,
}