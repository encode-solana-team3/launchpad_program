use anchor_lang::prelude::*;

declare_id!("BW6SPYkVKy7QzVRwAdstwDUyUYHxiLXBwP2cwwRQpgG6");

#[program]
pub mod encode_sol_team3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
