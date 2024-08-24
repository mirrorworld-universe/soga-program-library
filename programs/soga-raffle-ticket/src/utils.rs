use anchor_lang::prelude::*;

use crate::error::SogaRaffleTicketError;

pub fn check_main_signing_authority(main_signing_authority_from_account: Pubkey, main_signing_authority_from_input_accounts: Pubkey) -> Result<()> {
    if main_signing_authority_from_account != main_signing_authority_from_input_accounts {
        return Err(SogaRaffleTicketError::InvalidMainSigningAuthority.into());
    }

    Ok(())
}

pub fn check_signing_authority(signing_authority_from_account: Pubkey, signing_authority_from_input_accounts: Pubkey) -> Result<()> {
    if signing_authority_from_account != signing_authority_from_input_accounts {
        return Err(SogaRaffleTicketError::InvalidSigningAuthority.into());
    }

    Ok(())
}

pub fn check_value_is_zero(value: usize) -> Result<()> {
    if value <= 0 {
        return Err(SogaRaffleTicketError::ValueIsZero.into());
    }

    Ok(())
}

pub fn check_is_ticket_purchase_enable(enable: bool) -> Result<()> {
    if !enable {
        return Err(SogaRaffleTicketError::TicketPurchaseIsDisable.into());
    }

    Ok(())
}

pub fn check_is_ticket_refund_enable(enable: bool) -> Result<()> {
    if !enable {
        return Err(SogaRaffleTicketError::TicketRefundIsDisable.into());
    }

    Ok(())
}

pub fn check_is_payment_enable(enable: bool) -> Result<()> {
    if !enable {
        return Err(SogaRaffleTicketError::PaymentIsDisable.into());
    }

    Ok(())
}

pub fn check_is_payment_ticket_purchase_enable(enable: bool) -> Result<()> {
    if !enable {
        return Err(SogaRaffleTicketError::PaymentTicketPurchaseIsDisable.into());
    }

    Ok(())
}

pub fn check_is_payment_ticket_refund_enable(enable: bool) -> Result<()> {
    if !enable {
        return Err(SogaRaffleTicketError::PaymentTicketRefundIsDisable.into());
    }

    Ok(())
}

pub fn check_payment_supply(from_account: u64, from_param: u64) -> Result<()> {
    if from_param > from_account {
        return Err(SogaRaffleTicketError::InvalidPaymentSupply.into());
    }

    Ok(())
}

pub fn check_valid_ticket_winner_limit(from_account: u64, from_param: u64) -> Result<()> {
    if from_account > from_param {
        return Err(SogaRaffleTicketError::InvalidTicketWinnerLimit.into());
    }

    Ok(())
}

pub fn check_exceed_ticket_winner_limit(from_account: u64, from_param: u64) -> Result<()> {
    if from_account < from_param {
        return Err(SogaRaffleTicketError::ExceedTicketWinnerLimit.into());
    }

    Ok(())
}

pub fn check_user_ticket_quantity(from_account: u64, from_param: u64) -> Result<()> {
    if from_account < from_param {
        return Err(SogaRaffleTicketError::InvalidUserTicketQuantity.into());
    }

    Ok(())
}

pub fn check_ticket_claim(from_account: u64, from_param: u64) -> Result<()> {
    if from_account < from_param {
        return Err(SogaRaffleTicketError::InvalidTicketClaim.into());
    }

    Ok(())
}

