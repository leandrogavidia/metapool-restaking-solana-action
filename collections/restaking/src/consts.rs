use solana_sdk::{pubkey, pubkey::Pubkey};

pub const DEPOSIT_DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const STAKE_DISCRIMINATOR: [u8; 8] = [206, 176, 202, 18, 200, 209, 179, 108];
pub const UNSTAKE_DISCRIMINATOR: [u8; 8] = [90, 95, 107, 42, 205, 124, 50, 225];

// Mariande staking referral program

// pub const MARINADE_STAKING_REFERRAL_PROGRAM: Pubkey =
//     pubkey!("MR2LqxoSbw831bNy68utpu5n4YqBH3AzDmddkgk9LQv");
pub const MPSOL_MINT_ADDRESS: Pubkey = pubkey!("mPsoLV53uAGXnPJw63W91t2VDqCVZcU5rTh3PWzxnLr");

pub const MARINADE_SOL_LEG_ACCOUNT: Pubkey = pubkey!("UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q");
pub const MARINADE_MSOL_LEG_ACCOUNT: Pubkey =
    pubkey!("7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE");
pub const MARINADE_MSOL_LEG_AUTHORITY: Pubkey =
    pubkey!("EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL");
pub const MARINADE_RESERVE_SOL_PDA: Pubkey =
    pubkey!("Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN");
pub const MARINADE_MSOL_MINT_AUTHORITY: Pubkey =
    pubkey!("3JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM");
pub const REFERRAL_STATE: Pubkey = pubkey!("GL3wosUhC6u4bEvHrWhbig1QAXsXnWaGYxkHxbjptfom");
pub const MSOL_TOKEN_PARTNER_ACCOUNT: Pubkey =
    pubkey!("HjLKRFDp3Y7fJ36MxgdeNnxfKFjsVRNb4dQ2adc5ymTW");
pub const MARINADE_FINANCE_PROGRAM: Pubkey = pubkey!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

// # Metapool restaking program

pub const MAIN_STATE: Pubkey = pubkey!("mpsoLeuCF3LwrJWbzxNd81xRafePFfPhsNvGsAMhUAA");
pub const VAULT_ATA_PDA_AUTH: Pubkey = pubkey!("6frWEHsiEc2RscCmPRGMq3DH54QR1FHrw1fqFHQD51d4");

// ## MSOL

pub const MSOL_MINT_ADDRESS: Pubkey = pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
pub const MSOL_VAULT_STATE: Pubkey = pubkey!("5LsQaaarGCUcpB5rSL1DN9kH1ibQ99EKk4NPEvwxQtDq");
pub const MSOL_VAULT_LST_ACCOUNT: Pubkey = pubkey!("Bm5yHSY54ieTQGiutdy9U2MXzHUdiy3d63AQg5HDHUs1");
pub const MARINADE_LIQUID_STAKING_STATE: Pubkey = pubkey!("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC");

// ## BSOL

pub const BSOL_MINT_ADDRESS: Pubkey = pubkey!("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1");
pub const BSOL_VAULT_STATE: Pubkey = pubkey!("35XhJLVM4fCXXH9i2aXkHBSNSc4i1Gcwj6CkDQMjMdYc");
pub const BSOL_VAULT_LST_ACCOUNT: Pubkey = pubkey!("DQUN5vTACLbfYyeAuPfkMku9vY35xJbsT59h7qwBYnur");
pub const BLAZE_STAKE_POOL: Pubkey = pubkey!("stk9ApL5HeVAwPLr3TLhDXdZS8ptVu7zp6ov8HFDuMi");

// ## MPSOL

pub const MP_RESTAKIN_PROGRAM: Pubkey = pubkey!("MPSoLoEnfNRFReRZSVH2V8AffSmWSR4dVoBLFm1YpAW");
pub const MPSOL_MINT_AUTHORITY: Pubkey = pubkey!("2nGwwzMPEaTPEaRHVFjVojHJeB8uaWnTMRCN22zrQWSL");
