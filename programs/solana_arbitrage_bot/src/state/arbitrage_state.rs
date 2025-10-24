use anchor_lang::prelude::*;


#[account]
#[derive(Default)]
pub struct ArbitrageState {
    pub authority: Pubkey,
    pub total_profit: u64,
    pub total_trades: u64,
}

#[account]
#[derive(Default)]
pub struct SwapState {
    pub start_balance: u64,
    pub swap_input: u64,
    pub is_valid: bool,
    pub input_token: Pubkey,
    pub current_token: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone , Debug)]
pub enum ArbitrageStep {
    Orca(u64),
    Raydium(u64),
    Meteora(u64),
    Jupiter(u64),
}


impl ArbitrageStep {
    pub fn get_amount_token(&self) -> Pubkey {
        Pubkey::default()
    }
}