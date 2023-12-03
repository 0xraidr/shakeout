use anchor_lang::prelude::*;
use crate::{state::VaultState, 
    // GameError
};

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    let user_state = &mut ctx.accounts.state;
    user_state.auth_bump = *ctx.bumps.get("auth").unwrap();
    user_state.state_bump = *ctx.bumps.get("state").unwrap();
    user_state.vault_bump = *ctx.bumps.get("vault").unwrap();
    user_state.is_locked = true;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
        // PDA Signer for SPL Vault
    #[account(
            seeds = [b"auth", state.key().as_ref()],
            bump
        )]
        ///CHECK: NO NEED TO CHECK THIS
    pub auth: UncheckedAccount<'info>,
        // Where we store our SOL  
    #[account(
            seeds = [b"vault", state.key().as_ref()],
            bump
        )]
    pub vault: SystemAccount<'info>,
    #[account(
            init,
            payer = owner,
            space = VaultState::LEN,
            seeds = [b"state", owner.key().as_ref()],
            bump
        )]
    pub state: Account<'info, VaultState>, 
                // Where we store our SOL  
    #[account(mut)]
    pub tax_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}