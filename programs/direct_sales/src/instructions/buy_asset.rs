use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, transfer, Transfer},
};
use crate::models::*;

#[derive(Accounts)]
pub struct BuyAsset<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
    mut,
    associated_token::mint = usdc_mint,
    associated_token::authority = signer
    )]
    signer_usdc_account: Account<'info, TokenAccount>,
    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer
    )]
    signer_mint_account: Account<'info, TokenAccount>,

    /// CHECK: This will revisited
    creator_account: AccountInfo<'info>,
    #[account(
    associated_token::mint = usdc_mint,
    associated_token::authority = creator_account
    )]
    creators_usdc_account: Account<'info, TokenAccount>,
    usdc_mint: Account<'info, Mint>,

    #[account(
    mut,
    // init_if_needed,
    seeds = [constants::constants::VAULT, mint.key().as_ref()],
    bump,
    // payer = signer,
    token::mint = mint,
    token::authority = token_vault
    )]
    token_vault: Account<'info, TokenAccount>,

    #[account(
    init_if_needed,
    seeds = [constants::constants::INVENTORY, mint.key().as_ref()],
    bump,
    payer = signer,
    space = 8 + std::mem::size_of::<Inventory>()
    )]
    inventory: Account<'info, Inventory>,
    mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
    let price = &ctx.accounts.inventory.price;
    let total_amount = amount * price;

    //check balance in usdc account
    let sender_balance = ctx.accounts.signer_usdc_account.amount;
    msg!("sender usdc balance: ${}", sender_balance);
    println!("sender usdc balance: ${}", sender_balance);
    if sender_balance < total_amount {
        return Err(InsufficientUSDCBalance.into())
    }

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer{
                from: ctx.accounts.signer_usdc_account.to_account_info(),
                to: ctx.accounts.creators_usdc_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info()
            }
        ),
        total_amount
    )?;

    //transfer asset from vault to buyer
    let token_amount = ctx.accounts.inventory.amount;
    if token_amount < amount {
        return Err(InsufficientTokenBalance.into())
    }
    let bump = *ctx.bumps.get("token_vault").unwrap();
    let mint_key = ctx.accounts.mint.key();
    let seeds = &[INVENTORY, mint_key.as_ref(), &[bump]];
    let signer: &[&[&[u8]]] = &[seeds];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.signer_mint_account.to_account_info(),
                authority: ctx.accounts.token_vault.to_account_info()
            },
            signer
        ),
        amount
    )?;

    //update inventory
    ctx.accounts.inventory.amount -= amount;

    Ok(())
}