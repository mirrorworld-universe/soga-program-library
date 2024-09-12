use std::ops::{Add, Mul, Sub};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, Price, get_feed_id_from_hex};

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
    BuyEvent
};

use crate::utils::{check_signing_authority, check_price_feed, check_payment_receiver, check_phase_tier_is_completed, check_token_quantity_out_of_range, check_phase_buy, check_phase_tier_buy, check_invalid_discount, check_quantity, check_tier_id, check_order_id, check_mint_limit_with_quantity, check_value_is_zero, check_invalid_user_discount, check_token_whitelist_quantity_out_of_range};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
sale_phase_name: String, tier_id: String, order_id: String, quantity: u64)]
pub struct BuyInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(mut)]
    pub user_payer: Signer<'info>,

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

    pub price_update: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_buy<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, BuyInputAccounts<'info>>,
                                     _sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
                                     sale_phase_name: String, tier_id: String, order_id: String, quantity: u64,
                                     allow_full_discount: bool, full_discount: u16, allow_half_discount: bool, half_discount: u16,
                                     is_whitelist: bool, allow_user_discount: bool, user_discount: u16,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let tier_id_int: u32 = tier_id.clone().parse().unwrap();
    let order_id_int: u64 = order_id.clone().parse().unwrap();

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;
    let sale_phase_tier_detail: &Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &ctx.accounts.sale_phase_tier_detail;

    let payment_receiver = &ctx.remaining_accounts[0];
    let full_discount_receiver = &ctx.remaining_accounts[1];
    let half_discount_receiver = &ctx.remaining_accounts[2];

    // Checks
    check_value_is_zero(quantity as usize)?;

    check_phase_buy(sale_phase_detail.buy_enable)?;

    check_phase_tier_buy(sale_phase_tier_detail.buy_enable)?;

    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    check_price_feed(sale_phase_detail.price_feed_address, ctx.accounts.price_update.key())?;

    check_payment_receiver(sale_phase_detail.payment_receiver, payment_receiver.key())?;

    if is_whitelist {
        check_token_whitelist_quantity_out_of_range(sale_phase_tier_detail.total_whitelist_mint + quantity, sale_phase_tier_detail.whitelist_quantity)?;
    } else {
        check_tier_id(sale_phase_detail.total_completed_tiers + 1, tier_id_int)?;
    }

    check_phase_tier_is_completed(sale_phase_tier_detail.is_completed)?;

    check_quantity(sale_phase_tier_detail.mint_limit, quantity)?;

    check_token_quantity_out_of_range(sale_phase_tier_detail.total_mint + quantity, sale_phase_tier_detail.quantity)?;

    let user_detail: &Box<Account<UserDetailAccount>> = &ctx.accounts.user_detail;

    check_order_id(user_detail.total_orders + 1, order_id_int)?;

    let user_tier_detail: &Box<Account<UserTierDetailAccount>> = &ctx.accounts.user_tier_detail;

    check_mint_limit_with_quantity(sale_phase_tier_detail.mint_limit, user_tier_detail.total_mint + quantity)?;

    check_invalid_discount(full_discount, half_discount)?;


    // Make Payment
    let price_in_usd: u64 = sale_phase_tier_detail.price.mul(quantity);

    let price_update = &mut ctx.accounts.price_update;

    let feed_id: [u8; 32] = get_feed_id_from_hex(sale_phase_detail.price_feed_id.as_str())?;

    let price: Price = price_update.get_price_no_older_than(&Clock::get()?,
                                                            120,
                                                            &feed_id,
    )?;

    let pyth_expo: u64 = 10_u64.pow(price.exponent.abs().try_into().unwrap());
    let pyth_price: u64 = u64::try_from(price.price).unwrap();
    let price_in_lamport: u64 = LAMPORTS_PER_SOL.checked_mul(pyth_expo).unwrap().checked_div(pyth_price).unwrap().checked_mul(price_in_usd).unwrap();

    let mut user_discount_in_usd: u64 = 0;
    let mut user_discount_in_lamport: u64 = 0;

    let mut after_user_discount_in_usd: u64 = price_in_usd;
    let mut after_user_discount_in_lamport: u64 = price_in_lamport;

    let mut full_discount_amount_in_lamport: u64 = 0;
    let mut full_discount_amount_in_usd: u64 = 0;

    if allow_user_discount {
        check_value_is_zero(user_discount as usize)?;

        check_invalid_user_discount(user_discount)?;

        user_discount_in_lamport = (user_discount as u64 * price_in_lamport) / 10000;
        user_discount_in_usd = (user_discount as u64 * price_in_usd) / 10000;

        after_user_discount_in_usd = price_in_usd - user_discount_in_usd;
        after_user_discount_in_lamport = price_in_lamport - user_discount_in_lamport;
    }

    if allow_full_discount {
        check_value_is_zero(full_discount as usize)?;

        full_discount_amount_in_lamport = (full_discount as u64 * after_user_discount_in_lamport) / 10000;
        full_discount_amount_in_usd = (full_discount as u64 * after_user_discount_in_usd) / 10000;

        let deposit_full_discount_amount_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user_payer.key(),
            &full_discount_receiver.key(),
            full_discount_amount_in_lamport,
        );

        anchor_lang::solana_program::program::invoke(
            &deposit_full_discount_amount_ix,
            &[
                ctx.accounts.user_payer.to_account_info(),
                full_discount_receiver.to_account_info(),
            ],
        )?;
    };

    let mut half_discount_amount_in_lamport: u64 = 0;
    let mut half_discount_amount_in_usd: u64 = 0;

    if allow_half_discount {
        check_value_is_zero(half_discount as usize)?;

        half_discount_amount_in_lamport = (half_discount as u64 * after_user_discount_in_lamport) / 10000;
        half_discount_amount_in_usd = (half_discount as u64 * after_user_discount_in_usd) / 10000;

        let deposit_half_discount_amount_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user_payer.key(),
            &half_discount_receiver.key(),
            half_discount_amount_in_lamport,
        );

        anchor_lang::solana_program::program::invoke(
            &deposit_half_discount_amount_ix,
            &[
                ctx.accounts.user_payer.to_account_info(),
                half_discount_receiver.to_account_info(),
            ],
        )?;
    }

    let deposit_amount_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user_payer.key(),
        &payment_receiver.key(),
        after_user_discount_in_lamport.sub(full_discount_amount_in_lamport).sub(half_discount_amount_in_lamport),
    );

    anchor_lang::solana_program::program::invoke(
        &deposit_amount_ix,
        &[
            ctx.accounts.user_payer.to_account_info(),
            payment_receiver.to_account_info(),
        ],
    )?;


    // Update
    let order_detail: &mut Box<Account<OrderDetailAccount>> = &mut ctx.accounts.order_detail;
    order_detail.last_block_timestamp = timestamp;
    order_detail.tier_id = tier_id.clone().parse().unwrap();
    order_detail.is_completed = false;
    order_detail.quantity = quantity;
    order_detail.total_payment_in_usd = price_in_usd;
    order_detail.total_user_discount_in_usd = after_user_discount_in_usd;
    order_detail.total_discount_in_usd = full_discount_amount_in_usd.add(half_discount_amount_in_usd);
    order_detail.total_payment = price_in_lamport;
    order_detail.total_user_discount = after_user_discount_in_lamport;
    order_detail.total_discount = full_discount_amount_in_lamport.add(half_discount_amount_in_lamport);
    order_detail.payment_token_mint_account = None;
    order_detail.token_ids = Vec::with_capacity(quantity as usize);
    order_detail.is_token_ids_minted = Vec::with_capacity(quantity as usize);
    order_detail.is_whitelist = is_whitelist;

    let mut current_token_id: u64 = sale_phase_tier_detail.total_mint;

    for _i in 0..quantity {
        current_token_id += 1;
        order_detail.token_ids.push(current_token_id);
        order_detail.is_token_ids_minted.push(false);
    };

    let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;
    sale_phase_detail.total_buy += quantity;
    sale_phase_detail.total_mint += quantity;
    sale_phase_detail.total_payment += price_in_usd;
    sale_phase_detail.total_discount += full_discount_amount_in_usd;
    sale_phase_detail.total_discount += half_discount_amount_in_usd;
    sale_phase_detail.last_block_timestamp = timestamp;


    let sale_phase_tier_detail: &mut Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &mut ctx.accounts.sale_phase_tier_detail;
    sale_phase_tier_detail.total_mint += quantity;
    sale_phase_tier_detail.total_buy += quantity;
    sale_phase_tier_detail.total_payment += price_in_usd;
    sale_phase_tier_detail.total_discount += full_discount_amount_in_usd;
    sale_phase_tier_detail.total_discount += half_discount_amount_in_usd;
    sale_phase_tier_detail.last_block_timestamp = timestamp;

    if sale_phase_tier_detail.total_mint >= sale_phase_tier_detail.quantity {
        sale_phase_tier_detail.is_completed = true;
        sale_phase_detail.total_completed_tiers += 1;
    }

    let user_detail: &mut Box<Account<UserDetailAccount>> = &mut ctx.accounts.user_detail;
    user_detail.total_mint += quantity;
    user_detail.total_buy += quantity;
    user_detail.total_payment += price_in_usd;
    user_detail.total_discount += full_discount_amount_in_usd;
    user_detail.total_discount += half_discount_amount_in_usd;
    user_detail.total_orders += 1;
    user_detail.last_block_timestamp = timestamp;

    let user_tier_detail: &mut Box<Account<UserTierDetailAccount>> = &mut ctx.accounts.user_tier_detail;
    user_tier_detail.total_mint += quantity;
    user_tier_detail.total_buy += quantity;
    user_tier_detail.total_payment += price_in_usd;
    user_tier_detail.total_discount += full_discount_amount_in_usd;
    user_tier_detail.total_discount += half_discount_amount_in_usd;
    user_tier_detail.last_block_timestamp = timestamp;

    if is_whitelist {
        sale_phase_detail.total_whitelist_mint += quantity;
        sale_phase_tier_detail.total_whitelist_mint += quantity;
        user_detail.total_whitelist_mint += quantity;
        user_tier_detail.total_whitelist_mint += quantity;
    }

    // Event
    let event: BuyEvent = BuyEvent {
        timestamp,
        sale_phase_name,
        tier_id,
        order_id,
        user: ctx.accounts.user.key(),
        user_payer: ctx.accounts.user_payer.key(),
        price_feed: ctx.accounts.price_update.key(),
        payment_receiver: payment_receiver.key(),
        full_discount_receiver: full_discount_receiver.key(),
        half_discount_receiver: half_discount_receiver.key(),
        total_price_in_lamport: price_in_lamport,
        full_discount_in_lamport: full_discount_amount_in_lamport,
        half_discount_in_lamport: half_discount_amount_in_lamport,
        user_discount_in_lamport,
        pyth_expo,
        pyth_price,
        allow_full_discount,
        full_discount,
        allow_half_discount,
        half_discount,
        allow_user_discount,
        user_discount,
        total_price_in_usd: price_in_usd,
        full_discount_in_usd: full_discount_amount_in_usd,
        half_discount_in_usd: half_discount_amount_in_usd,
        user_discount_in_usd,
        quantity,
        is_whitelist,
    };

    emit!(event);

    Ok(())
}