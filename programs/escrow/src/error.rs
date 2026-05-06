use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Similar Pubkey")]
    SamePubkey,

    #[msg("InvalidReceive")]
    InvalidReceive,

    #[msg("InvalidExpiryTime")]
    InvalidExpiryTime,

    #[msg("InvalidMaker")]
    InvalidMaker,

    #[msg("InvalidTaker")]
    InvalidTaker,

    #[msg("InvalidRefundRequest")]
    InvalidRefundRequest,

    #[msg("EscrowExpired")]
    EscrowExpired,
}
