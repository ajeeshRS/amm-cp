use crate::error::PoolError;
use anchor_lang::prelude::*;
use integer_sqrt::IntegerSquareRoot;

pub fn get_lp_token_amount_init(x_tokens: u64, y_tokens: u64) -> Result<u64> {
    let product = (x_tokens as u128)
        .checked_mul(y_tokens as u128)
        .ok_or(PoolError::OverFlowError)?;
    Ok(product.integer_sqrt() as u64)
}

// getting the lp tokens to mint to LP without losing the ratio/constant
pub fn get_lp_tokens_to_mint(
    lp_suppy: u64,
    vault_x: u64,
    vault_y: u64,
    max_x_token: u64,
    max_y_token: u64,
) -> Result<u64> {
    let lp_x = (max_x_token as u128)
        .checked_mul(lp_suppy as u128)
        .ok_or(PoolError::OverFlowError)?
        .checked_div(vault_x as u128)
        .ok_or(PoolError::OverFlowError)? as u64;

    let lp_y = (max_y_token as u128)
        .checked_mul(lp_suppy as u128)
        .ok_or(PoolError::OverFlowError)?
        .checked_div(vault_y as u128)
        .ok_or(PoolError::OverFlowError)? as u64;

    let lp_tokens = lp_x.min(lp_y);

    Ok(lp_tokens)
}

// getting the X and Y token amount from the lp token amount
pub fn xy_from_l(lp_suppy: u64, vault_x: u64, vault_y: u64, lp_amount: u64) -> Result<(u64, u64)> {
    let required_x = (vault_x as u128)
        .checked_mul(lp_amount as u128)
        .ok_or(PoolError::OverFlowError)?
        .checked_div(lp_suppy as u128)
        .ok_or(PoolError::OverFlowError)? as u64;

    let required_y = (vault_y as u128)
        .checked_mul(lp_amount as u128)
        .unwrap()
        .checked_div(lp_suppy as u128)
        .unwrap() as u64;

    Ok((required_x, required_y))
}
