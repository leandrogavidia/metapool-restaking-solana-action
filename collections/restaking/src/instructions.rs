use crate::consts::{
    DEPOSIT_DISCRIMINATOR, MAIN_STATE, MARINADE_FINANCE_PROGRAM, MARINADE_LIQUID_STAKING_STATE,
    MARINADE_MSOL_LEG_ACCOUNT, MARINADE_MSOL_LEG_AUTHORITY, MARINADE_MSOL_MINT_AUTHORITY,
    MARINADE_RESERVE_SOL_PDA, MARINADE_SOL_LEG_ACCOUNT, MPSOL_MINT_ADDRESS, MPSOL_MINT_AUTHORITY,
    MP_RESTAKIN_PROGRAM, MSOL_MINT_ADDRESS, STAKE_DISCRIMINATOR, UNSTAKE_DISCRIMINATOR,
    VAULT_ATA_PDA_AUTH, VAULT_LST_ACCOUNT, VAULT_STATE, REFERRAL_STATE, MSOL_TOKEN_PARTNER_ACCOUNT
};
use borsh::{BorshDeserialize, BorshSerialize};
use chrono::prelude::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program::ID as SYSTEM_PROGRAM_PUBKEY,
};
use spl_token::ID as TOKEN_PROGRAM_PUBKEY;

pub fn deposit_transaction_msol(lamports: u64, from_pubkey: Pubkey, msol_ata: Pubkey) -> Instruction {
    let args = DepositInstructionArgs { lamports };

    let accounts = vec![
        AccountMeta::new(MARINADE_LIQUID_STAKING_STATE, false),
        AccountMeta::new(MSOL_MINT_ADDRESS, false),
        AccountMeta::new(MARINADE_SOL_LEG_ACCOUNT, false),
        AccountMeta::new(MARINADE_MSOL_LEG_ACCOUNT, false),
        AccountMeta::new_readonly(MARINADE_MSOL_LEG_AUTHORITY, false),
        AccountMeta::new(MARINADE_RESERVE_SOL_PDA, false),
        AccountMeta::new(from_pubkey, true),
        AccountMeta::new(msol_ata, false),
        AccountMeta::new_readonly(MARINADE_MSOL_MINT_AUTHORITY, false),
        AccountMeta::new_readonly(SYSTEM_PROGRAM_PUBKEY, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM_PUBKEY, false),
        // AccountMeta::new_readonly(MARINADE_FINANCE_PROGRAM, false),
        // AccountMeta::new(REFERRAL_STATE, false),
        // AccountMeta::new(MSOL_TOKEN_PARTNER_ACCOUNT, false),
    ];

    Instruction::new_with_borsh(
        MARINADE_FINANCE_PROGRAM,
        &(DEPOSIT_DISCRIMINATOR, args),
        accounts,
    )
}

pub fn restake_ix(
    lst_amount: u64,
    ref_code: u32,
    depositor: Pubkey,
    msol_ata: Pubkey,
    mpsol_ata: Pubkey,
) -> Instruction {
    let args = RestakeInstructionArgs {
        lst_amount,
        ref_code,
    };

    let accounts = vec![
        AccountMeta::new(MAIN_STATE, false),
        AccountMeta::new(MSOL_MINT_ADDRESS, false),
        AccountMeta::new(VAULT_STATE, false),
        AccountMeta::new_readonly(VAULT_ATA_PDA_AUTH, false),
        AccountMeta::new(VAULT_LST_ACCOUNT, false),
        AccountMeta::new(depositor, true),
        AccountMeta::new(msol_ata, false),
        AccountMeta::new(MPSOL_MINT_ADDRESS, false),
        AccountMeta::new_readonly(MPSOL_MINT_AUTHORITY, false),
        AccountMeta::new(mpsol_ata, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM_PUBKEY, false),
        AccountMeta::new(MARINADE_LIQUID_STAKING_STATE, false),
    ];

    Instruction::new_with_borsh(MP_RESTAKIN_PROGRAM, &(STAKE_DISCRIMINATOR, args), accounts)
}

pub fn unrestake_ix(mpsol_amount: u64, unstaker: Pubkey, mpsol_ata: Pubkey, new_ticket_account: Pubkey) -> Instruction {
    let args = UnrestakeInstructionArgs { mpsol_amount };

    // let now = Local::now();
    // let ten_days_later = now + chrono::Duration::days(10);
    // let timestamp = ten_days_later.timestamp() as u64;

    // println!("Timestamp 10 days from now: {}", timestamp);

    // let account_len: usize = (4 + unstaker.to_string().len()) + (4 + MAIN_STATE.to_string().len()) + 8;
    // let account_len: usize = 100;

    // let state = UnstakeTicket {
    //     beneficiary: unstaker,
    //     main_state: MAIN_STATE,
    //     ticket_sol_value: mpsol_amount,
    //     ticket_due_timestamp: timestamp

    // };

    // let rent = Rent::get().unwrap();
    // let rent_lamports = rent.minimum_balance(account_len);

    // let new_ticket_account = Account::new_data(rent_amount, &state, &MP_RESTAKIN_PROGRAM).unwrap();

    // println!("{:?}", state);

    // let pubkey = create_account(
    //     &unstaker,
    //     &new_ticket_account.pubkey(),
    //     rent_lamports,
    //     100,
    //     &MP_RESTAKIN_PROGRAM
    // );

    // let (note_pda_account, bump_seed) = Pubkey::find_program_address(&[note_creator.key.as_ref(), id.as_bytes().as_ref(),], program_id);

    let accounts = vec![
        AccountMeta::new(MAIN_STATE, false),
        AccountMeta::new(unstaker, true),
        AccountMeta::new(mpsol_ata, false),
        AccountMeta::new(MPSOL_MINT_ADDRESS, false),
        AccountMeta::new(
            pubkey!("Gde12qXKF3fALWTAQgzyqBhNE67eBxdmJDLw3EiTu4eu"),
            false,
        ),
        AccountMeta::new(new_ticket_account, true),
        AccountMeta::new_readonly(TOKEN_PROGRAM_PUBKEY, false),
        AccountMeta::new_readonly(SYSTEM_PROGRAM_PUBKEY, false),
    ];

    Instruction::new_with_borsh(
        MP_RESTAKIN_PROGRAM,
        &(UNSTAKE_DISCRIMINATOR, args),
        accounts,
    )
}

#[derive(BorshSerialize, BorshDeserialize)]
struct DepositInstructionArgs {
    pub lamports: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct RestakeInstructionArgs {
    pub lst_amount: u64,
    pub ref_code: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct UnrestakeInstructionArgs {
    pub mpsol_amount: u64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UnstakeTicket {
    pub main_state: Pubkey,
    /// auth that can withdraw the LSTs when due
    pub beneficiary: Pubkey,
    /// amount (lamports) this ticket is worth (set at unstake) -- can be updated on partial ticket withdraws
    pub ticket_sol_value: u64,
    /// when this ticket is due (unix timestamp)
    pub ticket_due_timestamp: u64,
}
