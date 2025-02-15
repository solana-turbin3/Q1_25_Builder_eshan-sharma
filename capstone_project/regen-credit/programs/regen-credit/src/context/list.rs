use crate::CarbonCredit;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"carbon_credit".as_ref(), maker.key().as_ref()],
        bump = carbon_credit.bump
    )]
    pub carbon_credit: Account<'info, CarbonCredit>,
    pub system_program: Program<'info, System>,
}
impl<'info> List<'info> {
    pub fn list(&mut self) -> Result<()> {
        self.carbon_credit.listed = true;
        Ok(())
    }
}
