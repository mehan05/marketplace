use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing{
    pub seller:Pubkey,
    pub price:u64,
    pub nft_mint:Pubkey,
    pub bump:u8,
    pub is_active:bool
}

