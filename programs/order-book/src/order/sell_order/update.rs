use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, transfer, Transfer};
use crate::order::{
    order::{OrderBook, Order},
    constants::*,
    error_code::*,
};
use crate::order::error_code::ErrorCode::InsufficientWithdrawableToken;

#[derive(Accounts)]
pub struct UpdateSellOrder<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        mut,
        seeds=[constants::ORDER_BOOK, constants::SELL, mint.key().as_ref()],
        bump,
    )]
    pub order_book: Account<'info, OrderBook>,

    #[account(
        mut,
        seeds=[signer.key.as_ref()],
        bump,
    )]
    asset_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer
    )]
    user_token_account: Account<'info, TokenAccount>,

    mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

fn withdraw_sell_order(ctx: Context<UpdateSellOrder>, quantity: u64) -> Result<()> {

    let bump = *ctx.bumps.get("asset_account").unwrap();
    // let payer_binary_pub_key = *ctx.accounts.asset_account.key().as_ref();
    let asset_account_seed: &[&[&[u8]]] = &[&[&[bump]]];
    let asset_balance = &ctx.accounts.asset_account.amount;
    if *asset_balance < quantity  {
        return Err(InsufficientWithdrawableToken.into())
    }

    //transfer asset out to user token account
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.asset_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.token_program.to_account_info()
            },
            asset_account_seed
        ),
            quantity
    )?;

    //if order does not exist return error
    // let mut order = ctx.accounts.order_book.orders.get(ctx.accounts.signer.key).unwrap();
    // if order.quantity < quantity {
    //     return Err(InsufficientWithdrawableToken.into())
    // }
    // order = &Order {
    //     price: order.price,
    //     total: order.total,
    //     quantity: order.quantity - quantity,
    //     time_stamp: order.time_stamp,
    //     is_open: order.is_open,
    //     owner: order.owner
    // };

    Ok(())
}