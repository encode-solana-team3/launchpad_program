use anchor_lang::{prelude::*, system_program};
use anchor_spl::token;

use crate::{
    constants::{USER_POOL_SEED, VAULT_SEED},
    errors::MyError,
    state::{CurrencyType, LaunchPool, LaunchPoolState, LaunchPoolType, UserPool},
};

#[event]
pub struct BuyTokenWithNativeEvent {
    pub buyer: Pubkey,
    pub amount: u64,
    pub token_amount: u64,
    pub vault_amount: u64,
}

#[derive(Accounts)]
pub struct BuyTokenWithNative<'info> {
    #[account(mut)]
    pub launch_pool: Box<Account<'info, LaunchPool>>,
    pub token_mint: Box<Account<'info, token::Mint>>,
    #[account(
        init_if_needed,
        seeds = [USER_POOL_SEED.as_ref(), user.key().as_ref(), launch_pool.key().as_ref(),token_mint.key().as_ref()],
        bump,
        payer = user,
        space = UserPool::LEN
    )]
    pub user_pool: Box<Account<'info, UserPool>>,
    /// CHECK: Create a new vault for the launch pool
    #[account(
        mut,
        seeds = [
            VAULT_SEED.as_ref(),
            launch_pool.key().as_ref(),
            launch_pool.authority.as_ref()
        ],
        bump ,
    )]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<BuyTokenWithNative>, amount: u64) -> Result<()> {
    let launch_pool = &mut ctx.accounts.launch_pool;
    let user_pool = &mut ctx.accounts.user_pool;

    require!(amount.gt(&0), MyError::InvalidAmount);

    require!(
        launch_pool.status == LaunchPoolState::Active,
        MyError::InvalidLaunchPoolStatus
    );

    require!(
        launch_pool.pool_type == LaunchPoolType::FairLaunch,
        MyError::InvalidLaunchPoolType
    );
    require!(
        launch_pool.currency == CurrencyType::SOL,
        MyError::InvalidCurrencyType
    );
    require!(
        launch_pool.pool_size_remaining.ge(&amount),
        MyError::PoolSizeRemainingNotEnough
    );

    require!(
        user_pool
            .amount
            .checked_add(amount)
            .unwrap()
            .ge(&launch_pool.minimum_token_amount),
        MyError::MinimumTokenAmountNotReached
    );

    require!(
        user_pool
            .amount
            .checked_add(amount)
            .unwrap()
            .le(&launch_pool.maximum_token_amount),
        MyError::MaximumTokenAmountReached
    );

    let user_must_pay = launch_pool.calculate_user_must_pay(amount);

    require!(user_must_pay.gt(&0), MyError::InvalidAmount);

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, user_must_pay)?;

    msg!(
        "User buy {} token {} with {} RENEC",
        amount,
        launch_pool.token_mint,
        user_must_pay
    );

    user_pool.amount = user_pool.amount.checked_add(amount).unwrap();
    user_pool.currency_amount = user_pool
        .currency_amount
        .checked_add(user_must_pay)
        .unwrap();
    launch_pool.pool_size_remaining = launch_pool.pool_size_remaining.checked_sub(amount).unwrap();
    launch_pool.vault_amount = launch_pool.vault_amount.checked_add(user_must_pay).unwrap();

    emit!(BuyTokenWithNativeEvent {
        buyer: *ctx.accounts.user.key,
        amount,
        token_amount: user_pool.amount,
        vault_amount: launch_pool.vault_amount,
    });

    Ok(())
}
