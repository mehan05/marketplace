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
        seeds = [b"listing",marketplace_state.key().as_ref(), seller.key().as_ref(), nft_mint.key().as_ref()],
        bump = listing_state.bump
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
        init_if_needed,
        payer=buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_account:InterfaceAccount<'info,TokenAccount>,


     #[account(
        seeds=[b"marketplace"],
        bump = marketplace_state.bump
    )]
    pub marketplace_state:Account<'info,Marketplace>,


    #[account(
        mut,
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
        require!(self.listing_state.is_active&& self.listing_state.seller==self.seller.key(),MarketplaceError::ListingNotOpen);

        let comission_fee = self.listing_state.price.checked_mul(self.marketplace_state.comission_fee).ok_or(MarketplaceError::Overflow)?.checked_div(100).ok_or(MarketplaceError::Overflow)?;

        let seller_fee = self.listing_state.price.checked_sub(comission_fee).ok_or(MarketplaceError::Overflow)?;


        let cpi_program_for_sol_transfer = self.system_program.to_account_info();

        let cpi_program_for_nft_transfer = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from:self.buyer.to_account_info(),
            to:self.seller.to_account_info(),
            authority:self.buyer.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program_for_sol_transfer.clone(),cpi_accounts);

        transfer(ctx, seller_fee);




          let cpi_accounts = Transfer{
            from:self.buyer.to_account_info(),
            to:self.comission.to_account_info(),
            authority:self.buyer.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program_for_sol_transfer,cpi_accounts);

        transfer(ctx, comission_fee);


        let marketplace_state = self.marketplace_state.key();
        let seller = self.seller.key();
        let nft_mint = self.nft_mint.key();
        let seeds = [
            b"listing",
            marketplace_state.as_ref(),
            seller.as_ref(),
            nft_mint.as_ref(),
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
