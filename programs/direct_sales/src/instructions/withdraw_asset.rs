use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, transfer, Transfer},
};
use crate::models::*;
#[derive(Accounts)]
pub struct WithdrawAsset<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer
    )]
    signer_token_account: Account<'info, TokenAccount>,

    #[account(
    init_if_needed,
    seeds = [constants::constants::INVENTORY, mint.key().as_ref()],
    bump,
    payer = signer,
    space = 8 + std::mem::size_of::<Inventory>()
    )]
    inventory: Account<'info, Inventory>,

    #[account(
    init_if_needed,
    seeds = [constants::constants::VAULT, mint.key().as_ref()],
    bump,
    payer = signer,
    token::mint = mint,
    token::authority = token_vault
    )]
    token_vault: Account<'info, TokenAccount>,
    mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}
pub fn withdraw_asset(ctx: Context<WithdrawAsset>, amount: u64) -> Result<()> {
    // transfer token to user ATA
    let bump = *ctx.bumps.get("token_vault").unwrap();
    let mint_key = ctx.accounts.mint.key();
    let signer: &[&[&[u8]]] = &[&[constants::constants::VAULT, mint_key.as_ref(), &[bump]]];
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.signer_token_account.to_account_info(),
                authority: ctx.accounts.token_vault.to_account_info()
            },
            signer
        ),
        amount
    )?;

    // update inventory
    ctx.accounts.inventory.amount -= amount;

    Ok(())
}