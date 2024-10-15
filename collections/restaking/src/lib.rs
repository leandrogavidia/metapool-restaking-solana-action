use errors::ActionError;
use solana_sdk::{ pubkey, pubkey::Pubkey, message::Message, transaction::Transaction };
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use std::str::FromStr;
use znap::prelude::*;

mod errors;

const MPSOL_MINT_ADDRESS: Pubkey = pubkey!("mPsoLV53uAGXnPJw63W91t2VDqCVZcU5rTh3PWzxnLr");
const MSOL_MINT_ADDRESS: Pubkey = pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");

#[collection]
pub mod restaking {


    use super::*;

    pub fn restake(ctx: Context<RestakingAction>) -> Result<ActionTransaction> {
        let account_pubkey = Pubkey::from_str(&ctx.payload.account)
            .or_else(|_| Err(Error::from(ActionError::InvalidAccountPubkey)))?;
    
        let create_mpsol_ata_instruction = create_associated_token_account_idempotent(
            &account_pubkey,
            &account_pubkey,
            &MPSOL_MINT_ADDRESS,
            &TOKEN_PROGRAM_ID,
        );

        let create_msol_ata_instruction = create_associated_token_account_idempotent(
            &account_pubkey,
            &account_pubkey,
            &MSOL_MINT_ADDRESS,
            &TOKEN_PROGRAM_ID,
        );

        let message = Message::new(&[], None);
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
        href = "/api/restaking?amount={amount}&method=restake",
        parameter = { label = "Amount", name = "amount"  }
    },
    link = {
        label = "Unrestake",
        href = "/api/restaking?amount={amount}&method=unrestake",
        parameter = { label = "Amount", name = "amount"  }
    }
)]
#[query(amount: f32, method: String)]
pub struct RestakingAction;
