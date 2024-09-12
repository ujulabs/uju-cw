use cosmwasm_std::{instantiate2_address, Addr, Binary, Deps};
use sha2::{Digest, Sha256};

use crate::error::CommonError;

pub fn generate_salt(data: Vec<impl AsRef<[u8]>>) -> Binary {
    let mut hasher = Sha256::new();
    for item in data {
        hasher.update(item.as_ref());
    }
    hasher.finalize().to_vec().into()
}

pub fn generate_instantiate_2_addr(
    deps: &Deps,
    contract_address: &Addr,
    code_id: u64,
    salt: &Binary,
) -> Result<Addr, CommonError> {
    let code_res = deps.querier.query_wasm_code_info(code_id)?;
    let creator = deps.api.addr_canonicalize(contract_address.as_str())?;
    let addr_raw = instantiate2_address(&code_res.checksum.as_slice(), &creator, salt)?;
    let addr = deps.api.addr_humanize(&addr_raw)?;
    Ok(addr)
}
