use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
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
        seeds=[b"marketplace"],
        bump
    )]
    pub marketplace_state:Account<'info,Marketplace>,

    #[account(
        init,
        payer = seller,
        space = DISCRIMINATOR + Listing::INIT_SPACE,
        seeds = [b"listing",marketplace_state.key().as_ref(), seller.key().as_ref(), nft_mint.key().as_ref()],
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
    
    pub collection_mint:InterfaceAccount<'info,Mint>,

    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref()== collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
        )]
        pub metadata:Account<'info,MetadataAccount>,

        #[account(
            seeds = [
                b"metadata",
                metadata_program.key().as_ref(),
                nft_mint.key().as_ref()
            ],
            seeds::program = metadata_program.key(),
            bump
        )]
        pub master_edition:Account<'info,MasterEditionAccount>,



    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub metadata_program:Program<'info,Metadata>

}

impl <'info> List<'info>{
    pub fn list_nft(&mut self,price:u64,bumps:ListBumps)->Result<()>{
        self.initialize_listing( price ,bumps);

        self.transfer_nft_vault();

        Ok(())
    }

    pub fn initialize_listing(&mut self,price:u64,bumps:ListBumps)->Result<()>{

        self.listing_state.set_inner(Listing{
            seller:self.seller.key(),
            price:price,
            nft_mint:self.nft_mint.key(),
            bump:bumps.listing_state,
            is_active:true
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