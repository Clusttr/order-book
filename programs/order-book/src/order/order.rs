use std::collections::HashMap;
use anchor_lang::prelude::*;

#[account]
pub struct Order {
    pub price: u64,
    pub total: u64,
    pub quantity: u64,
    pub time_stamp: u64,
    pub is_open: bool,
    pub owner: Pubkey,
}

#[account]
pub struct OrderBook {
    // pub orders: HashMap<Pubkey, Order>
}