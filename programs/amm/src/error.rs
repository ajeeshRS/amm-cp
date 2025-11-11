use anchor_lang::prelude::*;

#[error_code]
pub enum PoolError {
    #[msg("Pool is locked")]
    PoolIsLocked,
    #[msg("Lp token amount cannot be 0")]
    LpTokenAmountCannotBeZero,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
    #[msg("Overflow Error")]
    OverFlowError,
    #[msg("Invalid Amount")]
    InvalidAmount,
}
