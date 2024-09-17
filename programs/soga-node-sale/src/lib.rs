use anchor_lang::prelude::*;

pub mod meta;

use instructions::*;

mod error;
mod events;
mod instructions;
mod states;
mod utils;

declare_id!("4cQkDBqFZmuiuwsQnbrFEU5kcRUTCYS1agegC3yLVYQq");

#[program]
pub mod soga_node_sale {
    use super::*;

    pub fn initialize(ctx: Context<InitializeInputAccounts>) -> Result<()> {
        handle_initialize(ctx)
    }

    pub fn initialize_sale_phase(
        ctx: Context<InitializeSalePhaseInputAccounts>,
        _sale_config_bump: u8,
        sale_phase_name: String,
        total_tiers: u32,
        name: String,
        symbol: String,
        metadata_base_uri: String,
        price_feed_id: String,
    ) -> Result<()> {
        handle_initialize_sale_phase(
            ctx,
            _sale_config_bump,
            sale_phase_name,
            total_tiers,
            name,
            symbol,
            metadata_base_uri,
            price_feed_id,
        )
    }

    pub fn initialize_sale_phase_tier(
        ctx: Context<InitializeSalePhaseTierInputAccounts>,
        _sale_phase_detail_bump: u8,
        sale_phase_name: String,
        tier_id: String,
        price: u64,
        quantity: u64,
        mint_limit: u64,
        collection_name: String,
        collection_symbol: String,
        collection_url: String,
        whitelist_quantity: u64,
    ) -> Result<()> {
        handle_initialize_sale_phase_tier(
            ctx,
            _sale_phase_detail_bump,
            sale_phase_name,
            tier_id,
            price,
            quantity,
            mint_limit,
            collection_name,
            collection_symbol,
            collection_url,
            whitelist_quantity,
        )
    }

    pub fn buy<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, BuyInputAccounts<'info>>,
        _sale_phase_detail_bump: u8,
        _sale_phase_tier_detail_bump: u8,
        sale_phase_name: String,
        tier_id: String,
        order_id: String,
        quantity: u64,
        allow_full_discount: bool,
        full_discount: u16,
        allow_half_discount: bool,
        half_discount: u16,
        is_whitelist: bool,
        allow_user_discount: bool,
        user_discount: u16,
    ) -> Result<()> {
        handle_buy(
            ctx,
            _sale_phase_detail_bump,
            _sale_phase_tier_detail_bump,
            sale_phase_name,
            tier_id,
            order_id,
            quantity,
            allow_full_discount,
            full_discount,
            allow_half_discount,
            half_discount,
            is_whitelist,
            allow_user_discount,
            user_discount,
        )
    }

    pub fn airdrop<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AirdropInputAccounts<'info>>,
        _sale_phase_detail_bump: u8,
        _sale_phase_tier_detail_bump: u8,
        _collection_mint_account_bump: u8,
        sale_phase_name: String,
        tier_id: String,
        token_id: String,
    ) -> Result<()> {
        handle_airdrop(
            ctx,
            _sale_phase_detail_bump,
            _sale_phase_tier_detail_bump,
            _collection_mint_account_bump,
            sale_phase_name,
            tier_id,
            token_id,
        )
    }

    pub fn update_sale_phase(
        ctx: Context<UpdateSalePhaseInputAccounts>,
        _sale_phase_detail_bump: u8,
        sale_phase_name: String,
        name: String,
        symbol: String,
        metadata_base_uri: String,
        buy_enable: bool,
        buy_with_token_enable: bool,
        airdrop_enable: bool,
        price_feed_id: String,
    ) -> Result<()> {
        handle_update_sale_phase(
            ctx,
            _sale_phase_detail_bump,
            sale_phase_name,
            name,
            symbol,
            metadata_base_uri,
            buy_enable,
            buy_with_token_enable,
            airdrop_enable,
            price_feed_id,
        )
    }

    pub fn update_sale_phase_tier(
        ctx: Context<UpdateSalePhaseTierInputAccounts>,
        _sale_phase_detail_bump: u8,
        _sale_phase_tier_detail_bump: u8,
        sale_phase_name: String,
        tier_id: String,
        price: u64,
        mint_limit: u64,
        buy_enable: bool,
        buy_with_token_enable: bool,
        airdrop_enable: bool,
    ) -> Result<()> {
        handle_update_sale_phase_tier(
            ctx,
            _sale_phase_detail_bump,
            _sale_phase_tier_detail_bump,
            sale_phase_name,
            tier_id,
            price,
            mint_limit,
            buy_enable,
            buy_with_token_enable,
            airdrop_enable,
        )
    }

    pub fn initialize_sale_phase_token_payment(
        ctx: Context<InitializeSalePhasePaymentTokenInputAccounts>,
        _sale_phase_detail_bump: u8,
        sale_phase_name: String,
        price_feed_id: String,
    ) -> Result<()> {
        handle_initialize_sale_phase_token_payment(
            ctx,
            _sale_phase_detail_bump,
            sale_phase_name,
            price_feed_id,
        )
    }

    pub fn update_sale_phase_token_payment(
        ctx: Context<UpdateSalePhasePaymentTokenInputAccounts>,
        _sale_phase_detail_bump: u8,
        _sale_phase_payment_token_detail_bump: u8,
        sale_phase_name: String,
        enable: bool,
        price_feed_id: String,
    ) -> Result<()> {
        handle_update_sale_phase_token_payment(
            ctx,
            _sale_phase_detail_bump,
            _sale_phase_payment_token_detail_bump,
            sale_phase_name,
            enable,
            price_feed_id,
        )
    }

    pub fn buy_with_token<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, BuyWithTokenInputAccounts<'info>>,
        _sale_phase_detail_bump: u8,
        _sale_phase_tier_detail_bump: u8,
        sale_phase_name: String,
        tier_id: String,
        order_id: String,
        quantity: u64,
        allow_full_discount: bool,
        full_discount: u16,
        allow_half_discount: bool,
        half_discount: u16,
        is_whitelist: bool,
        allow_user_discount: bool,
        user_discount: u16,
    ) -> Result<()> {
        handle_buy_with_token(
            ctx,
            _sale_phase_detail_bump,
            _sale_phase_tier_detail_bump,
            sale_phase_name,
            tier_id,
            order_id,
            quantity,
            allow_full_discount,
            full_discount,
            allow_half_discount,
            half_discount,
            is_whitelist,
            allow_user_discount,
            user_discount,
        )
    }

    pub fn file_order<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, FillOrderInputAccounts<'info>>,
        _sale_phase_detail_bump: u8,
        _sale_phase_tier_detail_bump: u8,
        _collection_mint_account_bump: u8,
        _user_detail_bump: u8,
        _order_detail_bump: u8,
        sale_phase_name: String,
        tier_id: String,
        token_id: String,
        order_id: String,
    ) -> Result<()> {
        handle_file_order(
            ctx,
            _sale_phase_detail_bump,
            _sale_phase_tier_detail_bump,
            _collection_mint_account_bump,
            _user_detail_bump,
            _order_detail_bump,
            sale_phase_name,
            tier_id,
            token_id,
            order_id,
        )
    }

    pub fn create_order_receipt(
        ctx: Context<CreateOrderReceiptInputAccounts>,
        _sale_phase_detail_bump: u8,
        _sale_phase_tier_detail_bump: u8,
        sale_phase_name: String,
        tier_id: String,
        order_id: String,
        quantity: u64,
        follow_tiers: bool,
    ) -> Result<()> {
        handle_create_order_receipt(
            ctx,
            _sale_phase_detail_bump,
            _sale_phase_tier_detail_bump,
            sale_phase_name,
            tier_id,
            order_id,
            quantity,
            follow_tiers,
        )
    }

    pub fn update_key(ctx: Context<UpdateKeyInputAccounts>, _sale_config_bump: u8, _sale_phase_name: String, _sale_phase_detail_bump: u8) -> Result<()> {
        handle_update_key(ctx, _sale_config_bump, _sale_phase_name, _sale_phase_detail_bump)
    }
}
