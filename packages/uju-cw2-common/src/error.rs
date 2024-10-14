use cosmwasm_std::{Instantiate2AddressError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CommonError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Instantiate2AddressError(#[from] Instantiate2AddressError),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("InvalidInput: {0}")]
    InvalidInput(String),

    #[error("InsufficientFunds: {0}")]
    InsufficientFunds(String),

    #[error("InternalError: {0}")]
    InternalError(String),

    #[error("MigrationError: {0}")]
    MigrationError(String),
}
