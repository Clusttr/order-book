use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("No token to stake")]
    pub NoTokens
}