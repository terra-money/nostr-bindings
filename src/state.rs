use cosmwasm_std::Addr;
use cw_storage_plus::{index_list, IndexedMap, Item, UniqueIndex};

pub type TerraAddr = Addr;

use crate::definitions::{Config, UserInfo};

pub const CONFIG: Item<Config> = Item::new("config_key");

#[index_list(UserInfo)]
pub struct UserInfoIndexes<'a> {
    pub nostr_addr: UniqueIndex<'a, String, UserInfo, Addr>,
}

#[allow(non_snake_case)]
pub fn ADDRS<'a>() -> IndexedMap<'a, TerraAddr, UserInfo, UserInfoIndexes<'a>> {
    let indexes = UserInfoIndexes {
        nostr_addr: UniqueIndex::new(|val| val.nostr_addr.clone(), "ns_addrs_nostr_addr"),
    };

    IndexedMap::new("ns_addrs", indexes)
}
