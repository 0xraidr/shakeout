use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub state_bump: u8,
    pub balance: u32,
    pub is_locked: bool,
}

impl VaultState {
   pub const LEN: usize = 8 + 1 + 4 + 3 * 1;
}