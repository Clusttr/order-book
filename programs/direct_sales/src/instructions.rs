use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, transfer, Transfer},
};
use anchor_spl::associated_token::get_associated_token_address;
use mpl_token_metadata::accounts::{Metadata};
use mpl_token_metadata::types::Creator;

#[account]
pub struct Inventory {
    price: u64,
    amount: u64
}

mod constants {
    pub const INVENTORY: &[u8] = b"inventory";
    pub const VAULT: &[u8] = b"vault";
}

#[derive(Accounts)]
pub struct AddAsset<'info> {
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
    seeds = [constants::INVENTORY, mint.key().as_ref()],
    bump,
    payer = signer,
    space = 8 + std::mem::size_of::<Inventory>()
    )]
    inventory: Account<'info, Inventory>,

    //asset token pda
    #[account(
    init_if_needed,
    seeds = [constants::VAULT, mint.key().as_ref()],
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
pub fn add_asset(ctx: Context<AddAsset>, amount: u64, price_per_token: u64) -> Result<()> {
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
        seeds = [constants::INVENTORY, mint.key().as_ref()],
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
    seeds = [constants::INVENTORY, mint.key().as_ref()],
    bump,
    payer = signer,
    space = 8 + std::mem::size_of::<Inventory>()
    )]
    inventory: Account<'info, Inventory>,

    #[account(
    init_if_needed,
    seeds = [constants::VAULT, mint.key().as_ref()],
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
    let signer: &[&[&[u8]]] = &[&[constants::VAULT, mint_key.as_ref(), &[bump]]];
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
        seeds = [constants::VAULT, mint.key().as_ref()],
        bump,
        // payer = signer,
        token::mint = mint,
        token::authority = token_vault
    )]
    token_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [constants::INVENTORY, mint.key().as_ref()],
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
        return Err(ErrorCode::InsufficientUSDCBalance.into())
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
        return Err(ErrorCode::InsufficientUSDCBalance.into())
    }
    let bump = *ctx.bumps.get("token_vault").unwrap();
    let mint_key = ctx.accounts.mint.key();
    let seeds = &[constants::VAULT, mint_key.as_ref(), &[bump]];
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

fn get_creator(mint_account_info: &AccountInfo) -> Result<Creator> {
    let metadata = Metadata::try_from(mint_account_info).unwrap();
    let creators = metadata.creators.unwrap();
    let developer = creators.get(1); //get the second item

    return match developer {
        None => {
            Err(ErrorCode::NoCreator.into())
        }
        Some(developer) => {
            Ok(developer.clone())
        }
    };
}

#[error_code]
pub enum ErrorCode {
    #[msg("No creator found")]
    NoCreator,
    #[msg("Instruction is only permitted by Creator[1]")]
    OnlyPermittedByCreator,
    #[msg("Can't find asset")]
    AssetNotFound,
    #[msg("Insufficient usdc Balance")]
    InsufficientUSDCBalance,
    #[msg("USDC Account passed is not owned by creator")]
    FalseUSDCAccount
}