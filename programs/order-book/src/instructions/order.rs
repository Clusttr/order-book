use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

mod constants {
    const ORDER_BOOK: &[u8] = b"order_book";
}

enum OrderType {
    Sell,
    Bid
}

pub struct Order {
    pub size: u64,
    pub price: u64,
    pub total: u64,
    pub asset: Pubkey,
    pub owner: Pubkey,
}

pub struct OrderBook {
    pub orders: Vec<Order>
}

#[derive(Accounts)]
struct PlaceOrder<'info> {
    payer: Signer<'info>,

    #[account(init_if_needed, seeds=[])]
    order_book: Account<'info, OrderBook>,

    #[account(mut, seeds=[vault, payer.owner])]
    asset_vault: SystemAccount<'info>,
    system_program: Program<'info, System>
}

pub fn place_bid(ctx: Context<PlaceOrder>) -> Result<()> {
    let order_book = &ctx.accounts.order_book;

    //transfer asset first
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.asset_vault.to_account_info()
            }
        ),
        100
    )?;

    let order = Order {
        size: 2, price: 120, total: 2 * 120, asset: Pubkey::new_unique(), owner: Pubkey::new_unique()
    };
    order_book.orders.push(order);
    Ok(())
}