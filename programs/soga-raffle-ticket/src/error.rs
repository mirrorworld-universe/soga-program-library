use anchor_lang::prelude::*;

#[error_code]
pub enum SogaRaffleTicketError {
    #[msg("Invalid main signing authority")]
    InvalidMainSigningAuthority,

    #[msg("Invalid signing authority")]
    InvalidSigningAuthority,

    #[msg("Value is zero")]
    ValueIsZero,

    #[msg("Ticket purchase is disable")]
    TicketPurchaseIsDisable,

    #[msg("Ticket refund is disable")]
    TicketRefundIsDisable,

    #[msg("Payment is disable")]
    PaymentIsDisable,

    #[msg("Payment ticket purchase is disable")]
    PaymentTicketPurchaseIsDisable,

    #[msg("Payment ticket refund is disable")]
    PaymentTicketRefundIsDisable,

    #[msg("Invalid payment supply")]
    InvalidPaymentSupply,

    #[msg("Invalid ticket winner limit")]
    InvalidTicketWinnerLimit,

    #[msg("Exceed ticket winner limit")]
    ExceedTicketWinnerLimit,

    #[msg("Invalid user ticket quantity")]
    InvalidUserTicketQuantity,

    #[msg("Invalid ticket claim")]
    InvalidTicketClaim,
}