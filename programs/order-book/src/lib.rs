mod instructions;

use anchor_lang::prelude::*;

declare_id!("CyhM6NKXeZs3xsC9ZVJEQikAnUwRhW3qkzqPA9zkUbtz");

#[program]
pub mod order_book {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
