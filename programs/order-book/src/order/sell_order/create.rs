use anchor_lang::prelude::*;
use crate::order:: {
    order::*,
    error_code::{ErrorCode::NoTokens},
    constants::*
};
use anchor_spl::{
    token::{Token, TokenAccount, Mint, transfer, Transfer},
};

#[derive(Accounts)]
pub struct CreateSellOrder<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
    init_if_needed,
    seeds=[constants::ORDER_BOOK, constants::SELL, mint.key().as_ref()],
    bump,
    payer = signer,
    space = 8 + std::mem::size_of::<OrderBook>()
    )]
    pub order_book: Account<'info, OrderBook>,

    #[account(
    mut,
    seeds=[constants::ASSET_ACCOUNT, signer.key.as_ref()],
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

pub fn create_sell_order(ctx: Context<CreateSellOrder>, price: u64, quantity: u64) -> Result<()> {
    if quantity <= 0 {
        return Err(NoTokens.into())
    }

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.asset_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info()
            }
        ),
        quantity
    )?;

    let order_book = &mut ctx.accounts.order_book;
    let clock = Clock::get()?;
    let order = Order {
        price,
        quantity,
        total: quantity * price,
        time_stamp: clock.slot,
        is_open: true,
        owner: ctx.accounts.signer.key()
    };
    order_book.orders.insert(ctx.accounts.signer.key(), order);
    Ok(())
}