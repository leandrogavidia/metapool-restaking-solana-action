use consts::{MPSOL_MINT_ADDRESS, MSOL_MINT_ADDRESS, BSOL_MINT_ADDRESS};
use errors::ActionError;
use instructions::{deposit_ix, restake_ix};
use solana_sdk::{
    instruction::Instruction, message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, transaction::Transaction,
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
        let entry_amount = ctx.query.amount.clone();
        let lamports_amount = (entry_amount * (LAMPORTS_PER_SOL as f32)) as u64;

        let rpc = ctx.env.rpc_url.clone();
        
        let mpsol_ata = get_associated_token_address(&account_pubkey, &MPSOL_MINT_ADDRESS);
        
        let mut instructions: Vec<Instruction> = vec![];

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
                let sol_balance = get_account_balance(&account_pubkey, &rpc).await?;
    
                if sol_balance < entry_amount {
                    return Err(Error::from(ActionError::InsufficientFunds));
                } else {
                    let msol_amount = ((entry_amount * 0.2) * (LAMPORTS_PER_SOL as f32)) as u64;
                    let deposit_ix = deposit_ix(lamports_amount, account_pubkey, msol_ata);
                    let stake_ix = restake_ix(msol_amount, 2, account_pubkey, msol_ata, mpsol_ata, &token);
    
                    instructions.extend_from_slice(&[
                        create_msol_ata_ix,
                        create_mpsol_ata_ix,
                        deposit_ix,
                        stake_ix,
                    ]);
                }
            } else if token == "msol" {
                let msol_ata = get_associated_token_address(&account_pubkey, &MSOL_MINT_ADDRESS);
                let msol_balance = get_token_account_balance(&msol_ata, &rpc).await?;

                if msol_balance < entry_amount {
                    return Err(Error::from(ActionError::InsufficientFunds));
                } else {
                    let stake_ix = restake_ix(lamports_amount, 2, account_pubkey, msol_ata, mpsol_ata, &token);

                    instructions.extend_from_slice(&[
                        create_mpsol_ata_ix,
                        stake_ix,
                    ]);
                }
            } else if token == "bsol" {
                let bsol_ata = get_associated_token_address(&account_pubkey, &BSOL_MINT_ADDRESS);
                let bsol_balance = get_token_account_balance(&bsol_ata, &rpc).await?;

                if bsol_balance < entry_amount {
                    return Err(Error::from(ActionError::InsufficientFunds));
                } else {
                    let stake_ix = restake_ix(lamports_amount, 2, account_pubkey, bsol_ata, mpsol_ata, &token);
                    instructions.extend_from_slice(&[
                        create_mpsol_ata_ix,
                        stake_ix,
                    ]);
                }
            }
        } else {
            Err(Error::from(ActionError::InvalidMethod))?
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
        href = "/api/restaking?amount={amount}&token={token}&method={method}",
        parameter = { 
            label = "Token", 
            name = "token", 
            required = true,
            type = "select", 
            option = {
                label = "SOL",
                value = "sol"
            },
            option = {
                label = "mSOL",
                value = "msol"
            },
            option = {
                label = "BSOL",
                value = "bsol"
            }
        },
        parameter = { 
            label = "Method", 
            name = "method", 
            required = true,
            type = "radio", 
            option = {
                label = "Restake",
                value = "restake"
            }  
        },
        parameter = { 
            label = "Amount", 
            name = "amount", 
            required = true,
            type = "number", 
        }
    },
)]
#[query(amount: f32, token: String, method: String)]
pub struct RestakingAction;
