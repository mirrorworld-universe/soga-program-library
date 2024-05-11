use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata, mpl_token_metadata::types::DataV2,
    },
    token_interface::{Mint, TokenAccount, TokenInterface, MintTo, mint_to, FreezeAccount, freeze_account},
};

use crate::states::{
    SOGA_NODE_SALE_PHASE_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseDetailAccount,
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX,
    SogaNodeSalePhaseTierDetailAccount,
    COLLECTION_ACCOUNT_PREFIX,
};

use crate::events::{
    InitializeSalePhaseTierEvent
};

use crate::utils::{
    check_signing_authority,
    check_tier_id,
    check_tier_id_out_of_range,
    check_value_is_zero,
};

#[derive(Accounts)]
#[instruction(_sale_phase_detail_bump: u8, sale_phase_name: String, tier_id: String)]
pub struct InitializeSalePhaseTierInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub signing_authority: Signer<'info>,

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
    init,
    payer = payer,
    space = SogaNodeSalePhaseTierDetailAccount::space(),
    seeds = [
    SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
    sale_phase_detail.key().as_ref(),
    tier_id.as_ref()
    ],
    bump,
    )]
    pub sale_phase_tier_detail: Box<Account<'info, SogaNodeSalePhaseTierDetailAccount>>,

    #[account(
    init,
    payer = payer,
    seeds = [
    COLLECTION_ACCOUNT_PREFIX.as_ref(),
    sale_phase_tier_detail.key().as_ref(),
    ],
    bump,
    mint::decimals = 0,
    mint::authority = sale_phase_tier_detail.key(),
    mint::freeze_authority = sale_phase_tier_detail.key(),
    mint::token_program = token_program,
    )]
    pub collection_mint_account: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: collection
    #[account(mut)]
    pub collection_metadata: AccountInfo<'info>,

    /// CHECK: collection master edition
    #[account(mut)]
    pub collection_master_edition: AccountInfo<'info>,

    #[account(
    init,
    payer = payer,
    associated_token::mint = collection_mint_account,
    associated_token::authority = sale_phase_tier_detail,
    associated_token::token_program = token_program,
    )]
    pub collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_sale_phase_tier(ctx: Context<InitializeSalePhaseTierInputAccounts>,
                                         _sale_phase_detail_bump: u8, sale_phase_name: String,
                                         tier_id: String, price: u64, quantity: u64, mint_limit: u64,
                                         collection_name: String, collection_symbol: String, collection_url: String,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let tier_id_int: u32 = tier_id.clone().parse().unwrap();

    let sale_phase_detail: &Box<Account<SogaNodeSalePhaseDetailAccount>> = &ctx.accounts.sale_phase_detail;

    // Checks

    check_value_is_zero(price as usize)?;

    check_value_is_zero(quantity as usize)?;

    check_value_is_zero(mint_limit as usize)?;

    check_signing_authority(sale_phase_detail.signing_authority, ctx.accounts.signing_authority.key())?;

    check_tier_id_out_of_range(sale_phase_detail.total_initialize_tiers, tier_id_int, sale_phase_detail.total_tiers)?;

    let current_tier: u32 = sale_phase_detail.total_initialize_tiers + 1;

    check_tier_id(current_tier, tier_id_int)?;

    let sale_phase_detail_key: Pubkey = ctx.accounts.sale_phase_detail.key();


    let signer_seeds = &[
        SOGA_NODE_SALE_PHASE_TIER_DETAIL_ACCOUNT_PREFIX.as_ref(),
        sale_phase_detail_key.as_ref(),
        tier_id.as_ref(),
        &[ctx.bumps.sale_phase_tier_detail],
    ];

    let signer = &[&signer_seeds[..]];

    // create mint account
    let mint_to_cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.collection_mint_account.to_account_info(),
            to: ctx.accounts.collection_token_account.to_account_info(),
            authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
        },
        signer,
    );

    mint_to(mint_to_cpi_context, 1)?;

    // freeze delegate account
    let freeze_account_cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        FreezeAccount {
            mint: ctx.accounts.collection_mint_account.to_account_info(),
            account: ctx.accounts.collection_token_account.to_account_info(),
            authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
        },
        signer,
    );

    freeze_account(freeze_account_cpi_context)?;

    // create metadata account
    let data_v2 = DataV2 {
        name: collection_name,
        symbol: collection_symbol,
        uri: collection_url,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let create_metadata_accounts_v3_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.collection_metadata.to_account_info(), // the metadata account being created
            mint: ctx.accounts.collection_mint_account.to_account_info(), // the mint account of the metadata account
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
            edition: ctx.accounts.collection_master_edition.to_account_info(),
            mint: ctx.accounts.collection_mint_account.to_account_info(),
            update_authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
            mint_authority: ctx.accounts.sale_phase_tier_detail.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            metadata: ctx.accounts.collection_metadata.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer,
    );

    create_master_edition_v3(create_master_edition_v3_cpi_context, Some(0))?;


    let sale_phase_tier_detail: &mut Box<Account<SogaNodeSalePhaseTierDetailAccount>> = &mut ctx.accounts.sale_phase_tier_detail;

    sale_phase_tier_detail.last_block_timestamp = timestamp;
    sale_phase_tier_detail.collection_mint_address = ctx.accounts.collection_mint_account.key();
    sale_phase_tier_detail.price = price;
    sale_phase_tier_detail.quantity = quantity;
    sale_phase_tier_detail.mint_limit = mint_limit;
    sale_phase_tier_detail.buy_enable = true;
    sale_phase_tier_detail.buy_with_token_enable = true;
    sale_phase_tier_detail.airdrop_enable = true;

    let sale_phase_detail: &mut Box<Account<SogaNodeSalePhaseDetailAccount>> = &mut ctx.accounts.sale_phase_detail;
    sale_phase_detail.total_initialize_tiers += 1;
    sale_phase_detail.last_block_timestamp = timestamp;

    // Event

    let event: InitializeSalePhaseTierEvent = InitializeSalePhaseTierEvent {
        timestamp,
        sale_phase_name,
        tier_id,
        collection_mint_address: ctx.accounts.collection_mint_account.key(),
        price,
        quantity,
        mint_limit,
    };

    emit!(event);

    Ok(())
}