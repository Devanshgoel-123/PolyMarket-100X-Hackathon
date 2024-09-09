use anchor_lang::prelude::*;
use anchor_spl::token_2022::{initialize_account, InitializeAccount};
use std::mem::size_of;
use {crate::state::*, anchor_spl::*};
pub fn create_market(ctx: Context<InitializeMarket>, marketname: String) -> Result<()> {
    // let cpi_accounts = InitializeAccount {
    //     account: ctx.accounts.market_vault.to_account_info(),
    //     authority: ctx.accounts.market_authority.to_account_info(),
    //     mint: ctx.accounts.token_mint.to_account_info(),
    //     rent: ctx.accounts.rent.to_account_info(),
    // };
    // let cpi_program = ctx.accounts.token_program.to_account_info();
    // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    // initialize_account(cpi_ctx)?;
    let market_state = &mut ctx.accounts.market_state;
    market_state.marketname = marketname.clone();
    market_state.authority = ctx.accounts.market_authority.key();
    market_state.token_mint = ctx.accounts.token_mint.key();
    market_state.betArray = vec![]; //
    market_state.totalBets = 0;
    market_state.market_authority_bump = ctx.bumps.market_authority;
    market_state.market_state_bump = ctx.bumps.market_state;
    market_state.market_vault_bump = ctx.bumps.market_vault;
    market_state.creator = ctx.accounts.payer.key();
    market_state.balance = 0;
    msg!("{} Market Created", marketname);
    Ok(())
}

#[derive(Accounts)]
#[instruction(marketname:String)]
pub struct InitializeMarket<'info> {
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds=[marketname.as_bytes(),MARKET_AUTH.as_bytes()],
        bump
    )]
    pub market_authority: UncheckedAccount<'info>,
    #[account{
        init_if_needed,
        payer=payer,
        seeds=[marketname.as_bytes(),MARKET_STATE.as_bytes()],
        bump,
        space=8+size_of::<MarketState>()
    }]
    pub market_state: Account<'info, MarketState>,
    #[account(
        seeds=[CENTRAL_TOKEN_MINT.as_bytes()],
        bump,
        token::token_program=token_program
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(
        init_if_needed,//Creates the account if it doesnt alreeady doesnt exist
        payer=payer,
        seeds=[marketname.as_bytes(),token_mint.key().as_ref()],
        bump,
        token::mint=token_mint,
        token::authority=market_authority,
        token::token_program=token_program
    )]
    pub market_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}
