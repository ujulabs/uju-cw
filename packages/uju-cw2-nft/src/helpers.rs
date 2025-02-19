use cosmwasm_std::{ensure_eq, to_json_binary, Addr, QuerierWrapper, SubMsg, WasmMsg};
use uju_cw2_common::error::CommonError;

use crate::msg::{Cw721ExecuteMsg, Cw721QueryMsg, NumTokensResponse, OwnerOfResponse};

/// Invoke `transfer_nft` to build a `SubMsg` to transfer an NFT to an address.
pub fn transfer_nft(collection: &Addr, token_id: &str, recipient: &Addr) -> SubMsg {
    SubMsg::new(WasmMsg::Execute {
        contract_addr: collection.to_string(),
        msg: to_json_binary(&Cw721ExecuteMsg::TransferNft {
            token_id: token_id.to_string(),
            recipient: recipient.to_string(),
        })
        .unwrap(),
        funds: vec![],
    })
}

/// Invoke `only_owner` to check that the sender is the owner of the NFT.
pub fn only_owner(
    querier: &QuerierWrapper,
    sender: &Addr,
    collection: &Addr,
    token_id: &str,
) -> Result<(), CommonError> {
    let owner_of_response = querier.query_wasm_smart::<OwnerOfResponse>(
        collection.clone(),
        &Cw721QueryMsg::OwnerOf {
            token_id: token_id.to_string(),
            include_expired: Some(false),
        },
    );

    match owner_of_response {
        Ok(owner_of_response) => {
            ensure_eq!(
                owner_of_response.owner,
                sender.to_string(),
                CommonError::Unauthorized("sender is not owner".to_string())
            );

            Ok(())
        }
        Err(_) => Ok(()),
    }
}

/// Shallow validate a collection by checking that the num tokens query returns a valid response.
pub fn shallow_validate_collection(
    querier: &QuerierWrapper,
    collection: &Addr,
) -> Result<(), CommonError> {
    querier
        .query_wasm_smart::<NumTokensResponse>(collection.clone(), &Cw721QueryMsg::NumTokens {})
        .map_err(|_| CommonError::InvalidInput("invalid collection address".to_string()))?;

    Ok(())
}
