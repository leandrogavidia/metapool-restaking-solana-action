use consts::{MPSOL_MINT_ADDRESS, MSOL_MINT_ADDRESS};
use errors::ActionError;
use instructions::{deposit_transaction_msol, restake_ix, unrestake_ix};
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signature::Keypair, signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::str::FromStr;
use utils::{get_account_balance, get_token_account_balance};
use znap::prelude::*;

mod consts;
mod errors;
mod instructions;
mod utils;

#[collection]
pub mod restaking {
    use super::*;

    pub fn restaking(ctx: Context<RestakingAction>) -> Result<ActionTransaction> {
        let account_pubkey = Pubkey::from_str(&ctx.payload.account)
            .or_else(|_| Err(Error::from(ActionError::InvalidAccountPubkey)))?;

        let method = ctx.query.method.clone();
        let token = ctx.query.token.clone();
        let mpsol_ata = get_associated_token_address(&account_pubkey, &MPSOL_MINT_ADDRESS);
        let amount = (ctx.query.amount * (LAMPORTS_PER_SOL as f32)) as u64;
        let rpc = ctx.env.rpc_url.clone();
        let mut instructions: Vec<Instruction> = vec![];
        let new_ticket_account = Keypair::new();

        if method == "restake" {
            let create_mpsol_ata_ix = create_associated_token_account_idempotent(
                &account_pubkey,
                &account_pubkey,
                &MPSOL_MINT_ADDRESS,
                &TOKEN_PROGRAM_ID,
            );

            if token == "sol" {
                let create_msol_ata_ix = create_associated_token_account_idempotent(
                    &account_pubkey,
                    &account_pubkey,
                    &MSOL_MINT_ADDRESS,
                    &TOKEN_PROGRAM_ID,
                );
    
                let msol_ata = get_associated_token_address(&account_pubkey, &MSOL_MINT_ADDRESS);
    
                let account_balance = get_account_balance(&account_pubkey, &rpc).await?;
    
                if account_balance < ctx.query.amount {
                    return Err(Error::from(ActionError::InsufficientFunds));
                } else {
                    let lst_amount = ((ctx.query.amount * 0.2) * (LAMPORTS_PER_SOL as f32)) as u64;
                    let deposit_ix = deposit_transaction_msol(amount, account_pubkey, msol_ata);
                    let stake_ix = restake_ix(lst_amount, 2, account_pubkey, msol_ata, mpsol_ata);
    
                    instructions.extend_from_slice(&[
                        create_msol_ata_ix,
                        create_mpsol_ata_ix,
                        deposit_ix,
                        stake_ix,
                    ]);
                }
            } else if token == "msol" {
                let msol_ata = get_associated_token_address(&account_pubkey, &MSOL_MINT_ADDRESS);
                let current_lst_amount = get_token_account_balance(&msol_ata, &rpc).await?;

                if current_lst_amount < ctx.query.amount {
                    return Err(Error::from(ActionError::InsufficientFunds));
                } else {
                    let stake_ix = restake_ix(amount, 2, account_pubkey, msol_ata, mpsol_ata);

                    instructions.extend_from_slice(&[
                        create_mpsol_ata_ix,
                        stake_ix,
                    ]);
                }
            }

        } else {
            let current_lst_amount = get_token_account_balance(&mpsol_ata, &rpc).await?;
            if current_lst_amount < ctx.query.amount {
                return Err(Error::from(ActionError::InsufficientFunds));
            } else {
                instructions.push(unrestake_ix(
                    amount,
                    account_pubkey,
                    mpsol_ata,
                    new_ticket_account.pubkey(),
                ));
            }
        }

        let message = Message::new(&instructions, Some(&account_pubkey));
        let transaction = Transaction::new_unsigned(message);

        Ok(ActionTransaction {
            transaction,
            message: Some("Restake successfully made!".to_string()),
        })
    }
}

#[derive(Action)]
#[action(
    icon = "https://raw.githubusercontent.com/leandrogavidia/files/refs/heads/main/metapool-restaking.png",
    title = "Restake Aggregator",
    description = "Solana Restake Aggregator. To restake your LST tokens in Solana, such as mSOL, jitoSOL, bSOL, and receive mpSOL.",
    label = "Restake",
    link = {
        label = "Restake",
        href = "/api/restaking?amount={amount}&token={token}&method={restake}",
        parameter = { 
            label = "Token", 
            name = "token", 
            type = "select", 
            option = {
                label = "SOL",
                value = "sol"
            },
            option = {
                label = "mSOL",
                value = "msol"
            },
            // option = {
            //     label = "BSOL",
            //     value = "bsol"
            // },
            // option = {
            //     label = "JitoSOL",
            //     value = "jitosol"
            // }
        },
        parameter = { 
            label = "Method", 
            name = "method", 
            type = "radio", 
            option = {
                label = "Restake",
                value = "restake"
            }  
        },
        parameter = { 
            label = "Amount", 
            name = "amount", 
            type = "number"  
        }
    },
)]
#[query(amount: f32, token: String, method: String)]
pub struct RestakingAction;
