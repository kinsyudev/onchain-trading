use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    instruction::{InstructionMetadata, InstructionProcessorInputType},
    metrics::MetricsCollection,
    processor::Processor,
};
use carbon_raydium_amm_v4_decoder::{RaydiumAmmV4Instruction, SwapInstructionBaseIn, SwapInstructionBaseOut};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::messaging::MessagePublisher;
use crate::models::{RaydiumSwap, SwapEvent};

pub struct RaydiumSwapProcessor {
    publisher: MessagePublisher,
    swap_sender: mpsc::Sender<SwapEvent>,
}

impl RaydiumSwapProcessor {
    pub fn new(publisher: MessagePublisher, swap_sender: mpsc::Sender<SwapEvent>) -> Self {
        Self {
            publisher,
            swap_sender,
        }
    }

    async fn process_swap_base_in(
        &self,
        metadata: &InstructionMetadata,
        swap: &SwapInstructionBaseIn,
    ) -> CarbonResult<()> {
        debug!(
            "Processing SwapBaseIn - Signature: {}, Amount In: {}",
            metadata.signature, swap.amount_in
        );

        // Extract account keys for pool and trader
        let pool_id = metadata.account_keys.get(1)
            .map(|k| k.to_string())
            .unwrap_or_default();
        
        let trader = metadata.account_keys.get(16)
            .map(|k| k.to_string())
            .unwrap_or_default();

        // Extract token mints from the instruction accounts
        let token_a_mint = metadata.account_keys.get(8)
            .map(|k| k.to_string())
            .unwrap_or_default();
        
        let token_b_mint = metadata.account_keys.get(9)
            .map(|k| k.to_string())
            .unwrap_or_default();

        let raydium_swap = RaydiumSwap {
            signature: metadata.signature.clone(),
            slot: metadata.slot,
            timestamp: metadata.block_time,
            pool_id: pool_id.clone(),
            token_a_mint: token_a_mint.clone(),
            token_b_mint: token_b_mint.clone(),
            amount_in: swap.amount_in,
            amount_out: swap.minimum_amount_out, // Actual amount will be in logs
            is_token_a_to_b: true, // SwapBaseIn typically goes A to B
            trader: trader.clone(),
            fee_amount: 0, // Calculate from pool state
            price: 0.0, // Will be calculated
            pool_liquidity_a: 0, // Need to fetch from account data
            pool_liquidity_b: 0, // Need to fetch from account data
        };

        let event = SwapEvent::new(raydium_swap);
        
        // Send to processing channel
        if let Err(e) = self.swap_sender.send(event.clone()).await {
            error!("Failed to send swap event to channel: {}", e);
        }

        // Also publish directly for immediate processing
        if let Err(e) = self.publisher.publish_swap_event(event).await {
            error!("Failed to publish swap event: {}", e);
        }

        Ok(())
    }

    async fn process_swap_base_out(
        &self,
        metadata: &InstructionMetadata,
        swap: &SwapInstructionBaseOut,
    ) -> CarbonResult<()> {
        debug!(
            "Processing SwapBaseOut - Signature: {}, Amount Out: {}",
            metadata.signature, swap.amount_out
        );

        // Extract account keys for pool and trader
        let pool_id = metadata.account_keys.get(1)
            .map(|k| k.to_string())
            .unwrap_or_default();
        
        let trader = metadata.account_keys.get(16)
            .map(|k| k.to_string())
            .unwrap_or_default();

        // Extract token mints
        let token_a_mint = metadata.account_keys.get(8)
            .map(|k| k.to_string())
            .unwrap_or_default();
        
        let token_b_mint = metadata.account_keys.get(9)
            .map(|k| k.to_string())
            .unwrap_or_default();

        let raydium_swap = RaydiumSwap {
            signature: metadata.signature.clone(),
            slot: metadata.slot,
            timestamp: metadata.block_time,
            pool_id: pool_id.clone(),
            token_a_mint: token_a_mint.clone(),
            token_b_mint: token_b_mint.clone(),
            amount_in: swap.max_amount_in, // Max amount willing to pay
            amount_out: swap.amount_out,
            is_token_a_to_b: true, // Determine from account order
            trader: trader.clone(),
            fee_amount: 0,
            price: 0.0,
            pool_liquidity_a: 0,
            pool_liquidity_b: 0,
        };

        let event = SwapEvent::new(raydium_swap);
        
        // Send to processing channel
        if let Err(e) = self.swap_sender.send(event.clone()).await {
            error!("Failed to send swap event to channel: {}", e);
        }

        // Also publish directly
        if let Err(e) = self.publisher.publish_swap_event(event).await {
            error!("Failed to publish swap event: {}", e);
        }

        Ok(())
    }
}

#[async_trait]
impl Processor for RaydiumSwapProcessor {
    type InputType = InstructionProcessorInputType<RaydiumAmmV4Instruction>;

    async fn process(
        &mut self,
        input: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, instruction, _accounts, _remaining_accounts) = input;

        match instruction.data {
            RaydiumAmmV4Instruction::SwapBaseIn(swap) => {
                self.process_swap_base_in(&metadata, &swap).await?;
            }
            RaydiumAmmV4Instruction::SwapBaseOut(swap) => {
                self.process_swap_base_out(&metadata, &swap).await?;
            }
            _ => {
                // We only care about swap instructions
                debug!("Ignoring non-swap instruction: {:?}", metadata.signature);
            }
        }

        Ok(())
    }
}