use anchor_lang::prelude::*;

use crate::states::{
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseTierDetailAccount,
    USER_DETAIL_ACCOUNT_PREFIX,
    UserDetailAccount,
    USER_TIER_DETAIL_ACCOUNT_PREFIX,
    UserTierDetailAccount,
    ORDER_DETAIL_ACCOUNT_PREFIX,
    OrderDetailAccount,
};

use crate::events::{
    CreateOrderReceiptEvent
};

use crate::utils::{check_back_authority, check_mint_limit_with_quantity, check_order_id, check_phase_tier_is_completed, check_quantity,  check_tier_id, check_token_quantity_out_of_range, check_value_is_zero};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
sale_phase_name: String, tier_id: String, order_id: String, quantity: u64)]
pub struct CreateOrderReceiptInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub back_authority: Signer<'info>,

    /// CHECK: user
    pub user: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
        SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX.as_ref(),
        sale_phase_name.as_ref(),
        ],
        bump = _sale_phase_detail_bump,
    )]
    pub sale_phase_detail: Box<Account<'info, SogaNodeSalePhaseDetailAccount>>,

    #[account(
        mut,
        seeds = [
        SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
        sale_phase_detail.key().as_ref(),
        tier_id.as_ref()
        ],
        bump = _sale_phase_tier_detail_bump,
    )]
    pub sale_phase_tier_detail: Box<Account<'info, SogaNodeSalePhaseTierDetailAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = UserDetailAccount::space(),
        seeds = [
        USER_DETAIL_ACCOUNT_PREFIX.as_ref(),
        sale_phase_detail.key().as_ref(),
        user.key().as_ref(),
        ],
        bump,
    )]
    pub user_detail: Box<Account<'info, UserDetailAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = UserTierDetailAccount::space(),
        seeds = [
        USER_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
        user_detail.key().as_ref(),
        sale_phase_tier_detail.key().as_ref(),
        ],
        bump,
    )]
    pub user_tier_detail: Box<Account<'info, UserTierDetailAccount>>,

    #[account(
        init,
        payer = payer,
        space = OrderDetailAccount::space(quantity),
        seeds = [
        ORDER_DETAIL_ACCOUNT_PREFIX.as_ref(),
        sale_phase_detail.key().as_ref(),
        user_detail.key().as_ref(),
        order_id.as_ref(),
        ],
        bump,
    )]
    pub order_detail: Box<Account<'info, OrderDetailAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_order_receipt(ctx: Context<CreateOrderReceiptInputAccounts>,
                                     _sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
                                     sale_phase_name: String, tier_id: String, order_id: String, quantity: u64, follow_tiers: bool,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let tier_id_int: u32 = tier_id.clone().parse().unwrap();
    let order_id_int: u64 = order_id.clone().parse().unwrap();

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;
    let sale_phase_tier_detail: &Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &ctx.accounts.sale_phase_tier_detail;

    // Checks
    check_value_is_zero(quantity as usize)?;

    check_back_authority(sale_phase_detail.back_authority, ctx.accounts.back_authority.key())?;

    if follow_tiers {
        check_tier_id(sale_phase_detail.total_completed_tiers + 1, tier_id_int)?;
    };

    check_phase_tier_is_completed(sale_phase_tier_detail.is_completed)?;

    check_quantity(sale_phase_tier_detail.mint_limit, quantity)?;

    check_token_quantity_out_of_range(sale_phase_tier_detail.total_mint + quantity, sale_phase_tier_detail.quantity)?;

    let user_detail: &Box<Account<UserDetailAccount>> = &ctx.accounts.user_detail;

    check_order_id(user_detail.total_orders + 1, order_id_int)?;

    let user_tier_detail: &Box<Account<UserTierDetailAccount>> = &ctx.accounts.user_tier_detail;

    check_mint_limit_with_quantity(sale_phase_tier_detail.mint_limit, user_tier_detail.total_mint + quantity)?;


    // Update
    let order_detail: &mut Box<Account<OrderDetailAccount>> = &mut ctx.accounts.order_detail;
    order_detail.last_block_timestamp = timestamp;
    order_detail.tier_id = tier_id.clone().parse().unwrap();
    order_detail.is_completed = false;
    order_detail.quantity = quantity;
    order_detail.total_payment_in_usd = 0;
    order_detail.total_user_discount = 0;
    order_detail.total_discount_in_usd = 0;
    order_detail.total_payment = 0;
    order_detail.total_user_discount = 0;
    order_detail.total_discount = 0;
    order_detail.payment_token_mint_account = None;
    order_detail.token_ids = Vec::with_capacity(quantity as usize);
    order_detail.is_token_ids_minted = Vec::with_capacity(quantity as usize);
    order_detail.is_whitelist = false;

    let mut current_token_id: u64 = sale_phase_tier_detail.total_mint;

    for _i in 0..quantity {
        current_token_id += 1;
        order_detail.token_ids.push(current_token_id);
        order_detail.is_token_ids_minted.push(false);
    };

    let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;
    sale_phase_detail.total_airdrop += quantity;
    sale_phase_detail.total_mint += quantity;
    sale_phase_detail.last_block_timestamp = timestamp;


    let sale_phase_tier_detail: &mut Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &mut ctx.accounts.sale_phase_tier_detail;
    sale_phase_tier_detail.total_mint += quantity;
    sale_phase_tier_detail.total_airdrop += quantity;
    sale_phase_tier_detail.last_block_timestamp = timestamp;

    if sale_phase_tier_detail.total_mint >= sale_phase_tier_detail.quantity {
        sale_phase_tier_detail.is_completed = true;
        sale_phase_detail.total_completed_tiers += 1;
    }

    let user_detail: &mut Box<Account<UserDetailAccount>> = &mut ctx.accounts.user_detail;
    user_detail.total_mint += quantity;
    user_detail.total_airdrop += quantity;
    user_detail.total_orders += 1;
    user_detail.last_block_timestamp = timestamp;

    let user_tier_detail: &mut Box<Account<UserTierDetailAccount>> = &mut ctx.accounts.user_tier_detail;
    user_tier_detail.total_mint += quantity;
    user_tier_detail.total_airdrop += quantity;
    user_tier_detail.last_block_timestamp = timestamp;

    // Event
    let event: CreateOrderReceiptEvent = CreateOrderReceiptEvent {
        timestamp,
        sale_phase_name,
        tier_id,
        order_id,
        user: ctx.accounts.user.key(),
        quantity,
    };

    emit!(event);

    Ok(())
}