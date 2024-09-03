use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface};
use chrono::prelude::*;
use std::mem::size_of;
pub fn create_bet(
    ctx: Context<InitializeBet>,
    market_name: String,
    bet_title: String,
    bet_description: String,
) -> Result<()> {
    let bet_state = &mut ctx.accounts.bet_state;
    let market_state = &mut ctx.accounts.market_state;
    bet_state.betTitle = bet_title;
    bet_state.betDescription = bet_description;
    bet_state.betStatus = BetStatus::Active;
    bet_state.betId = bet_state.betId.checked_add(1).unwrap();
    bet_state.betOutcomes = vec![true, false];
    bet_state.totalStake = 0;
    bet_state.betMarket = market_name;
    let local_time: DateTime<Local> = Local::now();
    bet_state.betCreatedAt = local_time.to_string();
    bet_state.betEndTime = "0".to_string();
    bet_state.token_mint = ctx.accounts.token_mint.key();
    bet_state.users.push(ctx.accounts.payer.key());
    bet_state.market_authority = ctx.accounts.market_authority.key();
    market_state.betArray.push(ctx.accounts.bet_state.key());
    market_state.totalBets = market_state.totalBets.checked_add(1).unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction(market_name:String,bet_title:String,bet_description:String)]
pub struct InitializeBet<'info> {
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds=[market_name.as_bytes(),MARKET_AUTH.as_bytes()],
        bump
       )]
    pub market_authority: UncheckedAccount<'info>,
    #[account(
        seeds=[MARKET_STATE.as_bytes(),market_name.as_bytes()],
        bump,
    )]
    pub market_state: Account<'info, MarketState>,
    #[account(
        init,
        seeds=[payer.key().as_ref(),market_name.as_bytes(),bet_title.as_bytes()], //Think of seeds to find the particular bet account,
        payer=payer,
        space=8+size_of::<Bet>(),
        bump
    )]
    pub bet_state: Account<'info, Bet>,
    #[account(
        mint::token_program=token_program
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
