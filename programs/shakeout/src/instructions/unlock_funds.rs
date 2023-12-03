use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;
use crate::state::VaultState;

pub fn unlock_funds_handler(ctx: Context<Unlock>, amount: u32) -> Result<()> {

    require!(amount <= ctx.accounts.state.balance, ShakeOutError::InvalidWithdraw);

    //////////////////////////////////////////////////////////////
    //IMPLEMENT LOGIC TO CHECK FOR SOLANA TARGET PRICE USING PYTH BELOW


    // if SOL TP hits turn is_locked to false instead of true to allow users to withdraw unlocked funds.




    //////////////////////////////////////////////////////////////
    let accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.owner.to_account_info(),
    };

    let seeds = &[
        b"vault",
        ctx.accounts.state.to_account_info().key.as_ref(),
        &[ctx.accounts.state.vault_bump][..]
    ];

    let signer_seeds = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        accounts,
        signer_seeds
    );
    transfer(cpi, amount as u64)

}

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    // Where we store our SOL  
        #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump
    )]
    vault: SystemAccount<'info>,  
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump =  state.state_bump
    )]
    state: Account<'info, VaultState>,  
    system_program: Program<'info, System>
}

#[error_code] 
pub enum ShakeOutError {
    #[msg("Withdraw must be less than or equal to the amount you deposited")]
    InvalidWithdraw,
}