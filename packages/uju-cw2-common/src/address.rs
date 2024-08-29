use cosmwasm_std::Addr;

pub fn address_or(default: &Addr, addr: Option<&Addr>) -> Addr {
    addr.map_or(default.clone(), |addr| addr.clone())
}
