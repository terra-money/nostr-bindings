use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

#[cw_serde]
pub struct UserInfo {
    pub nostr_addr: String,
    pub chain_addr: Addr,
    pub timestamp_registration: u64,
}

#[cw_serde]
pub struct SignInfo {
    /// 32-bytes hex encoded public key
    pub nostr_addr: String,
    /// 64-bytes lowercase hex of the signature of the sha256 hash of the info.sender as message
    pub nostr_signature: String,
}
