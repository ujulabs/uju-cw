use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Attribute, Response, StdError, StdResult, Storage};
use cw_storage_plus::Item;
use sylvia::{
    interface, schemars,
    types::{ExecCtx, InstantiateCtx, QueryCtx},
};

use crate::error::OwnableError;

pub const OWNERSHIP_KEY: &str = "O";

#[cw_serde]
pub struct Ownership<T> {
    pub owner: Option<T>,
    pub pending_owner: Option<T>,
    pub pending_expiry: Option<Expiration>,
}

#[cw_serde]
pub enum Expiration {
    AtHeight(u64),
    AtTime(u64),
    Never {},
}

#[cw_serde]
pub enum Action {
    TransferOwnership {
        new_owner: String,
        expiry: Option<Expiration>,
    },
    AcceptOwnership,
    RenounceOwnership,
}

#[interface]
pub trait Ownable {
    type Error: From<StdError> + From<OwnableError>;

    #[sv::msg(exec)]
    fn update_ownership(&self, ctx: ExecCtx, action: Action) -> Result<Response, Self::Error> {
        let ownership_item: Item<Ownership<Addr>> = Item::new(OWNERSHIP_KEY);
        let mut ownership = ownership_item.load(ctx.deps.storage)?;

        match action {
            Action::TransferOwnership { new_owner, expiry } => {
                self.assert_owner(ctx.deps.storage, &ctx.info.sender)?;
                ownership.pending_owner = Some(ctx.deps.api.addr_validate(&new_owner)?);
                ownership.pending_expiry = expiry;
            }
            Action::AcceptOwnership => {
                if ownership.pending_owner != Some(ctx.info.sender.clone()) {
                    return Err(OwnableError::NotPendingOwner.into());
                }
                if let Some(expiry) = ownership.pending_expiry {
                    if expiry.is_expired(&ctx.env.block) {
                        return Err(OwnableError::TransferExpired.into());
                    }
                }
                ownership.owner = ownership.pending_owner.take();
                ownership.pending_expiry = None;
            }
            Action::RenounceOwnership => {
                self.assert_owner(ctx.deps.storage, &ctx.info.sender)?;
                ownership.owner = None;
                ownership.pending_owner = None;
                ownership.pending_expiry = None;
            }
        }

        ownership_item.save(ctx.deps.storage, &ownership)?;
        Ok(Response::new().add_attributes(ownership.into_attributes()))
    }

    #[sv::msg(query)]
    fn get_ownership(&self, ctx: QueryCtx) -> StdResult<Ownership<Addr>> {
        let ownership_item: Item<Ownership<Addr>> = Item::new(OWNERSHIP_KEY);
        ownership_item.load(ctx.deps.storage)
    }

    #[sv::msg(query)]
    fn is_owner(&self, ctx: QueryCtx, addr: Addr) -> StdResult<bool> {
        let ownership = self.get_ownership(ctx)?;
        Ok(ownership.owner == Some(addr))
    }

    fn initialize_owner(
        &self,
        ctx: InstantiateCtx,
        owner: Option<Addr>,
    ) -> Result<(), OwnableError> {
        let ownership_item: Item<Ownership<Addr>> = Item::new(OWNERSHIP_KEY);
        let ownership = Ownership {
            owner,
            pending_owner: None,
            pending_expiry: None,
        };
        ownership_item.save(ctx.deps.storage, &ownership)?;
        Ok(())
    }

    fn assert_owner(&self, storage: &dyn Storage, sender: &Addr) -> Result<(), OwnableError> {
        let ownership_item: Item<Ownership<Addr>> = Item::new(OWNERSHIP_KEY);
        let ownership = ownership_item.load(storage)?;

        match ownership.owner {
            Some(owner) if owner == *sender => Ok(()),
            Some(_) => Err(OwnableError::NotOwner),
            None => Err(OwnableError::NoOwner),
        }
    }
}

impl<T: Clone + PartialEq + AsRef<str>> Ownership<T> {
    pub fn into_attributes(self) -> Vec<Attribute> {
        vec![
            Attribute::new("owner", self.owner.as_ref().map_or("none", AsRef::as_ref)),
            Attribute::new(
                "pending_owner",
                self.pending_owner.as_ref().map_or("none", AsRef::as_ref),
            ),
            Attribute::new(
                "pending_expiry",
                self.pending_expiry
                    .map_or("none".to_string(), |e| format!("{:?}", e)),
            ),
        ]
    }
}

impl Expiration {
    pub fn is_expired(&self, block: &sylvia::cw_std::BlockInfo) -> bool {
        match self {
            Expiration::AtHeight(height) => block.height >= *height,
            Expiration::AtTime(time) => block.time.seconds() >= *time,
            Expiration::Never {} => false,
        }
    }
}
