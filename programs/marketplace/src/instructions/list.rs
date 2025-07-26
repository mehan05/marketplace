use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenAccount,TokenInterface,TransferChecked, transfer_checked}
};

use crate::{state::*,constants::*};

#[derive(Accounts)]
pub struct List<'info>{
    
    #[account(mut)]
    pub seller:Signer<'info>,


    pub nft_mint:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller
    )]
    pub seller_nft_account:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer = seller,
        space = DISCRIMINATOR + Listing::INIT_SPACE,
        seeds = [b"listing", seller.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub listing_state:Account<'info,Listing>,

    #[account(
        init,
        payer = seller,
        associated_token::mint = nft_mint,
        associated_token::authority = listing_state,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>

}

impl <'info> List<'info>{
    pub fn list_nft(&mut self,seller:Pubkey,price:u64,nft_mint:Pubkey,is_active:bool,bumps:ListBumps)->Result<()>{
        self.initialize_listing(seller, price, nft_mint, is_active, bumps);

        self.transfer_nft_vault();

        Ok(())
    }

    pub fn initialize_listing(&mut self,seller:Pubkey,price:u64,nft_mint:Pubkey,
    is_active:bool,bumps:ListBumps)->Result<()>{

        self.listing_state.set_inner(Listing{
            seller:seller,
            price:price,
            nft_mint:nft_mint,
            bump:bumps.listing_state,
            is_active:is_active
        });

        Ok(())
    }

    pub fn transfer_nft_vault(&mut self)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from:self.seller_nft_account.to_account_info(),
            to:self.vault.to_account_info(),
            authority: self.seller.to_account_info(),
            mint:self.nft_mint.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts);
        transfer_checked(ctx,1,self.nft_mint.decimals)?;

        Ok(())

    }
}