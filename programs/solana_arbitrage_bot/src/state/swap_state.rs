use anchor_lang::prelude::*;

#[account]
pub struct RaydiumSwapState {
    pub bump: u8,
    pub authority: Pubkey,
    pub initialized: bool,
    pub last_swap_timestamp: i64,
}

impl RaydiumSwapState {
    pub const LEN : usize = 8 + 1 + 32 + 1 + 8; // We can also use the anchor InitSpace . but we are using the Default Account ,AnchorSerialize , AnchorDeserialize
}
