use std::ops::{Add, Mul, Sub};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use pyth_sdk_solana::{Price, PriceFeed, state::SolanaPriceAccount};

use anchor_spl::{
    token_interface::{TransferChecked, transfer_checked},
};

use crate::states::{
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseTierDetailAccount,
    SogaNodeSalePhasePaymentTokenDetailAccount,
    USER_DETAIL_ACCOUNT_PREFIX,
    UserDetailAccount,
    USER_TIER_DETAIL_ACCOUNT_PREFIX,
    UserTierDetailAccount,
    ORDER_DETAIL_ACCOUNT_PREFIX,
    OrderDetailAccount,
};

use crate::events::{
    BuyWithTokenEvent
};

use crate::utils::{
    check_signing_authority,
    check_price_feed,
    check_payment_receiver,
    check_phase_tier_is_completed,
    check_token_quantity_out_of_range,
    check_mint_limit,
    check_invalid_discount,
    check_payment_token_mint_account,
    check_payment_token,
    check_phase_buy_with_token,
    check_phase_tier_buy_with_token,
    check_quantity,
};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
sale_phase_name: String, tier_id: String, order_id: String, quantity: u64)]
pub struct BuyWithTokenInputAccounts<'info> {
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

pub fn handle_buy_with_token<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, BuyWithTokenInputAccounts<'info>>,
                                                _sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8, sale_phase_name: String, tier_id: String,
                                                order_id: String, quantity: u64, allow_full_discount: bool, full_discount: u64, allow_half_discount: bool, half_discount: u64,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;
    let sale_phase_tier_detail: &Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &ctx.accounts.sale_phase_tier_detail;

    let price_feed_info = &ctx.remaining_accounts[0];
    let payment_receiver = &ctx.remaining_accounts[1];
    let full_discount_receiver = &ctx.remaining_accounts[2];
    let half_discount_receiver = &ctx.remaining_accounts[3];

    let sale_phase_payment_token_detail_info = &ctx.remaining_accounts[4];
    let payment_token_mint_account = &ctx.remaining_accounts[5];
    let payment_token_program = &ctx.remaining_accounts[6];

    let payment_token_user_token_account = &ctx.remaining_accounts[7];
    let payment_token_payment_receiver_token_account = &ctx.remaining_accounts[8];
    let payment_token_full_discount_receiver_token_account = &ctx.remaining_accounts[9];
    let payment_token_half_discount_receiver_token_account = &ctx.remaining_accounts[10];

    let sale_phase_payment_token_detail = SogaNodeSalePhasePaymentTokenDetailAccount::try_deserialize(&mut &**sale_phase_payment_token_detail_info.try_borrow_mut_data()?).unwrap();

    // Checks
    check_phase_buy_with_token(sale_phase_detail.buy_with_token_enable)?;

    check_phase_tier_buy_with_token(sale_phase_tier_detail.buy_with_token_enable)?;

    check_payment_token(sale_phase_payment_token_detail.enable)?;

    check_payment_token_mint_account(sale_phase_payment_token_detail.mint, payment_token_mint_account.key())?;

    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    check_price_feed(sale_phase_payment_token_detail.price_feed_address, price_feed_info.key())?;

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

        let cpi_accounts = TransferChecked {
            from: payment_token_user_token_account.to_account_info(),
            mint: payment_token_mint_account.to_account_info(),
            to: payment_token_full_discount_receiver_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = payment_token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_context, full_discount_amount_in_lamport, sale_phase_payment_token_detail.decimals)?;
    };

    let mut half_discount_amount_in_lamport: u64 = 0;
    let mut half_discount_amount_in_usd: u64 = 0;

    if allow_half_discount {
        half_discount_amount_in_lamport = (half_discount * price_in_lamport) / 100;
        half_discount_amount_in_usd = (half_discount * price_in_usd) / 100;

        let cpi_accounts = TransferChecked {
            from: payment_token_user_token_account.to_account_info(),
            mint: payment_token_mint_account.to_account_info(),
            to: payment_token_half_discount_receiver_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = payment_token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_context, half_discount_amount_in_lamport, sale_phase_payment_token_detail.decimals)?;
    }

    let cpi_accounts = TransferChecked {
        from: payment_token_user_token_account.to_account_info(),
        mint: payment_token_mint_account.to_account_info(),
        to: payment_token_payment_receiver_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = payment_token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_context,
                     price_in_lamport.sub(full_discount_amount_in_lamport).sub(half_discount_amount_in_lamport),
                     sale_phase_payment_token_detail.decimals)?;


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
    order_detail.payment_token_mint_account = Some(payment_token_mint_account.key());
    order_detail.token_ids = Vec::with_capacity(quantity as usize);
    order_detail.is_token_ids_minted = Vec::with_capacity(quantity as usize);

    let mut current_token_id: u64 = sale_phase_tier_detail.total_mint;

    for _i in 0..quantity {
        current_token_id += 1;
        order_detail.token_ids.push(current_token_id);
        order_detail.is_token_ids_minted.push(false);
    };

    let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;
    sale_phase_detail.total_buy_with_token += quantity;
    sale_phase_detail.total_mint += quantity;
    sale_phase_detail.total_payment += price_in_usd;
    sale_phase_detail.total_discount += full_discount_amount_in_usd;
    sale_phase_detail.total_discount += half_discount_amount_in_usd;
    sale_phase_detail.last_block_timestamp = timestamp;


    let sale_phase_tier_detail: &mut Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &mut ctx.accounts.sale_phase_tier_detail;
    sale_phase_tier_detail.total_mint += quantity;
    sale_phase_tier_detail.total_buy_with_token += quantity;
    sale_phase_tier_detail.total_payment += price_in_usd;
    sale_phase_tier_detail.total_discount += full_discount_amount_in_usd;
    sale_phase_tier_detail.total_discount += half_discount_amount_in_usd;
    sale_phase_tier_detail.last_block_timestamp = timestamp;

    if sale_phase_tier_detail.total_mint >= sale_phase_tier_detail.quantity {
        sale_phase_tier_detail.is_completed = true;
        sale_phase_detail.total_completed_tiers += 1;
    }

    let user_detail: &mut Box<Account<UserDetailAccount>> = &mut ctx.accounts.user_detail;
    user_detail.total_buy_with_token += quantity;
    user_detail.total_mint += quantity;
    user_detail.total_payment += price_in_usd;
    user_detail.total_discount += full_discount_amount_in_usd;
    user_detail.total_discount += half_discount_amount_in_usd;
    user_detail.total_orders += 1;
    user_detail.last_block_timestamp = timestamp;

    let user_tier_detail: &mut Box<Account<UserTierDetailAccount>> = &mut ctx.accounts.user_tier_detail;
    user_tier_detail.total_buy_with_token += quantity;
    user_tier_detail.total_mint += quantity;
    user_tier_detail.total_payment += price_in_usd;
    user_tier_detail.total_discount += full_discount_amount_in_usd;
    user_tier_detail.total_discount += half_discount_amount_in_usd;
    user_tier_detail.last_block_timestamp = timestamp;

    // Event
    let event: BuyWithTokenEvent = BuyWithTokenEvent {
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
        payment_token_mint_account: payment_token_mint_account.key(),
        payment_token_user_token_account: payment_token_user_token_account.key(),
        payment_token_payment_receiver_token_account: payment_token_payment_receiver_token_account.key(),
        payment_token_full_discount_receiver_token_account: payment_token_full_discount_receiver_token_account.key(),
        payment_token_half_discount_receiver_token_account: payment_token_half_discount_receiver_token_account.key(),
        quantity,
    };

    emit!(event);

    Ok(())
}