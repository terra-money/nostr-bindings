use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use rhaki_cw_plus::traits::IntoBinary;

use crate::definitions::Config;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::CONFIG;
use crate::{execute, query};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:nostr-binding";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(
        deps.storage,
        &Config {
            owner: info.sender.clone(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bound(sign_info) => execute::bound(deps, env, info, sign_info),
        ExecuteMsg::Unbound(sign_info) => execute::unbound(deps, info, sign_info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUserInfoByNostrAddr { nostr_addr } => {
            query::qy_get_by_nostr_addr(deps, nostr_addr).into_binary()
        }
        QueryMsg::GetUserInfoByTerraAddr { terra_addr } => {
            query::qy_get_by_terra_addr(deps, terra_addr).into_binary()
        }
        QueryMsg::GetUsers { limit, start_after } => {
            query::qy_get_users(deps, limit, start_after).into_binary()
        }
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new())
}
