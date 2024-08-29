use cosmwasm_std::{ensure_eq, Env, MessageInfo, QuerierWrapper};

use crate::error::CommonError;

pub fn only_contract_admin(
    querier: &QuerierWrapper,
    info: &MessageInfo,
    env: &Env,
) -> Result<(), CommonError> {
    let contract_info_resp = querier.query_wasm_contract_info(&env.contract.address)?;

    let admin = contract_info_resp
        .admin
        .ok_or_else(|| CommonError::Unauthorized("contract admin unset".to_string()))?;

    ensure_eq!(
        info.sender,
        admin,
        CommonError::Unauthorized("only the admin of contract can perform this action".to_string())
    );

    Ok(())
}
