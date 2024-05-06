use anchor_lang::prelude::*;

use crate::error::SogaNodeSaleError;

pub fn check_main_signing_authority(main_signing_authority_from_account: Pubkey, main_signing_authority_from_input_accounts: Pubkey) -> Result<()> {
    if main_signing_authority_from_account != main_signing_authority_from_input_accounts {
        return Err(SogaNodeSaleError::InvalidMainSigningAuthority.into());
    }

    Ok(())
}

pub fn check_signing_authority(signing_authority_from_account: Pubkey, signing_authority_from_input_accounts: Pubkey) -> Result<()> {
    if signing_authority_from_account != signing_authority_from_input_accounts {
        return Err(SogaNodeSaleError::InvalidSigningAuthority.into());
    }

    Ok(())
}

pub fn check_tier_id(current_tier_id_from_account: u32, current_tier_id_from_param: u32) -> Result<()> {
    if current_tier_id_from_account != current_tier_id_from_param {
        return Err(SogaNodeSaleError::InvalidTierId.into());
    }

    Ok(())
}

pub fn check_tier_id_out_of_range(total_initialize_tier_id_from_account: u32, current_tier_id_from_param: u32, total_tier_id: u32) -> Result<()> {
    if total_initialize_tier_id_from_account == current_tier_id_from_param || current_tier_id_from_param > total_tier_id {
        return Err(SogaNodeSaleError::TierIdOutOfRange.into());
    }

    Ok(())
}

pub fn check_price_feed(value_from_account: Pubkey, value_from_input_accounts: Pubkey) -> Result<()> {
    if value_from_account != value_from_input_accounts {
        return Err(SogaNodeSaleError::InvalidPriceFeedAddress.into());
    }

    Ok(())
}

pub fn check_payment_receiver(value_from_account: Pubkey, value_from_input_accounts: Pubkey) -> Result<()> {
    if value_from_account != value_from_input_accounts {
        return Err(SogaNodeSaleError::InvalidPaymentReceiverAddress.into());
    }

    Ok(())
}

pub fn check_phase_tier_collection(value_from_account: Pubkey, value_from_input_accounts: Pubkey) -> Result<()> {
    if value_from_account != value_from_input_accounts {
        return Err(SogaNodeSaleError::InvalidPhaseTierCollectionAddress.into());
    }

    Ok(())
}

pub fn check_phase_tier_is_completed(value: bool) -> Result<()> {
    if value {
        return Err(SogaNodeSaleError::PhaseTierIsCompleted.into());
    }

    Ok(())
}

pub fn check_token_id(current_token_id_from_account: u64, current_token_id_from_param: u64) -> Result<()> {
    if current_token_id_from_account != current_token_id_from_param {
        return Err(SogaNodeSaleError::InvalidTokenId.into());
    }

    Ok(())
}

pub fn check_token_id_out_of_range(total_minted_token_id_from_account: u64, current_token_id_from_param: u64, quantity: u64) -> Result<()> {
    if total_minted_token_id_from_account == current_token_id_from_param || current_token_id_from_param > quantity {
        return Err(SogaNodeSaleError::TokenIdOutOfRange.into());
    }

    Ok(())
}

pub fn check_mint_limit(mint_limit_from_config: u64, mint_limit_from_user: u64) -> Result<()> {
    if mint_limit_from_user >= mint_limit_from_config  {
        return Err(SogaNodeSaleError::MintLimitExceeded.into());
    }

    Ok(())
}

pub fn check_phase_buy(value: bool) -> Result<()> {
    if !value  {
        return Err(SogaNodeSaleError::PhaseBuyIsDisable.into());
    }

    Ok(())
}

pub fn check_phase_airdrop(value: bool) -> Result<()> {
    if !value  {
        return Err(SogaNodeSaleError::PhaseAirdropIsDisable.into());
    }

    Ok(())
}

pub fn check_phase_tier_buy(value: bool) -> Result<()> {
    if !value  {
        return Err(SogaNodeSaleError::PhaseTierBuyIsDisable.into());
    }

    Ok(())
}

pub fn check_phase_tier_airdrop(value: bool) -> Result<()> {
    if !value  {
        return Err(SogaNodeSaleError::PhaseTierAirdropIsDisable.into());
    }

    Ok(())
}

pub fn check_value_is_zero(value: usize) -> Result<()> {
    if value <= 0  {
        return Err(SogaNodeSaleError::ValueIsZero.into());
    }

    Ok(())
}

pub fn check_invalid_discount(full_value: u64, half_value: u64) -> Result<()> {

    let value: u64 = full_value + half_value;

    if value >= 100  {
        return Err(SogaNodeSaleError::InvalidDiscount.into());
    }

    Ok(())
}

