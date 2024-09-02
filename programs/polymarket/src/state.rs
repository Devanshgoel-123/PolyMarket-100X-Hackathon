use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

pub const MARKET_AUTH: &str = "market authority";
pub const MARKET_STATE: &str = "state";
pub const MARKET_VAULT: &str = "vault";
pub const CENTRAL_TOKEN_MINT: &str = "central token mint";

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum BetStatus {
    Active,
    BettingClose,
    Resolved,
    Cancelled,
}
#[account]
pub struct MarketState {
    pub marketname: String,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub betArray: Vec<Pubkey>,
    pub totalBets: u64,
    pub market_authority_bump: u8,
    pub market_state_bump: u8,
    pub market_vault_bump: u8,
    pub creator: Pubkey,
    pub balance: u64,
}

#[account]
pub struct Bet {
    pub betTitle: String,
    pub betDescription: String,
    pub betStatus: BetStatus,
    pub betId: u64,
    pub betOutcomes: Vec<bool>,
    pub totalStake: u64,
    pub betMarket: String,
    pub betCreatedAt: String,
    pub betEndTime: String,
    pub betCreator: Pubkey,
    pub token_mint: Pubkey,
    pub users: Vec<Pubkey>,
    pub market_authority: Pubkey,
}
