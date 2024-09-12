use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface};
pub fn get_bet(
    _ctx: Context<GetAParticularBet>,
    market_name: String,
    bet_title: String,
    bet_description: String,
) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(market_name:String,bet_title:String,bet_description:String)]
pub struct GetAParticularBet<'info> {
    #[account(
        mut,
        seeds=[market_name.as_bytes(),bet_title.as_bytes(),bet_description.as_bytes()],
        bump
    )]
    pub bet_state: Account<'info, Bet>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}
