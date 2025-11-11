use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub authority: Pubkey,

    pub mint_x: Pubkey,
    pub mint_y: Pubkey,

    pub ata_x: Pubkey,
    pub ata_y: Pubkey,

    pub lp_mint: Pubkey,
    pub lp_supply: u64,

    pub fee_bp: u16,

    pub fee_collected_x: u64,
    pub fee_collected_y: u64,

    pub pool_bump: u8,
    pub lp_bump: u8,

    pub locked: bool,
}
