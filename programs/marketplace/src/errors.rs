use anchor_lang::prelude::*;
#[error_code]
pub enum MarketplaceError{

    #[msg("Listing isn't open to purchase")]
    ListingNotOpen,


    #[msg("Insufficient amount")]
    InsufficuentAmount
}