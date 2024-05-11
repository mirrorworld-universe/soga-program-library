use anchor_lang::prelude::*;

#[error_code]
pub enum SogaNodeSaleError {
    #[msg("Invalid main signing authority")]
    InvalidMainSigningAuthority,

    #[msg("Invalid signing authority")]
    InvalidSigningAuthority,

    #[msg("Invalid tier id")]
    InvalidTierId,

    #[msg("Tier id out of range")]
    TierIdOutOfRange,

    #[msg("Invalid price feed address")]
    InvalidPriceFeedAddress,

    #[msg("Invalid payment receiver address")]
    InvalidPaymentReceiverAddress,

    #[msg("Invalid phase tier collection address")]
    InvalidPhaseTierCollectionAddress,

    #[msg("Phase tier is completed")]
    PhaseTierIsCompleted,

    #[msg("Invalid token id")]
    InvalidTokenId,

    #[msg("token id out of range")]
    TokenIdOutOfRange,

    #[msg("token quantity out of range")]
    TokenQuantityOutOfRange,

    #[msg("Mint limit exceeded")]
    MintLimitExceeded,

    #[msg("Phase buy is disable")]
    PhaseBuyIsDisable,

    #[msg("Phase buy with token is disable")]
    PhaseBuyWithTokenIsDisable,

    #[msg("Phase airdrop is disable")]
    PhaseAirdropIsDisable,

    #[msg("Phase tier buy is disable")]
    PhaseTierBuyIsDisable,

    #[msg("Phase tier buy is disable")]
    PhaseTierBuyWithTokenIsDisable,

    #[msg("Phase tier airdrop is disable")]
    PhaseTierAirdropIsDisable,

    #[msg("Value is zero")]
    ValueIsZero,

    #[msg("Invalid discount")]
    InvalidDiscount,

    #[msg("Invalid payment token mint account")]
    InvalidPaymentTokenMintAccount,

    #[msg("Payment token is disable")]
    PaymentTokenIsDisable,

    #[msg("Invalid quantity")]
    InvalidQuantity,

    #[msg("Invalid order token id")]
    InvalidOrderTokenId,

    #[msg("Order token id filled")]
    OrderTokenIdFilled,

    #[msg("Order is filled")]
    OrderIsFilled,
}