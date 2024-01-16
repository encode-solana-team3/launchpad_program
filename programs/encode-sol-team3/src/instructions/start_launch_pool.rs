use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::{
    constants::{LAUNCH_POOL_SEED, TREASURER_SEED},
    errors::MyError,
    state::{LaunchPool, LaunchPoolState, Treasurer},
};

#[derive(Accounts)]
pub struct StartLaunchPool<'info> {
    #[account(mut, seeds = [LAUNCH_POOL_SEED.as_ref(), authority.key().as_ref(), token_mint.key().as_ref()], bump)]
    pub launch_pool: Account<'info, LaunchPool>,
    pub token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub source_token_account: Account<'info, token::TokenAccount>,
    #[account(mut, seeds = [TREASURER_SEED.as_ref(), launch_pool.key().as_ref(), token_mint.key().as_ref()], bump)]
    pub treasurer: Box<Account<'info, Treasurer>>,
    #[account(mut, constraint = treasury.mint == launch_pool.token_mint)]
    pub treasury: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<StartLaunchPool>) -> Result<()> {
    let launch_pool = &mut ctx.accounts.launch_pool;
    let treasurer = &mut ctx.accounts.treasurer;
    let source_token_account = &mut ctx.accounts.source_token_account;
    let treasury = &mut ctx.accounts.treasury;
    let authority = &ctx.accounts.authority;
    let token_mint = &ctx.accounts.token_mint;
    let token_program = &ctx.accounts.token_program;

    require!(
        launch_pool.status == LaunchPoolState::Pending,
        MyError::InvalidLaunchPoolStatus
    );
    require!(
        launch_pool.authority == *authority.key,
        MyError::InvalidAuthority
    );

    require!(
        token_mint.to_account_info().key.eq(&launch_pool.token_mint),
        MyError::InvalidTokenMint
    );

    let transfer_amount = launch_pool.pool_size;
    launch_pool.pool_size_remaining = transfer_amount;
    launch_pool.status = LaunchPoolState::Active;
    treasurer.amount = transfer_amount;

    msg!("Transfering {} tokens to treasury", transfer_amount);

    let cpi_context = CpiContext::new(
        token_program.to_account_info(),
        token::Transfer {
            from: source_token_account.to_account_info(),
            to: treasury.to_account_info(),
            authority: authority.to_account_info(),
        },
    );
    Ok(token::transfer(cpi_context, transfer_amount)?)
}
