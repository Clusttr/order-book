mod order;
mod direct_sales;

use order::{
    sell_order::create::*
};

use anchor_lang::prelude::*;

declare_id!("CyhM6NKXeZs3xsC9ZVJEQikAnUwRhW3qkzqPA9zkUbtz");

#[program]
pub mod order_book {
    use super::*;

    pub fn create_ask_order(ctx: Context<CreateSellOrder>, price: u64, quantity: u64) -> Result<()> {
        create_sell_order(ctx, price, quantity)
    }
}
