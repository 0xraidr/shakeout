use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;



declare_id!("GauoQBkGoNCjzZ9CnFwijL9FZutmjYBHZovVc2MenTnL");

#[program]
pub mod shakeout {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.auth_bump = *ctx.bumps.get("auth").unwrap();
        ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u32) -> Result<()> {
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

    pub fn early_withdraw(ctx: Context<EarlyWithdraw>, amount: u32) -> Result<()> {

        require!(amount <= ctx.accounts.state.balance, ShakeOutError::InvalidWithdraw);

        let amt_after_tax = calculate_75_percent(amount);

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
        transfer(cpi, amt_after_tax as u64)
    }


    pub fn unlock_funds(ctx: Context<Unlock>, amount: u32) -> Result<()> {

        require!(amount <= ctx.accounts.state.balance, ShakeOutError::InvalidWithdraw);

        //////////////////////////////////////////////////////////////
        //IMPLEMENT LOGIC TO CHECK FOR SOLANA TARGET PRICE USING PYTH BELOW






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
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>,
        // PDA Signer for SPL Vault
        #[account(
            seeds = [b"auth", state.key().as_ref()],
            bump
        )]
        ///CHECK: NO NEED TO CHECK THIS
        auth: UncheckedAccount<'info>,
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



#[derive(Accounts)]
pub struct EarlyWithdraw<'info> {
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



#[account]
pub struct VaultState {
    auth_bump: u8,
    vault_bump: u8,
    state_bump: u8,
    balance: u32,
}

impl VaultState {
    const LEN: usize = 8 + 4 + 3 * 1;
}

#[error_code] 
pub enum ShakeOutError {
    #[msg("Withdraw must be less than or equal to the amount you deposited")]
    InvalidWithdraw,
}

fn calculate_75_percent(amount: u32) -> u32 {
    // Scale up by 100
    let scaled_amount = amount * 100;
    // Multiply by 75 (for 75%)
    let seventy_five_percent = scaled_amount * 75;
    // Scale back down
    seventy_five_percent / 100
}
