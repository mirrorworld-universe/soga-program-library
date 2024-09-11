use anchor_lang::prelude::*;

pub mod meta;

use instructions::*;

mod instructions;
mod states;
mod events;
mod error;
mod utils;

declare_id!("GeYppgZZK83wKvfbwpnuipDENcYkw4uabZqvfHWf6Rfq");

#[program]
pub mod soga_raffle_ticket {
    use super::*;

    pub fn initialize(ctx: Context<InitializeInputAccounts>) -> Result<()> {
        handle_initialize(ctx)
    }

    pub fn create_ticket_config(ctx: Context<CreateTicketInputAccounts>, _config_bump: u8, ticket_config_name: String, winner_ticket_limit: u64) -> Result<()> {
        handle_create_ticket_config(ctx, _config_bump, ticket_config_name, winner_ticket_limit)
    }

    pub fn update_ticket_config(ctx: Context<UpdateTicketConfigInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, ticket_purchase_enable: bool, ticket_refund_enable: bool, winner_ticket_limit: u64) -> Result<()> {
        handle_update_ticket_config(ctx, ticket_config_name, _ticket_config_bump, ticket_purchase_enable, ticket_refund_enable, winner_ticket_limit)
    }

    pub fn create_payment_config(ctx: Context<CreatePaymentConfigInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, ticket_price: u64, refund_amount: u64) -> Result<()> {
        handle_create_payment_config(ctx, ticket_config_name, _ticket_config_bump, ticket_price, refund_amount)
    }

    pub fn update_payment_config(ctx: Context<UpdatePaymentConfigInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, ticket_price: u64, refund_amount: u64, enable: bool, ticket_purchase_enable: bool, ticket_refund_enable: bool) -> Result<()> {
        handle_update_payment_config(ctx, ticket_config_name, _ticket_config_bump, _payment_config_bump, ticket_price, refund_amount, enable, ticket_purchase_enable, ticket_refund_enable)
    }

    pub fn add_payment_supply(ctx: Context<AddPaymentSupplyInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, amount: u64) -> Result<()> {
        handle_add_payment_supply(ctx, ticket_config_name, _ticket_config_bump, _payment_config_bump, amount)
    }

    pub fn withdraw_payment_supply(ctx: Context<WithdrawPaymentSupplyInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, payment_config_bump: u8, amount: u64) -> Result<()> {
        handle_withdraw_payment_supply(ctx, ticket_config_name, _ticket_config_bump, payment_config_bump, amount)
    }

    pub fn buy_ticket(ctx: Context<BuyTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, quantity: u64) -> Result<()> {
        handle_buy_ticket(ctx, ticket_config_name, _ticket_config_bump, _payment_config_bump, quantity)
    }

    pub fn add_ticket_winner(ctx: Context<AddWinnerTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _payment_config_bump: u8, _user_config_bump: u8, _user_payment_config_bump: u8, quantity: u64) -> Result<()> {
        handle_add_ticket_winner(ctx, ticket_config_name, _ticket_config_bump, _payment_config_bump, _user_config_bump, _user_payment_config_bump, quantity)
    }

    pub fn add_claimed_ticket(ctx: Context<AddClaimedTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, _user_config_bump: u8) -> Result<()> {
        handle_add_claimed_ticket(ctx, ticket_config_name, _ticket_config_bump, _user_config_bump)
    }

    pub fn refund_ticket(ctx: Context<RefundTicketInputAccounts>, ticket_config_name: String, _ticket_config_bump: u8, payment_config_bump: u8, _user_config_bump: u8, _user_payment_config_bump: u8) -> Result<()> {
        handle_refund_ticket(ctx, ticket_config_name, _ticket_config_bump, payment_config_bump, _user_config_bump, _user_payment_config_bump)
    }
}
