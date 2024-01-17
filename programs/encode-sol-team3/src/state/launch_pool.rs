use anchor_lang::prelude::*;

use crate::{
    constants::{
        BOOL_SIZE, CURRENCY_DECIMALS, DISCRIMINATOR_SIZE, I64_SIZE, PUBKEY_SIZE, U64_SIZE, U8_SIZE,
    },
    errors::MyError,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct LaunchPoolBumps {
    pub launchpool_bump: u8,
    pub treasurer_bump: u8,
    pub vault_bump: u8,
}

// struct for launchpad token pool
#[account]
pub struct LaunchPool {
    pub unlock_date: i64,
    pub pool_size: u64,
    pub minimum_token_amount: u64,
    pub maximum_token_amount: u64,
    pub rate: u64,
    pub pool_size_remaining: u64,
    pub token_mint: Pubkey,
    pub token_mint_decimals: u8,
    pub authority: Pubkey,
    pub vault_amount: u64,
    pub is_vesting: bool,
    pub currency: CurrencyType,
    pub pool_type: LaunchPoolType,
    pub status: LaunchPoolState,
}

// enum for currency token type
#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone, Copy)]
pub enum CurrencyType {
    SOL,
    USDC,
}

impl From<u8> for CurrencyType {
    fn from(val: u8) -> Self {
        match val {
            0 => CurrencyType::SOL,
            1 => CurrencyType::USDC,
            _ => panic!("Invalid CurrencyType"),
        }
    }
}

// enum for launchpad type
#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone, Copy)]
pub enum LaunchPoolType {
    FairLaunch,
    WhiteList,
}

impl From<u8> for LaunchPoolType {
    fn from(val: u8) -> Self {
        match val {
            0 => LaunchPoolType::FairLaunch,
            1 => LaunchPoolType::WhiteList,
            _ => panic!("Invalid LaunchPoolType"),
        }
    }
}

// enum for launchpad token pool status
#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone, Copy)]
pub enum LaunchPoolState {
    Pending,
    Active,
    Completed,
    Cancelled,
}

impl LaunchPool {
    pub const LEN: usize = DISCRIMINATOR_SIZE +
        I64_SIZE +
        U64_SIZE +
        U64_SIZE +
        U64_SIZE +
        U64_SIZE +
        U64_SIZE +
        PUBKEY_SIZE +
        U8_SIZE + // token_mint_decimals
        PUBKEY_SIZE +
        U64_SIZE +
        BOOL_SIZE + // is_vesting
        1 +
        1 + // enum CurrencyType
        1 +
        1 + // enum LaunchPoolType
        1 +
        1; // enum LaunchPoolState

    pub fn initialize(
        &mut self,
        unlock_date: i64,
        pool_size: u64,
        minimum_token_amount: u64,
        maximum_token_amount: u64,
        rate: u64,
        token_mint_decimals: u8,
        token_mint: Pubkey,
        authority: Pubkey,
        currency: CurrencyType,
        pool_type: LaunchPoolType,
    ) -> Result<()> {
        require!(
            unlock_date.gt(&Clock::get()?.unix_timestamp),
            MyError::InvalidUnlockDate
        );

        self.unlock_date = unlock_date;
        self.pool_size = pool_size;
        self.minimum_token_amount = minimum_token_amount;
        self.maximum_token_amount = maximum_token_amount;
        self.rate = rate;
        self.pool_size_remaining = 0;
        self.token_mint = token_mint;
        self.token_mint_decimals = token_mint_decimals;
        self.authority = authority;
        self.vault_amount = 0;
        self.currency = currency;
        self.pool_type = pool_type;
        self.status = LaunchPoolState::Pending;
        self.is_vesting = false;
        Ok(())
    }

    pub fn calculate_user_must_pay(&self, amount: u64) -> u64 {
        ((amount
            .checked_div(self.rate)
            .unwrap()
            .checked_mul(10_i32.pow(CURRENCY_DECIMALS) as u64))
        .unwrap() as u128)
            .checked_div(10_i32.pow(self.token_mint_decimals as u32) as u128)
            .unwrap() as u64
    }
}
