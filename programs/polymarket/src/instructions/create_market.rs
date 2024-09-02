use anchor_lang::{prelude::*, solana_program::stake::instruction};
use std::mem::size_of;
use {crate::state::*, anchor_spl::*};
pub fn create_market(ctx: Context<InitializeMarket>, marketname: String) -> Result<()> {
    let market_state = &mut ctx.accounts.market_state;
    market_state.marketname = marketname;
    market_state.authority = ctx.accounts.marketauthority.key();
    market_state.token_mint = ctx.accounts.token_mint_betting.key();
    market_state.betArray = vec![];
    market_state.totalBets = 0;
    market_state.market_authority_bump = ctx.bumps.marketauthority;
    market_state.market_state_bump = ctx.bumps.market_state;
    market_state.market_vault_bump = ctx.bumps.market_vault;
    market_state.creator = ctx.accounts.payer.key();
    market_state.balance = 0;
    Ok(())
}

#[derive(Accounts)]
#[instructio(marketname:String)]
pub struct InitializeMarket<'info> {
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds=[marketname.as_bytes(),MARKET_AUTH.as_bytes()],
        bump
    )]
    pub market_authority: UncheckedAccount<'info>,
    #[account{
        init,
        payer=payer,
        seeds=[marketname.as_bytes(),MARKET_STATE.as_bytes()],
        bump,
        space=8+size_of::<MarketState>()
    }]
    pub market_state: Account<'info, MarketState>,
    #[account(
        seeds=[CENTRAL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer=payer,
        seeds=[marketname.as_bytes(),],
        bump,
        token::mint=token_mint,
        token::authority=market_authority,
    )]
    pub market_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}
