use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::Config;



#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        init, 
        payer=initializer,
        seeds=[b"lp",config.key.as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=config
    )]
    pub mint_lp:Account<'info, Mint>,
    pub system_program:Program<'info, System>,
    pub token_program:Program<'info, Token>,

    #[account(
        init, 
        payer=initializer,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_x:Account<'info,TokenAccount>,
    #[account(
        init, 
        payer=initializer,
        associated_token::mint=mint_y,
        associated_token::authority=config
    )]
    pub vault_y:Account<'info,TokenAccount>,
    pub associated_token_program:Program<'info, AssociatedToken>,

    #[account(
        init, 
        payer=initializer,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump,
        space=Config::INIT_SPACE
    )]
    pub config:Account<'info, Config>
}
