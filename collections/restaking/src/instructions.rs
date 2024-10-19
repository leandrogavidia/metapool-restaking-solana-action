use crate::consts::{
    DEPOSIT_DISCRIMINATOR, MAIN_STATE, MARINADE_FINANCE_PROGRAM, MARINADE_LIQUID_STAKING_STATE,
    MARINADE_MSOL_LEG_ACCOUNT, MARINADE_MSOL_LEG_AUTHORITY, MARINADE_MSOL_MINT_AUTHORITY,
    MARINADE_RESERVE_SOL_PDA, MARINADE_SOL_LEG_ACCOUNT, MPSOL_MINT_ADDRESS, MPSOL_MINT_AUTHORITY,
    MP_RESTAKIN_PROGRAM, MSOL_MINT_ADDRESS, STAKE_DISCRIMINATOR, VAULT_ATA_PDA_AUTH,
    VAULT_LST_ACCOUNT, VAULT_STATE,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_PUBKEY,
};
use spl_token::ID as TOKEN_PROGRAM_PUBKEY;

pub fn deposit_transaction(lamports: u64, from_pubkey: Pubkey, msol_ata: Pubkey) -> Instruction {
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

#[derive(BorshSerialize, BorshDeserialize)]
struct DepositInstructionArgs {
    pub lamports: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct RestakeInstructionArgs {
    pub lst_amount: u64,
    pub ref_code: u32,
}
