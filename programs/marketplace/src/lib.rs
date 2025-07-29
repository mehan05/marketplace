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

    pub fn initialize_marketplace(ctx: Context<Initialize>,comission_fee:u64) -> Result<()> {
       ctx.accounts.initialize_marketplace(comission_fee, ctx.bumps);
        Ok(())
    }

    pub fn list_nft(ctx:Context<List>,price:u64)->Result<()>{
        ctx.accounts.list_nft(price,ctx.bumps);
        Ok(())
    }

    pub fn purchase(ctx:Context<Purchase>)->Result<()>{
        ctx.accounts.purchase();
        Ok(())
    }

    pub fn delist(ctx:Context<Delist>)->Result<()>{
        ctx.accounts.delist();
        Ok(())
    }




}


