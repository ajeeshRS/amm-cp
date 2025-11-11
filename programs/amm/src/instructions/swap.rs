use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};

use crate::{error::PoolError, swap, Pool, LP_TOKEN_MINT_SEED, POOL_SEED};

#[derive(Accounts)]
pub struct SwapTokens<'info> {
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
        init,
        payer = user,
        associated_token::mint = lp_token_mint,
        associated_token::authority = user
    )]
    pub user_lp_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SwapTokens<'info> {
    pub fn swap(&mut self, amount_in: u64, amount_out_min: u64, is_x: bool) -> Result<()> {
        // calculating the swap fee (swap amount * fee in basis points / 10_000) //if 30 is bp, then 0.003 which is 0.3%
        let swap_fee = amount_in
            .checked_mul(self.pool.fee_bp as u64)
            .unwrap()
            .checked_div(10_000)
            .unwrap();

        match is_x {
            true => self.pool.fee_collected_x += swap_fee,
            false => self.pool.fee_collected_y += swap_fee,
        }

        // amount after fee which we need to calc the amount of out token to the user
        let amount_after_fee = amount_in.checked_sub(swap_fee).unwrap();

        // vault balances
        let vault_x = self.ata_x.amount;
        let vault_y = self.ata_y.amount;

        // constant K = X * Y
        let k = vault_x.checked_mul(vault_y).unwrap();

        let new_vault_x = if is_x {
            vault_x.checked_add(amount_after_fee).unwrap()
        } else {
            vault_x
        };

        let new_vault_y = if !is_x {
            vault_y.checked_add(amount_after_fee).unwrap()
        } else {
            vault_y
        };

        let amount_out = if is_x {
            let new_vault_y = (k as u128).checked_div(new_vault_x as u128).unwrap() as u64;

            let amount_out = vault_y.checked_sub(new_vault_y).unwrap();

            amount_out
        } else {
            let new_vault_x = (k as u128).checked_div(new_vault_y as u128).unwrap() as u64;

            let amount_out = vault_x.checked_sub(new_vault_x).unwrap();
            amount_out
        };

        require!(amount_out >= amount_out_min, PoolError::SlippageExceeded);

        self.deposit_tokens_from_user(is_x, amount_in)?;
        self.deposit_tokens_to_user(is_x, amount_out)?;

        Ok(())
    }

    fn deposit_tokens_from_user(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (
                self.user_ata_x.to_account_info(),
                self.ata_x.to_account_info(),
            ),
            false => (
                self.user_ata_y.to_account_info(),
                self.ata_y.to_account_info(),
            ),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(ctx, amount)?;

        Ok(())
    }

    fn deposit_tokens_to_user(&mut self, is_x: bool, amount: u64) -> Result<()> {
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

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(ctx, amount)?;

        Ok(())
    }
}
