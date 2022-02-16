use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use provider::{Provider};


declare_id!("CRuuNGo8mY26RPw4RXchR2ZDHDZA9MBRaZQWAWbQF3ri");

const MAX_NAME_LENGTH: usize     = 32;
const DATA_SIZE: usize = 68;

#[program]
pub mod oracle {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, name: String, data: Vec<OracleData>, bump: u8) -> ProgramResult {

        let oracle = &mut ctx.accounts.oracle;
        oracle.name = name;
        oracle.bump = bump;
        oracle.data = data;
        oracle.provider_program = *ctx.accounts.oracle_provider.to_account_info().owner;

        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: Vec<OracleData>) -> ProgramResult {

        let oracle = &mut ctx.accounts.oracle;
        oracle.data = data;

        Ok(())
    }
}

fn name_seed(name: &str) -> &[u8] {
    let b = name.as_bytes();
    if b.len() > 32 {
        &b[0..32]
    } else {
        b
    }
}

#[derive(Accounts)]
#[instruction(name: String, data: Vec<OracleData>, bump: u8)]
pub struct Initialize<'info> {
    #[account(init,
        payer=user,
        space=Oracle::space(&name, &oracle_provider.data_size),
        seeds=[oracle_provider.to_account_info().key.as_ref(), name_seed(&name)],
        bump=bump
    )]
    pub oracle: Account<'info, Oracle>,

    #[account(
        //verify that the user is the oracle update authority
        constraint = oracle_provider.authority == *user.key,
    )]
    pub oracle_provider: Account<'info, Provider>,

    // User initiating the transaction
    #[account(mut)]
    pub user: Signer<'info>,

    // System program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,

    #[account(
        //verify that the user is the oracle update authority
        constraint = provider.authority == *user.key,
        //verify that the template account belongs to the oracle program
        constraint = *provider.to_account_info().owner == oracle.provider_program,
    )]
    pub provider: Account<'info, Provider>,

    // User initiating the operation
    #[account(mut)]
    pub user: Signer<'info>,

    //System program
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Default)]
pub struct OracleData {
    /// The name of the data
    pub name: String, //max 32

    /// Oracle value
    pub value: String, //max 32
}

impl OracleData {
    pub fn space() -> usize {
        // discriminator + nonce
        8 + 32 + 32
    }
}

#[account]
pub struct Oracle {
    // Provider Program ID
    provider_program: Pubkey,

    // Oracle Name
    name: String,

    // Oracle Data
    data: Vec<OracleData>,

    // Bump seed
    pub bump: u8,
}

impl Oracle {
    fn space(name: &str, size: &u32) -> usize {
        // discriminator + nonce
        8 + 32 + name.len() + MAX_NAME_LENGTH + (*size as usize) * DATA_SIZE + 8
    }
}

#[error]
pub enum ErrorCode {
    #[msg("The given oracle has already been initialized.")]
    OracleAlreadyInitialized,
    #[msg("The usesr is not authorized to update the oracle.")]
    OracleUnauthorizedUser,
}
