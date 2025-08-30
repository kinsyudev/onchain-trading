use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_raydium_amm_v4_decoder::instructions::RaydiumAmmV4Instruction,
    solana_sdk::instruction::Instruction,
    std::sync::Arc,
};

pub struct RaydiumAmmV4InstructionProcessor;

impl RaydiumAmmV4InstructionProcessor {
    pub fn new() -> Self {
        Self
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
            }
            RaydiumAmmV4Instruction::Initialize(init) => {
                log::info!(
                    "🏗️ Raydium Initialize - Signature: {}, Slot: {}, Nonce: {}, Open Time: {}, Accounts: {}",
                    signature,
                    slot,
                    init.nonce,
                    init.open_time,
                    accounts.len()
                );
            }
            RaydiumAmmV4Instruction::Deposit(deposit) => {
                log::info!(
                    "💰 Raydium Deposit - Signature: {}, Slot: {}, Max Coin Amount: {}, Max PC Amount: {}, Base Side: {}, Accounts: {}",
                    signature,
                    slot,
                    deposit.max_coin_amount,
                    deposit.max_pc_amount,
                    deposit.base_side,
                    accounts.len()
                );
            }
            RaydiumAmmV4Instruction::Withdraw(withdraw) => {
                log::info!(
                    "💸 Raydium Withdraw - Signature: {}, Slot: {}, Amount: {}, Accounts: {}",
                    signature,
                    slot,
                    withdraw.amount,
                    accounts.len()
                );
            }
            _ => {
                log::info!(
                    "📝 Raydium Instruction - Signature: {}, Slot: {}, Accounts: {}",
                    signature,
                    slot,
                    accounts.len()
                );
            }
        }

        Ok(())
    }
}
