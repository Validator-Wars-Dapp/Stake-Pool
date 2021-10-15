// SPDX-FileCopyrightText: 2021 Chorus One AG
// SPDX-License-Identifier: GPL-3.0

use solana_program::{program_pack::Pack, pubkey::Pubkey, system_instruction};
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signer},
};

use crate::snapshot::Result;
use crate::SnapshotConfig;

/// Push instructions to create and initialize and SPL token mint.
///
/// This uses the default number of decimals: 9. Returns the mint address.
pub fn push_create_spl_token_mint(
    config: &mut SnapshotConfig,
    instructions: &mut Vec<Instruction>,
    mint_authority: &Pubkey,
) -> Result<Keypair> {
    let mint_account_min_sol_balance = config
        .client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    let keypair = Keypair::new();

    instructions.push(system_instruction::create_account(
        &config.signer.pubkey(),
        &keypair.pubkey(),
        // Deposit enough SOL to make it rent-exempt.
        mint_account_min_sol_balance.0,
        spl_token::state::Mint::LEN as u64,
        // The new account should be owned by the SPL token program.
        &spl_token::id(),
    ));

    let num_decimals = 9;
    assert_eq!(spl_token::native_mint::DECIMALS, num_decimals);
    let freeze_authority = None;

    instructions.push(spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &keypair.pubkey(),
        mint_authority,
        freeze_authority,
        num_decimals,
    )?);

    Ok(keypair)
}

/// Push instructions to create and initialize an SPL token account.
///
/// Returns the keypair for the account. This keypair needs to sign the
/// transaction.
pub fn push_create_spl_token_account(
    config: &mut SnapshotConfig,
    instructions: &mut Vec<Instruction>,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<Keypair> {
    let spl_token_min_sol_balance = config
        .client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?;

    let keypair = Keypair::new();

    instructions.push(system_instruction::create_account(
        &config.signer.pubkey(),
        &keypair.pubkey(),
        // Deposit enough SOL to make it rent-exempt.
        spl_token_min_sol_balance.0,
        spl_token::state::Account::LEN as u64,
        // The new account should be owned by the SPL token program.
        &spl_token::id(),
    ));
    instructions.push(spl_token::instruction::initialize_account(
        &spl_token::id(),
        &keypair.pubkey(),
        mint,
        owner,
    )?);

    Ok(keypair)
}
