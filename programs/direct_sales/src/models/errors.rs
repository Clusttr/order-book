use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("No creator found")]
    NoCreator,
    #[msg("Instruction is only permitted by Creator[1]")]
    pub OnlyPermittedByCreator,
    #[msg("Can't find asset")]
    pub AssetNotFound,
    #[msg("Insufficient usdc Balance")]
    pub InsufficientUSDCBalance,
    #[msg("Insufficient token Balance")]
    pub InsufficientTokenBalance,
    #[msg("USDC Account passed is not owned by creator")]
    pub FalseUSDCAccount
}