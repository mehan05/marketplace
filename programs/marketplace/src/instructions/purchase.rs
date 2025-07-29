use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenAccount,TokenInterface,TransferChecked, transfer_checked,Transfer,transfer}
};

use crate::{state::*,constants::*,errors::*};

#[derive(Accounts)]
pub struct Purchase<'info>{
    #[account(mut)] 
    pub buyer:Signer<'info>,

    ///CHECK: Default anchor security checks not happen for this acc
    #[account(mut)]
    pub seller : AccountInfo<'info>,

    pub nft_mint:InterfaceAccount<'info,Mint>,


    #[account(
        seeds = [b"listing", seller.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub listing_state:Account<'info,Listing>,

        #[account(
        associated_token::mint = nft_mint,
        associated_token::authority = listing_state,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_account:InterfaceAccount<'info,TokenAccount>,


     #[account(
        seeds=[b"marketplace"],
        bump
    )]
    pub marketplace_state:Account<'info,Marketplace>,


    #[account(
        seeds = [b"comisssion",marketplace_state.key().as_ref()],
        bump
    )]
    pub comission:SystemAccount<'info>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl<'info> Purchase<'info>{
    pub fn purchase(& mut self)->Result<()>{
        require!(self.listing_state.is_active,MarketplaceError::ListingNotOpen);

        let buyerBalance = self.buyer.to_account_info().lamports as u64;

        require!(buyerBalance>=self.listing_state.price,MarketplaceError::InsufficuentAmount);

        let cpi_program_for_sol_transfer = self.system_program.to_account_info();

        let cpi_program_for_nft_transfer = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from:self.buyer.to_account_info(),
            to:self.seller.to_account_info(),
            authority:self.buyer.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program_for_sol_transfer,cpi_accounts);

        transfer(ctx, self.listing_state.price);


        let comission_fee = (self.listing_state.price*self.marketplace_state.comission_fee)/100;

        let buyerBalance = self.buyer.to_account_info().lamports;

        require!(buyerBalance>=self.listing_state.price,MarketplaceError::InsufficuentAmount);


          let cpi_accounts = Transfer{
            from:self.buyer.to_account_info(),
            to:self.comission.to_account_info(),
            authority:self.buyer.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program_for_sol_transfer,cpi_accounts);

        transfer(ctx, comission_fee);


        let seeds = [
            b"listing",
            self.marketplace_state.key().as_ref(),
            self.seller.key().as_ref(),
            self.nft_mint.key().as_ref(),
            &[self.listing_state.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = TransferChecked{
            from:self.vault.to_account_info(),
            to:self.buyer_nft_account.to_account_info(),
            authority: self.listing_state.to_account_info(),
            mint:self.nft_mint.to_account_info(),
        };
                let ctx = CpiContext::new_with_signer(cpi_program_for_nft_transfer, cpi_accounts, signer_seeds);

        transfer_checked(ctx,1,self.nft_mint.decimals)?;

        self.listing_state.is_active = false;

        Ok(())


    }
}
