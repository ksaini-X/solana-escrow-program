use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

pub use instructions::*;
declare_id!("7ZkbSYngFndHLCNmTnq6rweKoheTiPmcXf1yr9KpRsqM");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, amount: u64, recieve: u64, expiry: i64) -> Result<()> {
        make::make(ctx, amount, recieve, expiry)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        refund::refund(ctx)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        take::take(ctx)
    }
}
