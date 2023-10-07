use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult};
use rhaki_cw_plus::traits::IntoStdResult;

use crate::{
    definitions::{SignInfo, UserInfo},
    error::ContractError,
    state::ADDRS,
};

use secp256k1::{ecdsa::Signature, hashes::sha256, Message, PublicKey, Secp256k1};

pub fn bound(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sign_info: SignInfo,
) -> Result<Response, ContractError> {
    verify_ecdsa(
        &sign_info.nostr_addr,
        &sign_info.nostr_signature,
        &info.sender,
    )?;

    if ADDRS().has(deps.storage, info.sender.clone()) {
        return Err(ContractError::AddressAlredyBounded {
            address: info.sender,
        });
    }

    ADDRS().save(
        deps.storage,
        info.sender.clone(),
        &UserInfo {
            nostr_addr: sign_info.nostr_addr.clone(),
            chain_addr: info.sender.clone(),
            timestamp_registration: env.block.time.seconds(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "bound")
        .add_attribute("nostr_addr", sign_info.nostr_addr)
        .add_attribute("terra_addr", info.sender))
}

pub fn unbound(
    deps: DepsMut,
    info: MessageInfo,
    sign_info: SignInfo,
) -> Result<Response, ContractError> {
    verify_ecdsa(
        &sign_info.nostr_addr,
        &sign_info.nostr_signature,
        &info.sender,
    )?;

    ADDRS().remove(deps.storage, info.sender.clone())?;

    Ok(Response::new()
        .add_attribute("action", "unbound")
        .add_attribute("nostr_addr", sign_info.nostr_addr)
        .add_attribute("terra_addr", info.sender))
}

fn verify_ecdsa(pub_key: &str, sign: &str, sender_addr: &Addr) -> StdResult<()> {
    let sepc = Secp256k1::new();

    let message = Message::from_hashed_data::<sha256::Hash>(sender_addr.to_string().as_bytes());
    let sign = Signature::from_compact(&hex::decode(sign).into_std_result()?).into_std_result()?;
    let public_key =
        PublicKey::from_slice(&hex::decode(pub_key).into_std_result()?).into_std_result()?;

    sepc.verify_ecdsa(&message, &sign, &public_key)
        .into_std_result()?;

    Ok(())
}
