use anchor_lang::prelude::*;

declare_id!("2p5itgNZjWZbQkES8mygo7khSV5nD19H91Sb1ur2KyLH");

pub const PREFIX: &str = "oracle";
pub const VERSION: &str = "v1";
pub const MAX_DATA_SIZE: u32 = 10;

#[program]
pub mod provider {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, name: String, size: u32, bump: u8) -> ProgramResult {
        if !(size >= 0 as u32 && size <= MAX_DATA_SIZE) {
            return Err(ErrorCode::ProviderInvalidSize.into());
        }

        let provider = &mut ctx.accounts.provider;
        provider.name = name;
        provider.data_size = size;
        provider.bump = bump;
        provider.authority = *ctx.accounts.user.to_account_info().key;

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
#[instruction(name: String, size: u32, bump: u8)]
pub struct Initialize<'info> {
    #[account(init,
        payer=user,
        space=Provider::space(&name),
        seeds=[name_seed(&name)],
        bump=bump
    )]
    pub provider: Account<'info, Provider>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Provider {
    //Provider name
    pub name: String,

    // Authority for setting oracle value
    pub authority: Pubkey,

    // Number of oracle data points
    pub data_size: u32,

    // Bump seed
    pub bump: u8,
}

impl Provider {
    fn space(name: &str) -> usize {
        // discriminator + name + authority + data_size + bump
        8 + name.len() + 32 + 32 + 8
    }
}

#[error]
pub enum ErrorCode {
    #[msg("The given oracle data size is invalid.")]
    ProviderInvalidSize,
}

