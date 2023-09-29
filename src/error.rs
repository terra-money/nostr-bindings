use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("No counterparty address available for {0}")]
    PairNotFound(String),

    #[error("Unauthorized")]
    Unauthorized {},
}
