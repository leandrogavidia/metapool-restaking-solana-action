use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use znap::prelude::*;

use crate::errors::ActionError;

pub async fn get_account_balance(pubkey: &Pubkey, rpc: &str) -> Result<f32> {
    let client = Client::new();

    let req: Value = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "getBalance",
        "params": [pubkey.to_string()]
    });

    let res = client
        .post(rpc)
        .header("Accept", "application/json")
        .json(&req)
        .send()
        .await
        .or_else(|_| Err(Error::from(ActionError::GettingTokenAccountBalance)))?;

    if res.status() == StatusCode::OK {
        let res_json = res
            .json::<Value>()
            .await
            .or_else(|_| Err(Error::from(ActionError::ParsingHeliusResponse)))?;

        let res_amount = res_json["result"]["value"].as_u64().unwrap();

        let amount: f32 = res_amount as f32 / LAMPORTS_PER_SOL as f32;

        Ok(amount)
    } else {
        Err(Error::from(ActionError::ServerError))
    }
}

pub async fn get_token_account_balance(pubkey: &Pubkey, rpc: &str) -> Result<f32> {
    let client = Client::new();

    let req: Value = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "getTokenAccountBalance",
        "params": [pubkey.to_string()]
    });

    let res = client
        .post(rpc)
        .header("Accept", "application/json")
        .json(&req)
        .send()
        .await
        .or_else(|_| Err(Error::from(ActionError::GettingTokenAccountBalance)))?;

    if res.status() == StatusCode::OK {
        let res_json = res
            .json::<Value>()
            .await
            .or_else(|_| Err(Error::from(ActionError::ParsingHeliusResponse)))?;
        let amount = res_json["result"]["value"]["uiAmount"]
            .as_str()
            .unwrap()
            .parse::<f32>()
            .unwrap();
        Ok(amount)
    } else {
        Err(Error::from(ActionError::ServerError))
    }
}
