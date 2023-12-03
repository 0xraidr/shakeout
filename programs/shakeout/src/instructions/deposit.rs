use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;
use crate::state::VaultState;

pub fn deposit_handler(ctx: Context<Deposit>, amount: u32) -> Result<()> {
    let accounts = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        accounts,
    );
    transfer(cpi, amount as u64)
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    owner: Signer<'info>,
        // Where we store our SOL  
        #[account(
            seeds = [b"vault", state.key().as_ref()],
            bump
        )]
        vault: SystemAccount<'info>,
        #[account(
            init,
            payer = owner,
            space = VaultState::LEN,
            seeds = [b"state", owner.key().as_ref()],
            bump
        )]
        state: Account<'info, VaultState>,  
        system_program: Program<'info, System>
}