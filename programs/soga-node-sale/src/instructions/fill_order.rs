use anchor_lang::prelude::*;

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
    COLLECTION_ACCOUNT_PREFIX,
    NODE_ACCOUNT_PREFIX,
    ORDER_DETAIL_ACCOUNT_PREFIX,
    OrderDetailAccount,
};

use crate::events::{FillOrderEvent};

use crate::utils::{
    check_signing_authority,
    check_phase_tier_collection,
    check_order_token_id_filled,
    check_order_token_id,
    check_order_is_filled,
};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8, _collection_mint_account_bump: u8,
_user_detail_bump: u8, _order_detail_bump: u8, sale_phase_name: String, tier_id: String, token_id: String, order_id: String)]
pub struct FillOrderInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

    /// CHECK: user
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(
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
    seeds = [
    USER_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_detail.key().as_ref(),
    user.key().as_ref(),
    ],
    bump = _user_detail_bump,
    )]
    pub user_detail: Box<Account<'info, UserDetailAccount>>,

    #[account(
    mut,
    seeds = [
    ORDER_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_detail.key().as_ref(),
    user_detail.key().as_ref(),
    order_id.as_ref(),
    ],
    bump = _order_detail_bump,
    )]
    pub order_detail: Box<Account<'info, OrderDetailAccount>>,

    #[account(
    mut,
    seeds = [
    COLLECTION_ACCOUNT_PREFIX.as_ref(),
    sale_phase_tier_detail.key().as_ref(),
    ],
    bump = _collection_mint_account_bump,
    mint::token_program = token_program,
    )]
    pub collection_mint_account: Box<InterfaceAccount<'info, Mint>>,

    #[account(
    init,
    payer = payer,
    seeds = [
    NODE_ACCOUNT_PREFIX.as_ref(),
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

pub fn handle_file_order<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, FillOrderInputAccounts<'info>>,
                                            _sale_phase_detail_bump: u8, _sale_phase_tier_detail_bump: u8, _collection_mint_account_bump: u8,
                                            _user_detail_bump: u8, _order_detail_bump: u8, sale_phase_name: String, tier_id: String, token_id: String, order_id: String,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let collection_metadata = &ctx.remaining_accounts[0];
    let collection_master_edition = &ctx.remaining_accounts[1];
    let node_metadata = &ctx.remaining_accounts[2];
    let node_master_edition = &ctx.remaining_accounts[3];

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;
    let sale_phase_tier_detail: &Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &ctx.accounts.sale_phase_tier_detail;

    let token_id_int: u64 = token_id.clone().parse().unwrap();
    let order_detail: &Box<Account<OrderDetailAccount>> = &ctx.accounts.order_detail;

    // Checks
    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;
    check_phase_tier_collection(sale_phase_tier_detail.collection_mint_address, ctx.accounts.collection_mint_account.key())?;

    check_order_is_filled(order_detail.is_completed)?;

    check_order_token_id(order_detail.token_ids.contains(&token_id_int))?;

    let index = order_detail.token_ids.iter().position(|r| r == &token_id_int).unwrap();

    check_order_token_id_filled(order_detail.is_token_ids_minted[index])?;

    let sale_phase_detail_key: Pubkey = ctx.accounts.sale_phase_detail.key();

    let signer_seeds = &[
        SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
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
    let order_detail: &mut Box<Account<OrderDetailAccount>> = &mut ctx.accounts.order_detail;
    order_detail.last_block_timestamp = timestamp;

    order_detail.is_token_ids_minted[index] = true;

    if !order_detail.is_token_ids_minted.contains(&false) {
        order_detail.is_completed = true;
    }

    // Event
    let event: FillOrderEvent = FillOrderEvent {
        timestamp,
        sale_phase_name,
        tier_id,
        order_id,
        token_id,
        user: ctx.accounts.user.key(),
        collection_mint_account: ctx.accounts.collection_mint_account.key(),
        node_mint_account: ctx.accounts.node_mint_account.key(),
        is_completed: order_detail.is_completed,
    };

    emit!(event);

    Ok(())
}