use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{TransferChecked, transfer_checked},
    token_interface::{Mint,TokenAccount,TokenInterface}
};

use crate::{state::*,constants::*,errors::*};

#[derive(Accounts)]
pub struct Delist<'info>{
    #[account(mut)]
    pub seller:Signer<'info>,

        pub nft_mint:InterfaceAccount<'info,Mint>,


    #[account(
        mut,
        seeds = [b"listing", seller.key().as_ref(), nft_mint.key().as_ref()],
        bump = listing_state.bump,
        close = seller
    )]
    pub listing_state:Account<'info,Listing>,

        #[account(
            mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing_state,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller
    )]
    pub seller_nft_account:InterfaceAccount<'info,TokenAccount>,


     #[account(
        seeds=[b"marketplace"],
        bump = marketplace_state.bump
    )]
    pub marketplace_state:Account<'info,Marketplace>,
     pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl <'info> Delist<'info>{
    pub fn delist(&mut self)->Result<()>{
        require!(self.listing_state.is_active&& self.listing_state.seller==self.seller.key(),MarketplaceError::ListingNotOpen);

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from:self.vault.to_account_info(),
            to:self.seller_nft_account.to_account_info(),
            authority: self.listing_state.to_account_info(),
            mint:self.nft_mint.to_account_info()
        };

        let marketplace = self.marketplace_state.key();
        let seller = self.seller.key();
        let nft_mint = self.nft_mint.key();
        let seeds = [
            b"listing",
            marketplace.as_ref(),
            seller.as_ref(),
            nft_mint.as_ref(),
            &[self.listing_state.bump]
        ];

        let signer = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer);
        transfer_checked(ctx,1,self.nft_mint.decimals)?;

        self.listing_state.is_active=false;
        Ok(())
    }
}