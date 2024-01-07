use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, transfer, Transfer},
};
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
    singer_token_account: Account<'info, TokenAccount>,

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
                from: ctx.accounts.singer_token_account.to_account_info(),
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

// fn withdraw_asset() -> Result<()> {
//     Ok(())
// }
//
// #[derive(Accounts)]
// struct BuyAsset<'info> {
//     #[account(mut)]
//     signer: Signer<'info>,
//
//     #[account()]
//     creators_usdc_account: Account<'info, TokenAccount>,
//
//     #[account(
//         init_if_needed,
//         seeds = [],
//         bump,
//         payer = signer,
//         space = 8 + std::mem::size_of::<Inventory>()
//     )]
//     pub price_list: Account<'info, Inventory>,
//
//     #[account(
//         mut,
//         seeds = [],
//         bump,
//     )]
//     token_vault: Account<'info, TokenAccount>,
//     mint: Account<'info, Mint>,
//     token_program: Program<'info, Token>,
//     system_program: Program<'info, System>
// }
//
// fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
//     let creator = get_creator(&ctx.accounts.token_vault.to_account_info())
//         .unwrap();
//
//     let asset = &ctx.accounts.mint.to_account_info();
//     let price = 0;//&ctx.accounts.price_list.list.get(asset.key);
//     // if price.is_none() { return Err(ErrorCode::AssetNotFound.into()) }
//
//     let total_amount = amount * price;//.unwrap();
//
//     //confirm creator owns usdc account
//     if !&ctx.accounts.creators_usdc_account.owner.eq(&creator.address) {
//         return Err(ErrorCode::FalseUSDCAccount.into())
//     }
//
//     //transfer amount from user wallet to creators wallet
//     transfer(
//         CpiContext::new(
//             ctx.accounts.token_program.to_account_info(),
//             Transfer{
//                 from: ctx.accounts.signer.to_account_info(),
//                 to: ctx.accounts.creators_usdc_account.to_account_info(),
//                 authority: ctx.accounts.signer.to_account_info()
//             }
//         ),
//         total_amount
//     )?;
//
//     let bump = *ctx.bumps.get("creators_usdc_account").unwrap();
//     let seed: &[&[&[u8]]] = &[&[&[bump]]];
//
//     //transfer asset from vault to buyer
//     transfer(
//         CpiContext::new_with_signer(
//             ctx.accounts.token_program.to_account_info(),
//             Transfer {
//                 from: ctx.accounts.token_vault.to_account_info(),
//                 to: ctx.accounts.creators_usdc_account.to_account_info(),
//                 authority: ctx.accounts.token_vault.to_account_info()
//             },
//             seed
//         ),
//         amount
//     )?;
//
//     Ok(())
// }
//
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
    #[msg("USDC Account passed is not owned by creator")]
    FalseUSDCAccount
}