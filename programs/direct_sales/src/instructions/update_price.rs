use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ TokenAccount, Mint },
};
use crate::models::*;

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer
    )]
    singer_token_account: Account<'info, TokenAccount>,

    #[account(
    init_if_needed,
    seeds = [constants::constants::INVENTORY, mint.key().as_ref()],
    bump,
    payer = signer,
    space = 8 + std::mem::size_of::<Inventory>()
    )]
    inventory: Account<'info, Inventory>,
    mint: Account<'info, Mint>,
    system_program: Program<'info, System>
}

pub fn update_price(ctx: Context<UpdatePrice>, price_per_token: u64) -> Result<()> {
    ctx.accounts.inventory.price = price_per_token;
    Ok(())
}