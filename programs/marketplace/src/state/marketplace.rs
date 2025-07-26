use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace{
    pub admin:Pubkey,
    pub comission_fee:u64,
    pub bump:u8,
    pub comission_fee_bump:u8
}

