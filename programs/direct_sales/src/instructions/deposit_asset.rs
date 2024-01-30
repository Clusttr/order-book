use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, transfer, Transfer},
};
use crate::models::*;

#[derive(Accounts)]
pub struct DepositAsset<'info> {
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

    //asset token pda
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
pub fn deposit_asset(ctx: Context<DepositAsset>, amount: u64, price_per_token: u64) -> Result<()> {
    // let creator = get_creator(&ctx.accounts.mint.to_account_info()).unwrap();
    // if !creator.address.eq(ctx.accounts.signer.key) { return Err(ErrorCode::OnlyPermittedByCreator.into())}

    //transfer asset
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer{
                from: ctx.accounts.signer_token_account.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.signer.to_account_info()
            },
        ),
        amount
    )?;

    // update inventory
    ctx.accounts.inventory.amount += amount;
    ctx.accounts.inventory.price = price_per_token;

    Ok(())
}
