use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
use instructions::*;

declare_id!("BW6SPYkVKy7QzVRwAdstwDUyUYHxiLXBwP2cwwRQpgG6");

#[program]
pub mod encode_sol_team3 {
    use super::*;

    pub fn create_native_pool(
        ctx: Context<CreateFairlaunchPool>,
        unlock_date: i64,
        pool_size: u64,
        minimum_token_amount: u64,
        maximum_token_amount: u64,
        rate: u64,
        token_mint_decimals: u8,
    ) -> Result<()> {
        instructions::create_fairlaunch_pool::handler(
            ctx,
            unlock_date,
            pool_size,
            minimum_token_amount,
            maximum_token_amount,
            rate,
            token_mint_decimals,
        )
    }
}
