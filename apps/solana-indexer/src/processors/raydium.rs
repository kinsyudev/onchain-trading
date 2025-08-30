use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_raydium_amm_v4_decoder::instructions::RaydiumAmmV4Instruction,
    crate::messaging::RabbitMQPublisher,
    solana_sdk::instruction::Instruction,
    std::sync::Arc,
};

pub struct RaydiumAmmV4InstructionProcessor {
    publisher: RabbitMQPublisher,
}

impl RaydiumAmmV4InstructionProcessor {
    pub fn new(rabbitmq_url: String) -> Self {
        Self {
            publisher: RabbitMQPublisher::new(rabbitmq_url),
        }
    }
}

#[async_trait]
impl Processor for RaydiumAmmV4InstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<RaydiumAmmV4Instruction>,
        NestedInstructions,
        Instruction,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions, _raw_instruction): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;
        let slot = metadata.transaction_metadata.slot;

        match &instruction.data {
            RaydiumAmmV4Instruction::SwapBaseIn(swap) => {
                log::info!(
                    "🔄 Raydium SwapBaseIn - Signature: {}, Slot: {}, Amount In: {}, Min Amount Out: {}, Accounts: {}",
                    signature,
                    slot,
                    swap.amount_in,
                    swap.minimum_amount_out,
                    accounts.len()
                );

                // Create and publish trade event to RabbitMQ
                let trade_event = crate::messaging::RaydiumTradeEvent {
                    signature: signature.to_string(),
                    slot,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    program_id: "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string(),
                    swap_type: "swapBaseIn".to_string(),
                    pool_id: "unknown".to_string(), // TODO: Extract pool ID from accounts
                    user: "unknown".to_string(),    // TODO: Extract user from accounts
                    amount_in: Some(swap.amount_in.to_string()),
                    minimum_amount_out: Some(swap.minimum_amount_out.to_string()),
                    max_amount_in: None,
                    amount_out: None,
                };

                if let Err(e) = self.publisher.publish_raydium_trade(trade_event).await {
                    log::error!("Failed to publish Raydium SwapBaseIn event: {}", e);
                }
            }
            RaydiumAmmV4Instruction::SwapBaseOut(swap) => {
                log::info!(
                    "🔄 Raydium SwapBaseOut - Signature: {}, Slot: {}, Max Amount In: {}, Amount Out: {}, Accounts: {}",
                    signature,
                    slot,
                    swap.max_amount_in,
                    swap.amount_out,
                    accounts.len()
                );

                // Create and publish trade event to RabbitMQ
                let trade_event = crate::messaging::RaydiumTradeEvent {
                    signature: signature.to_string(),
                    slot,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    program_id: "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string(),
                    swap_type: "swapBaseOut".to_string(),
                    pool_id: "unknown".to_string(), // TODO: Extract pool ID from accounts
                    user: "unknown".to_string(),    // TODO: Extract user from accounts
                    amount_in: None,
                    minimum_amount_out: None,
                    max_amount_in: Some(swap.max_amount_in.to_string()),
                    amount_out: Some(swap.amount_out.to_string()),
                };

                if let Err(e) = self.publisher.publish_raydium_trade(trade_event).await {
                    log::error!("Failed to publish Raydium SwapBaseOut event: {}", e);
                }
            }
            _ => {
                log::debug!(
                    "📝 Raydium Instruction (not swap) - Signature: {}, Slot: {}, Accounts: {}",
                    signature,
                    slot,
                    accounts.len()
                );
            }
        }

        Ok(())
    }
}
