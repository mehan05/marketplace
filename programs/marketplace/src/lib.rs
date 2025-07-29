use anchor_lang::prelude::*;

declare_id!("3eT2VyTuEfdgNfywZeUbDMRgnu9matcMLLek7hua7sC1");

pub mod state;
pub use state::*;

pub mod constants;
pub use constants::*;

pub mod instructions;
pub use instructions::*;

pub mod errors;
pub use errors::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}


