#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:nostr-binding";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bound {
            nostr_addr,
            nostr_signature,
        } => execute::bound(deps, info, nostr_addr, nostr_signature),
        ExecuteMsg::UnBound {
            nostr_addr,
            nostr_signature,
        } => execute::unbound(deps, info, nostr_addr, nostr_signature),
    }
}

pub mod execute {
    use super::*;

    pub fn bound(
        deps: DepsMut,
        info: MessageInfo,
        nostr_addr: String,
        nostr_signature: String,
    ) -> Result<Response, ContractError> {
        Ok(Response::new()
            .add_attribute("action", "bound"))
    }

    pub fn unbound(
        deps: DepsMut,
        info: MessageInfo,
        nostr_addr: String,
        nostr_signature: String,
    ) -> Result<Response, ContractError> {
        Ok(Response::new()
            .add_attribute("action", "unbound"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBoundedCounterpartyByNostrAddr(addr) => {
            to_binary(&query::get_by_address(deps, addr)?)
        }
        QueryMsg::GetBoundedCounterpartyByTerraAddr(addr) => {
            to_binary(&query::get_by_address(deps, addr)?)
        }
    }
}

pub mod query {
    use super::*;
    use crate::{msg::CounterpartyAddrRes, state::ADDRS};
    use cosmwasm_std::Order::Descending;
    use cosmwasm_std::{Addr, StdError};

    pub fn get_by_address(deps: Deps, addr: String) -> StdResult<CounterpartyAddrRes> {
        let found_addr: Addr;

        if addr.starts_with("terra") {
            let addr = deps.api.addr_validate(&addr)?;
            let addrs = ADDRS.may_load(deps.storage, addr.clone())?;

            if addrs.is_none() {
                return Err(StdError::not_found(addr.into_string()));
            }

            found_addr = addrs.unwrap();
        } else {
            let item = ADDRS
                .range(deps.storage, None, None, Descending)
                .find(|f| {
                    let (_, value) = f.as_ref().unwrap();

                    value == &addr
                });

            match item {
                Some(addr) => {
                    found_addr = addr.unwrap().0;
                }
                None => {
                    return Err(StdError::NotFound {
                        kind: "No counterparty address available".to_string(),
                    });
                }
            }
        }

        Ok(CounterpartyAddrRes {
            addr: found_addr.into_string(),
        })
    }
}
