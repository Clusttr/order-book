use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("No token to stake")]
    pub NoTokens,
    #[msg("Order does not exist")]
    pub Order404,
    #[msg("Insufficient withdrawable token")]
    pub InsufficientWithdrawableToken,
}