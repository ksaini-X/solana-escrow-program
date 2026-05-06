use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Similar Pubkey")]
    SamePubkey,

    #[msg("InvalidReceive")]
    InvalidReceive,

    #[msg("InvalidExpiryTime")]
    InvalidExpiryTime,
}
