use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg { }

#[cw_serde]
pub enum ExecuteMsg {
    Bound {
        nostr_addr: String,
        nostr_signature: String,
    },
    UnBound {
        nostr_addr: String,
        nostr_signature: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CounterpartyAddrRes)]
    GetBoundedCounterpartyByNostrAddr(String),
    
    #[returns(CounterpartyAddrRes)]
    GetBoundedCounterpartyByTerraAddr(String),
}

#[cw_serde]
pub struct CounterpartyAddrRes {
    pub addr: String,
}
