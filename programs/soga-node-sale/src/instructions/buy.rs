use std::ops::Sub;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use pyth_sdk_solana::{Price, PriceFeed, state::SolanaPriceAccount};

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata, mpl_token_metadata::types::{DataV2, Collection},
        VerifyCollection, verify_collection,
    },
    token_interface::{Mint, TokenAccount, TokenInterface, MintTo, mint_to, FreezeAccount, freeze_account},
};

use crate::states::{
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseTierDetailAccount,
    USER_DETAIL_ACCOUNT_PREFIX,
    UserDetailAccount,
    USER_TIER_DETAIL_ACCOUNT_PREFIX,
    UserTierDetailAccount,
};

use crate::events::{
    BuyEvent
};

use crate::utils::{
    check_signing_authority,
    check_price_feed,
    check_payment_receiver,
    check_phase_tier_collection,
    check_phase_tier_is_completed,
    check_token_id,
    check_token_id_out_of_range,
    check_mint_limit,
    check_phase_buy,
    check_phase_tier_buy,
    check_invalid_discount
};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
_collection_mint_account_bump: u8, sale_phase_name: String, tier_id: String, token_id: String)]
pub struct BuyInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    // /// CHECK: pyth price feed
    // pub price_feed: AccountInfo<'info>,
    //
    // /// CHECK: payment receiver
    // #[account(mut)]
    // pub payment_receiver: AccountInfo<'info>,
    //
    // /// CHECK: full receiver
    // #[account(mut)]
    // pub full_discount_receiver: AccountInfo<'info>,
    //
    // /// CHECK: half discount receiver
    // #[account(mut)]
    // pub half_discount_receiver: AccountInfo<'info>,

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
    sale_phase_name.as_ref(),
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
    sale_phase_name.as_ref(),
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
    sale_phase_name.as_ref(),
    sale_phase_detail.key().as_ref(),
    user.key().as_ref(),
    user_detail.key().as_ref(),
    tier_id.as_ref(),
    sale_phase_tier_detail.key().as_ref(),
    ],
    bump,
    )]
    pub user_tier_detail: Box<Account<'info, UserTierDetailAccount>>,

    #[account(
    mut,
    seeds = [
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_name.as_ref(),
    sale_phase_detail.key().as_ref(),
    tier_id.as_ref(),
    sale_phase_tier_detail.key().as_ref(),
    ],
    bump = _collection_mint_account_bump,
    mint::token_program = token_program,
    )]
    pub collection_mint_account: Box<InterfaceAccount<'info, Mint>>,

    // /// CHECK: collection metadata
    // #[account(mut)]
    // pub collection_metadata: AccountInfo<'info>,
    //
    // /// CHECK: collection master edition
    // #[account(mut)]
    // pub collection_master_edition: AccountInfo<'info>,

    #[account(
    init,
    payer = payer,
    seeds = [
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_name.as_ref(),
    sale_phase_detail.key().as_ref(),
    tier_id.as_ref(),
    sale_phase_tier_detail.key().as_ref(),
    collection_mint_account.key().as_ref(),
    token_id.as_ref()
    ],
    bump,
    mint::decimals = 0,
    mint::authority = sale_phase_tier_detail.key(),
    mint::freeze_authority = sale_phase_tier_detail.key(),
    mint::token_program = token_program,
    )]
    pub node_mint_account: Box<InterfaceAccount<'info, Mint>>,

    // /// CHECK: node metadata
    // #[account(mut)]
    // pub node_metadata: AccountInfo<'info>,
    //
    // /// CHECK: note master edition
    // #[account(mut)]
    // pub node_master_edition: AccountInfo<'info>,

    #[account(
    init,
    payer = payer,
    associated_token::mint = node_mint_account,
    associated_token::authority = user,
    associated_token::token_program = token_program,
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_buy<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, BuyInputAccounts<'info>>,
                                     _sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8,
                                     _collection_mint_account_bump: u8, sale_phase_name: String, tier_id: String, token_id: String,
                                     allow_full_discount: bool, full_discount: u64, allow_half_discount: bool, half_discount: u64,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;
    let sale_phase_tier_detail: &Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &ctx.accounts.sale_phase_tier_detail;

    let price_feed_info = &ctx.remaining_accounts[0];
    let payment_receiver = &ctx.remaining_accounts[1];
    let full_discount_receiver = &ctx.remaining_accounts[2];
    let half_discount_receiver = &ctx.remaining_accounts[3];
    let collection_metadata = &ctx.remaining_accounts[4];
    let collection_master_edition = &ctx.remaining_accounts[5];
    let node_metadata = &ctx.remaining_accounts[6];
    let node_master_edition = &ctx.remaining_accounts[7];

    // Checks

    check_phase_buy(sale_phase_detail.buy_enable)?;

    check_phase_tier_buy(sale_phase_tier_detail.buy_enable)?;

    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    check_price_feed(sale_phase_detail.price_feed_address, price_feed_info.key())?;

    check_payment_receiver(sale_phase_detail.payment_receiver, payment_receiver.key())?;

    check_phase_tier_collection(sale_phase_tier_detail.collection_mint_address, ctx.accounts.collection_mint_account.key())?;

    check_phase_tier_is_completed(sale_phase_tier_detail.is_completed)?;

    let token_id_int: u64 = token_id.clone().parse().unwrap();
    let current_token_id: u64 = sale_phase_tier_detail.total_mint + 1;

    check_token_id(current_token_id, token_id_int)?;

    check_token_id_out_of_range(sale_phase_tier_detail.total_mint, token_id_int, sale_phase_tier_detail.quantity)?;

    let user_tier_detail: &Box<Account<UserTierDetailAccount>> = &ctx.accounts.user_tier_detail;

    check_mint_limit(sale_phase_tier_detail.mint_limit, user_tier_detail.total_mint)?;

    check_invalid_discount(full_discount, half_discount)?;

    // // Make Payment
    let price_in_usd: u64 = sale_phase_tier_detail.price;

    let price_feed: PriceFeed = SolanaPriceAccount::account_info_to_feed(&price_feed_info).unwrap();

    let emo_price: Price = price_feed.get_ema_price_no_older_than(timestamp, 60).unwrap();

    let pyth_expo: u64 = 10_u64.pow(u32::try_from(-emo_price.expo).unwrap());
    let pyth_price: u64 = u64::try_from(emo_price.price).unwrap();
    let price_in_lamport: u64 = LAMPORTS_PER_SOL.checked_mul(pyth_expo).unwrap().checked_div(pyth_price).unwrap().checked_mul(price_in_usd).unwrap();

    let mut full_discount_amount_in_lamport: u64 = 0;

    if allow_full_discount {
        full_discount_amount_in_lamport = (full_discount * price_in_lamport) / 100;

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

    if allow_half_discount {
        half_discount_amount_in_lamport = (half_discount * price_in_lamport) / 100;

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

    let sale_phase_detail_key: Pubkey = ctx.accounts.sale_phase_detail.key();

    let signer_seeds = &[
        SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
        sale_phase_name.as_ref(),
        sale_phase_detail_key.as_ref(),
        tier_id.as_ref(),
        &[_sale_phase_tier_detail_bump],
    ];

    let signer = &[&signer_seeds[..]];

    // create mint account
    let mint_to_cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.node_mint_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
        },
        signer,
    );

    mint_to(mint_to_cpi_context, 1)?;

    // freeze delegate account
    let freeze_account_cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        FreezeAccount {
            mint: ctx.accounts.node_mint_account.to_account_info(),
            account: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
        },
        signer,
    );

    freeze_account(freeze_account_cpi_context)?;

    // create metadata account
    let data_v2 = DataV2 {
        name: sale_phase_detail.name.clone(),
        symbol: sale_phase_detail.symbol.clone(),
        uri: sale_phase_detail.metadata_base_uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: Some(Collection {
            verified: false,
            key: ctx.accounts.collection_mint_account.key(),
        }),
        uses: None,
    };

    let create_metadata_accounts_v3_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: node_metadata.to_account_info(), // the metadata account being created
            mint: ctx.accounts.node_mint_account.to_account_info(), // the mint account of the metadata account
            mint_authority: ctx.accounts.sale_phase_tier_detail.to_account_info(), // the mint authority of the mint account
            update_authority: ctx.accounts.sale_phase_tier_detail.to_account_info(), // the update authority of the metadata account
            payer: ctx.accounts.payer.to_account_info(), // the payer for creating the metadata account
            system_program: ctx.accounts.system_program.to_account_info(), // the system program account
            rent: ctx.accounts.rent.to_account_info(), // the rent sysvar account
        },
        signer,
    );

    create_metadata_accounts_v3(
        create_metadata_accounts_v3_cpi_ctx, // cpi context
        data_v2, // token metadata
        true,    // is_mutable
        true,    // update_authority_is_signer
        None,    // collection details
    )?;

    //create master edition account
    let create_master_edition_v3_cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMasterEditionV3 {
            edition: node_master_edition.to_account_info(),
            mint: ctx.accounts.node_mint_account.to_account_info(),
            update_authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
            mint_authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            metadata: node_metadata.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer,
    );

    create_master_edition_v3(create_master_edition_v3_cpi_context, Some(0))?;

    //verify collection
    let verify_collection_cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        VerifyCollection {
            payer: ctx.accounts.payer.to_account_info(),
            metadata: node_metadata.to_account_info(),
            collection_authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
            collection_mint: ctx.accounts.collection_mint_account.to_account_info(),
            collection_metadata: collection_metadata.to_account_info(),
            collection_master_edition: collection_master_edition.to_account_info(),
        },
        signer,
    );

    verify_collection(verify_collection_cpi_context, None)?;


    // Update
    let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;
    sale_phase_detail.total_buy += 1;
    sale_phase_detail.total_mint += 1;
    sale_phase_detail.total_payment += price_in_lamport;
    sale_phase_detail.total_discount += full_discount_amount_in_lamport;
    sale_phase_detail.total_discount += half_discount_amount_in_lamport;
    sale_phase_detail.last_block_timestamp = timestamp;


    let sale_phase_tier_detail: &mut Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &mut ctx.accounts.sale_phase_tier_detail;
    sale_phase_tier_detail.total_mint += 1;
    sale_phase_tier_detail.total_buy += 1;
    sale_phase_tier_detail.total_payment += price_in_lamport;
    sale_phase_tier_detail.total_discount += full_discount_amount_in_lamport;
    sale_phase_tier_detail.total_discount += half_discount_amount_in_lamport;
    sale_phase_tier_detail.last_block_timestamp = timestamp;

    if sale_phase_tier_detail.total_mint >= sale_phase_tier_detail.quantity {
        sale_phase_tier_detail.is_completed = true;
        sale_phase_detail.total_completed_tiers += 1;
    }

    let user_detail: &mut Box<Account<UserDetailAccount>> = &mut ctx.accounts.user_detail;
    user_detail.total_buy += 1;
    user_detail.total_mint += 1;
    user_detail.total_payment += price_in_lamport;
    user_detail.total_discount += full_discount_amount_in_lamport;
    user_detail.total_discount += half_discount_amount_in_lamport;
    user_detail.last_block_timestamp = timestamp;

    let user_tier_detail: &mut Box<Account<UserTierDetailAccount>> = &mut ctx.accounts.user_tier_detail;
    user_tier_detail.total_buy += 1;
    user_tier_detail.total_mint += 1;
    user_tier_detail.total_payment += price_in_lamport;
    user_tier_detail.total_discount += full_discount_amount_in_lamport;
    user_tier_detail.total_discount += half_discount_amount_in_lamport;
    user_tier_detail.last_block_timestamp = timestamp;

    // Event
    let event: BuyEvent = BuyEvent {
        timestamp,
        sale_phase_name,
        tier_id,
        token_id,
        user: ctx.accounts.user.key(),
        collection_mint_account: ctx.accounts.collection_mint_account.key(),
        node_mint_account: ctx.accounts.node_mint_account.key(),
        price_feed: price_feed_info.key(),
        payment_receiver: payment_receiver.key(),
        full_discount_receiver: full_discount_receiver.key(),
        half_discount_receiver: half_discount_receiver.key(),
        total_price_in_lamport: price_in_lamport,
        sub_price_in_lamport: price_in_lamport.sub(full_discount_amount_in_lamport).sub(half_discount_amount_in_lamport),
        full_discount_in_lamport: full_discount_amount_in_lamport,
        half_discount_in_lamport: half_discount_amount_in_lamport,
        price_in_usd,
        pyth_expo,
        pyth_price,
        allow_full_discount,
        full_discount,
        allow_half_discount,
        half_discount
    };

    emit!(event);

    Ok(())
}