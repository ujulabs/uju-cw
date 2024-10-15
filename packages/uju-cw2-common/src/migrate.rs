use cosmwasm_std::{ensure, ensure_eq};
use cw2::{get_contract_version, set_contract_version, ContractVersion};
use semver::Version;
use sylvia::types::MigrateCtx;

use crate::error::CommonError;

pub fn handle_migration(
    ctx: MigrateCtx,
    next_name: &str,
    next_version: &str,
) -> Result<ContractVersion, CommonError> {
    let next_contract_version = ContractVersion {
        contract: next_name.to_string(),
        version: next_version.to_string(),
    };

    let prev_contract_version = get_contract_version(ctx.deps.storage)?;

    ensure_eq!(
        prev_contract_version.contract,
        next_contract_version.contract,
        CommonError::MigrationError("Invalid contract name for migration".to_string())
    );

    ensure!(
        Version::parse(&prev_contract_version.version).unwrap()
            < Version::parse(&next_contract_version.version).unwrap(),
        CommonError::MigrationError("Must upgrade contract version".to_string())
    );

    set_contract_version(
        ctx.deps.storage,
        &next_contract_version.contract,
        &next_contract_version.version,
    )?;

    Ok(next_contract_version)
}
