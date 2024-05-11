use std::ops::{Add, Mul, Sub};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use pyth_sdk_solana::{Price, PriceFeed, state::SolanaPriceAccount};

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

use crate::utils::{
    check_signing_authority,
    check_price_feed,
    check_payment_receiver,
    check_phase_tier_is_completed,
    check_token_quantity_out_of_range,
    check_mint_limit,
    check_phase_buy,
    check_phase_tier_buy,
    check_invalid_discount,
    check_quantity,
};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
sale_phase_name: String, tier_id: String, order_id: String, quantity: u64)]
pub struct BuyInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

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

pub fn handle_buy<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, BuyInputAccounts<'info>>,
                                     _sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
                                     sale_phase_name: String, tier_id: String, order_id: String, quantity: u64,
                                     allow_full_discount: bool, full_discount: u64, allow_half_discount: bool, half_discount: u64,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;
    let sale_phase_tier_detail: &Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &ctx.accounts.sale_phase_tier_detail;

    let price_feed_info = &ctx.remaining_accounts[0];
    let payment_receiver = &ctx.remaining_accounts[1];
    let full_discount_receiver = &ctx.remaining_accounts[2];
    let half_discount_receiver = &ctx.remaining_accounts[3];

    // Checks
    check_phase_buy(sale_phase_detail.buy_enable)?;

    check_phase_tier_buy(sale_phase_tier_detail.buy_enable)?;

    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    check_price_feed(sale_phase_detail.price_feed_address, price_feed_info.key())?;

    check_payment_receiver(sale_phase_detail.payment_receiver, payment_receiver.key())?;

    check_phase_tier_is_completed(sale_phase_tier_detail.is_completed)?;

    check_quantity(sale_phase_tier_detail.mint_limit, quantity)?;

    check_token_quantity_out_of_range(sale_phase_tier_detail.total_mint + quantity, sale_phase_tier_detail.quantity)?;

    let user_tier_detail: &Box<Account<UserTierDetailAccount>> = &ctx.accounts.user_tier_detail;

    check_mint_limit(sale_phase_tier_detail.mint_limit, user_tier_detail.total_mint + quantity)?;

    check_invalid_discount(full_discount, half_discount)?;


    // Make Payment
    let price_in_usd: u64 = sale_phase_tier_detail.price.mul(quantity);

    let price_feed: PriceFeed = SolanaPriceAccount::account_info_to_feed(&price_feed_info).unwrap();

    let emo_price: Price = price_feed.get_ema_price_no_older_than(timestamp, 60).unwrap();

    let pyth_expo: u64 = 10_u64.pow(u32::try_from(-emo_price.expo).unwrap());
    let pyth_price: u64 = u64::try_from(emo_price.price).unwrap();
    let price_in_lamport: u64 = LAMPORTS_PER_SOL.checked_mul(pyth_expo).unwrap().checked_div(pyth_price).unwrap().checked_mul(price_in_usd).unwrap();

    let mut full_discount_amount_in_lamport: u64 = 0;
    let mut full_discount_amount_in_usd: u64 = 0;

    if allow_full_discount {
        full_discount_amount_in_lamport = (full_discount * price_in_lamport) / 100;
        full_discount_amount_in_usd = (full_discount * price_in_usd) / 100;

        let deposit_full_discount_amount_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &full_discount_receiver.key(),
            full_discount_amount_in_lamport,
        );

        anchor_lang::solana_program::program::invoke(
            &deposit_full_discount_amount_ix,
            &[
                ctx.accounts.user.to_account_info(),
                full_discount_receiver.to_account_info(),
            ],
        )?;
    };

    let mut half_discount_amount_in_lamport: u64 = 0;
    let mut half_discount_amount_in_usd: u64 = 0;

    if allow_half_discount {
        half_discount_amount_in_lamport = (half_discount * price_in_lamport) / 100;
        half_discount_amount_in_usd = (half_discount * price_in_usd) / 100;

        let deposit_half_discount_amount_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &half_discount_receiver.key(),
            half_discount_amount_in_lamport,
        );

        anchor_lang::solana_program::program::invoke(
            &deposit_half_discount_amount_ix,
            &[
                ctx.accounts.user.to_account_info(),
                half_discount_receiver.to_account_info(),
            ],
        )?;
    }

    let deposit_amount_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &payment_receiver.key(),
        price_in_lamport.sub(full_discount_amount_in_lamport).sub(half_discount_amount_in_lamport),
    );

    anchor_lang::solana_program::program::invoke(
        &deposit_amount_ix,
        &[
            ctx.accounts.user.to_account_info(),
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
    order_detail.total_discount_in_usd = full_discount_amount_in_usd.add(half_discount_amount_in_usd);
    order_detail.total_payment = price_in_lamport;
    order_detail.total_discount = full_discount_amount_in_lamport.add(half_discount_amount_in_lamport);
    order_detail.payment_token_mint_account = None;
    order_detail.token_ids = Vec::new();
    order_detail.is_token_ids_minted = Vec::new();

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
    user_detail.total_buy += quantity;
    user_detail.total_mint += quantity;
    user_detail.total_payment += price_in_usd;
    user_detail.total_discount += full_discount_amount_in_usd;
    user_detail.total_discount += half_discount_amount_in_usd;
    user_detail.last_block_timestamp = timestamp;

    let user_tier_detail: &mut Box<Account<UserTierDetailAccount>> = &mut ctx.accounts.user_tier_detail;
    user_tier_detail.total_buy += quantity;
    user_tier_detail.total_mint += quantity;
    user_tier_detail.total_payment += price_in_usd;
    user_tier_detail.total_discount += full_discount_amount_in_usd;
    user_tier_detail.total_discount += half_discount_amount_in_usd;
    user_tier_detail.last_block_timestamp = timestamp;

    // Event
    let event: BuyEvent = BuyEvent {
        timestamp,
        sale_phase_name,
        tier_id,
        order_id,
        user: ctx.accounts.user.key(),
        price_feed: price_feed_info.key(),
        payment_receiver: payment_receiver.key(),
        full_discount_receiver: full_discount_receiver.key(),
        half_discount_receiver: half_discount_receiver.key(),
        total_price_in_lamport: price_in_lamport,
        full_discount_in_lamport: full_discount_amount_in_lamport,
        half_discount_in_lamport: half_discount_amount_in_lamport,
        pyth_expo,
        pyth_price,
        allow_full_discount,
        full_discount,
        allow_half_discount,
        half_discount,
        total_price_in_usd: price_in_usd,
        full_discount_in_usd: full_discount_amount_in_usd,
        half_discount_in_usd: half_discount_amount_in_usd,
        quantity,
    };

    emit!(event);

    Ok(())
}