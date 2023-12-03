use anchor_lang::prelude::*;

declare_id!("GauoQBkGoNCjzZ9CnFwijL9FZutmjYBHZovVc2MenTnL");

#[program]
pub mod shakeout {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
