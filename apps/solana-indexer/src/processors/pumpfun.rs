use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_pumpfun_decoder::instructions::PumpfunInstruction,
    crate::messaging::RabbitMQPublisher,
    solana_sdk::instruction::Instruction,
    std::sync::Arc,
};

pub struct PumpfunInstructionProcessor {
    publisher: RabbitMQPublisher,
}

impl PumpfunInstructionProcessor {
    pub fn new(rabbitmq_url: String) -> Self {
        Self {
            publisher: RabbitMQPublisher::new(rabbitmq_url),
        }
    }
}

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<PumpfunInstruction>,
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
            PumpfunInstruction::TradeEvent(event) => {
                log::info!(
                    "💱 Pumpfun TradeEvent - Signature: {}, Slot: {}, Mint: {}, Sol Amount: {}, Token Amount: {}, Is Buy: {}, Accounts: {}",
                    signature,
                    slot,
                    event.mint,
                    event.sol_amount,
                    event.token_amount,
                    event.is_buy,
                    accounts.len()
                );

                // Create and publish trade event to RabbitMQ
                let trade_event = crate::messaging::PumpfunTradeEvent {
                    signature: signature.to_string(),
                    slot,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string(),
                    mint: event.mint.to_string(),
                    sol_amount: event.sol_amount.to_string(),
                    token_amount: event.token_amount.to_string(),
                    is_buy: event.is_buy,
                    trader: event.user.to_string(),
                };

                if let Err(e) = self.publisher.publish_pumpfun_trade(trade_event).await {
                    log::error!("Failed to publish Pumpfun trade event: {}", e);
                }
            }
            _ => {
                log::debug!(
                    "📝 Pumpfun Instruction (not trade) - Signature: {}, Slot: {}, Accounts: {}",
                    signature,
                    slot,
                    accounts.len()
                );
            }
        }

        Ok(())
    }
}
