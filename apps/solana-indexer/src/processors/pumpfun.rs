use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_pumpfun_decoder::instructions::PumpfunInstruction,
    solana_sdk::instruction::Instruction,
    std::sync::Arc,
};

pub struct PumpfunInstructionProcessor;

impl PumpfunInstructionProcessor {
    pub fn new() -> Self {
        Self
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
            PumpfunInstruction::CreateEvent(event) => {
                log::info!(
                    "🚀 Pumpfun CreateEvent - Signature: {}, Slot: {}, Mint: {}, Name: {}, Symbol: {}, Accounts: {}",
                    signature,
                    slot,
                    event.mint,
                    event.name,
                    event.symbol,
                    accounts.len()
                );
            }
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
            }
            PumpfunInstruction::CompleteEvent(event) => {
                log::info!(
                    "🎯 Pumpfun CompleteEvent - Signature: {}, Slot: {}, Mint: {}, Accounts: {}",
                    signature,
                    slot,
                    event.mint,
                    accounts.len()
                );
            }
            _ => {
                log::info!(
                    "📝 Pumpfun Instruction - Signature: {}, Slot: {}, Accounts: {}",
                    signature,
                    slot,
                    accounts.len()
                );
            }
        }

        Ok(())
    }
}
