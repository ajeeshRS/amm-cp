use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{Pool, LP_TOKEN_MINT_SEED, POOL_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint_x,
        associated_token::authority = pool,
    )]
    pub ata_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint_y,
        associated_token::authority = pool,
    )]
    pub ata_y: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = pool,
        seeds = [LP_TOKEN_MINT_SEED.as_bytes(),pool.key().as_ref()],
        bump
    )]
    pub lp_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = 8 + Pool::INIT_SPACE,
        seeds = [POOL_SEED.as_bytes(),authority.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_pool(
        &mut self,
        fee_bp: u16,
        lp_supply: u64,
        bumps: InitializeBumps,
    ) -> Result<()> {
        self.pool.set_inner(Pool {
            authority: self.authority.key(),
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            ata_x: self.ata_x.key(),
            ata_y: self.ata_y.key(),
            lp_mint: self.lp_token_mint.key(),
            lp_supply,
            fee_bp,
            fee_collected_x: 0,
            fee_collected_y: 0,
            pool_bump: bumps.pool,
            lp_bump: bumps.lp_token_mint,
            locked: false,
        });
        Ok(())
    }
}
