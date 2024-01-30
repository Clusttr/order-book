use anchor_lang::prelude::*;

declare_id!("EVH3WpdE7b8w28mm55T392dA24dVk58tDk78Dq6CgGjK");

// mod dep_instructions;
mod instructions;
mod models;
mod utils;

use instructions::*;


#[program]
pub mod direct_sales {
    use super::*;

    pub fn add(ctx: Context<DepositAsset>, amount: u64, price_per_token: u64) -> Result<()> {
        deposit_asset(ctx, amount, price_per_token)
    }

    pub fn update_asset_price(ctx: Context<UpdatePrice>, price_per_token: u64) -> Result<()> {
        update_price(ctx, price_per_token)
    }

    pub fn withdraw(ctx: Context<WithdrawAsset>, amount: u64) -> Result<()> {
        withdraw_asset(ctx, amount)
    }

    pub fn buy(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
        buy_asset(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
