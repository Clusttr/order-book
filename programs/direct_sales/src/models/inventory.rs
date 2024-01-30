use anchor_lang::prelude::*;

#[account]
pub struct Inventory {
    pub price: u64,
    pub amount: u64
}