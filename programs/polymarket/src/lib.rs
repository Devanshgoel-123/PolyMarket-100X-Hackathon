pub mod instructions;
use anchor_lang::prelude::*;
use instructions::*;
pub mod state;
declare_id!("8AFtVMxdjwHrLJuFt3FTCHwuYbXUAqpBeDhdTtEcyqm2");

#[program]
pub mod polymarket {
    use super::*;
    pub fn create_market(ctx: Context<InitializeMarket>, market_name: String) -> Result<()> {
        create_market::create_market(ctx, market_name);
        Ok(())
    }
    pub fn create_bet(
        ctx: Context<InitializeBet>,
        market_name: String,
        bet_title: String,
        bet_description: String,
    ) -> Result<()> {
        create_bet::create_bet(ctx, market_name, bet_title, bet_description);
        Ok(())
    }
}
