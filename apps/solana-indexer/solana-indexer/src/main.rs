use {
    async_trait::async_trait,
    carbon_core::{
        deserialize::ArrangeAccounts,
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics, 
    carbon_pumpfun_decoder::{instructions::PumpfunInstruction, PumpfunDecoder, PROGRAM_ID as PUMPFUN_PROGRAM_ID,}, 
    carbon_raydium_amm_v4_decoder::{instructions::RaydiumAmmV4Instruction, RaydiumAmmV4Decoder, PROGRAM_ID as RAYDIUMAMMV4_PROGRAM_ID,},
    std::{env, sync::Arc},
    std::collections::HashMap,
    carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient,
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();
    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![
                PUMPFUN_PROGRAM_ID.to_string().clone(),
                RAYDIUMAMMV4_PROGRAM_ID.to_string().clone(), 
            ],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![
            PUMPFUN_PROGRAM_ID.to_string().clone(),
            RAYDIUMAMMV4_PROGRAM_ID.to_string().clone(), 
        ],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("transaction_filter".to_string(), transaction_filter);

    let datasource = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(5)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .instruction(RaydiumAmmV4Decoder, RaydiumAmmV4InstructionProcessor) 
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
} 
pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<PumpfunInstruction>,
        NestedInstructions,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;

        match instruction.data {
            _ => {
                log::info!("received the Pumpfun instruction, sig: {}, accounts len: {}", signature, accounts.len());
            }
        };

        Ok(())
    }
} 
pub struct RaydiumAmmV4InstructionProcessor;

#[async_trait]
impl Processor for RaydiumAmmV4InstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<RaydiumAmmV4Instruction>,
        NestedInstructions,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;

        match instruction.data {
            _ => {
                log::info!("received the RaydiumAmmV4 instruction, sig: {}, accounts len: {}", signature, accounts.len());
            }
        };

        Ok(())
    }
} 