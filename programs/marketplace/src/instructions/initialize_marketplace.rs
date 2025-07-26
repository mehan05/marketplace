use anchor_lang::prelude::*;

use crate::{Marketplace,constants::*};

#[derive(Accounts)]
pub struct Initialize<'info>{

    #[account(mut)]
    pub admin:Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds=[b"marketplace", admin.key().as_ref()],
        space = DISCRIMINATOR + Marketplace::INIT_SPACE,
        bump
    )]
    pub marketplace_state:Account<'info,Marketplace>,

    #[account(
        mut,
        seeds=[b"comission",admin.key().as_ref()],
        bump
    )]
    pub comission_fee:SystemAccount<'info>,

    pub system_program:Program<'info,System>
}

impl <'info> Initialize<'info>{
    pub fn initialize_marketplace(&mut self, comission_fee:u64,bumps:InitializeBumps)->Result<()>{
        self.marketplace_state.set_inner(Marketplace{
            admin:self.admin.key(),
            comission_fee:comission_fee,
            bump:self.marketplace_state.bump,
            comission_fee_bump:bumps.comission_fee
        });
        Ok(())
    }
}

