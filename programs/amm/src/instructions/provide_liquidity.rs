use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};

use crate::{
    error::PoolError,
    helper::{get_lp_token_amount_init, get_lp_tokens_to_mint, xy_from_l},
    Pool, LP_TOKEN_MINT_SEED, POOL_SEED,
};

#[derive(Accounts)]
pub struct ProvideLiquidity<'info> {
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

impl<'info> ProvideLiquidity<'info> {
    pub fn deposit_tokens(
        &mut self,
        // lp_token_asking: u64,
        max_x_token: u64,
        max_y_token: u64,
    ) -> Result<()> {
        require!(self.pool.locked == false, PoolError::PoolIsLocked);

        let (x, y, amount) = match self.lp_token_mint.supply == 0
            && self.ata_x.amount == 0
            && self.ata_y.amount == 0
        {
            true => {
                let lp_tokens_to_mint = get_lp_token_amount_init(max_x_token, max_y_token).unwrap();
                (max_x_token, max_y_token, lp_tokens_to_mint)
            }
            false => {
                let total_lp_supply = self.lp_token_mint.supply;
                let vault_x = self.ata_x.amount;
                let vault_y = self.ata_y.amount;

                let lp_tokens_to_mint = get_lp_tokens_to_mint(
                    total_lp_supply,
                    vault_x,
                    vault_y,
                    max_x_token,
                    max_y_token,
                )
                .unwrap();

                let (required_x, required_y) =
                    xy_from_l(total_lp_supply, vault_x, vault_y, lp_tokens_to_mint).unwrap();

                (required_x, required_y, lp_tokens_to_mint)
            }
        };

        require!(x <= max_x_token, PoolError::SlippageExceeded);
        require!(y <= max_y_token, PoolError::SlippageExceeded);

        let _ = self.token_depositor(true, x);
        let _ = self.token_depositor(false, y);

        let _ = self.mint_lp_tokens(amount);

        Ok(())
    }

    fn token_depositor(&mut self, is_x: bool, amount: u64) -> Result<()> {
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

    fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.lp_token_mint.to_account_info(),
            to: self.user_lp_ata.to_account_info(),
            authority: self.pool.to_account_info(),
        };

        let auth_key = self.authority.key();
        let seeds = &[
            POOL_SEED.as_bytes(),
            auth_key.as_ref(),
            &[self.pool.pool_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(ctx, amount)?;

        self.pool.lp_supply += amount;
        Ok(())
    }
}
