use anchor_lang::prelude::*;

#[error_code]
pub enum TransitError {
    #[msg("Too many couriers specified.")]
    TooManyCouriers,
    #[msg("Unauthorized courier for this operation.")]
    UnauthorizedCourier,
    #[msg("Confirmation not found.")]
    NoConfirmationFound,
    #[msg("Courier not found in profiles.")]
    CourierNotFound,
    #[msg("Internal error occurred.")]
    InternalError,
    #[msg("Invalid reward account.")]
    InvalidRewardAccount,
}
