use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR_SIZE, U64_SIZE};

#[account]
pub struct UserPool {
    pub amount: u64,
    pub currency_amount: u64,
    pub claimed: u64,
}

impl UserPool {
    pub const LEN: usize = DISCRIMINATOR_SIZE + U64_SIZE + U64_SIZE + U64_SIZE;
}
