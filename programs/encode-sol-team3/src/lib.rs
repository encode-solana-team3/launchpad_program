use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
use instructions::*;

declare_id!("Eo9a3Zjn5HbGnL9wqkjDmajQ5EGzgaBbW77YhUZNVLo5");

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

    pub fn start_launch_pool(ctx: Context<StartLaunchPool>) -> Result<()> {
        instructions::start_launch_pool::handler(ctx)
    }

    pub fn buy_token_with_native(ctx: Context<BuyTokenWithNative>, amount: u64) -> Result<()> {
        instructions::buy_token_with_native::handler(ctx, amount)
    }
}
