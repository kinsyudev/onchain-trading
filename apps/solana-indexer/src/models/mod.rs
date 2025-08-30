use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaydiumSwap {
    pub signature: String,
    pub slot: u64,
    pub timestamp: i64,
    pub pool_id: String,
    pub token_a_mint: String,
    pub token_b_mint: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub is_token_a_to_b: bool,
    pub trader: String,
    pub fee_amount: u64,
    pub price: f64,
    pub pool_liquidity_a: u64,
    pub pool_liquidity_b: u64,
}

impl RaydiumSwap {
    pub fn calculate_price(&self) -> f64 {
        if self.is_token_a_to_b {
            self.amount_out as f64 / self.amount_in as f64
        } else {
            self.amount_in as f64 / self.amount_out as f64
        }
    }

    pub fn to_message(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapEvent {
    pub event_type: String,
    pub chain: String,
    pub dex: String,
    pub data: RaydiumSwap,
    pub indexed_at: DateTime<Utc>,
}

impl SwapEvent {
    pub fn new(swap: RaydiumSwap) -> Self {
        SwapEvent {
            event_type: "swap".to_string(),
            chain: "solana".to_string(),
            dex: "raydium".to_string(),
            data: swap,
            indexed_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolState {
    pub pool_id: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
}

impl PoolState {
    pub fn fee_rate(&self) -> f64 {
        self.fee_numerator as f64 / self.fee_denominator as f64
    }
}