mod config;
mod datasources;
mod messaging;
mod processors;

use {
    carbon_core::pipeline::{Pipeline, ShutdownStrategy},
    carbon_log_metrics::LogMetrics,
    carbon_pumpfun_decoder::PumpfunDecoder,
    carbon_raydium_amm_v4_decoder::RaydiumAmmV4Decoder,
    config::Config,
    datasources::{create_geyser_datasource, create_rpc_datasource},
    processors::{PumpfunInstructionProcessor, RaydiumAmmV4InstructionProcessor},
    std::sync::Arc,
};

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Load configuration
    let config = Config::from_env().map_err(|e| {
        log::error!("Failed to load configuration: {}", e);
        eyre::eyre!("Config error: {}", e)
    })?;
    
    // Validate configuration
    config.validate().map_err(|e| {
        log::error!("Invalid configuration: {}", e);
        eyre::eyre!("Validation error: {}", e)
    })?;

    log::info!("🚀 Starting Solana Indexer");
    log::info!("📊 Data Source: {:?}", config.data_source);
    log::info!("🔗 RPC URL: {}", config.rpc_url);
    log::info!("🌐 WebSocket URL: {}", config.rpc_ws_url);

    // Build the pipeline
    let mut pipeline_builder = Pipeline::builder();

    // Add datasources based on configuration
    use crate::config::DataSourceType;
    match config.data_source {
        DataSourceType::Geyser => {
            let datasource = create_geyser_datasource(&config).map_err(|e| {
                log::error!("Failed to create Geyser datasource: {}", e);
                eyre::eyre!("Geyser datasource error: {}", e)
            })?;
            pipeline_builder = pipeline_builder.datasource(datasource);
        }
        DataSourceType::Rpc => {
            let datasource = create_rpc_datasource(&config).map_err(|e| {
                log::error!("Failed to create RPC datasource: {}", e);
                eyre::eyre!("RPC datasource error: {}", e)
            })?;
            pipeline_builder = pipeline_builder.datasource(datasource);
        }
        DataSourceType::Both => {
            let geyser_datasource = create_geyser_datasource(&config).map_err(|e| {
                log::error!("Failed to create Geyser datasource: {}", e);
                eyre::eyre!("Geyser datasource error: {}", e)
            })?;
            let rpc_datasource = create_rpc_datasource(&config).map_err(|e| {
                log::error!("Failed to create RPC datasource: {}", e);
                eyre::eyre!("RPC datasource error: {}", e)
            })?;
            pipeline_builder = pipeline_builder
                .datasource(geyser_datasource)
                .datasource(rpc_datasource);
        }
    }

    // Add processors
    pipeline_builder = pipeline_builder
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor::new(config.rabbitmq_url.clone()))
        .instruction(RaydiumAmmV4Decoder, RaydiumAmmV4InstructionProcessor::new(config.rabbitmq_url.clone()));

    // Configure metrics and run
    let mut pipeline = pipeline_builder
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(5)
        .shutdown_strategy(ShutdownStrategy::ProcessPending)
        .build().map_err(|e| eyre::eyre!("Pipeline build error: {}", e))?;

    log::info!("✅ Pipeline built successfully, starting to process transactions...");

    pipeline.run().await.map_err(|e| eyre::eyre!("Pipeline error: {}", e))?;

    log::info!("🛑 Indexer stopped");
    Ok(())
} 