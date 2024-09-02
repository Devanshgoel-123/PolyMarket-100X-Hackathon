use anchor_lang::prelude::*;

declare_id!("HSz3AezvmqLYVrpUFPbKf8yMLs2tVs1pDh6juqGGmvrC");

#[program]
pub mod polymarket {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
