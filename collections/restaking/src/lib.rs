use consts::{MPSOL_MINT_ADDRESS, MSOL_MINT_ADDRESS};
use errors::ActionError;
use instructions::{deposit_transaction, restake_ix};
use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::str::FromStr;
use utils::get_account_balance;

use znap::prelude::*;

mod consts;
mod errors;
mod instructions;
mod utils;

#[collection]
pub mod restaking {

    use solana_sdk;

    use super::*;

    pub fn restake(ctx: Context<RestakingAction>) -> Result<ActionTransaction> {
        let account_pubkey = Pubkey::from_str(&ctx.payload.account)
            .or_else(|_| Err(Error::from(ActionError::InvalidAccountPubkey)))?;

        let create_msol_ata_ix = create_associated_token_account_idempotent(
            &account_pubkey,
            &account_pubkey,
            &MSOL_MINT_ADDRESS,
            &TOKEN_PROGRAM_ID,
        );

        let create_mpsol_ata_ix = create_associated_token_account_idempotent(
            &account_pubkey,
            &account_pubkey,
            &MPSOL_MINT_ADDRESS,
            &TOKEN_PROGRAM_ID,
        );

        let msol_ata = get_associated_token_address(&account_pubkey, &MSOL_MINT_ADDRESS);

        let mpsol_ata = get_associated_token_address(&account_pubkey, &MPSOL_MINT_ADDRESS);

        let account_balance = get_account_balance(&account_pubkey, &ctx.env.rpc_url).await?;

        if account_balance < ctx.query.amount {
            Err(Error::from(ActionError::InsufficientFunds))
        } else {
            let amount = (ctx.query.amount * (LAMPORTS_PER_SOL as f32)) as u64;
            let lst_amount = ((ctx.query.amount * 0.2) * (LAMPORTS_PER_SOL as f32)) as u64;

            let deposit_ix = deposit_transaction(amount, account_pubkey, msol_ata);
            let stake_ix = restake_ix(lst_amount, 2, account_pubkey, msol_ata, mpsol_ata);

            let instructions = vec![
                create_msol_ata_ix,
                create_mpsol_ata_ix,
                deposit_ix,
                stake_ix,
            ];

            let message = Message::new(&instructions, None);
            let transaction = Transaction::new_unsigned(message);

            Ok(ActionTransaction {
                transaction,
                message: Some("Restake successfully made!".to_string()),
            })
        }
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
        href = "/api/restaking?amount={amount}&method=restake",
        parameter = { label = "Amount", name = "amount"  }
    },
    // link = {
    //     label = "Unrestake",
    //     href = "/api/restaking?amount={amount}&method=unrestake",
    //     parameter = { label = "Amount", name = "amount"  }
    // }
)]
#[query(amount: f32, method: String)]
pub struct RestakingAction;
