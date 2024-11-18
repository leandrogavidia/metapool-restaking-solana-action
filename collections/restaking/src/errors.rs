use znap::prelude::*;

#[derive(ErrorCode)]
pub enum ActionError {
    #[error(msg  = "Invalid account public key")]
    InvalidAccountPubkey,
    #[error(msg  = "Insufficient funds")]
    InsufficientFunds,
    #[error(msg  = "Error getting token account balance")]
    GettingTokenAccountBalance,
    #[error(msg  = "Error parsing Helius Response")]
    ParsingHeliusResponse,
    #[error(msg  = "ServerError")]
    ServerError,
    #[error(msg  = "Method is not valid")]
    InvalidMethod,
}