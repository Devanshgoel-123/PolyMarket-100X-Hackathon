use anchor_lang::prelude::*;
use {crate::state::*, anchor_spl::*};
pub fn create_central_mint(ctx: Context<InitializeCentralMint>) -> Result<()> {
    let cpi_accounts = token::InitializeMint {
        mint: ctx.accounts.token_mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::initialize_mint(cpi_ctx, 9, ctx.accounts.payer.key, None)?;

    let cpi_vault_accounts = token::InitializeAccount {
        account: ctx.accounts.token_vault.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_vault_ctx = CpiContext::new(cpi_program, cpi_vault_accounts);
    token::initialize_account(cpi_vault_ctx)?;
    msg!("Mint Address:{}", ctx.accounts.token_mint.key());
    msg!("Vault Address:{}", ctx.accounts.token_vault.key());
    Ok(())
}
#[derive(Accounts)]
pub struct InitializeCentralMint<'info> {
    #[account(
        init,
        seeds = [b"central_token_mint".as_ref()],
        bump,
        payer=payer,
        mint::token_program=token_program,
        mint::authority=payer,
        mint::decimals=9,
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(
        seeds=[b"central_token_mint_vault".as_ref()],
        bump,
        token::mint=token_mint,
        token::authority=payer
    )]
    pub token_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}
