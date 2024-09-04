use crate::state::*;
use anchor_lang::{prelude::*, system_program::{CreateAccount, CreateAccountWithSeed}};
use anchor_spl::{token::*, token_interface,};
pub fn create_mint_ata(_ctx: Context<InitializeVault>) -> Result<()> {
    let cpi_accounts=InitializeAccount{
        account:_ctx.accounts.token_vault,
        mint:
    }
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        seeds=[CENTRAL_TOKEN_MINT_VAULT.as_bytes()],
        bump,
        payer=payer,
        space=8+TokenAccount::LEN
    )]
    pub token_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        seeds=[CENTRAL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub rent: Sysvar<'info, Rent>,
}
