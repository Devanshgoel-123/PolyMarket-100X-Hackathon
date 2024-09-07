use anchor_lang::prelude::*;
use anchor_spl::{token, token_2022::mint_to};
use {
    crate::state::*,
    anchor_spl::{
        token_2022::{
            initialize_account, initialize_mint, InitializeAccount, InitializeMint, MintTo,
        },
        token_interface,
    },
};

pub fn create_central_mint(ctx: Context<InitializeCentralMint>) -> Result<()> {
    msg!("Mint Address: {}", ctx.accounts.token_mint.key());
    msg!("Vault Address: {}", ctx.accounts.token_vault.key());

    // if mint_account_data[0] != 0 {
    //     return Err(ProgramError::AccountAlreadyInitialized.into());
    // }
    // // Initialize the mint account
    // let cpi_accounts = InitializeMint {
    //     mint: ctx.accounts.token_mint.to_account_info(),
    //     rent: ctx.accounts.rent.to_account_info(),
    // };
    // let cpi_program = ctx.accounts.token_program.to_account_info();
    // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    // initialize_mint(cpi_ctx, 9, ctx.accounts.payer.key, None)?;
    // let info=createTramsh
    // Initialize the vault account
    // let cpi_vault_accounts = InitializeAccount {
    //     account: ctx.accounts.token_vault.to_account_info(),
    //     mint: ctx.accounts.token_mint.to_account_info(),
    //     authority: ctx.accounts.payer.to_account_info(),
    //     rent: ctx.accounts.rent.to_account_info(),
    // };
    // let cpi_vault_program = ctx.accounts.token_program.to_account_info();
    // let cpi_vault_ctx = CpiContext::new(cpi_vault_program, cpi_vault_accounts);
    // initialize_account(cpi_vault_ctx)?;

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = MintTo {
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.token_vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    mint_to(cpi_context, 10000)?;
    let mint_info = ctx.accounts.token_mint.supply;
    msg!("The amount od token is : {}", mint_info);
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCentralMint<'info> {
    #[account(
        init_if_needed,
        seeds = [b"central_token_mint".as_ref()],
        bump,
        payer = payer,
        mint::token_program = token_program,
        mint::authority = payer,
        mint::decimals = 9,
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,

    #[account(
        init_if_needed,
        seeds = [b"central_token_mint_vault".as_ref()],
        bump,
        payer=payer,
        token::mint = token_mint,
        token::authority = payer
    )]
    pub token_vault: InterfaceAccount<'info, token_interface::TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}
