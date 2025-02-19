use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_utils::Expiration;

#[cw_serde]
pub enum Cw721ExecuteMsg {
    TransferNft { token_id: String, recipient: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw721QueryMsg {
    #[returns(OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Total number of tokens issued
    #[returns(NumTokensResponse)]
    NumTokens {},
}

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the token
    pub owner: String,
    /// If set this address is approved to transfer/send the token as well
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct NumTokensResponse {
    pub count: u64,
}
