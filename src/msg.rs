use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::definitions::{SignInfo, UserInfo};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Bound(SignInfo),
    Unbound(SignInfo),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserInfo)]
    GetUserInfoByNostrAddr { nostr_addr: String },

    #[returns(UserInfo)]
    GetUserInfoByTerraAddr { terra_addr: String },

    #[returns(Vec<UserInfo>)]
    GetUsers {
        limit: Option<u32>,
        start_after: Option<String>,
    },
}

pub struct MigrateMsg {}
