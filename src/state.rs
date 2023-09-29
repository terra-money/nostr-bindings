use cosmwasm_std::Addr;
use cw_storage_plus::Map;

// TERRA Address, NOSTR Address
pub const ADDRS: Map<Addr, Addr> = Map::new("addrs");
