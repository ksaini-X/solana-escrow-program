use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub vault: Pubkey,
    pub recieve: u64,
    pub expiry: i64,
    pub bump: u8,
}

impl Escrow {
    pub fn size() -> usize {
        8 + Escrow::INIT_SPACE
    }
}
