use crate::consts::{
    BSOL_MINT_ADDRESS, BSOL_VAULT_LST_ACCOUNT, BSOL_VAULT_STATE, DEPOSIT_DISCRIMINATOR, MAIN_STATE,
    MARINADE_FINANCE_PROGRAM, MARINADE_LIQUID_STAKING_STATE, MARINADE_MSOL_LEG_ACCOUNT,
    MARINADE_MSOL_LEG_AUTHORITY, MARINADE_MSOL_MINT_AUTHORITY, MARINADE_RESERVE_SOL_PDA,
    MARINADE_SOL_LEG_ACCOUNT, MPSOL_MINT_ADDRESS, MPSOL_MINT_AUTHORITY, MP_RESTAKIN_PROGRAM,
    MSOL_MINT_ADDRESS, MSOL_VAULT_LST_ACCOUNT, MSOL_VAULT_STATE, STAKE_DISCRIMINATOR, VAULT_ATA_PDA_AUTH, BLAZE_STAKE_POOL, JITOSOL_MINT_ADDRESS, JITOSOL_STAKE_POOL, JITOSOL_VAULT_LST_ACCOUNT, JITOSOL_VAULT_STATE
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_PUBKEY,
};
use spl_token::ID as TOKEN_PROGRAM_PUBKEY;

pub fn deposit_ix(
    lamports: u64,
    from_pubkey: Pubkey,
    msol_ata: Pubkey,
) -> Instruction {
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

        // These 3 accounts are commented on because there is an overflow in the transaction weight allowed. It will be patched eventually.

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
    depositor_ata: Pubkey,
    mpsol_ata: Pubkey,
    token: &str,
) -> Instruction {
    let args = RestakeInstructionArgs {
        lst_amount,
        ref_code,
    };

    let token_mint_address = match token {
        "sol" | "msol" => MSOL_MINT_ADDRESS,
        "bsol" => BSOL_MINT_ADDRESS,
        "jito-sol" => JITOSOL_MINT_ADDRESS,
        _ => MSOL_MINT_ADDRESS,
    };

    let vault_pubkey = match token {
        "sol" | "msol" => MSOL_VAULT_STATE,
        "bsol" => BSOL_VAULT_STATE,
        "jito-sol" => JITOSOL_VAULT_STATE,
        _ => MSOL_VAULT_STATE,
    };

    let vault_lst_account = match token {
        "sol" | "msol" => MSOL_VAULT_LST_ACCOUNT,
        "bsol" => BSOL_VAULT_LST_ACCOUNT,
        "jito-sol" => JITOSOL_VAULT_LST_ACCOUNT,
        _ => MSOL_VAULT_LST_ACCOUNT,
    };

    let lst_pool = match token {
        "sol" | "msol" => MARINADE_LIQUID_STAKING_STATE,
        "bsol" => BLAZE_STAKE_POOL,
        "jito-sol" => JITOSOL_STAKE_POOL,
        _ => MARINADE_LIQUID_STAKING_STATE
    };

    let accounts = vec![
        AccountMeta::new(MAIN_STATE, false),
        AccountMeta::new(token_mint_address, false),
        AccountMeta::new(vault_pubkey, false),
        AccountMeta::new_readonly(VAULT_ATA_PDA_AUTH, false),
        AccountMeta::new(vault_lst_account, false),
        AccountMeta::new(depositor, true),
        AccountMeta::new(depositor_ata, false),
        AccountMeta::new(MPSOL_MINT_ADDRESS, false),
        AccountMeta::new_readonly(MPSOL_MINT_AUTHORITY, false),
        AccountMeta::new(mpsol_ata, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM_PUBKEY, false),
        AccountMeta::new(lst_pool, false),
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
