use anchor_lang::prelude::*;

declare_id!("EVH3WpdE7b8w28mm55T392dA24dVk58tDk78Dq6CgGjK");

mod instructions;
use instructions::*;

#[program]
pub mod direct_sales {
    use super::*;
    
    pub fn add(ctx: Context<AddAsset>, amount: u64, price_per_token: u64) -> Result<()> {
        add_asset(ctx, amount, price_per_token)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
