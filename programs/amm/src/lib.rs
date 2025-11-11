pub mod constants;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5cSLhwbvEJAmj7j1BrzVyHZENnohtPQhqwdLUFrK4vQQ");

#[program]
pub mod amm {
    use anchor_spl::token::accessor::amount;

    use super::*;

    pub fn initialize_pool(ctx: Context<Initialize>, fee: u16, lp_supply: u64) -> Result<()> {
        ctx.accounts.initialize_pool(fee, lp_supply, ctx.bumps)?;
        Ok(())
    }
    pub fn provide_liquidity(
        ctx: Context<ProvideLiquidity>,
        max_x_token: u64,
        max_y_token: u64,
    ) -> Result<()> {
        ctx.accounts.deposit_tokens(max_x_token, max_y_token)?;
        Ok(())
    }
    pub fn swap(
        ctx: Context<SwapTokens>,
        amount_in: u64,
        amount_out_min: u64,
        is_x: bool,
    ) -> Result<()> {
        ctx.accounts.swap(amount_in, amount_out_min, is_x)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
}
