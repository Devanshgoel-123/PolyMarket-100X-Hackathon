pub mod instructions;
use anchor_lang::prelude::*;
use instructions::*;
pub mod state;
declare_id!("B73FqiLsiHmCPt17LmTdP5iKKfr1K28RkUUrgqGSK8Cg");

#[program]
pub mod polymarket {
    use super::*;
    pub fn create_central_mint(_ctx: Context<InitializeCentralMint>) -> Result<()> {
        create_mint::create_central_mint(_ctx)?;
        Ok(())
    }
    // pub fn create_market(_ctx: Context<InitializeMarket>, market_name: String) -> Result<()> {
    //     create_market::create_market(_ctx, market_name)?;
    //     Ok(())
    // }
    // pub fn create_bet(
    //     _ctx: Context<InitializeBet>,
    //     market_name: String,
    //     bet_title: String,
    //     bet_description: String,
    // ) -> Result<()> {
    //     create_bet::create_bet(_ctx, market_name, bet_title, bet_description)?;
    //     Ok(())
    // }
}
