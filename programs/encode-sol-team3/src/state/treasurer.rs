use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR_SIZE, PUBKEY_SIZE, U64_SIZE};

#[account]
pub struct Treasurer {
    pub authority: Pubkey,
    pub launch_pool: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}

impl Treasurer {
    pub const LEN: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE + PUBKEY_SIZE + PUBKEY_SIZE + U64_SIZE;

    pub fn initialize(&mut self, authority: Pubkey, launch_pool: Pubkey, token_mint: Pubkey) {
        self.authority = authority;
        self.launch_pool = launch_pool;
        self.token_mint = token_mint;
        self.amount = 0;
    }
}
