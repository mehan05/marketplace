use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenAccount,TokenInterface,TransferChecked, transfer_checked}
};

use crate::{state::*,constants::*};

#[derive(Accounts)]
pub struct Purchase<'info>{
    #[account(mut)] 
    pub buyer:Signer<'info>,

      
}