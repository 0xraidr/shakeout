use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;

declare_id!("GauoQBkGoNCjzZ9CnFwijL9FZutmjYBHZovVc2MenTnL");

#[program]
pub mod shakeout {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u32) -> Result<()> {
        deposit_handler(ctx, amount)
    }

    pub fn early_withdraw(ctx: Context<EarlyWithdraw>, amount: u32) -> Result<()> {
        early_withdraw_handler(ctx, amount)
    }


    pub fn unlock_funds(ctx: Context<Unlock>, amount: u32) -> Result<()> {
        unlock_funds_handler(ctx, amount)
    }
}
