use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::PoolError, Pool, LP_TOKEN_MINT_SEED, POOL_SEED};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = pool,
    )]
    pub ata_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = pool,
    )]
    pub ata_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [LP_TOKEN_MINT_SEED.as_bytes(),pool.key().as_ref()],
        bump = pool.lp_bump
    )]
    pub lp_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [POOL_SEED.as_bytes(),authority.key().as_ref()],
        bump = pool.pool_bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_ata_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_ata_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = user
    )]
    pub user_lp_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let total_lp_supply = self.pool.lp_supply;

        require!(amount > 0, PoolError::InvalidAmount);

        let return_x = (self.ata_x.amount as u128)
            .checked_mul(amount as u128)
            .ok_or(PoolError::OverFlowError)?
            .checked_div(total_lp_supply as u128)
            .ok_or(PoolError::OverFlowError)? as u64;

        let return_y = (self.ata_y.amount as u128)
            .checked_mul(amount as u128)
            .ok_or(PoolError::OverFlowError)?
            .checked_div(total_lp_supply as u128)
            .ok_or(PoolError::OverFlowError)? as u64;

        self.burn_tokens(amount)?;
        self.withdraw_tokens(true, return_x)?;
        self.withdraw_tokens(false, return_y)?;

        self.pool.lp_supply -= amount;

        Ok(())
    }

    pub fn burn_tokens(&mut self, token_amount: u64) -> Result<()> {
        let burn_accounts = Burn {
            mint: self.lp_token_mint.to_account_info(),
            from: self.user_lp_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        let ctx = CpiContext::new(cpi_program, burn_accounts);

        burn(ctx, token_amount)?;

        Ok(())
    }

    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (
                self.ata_x.to_account_info(),
                self.user_ata_x.to_account_info(),
            ),
            false => (
                self.ata_y.to_account_info(),
                self.user_ata_y.to_account_info(),
            ),
        };

        let auth_key = self.authority.key();

        let seeds = &[
            POOL_SEED.as_bytes(),
            auth_key.as_ref(),
            &[self.pool.pool_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.pool.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
