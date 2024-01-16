use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::{
    errors::MyError,
    state::{LaunchPool, LaunchPoolState},
};

#[derive(Accounts)]
pub struct CompleteLaunchPool<'info> {
    #[account(mut)]
    pub launch_pool: Box<Account<'info, LaunchPool>>,
    pub token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<CompleteLaunchPool>) -> Result<()> {
    let launch_pool = &mut ctx.accounts.launch_pool;
    require!(
        launch_pool.status == LaunchPoolState::Active,
        MyError::InvalidLaunchPoolStatus
    );
    require!(
        launch_pool.authority == *ctx.accounts.authority.key,
        MyError::InvalidAuthority
    );

    launch_pool.status = LaunchPoolState::Completed;

    msg!("Launch pool completed");

    Ok(())
}
