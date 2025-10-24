use anchor_lang::prelude::*;

#[derive(AcnhorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SwapData {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}