use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OwnableError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Contract ownership has been renounced")]
    NoOwner,

    #[error("Caller is not the contract's current owner")]
    NotOwner,

    #[error("Caller is not the contract's pending owner")]
    NotPendingOwner,

    #[error("There isn't a pending ownership transfer")]
    TransferNotFound,

    #[error("A pending ownership transfer exists but it has expired")]
    TransferExpired,
}
