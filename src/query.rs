use cosmwasm_std::{Deps, Order, StdResult};
use rhaki_cw_plus::traits::IntoAddr;

use crate::{definitions::UserInfo, state::ADDRS};

pub fn qy_get_by_terra_addr(deps: Deps, addr: String) -> StdResult<UserInfo> {
    ADDRS().load(deps.storage, addr.into_addr(deps.api)?)
}

pub fn qy_get_by_nostr_addr(deps: Deps, addr: String) -> StdResult<UserInfo> {
    rhaki_cw_plus::storage::multi_index::get_unique_value(
        deps.storage,
        addr,
        ADDRS().idx.nostr_addr,
    )
    .map(|(_, v)| v)
}

pub fn qy_get_users(
    deps: Deps,
    limit: Option<u32>,
    start_after: Option<String>,
) -> StdResult<Vec<UserInfo>> {
    rhaki_cw_plus::storage::multi_index::get_items(
        deps.storage,
        ADDRS(),
        Order::Ascending,
        limit,
        start_after.map(|addr| addr.into_addr(deps.api).unwrap()),
    )
    .map(|res| res.into_iter().map(|(_, v)| v).collect())
}
