mod config;
mod health;
mod messaging;
mod models;
mod processors;

use anyhow::{Context, Result};
use carbon_core::{datasource::Datasource, metrics::MetricsCollection, pipeline::Pipeline};
use carbon_prometheus_metrics::PrometheusMetrics;
use carbon_raydium_amm_v4_decoder::RaydiumAmmV4Decoder;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::Config;
use health::{start_health_server, HealthState};
use messaging::MessagePublisher;
use models::SwapEvent;
use processors::RaydiumSwapProcessor;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let config = Config::from_env()?;
    init_tracing(&config.monitoring.log_level)?;

    info!("Starting Solana Raydium Indexer");
    info!("RPC URL: {}", config.solana.rpc_url);
    info!("Raydium Program ID: {}", config.indexer.raydium_program_id);

    // Initialize RabbitMQ publisher
    let publisher = MessagePublisher::new(config.rabbitmq.clone())
        .await
        .context("Failed to initialize RabbitMQ publisher")?;

    // Create channel for swap events
    let (swap_sender, mut swap_receiver) = mpsc::channel::<SwapEvent>(1000);

    // Initialize health state
    let health_state = HealthState {
        start_time: chrono::Utc::now(),
        events_processed: Arc::new(RwLock::new(0)),
        last_event_time: Arc::new(RwLock::new(None)),
        publisher: publisher.clone(),
    };

    // Spawn health check server
    let health_state_clone = health_state.clone();
    tokio::spawn(async move {
        start_health_server(config.monitoring.health_check_port, health_state_clone).await;
    });

    // Spawn message processor task
    let events_processed = health_state.events_processed.clone();
    let last_event_time = health_state.last_event_time.clone();
    let publisher_clone = publisher.clone();
    
    tokio::spawn(async move {
        let mut batch = Vec::new();
        let batch_size = 10;
        let batch_timeout = tokio::time::Duration::from_secs(1);
        let mut last_batch_time = tokio::time::Instant::now();

        loop {
            tokio::select! {
                Some(event) = swap_receiver.recv() => {
                    batch.push(event);
                    
                    if batch.len() >= batch_size {
                        process_batch(&publisher_clone, &batch, &events_processed, &last_event_time).await;
                        batch.clear();
                        last_batch_time = tokio::time::Instant::now();
                    }
                }
                _ = tokio::time::sleep_until(last_batch_time + batch_timeout) => {
                    if !batch.is_empty() {
                        process_batch(&publisher_clone, &batch, &events_processed, &last_event_time).await;
                        batch.clear();
                        last_batch_time = tokio::time::Instant::now();
                    }
                }
            }
        }
    });

    // Initialize processor
    let processor = RaydiumSwapProcessor::new(publisher.clone(), swap_sender);

    // Initialize metrics
    let metrics = Arc::new(PrometheusMetrics::new());

    // Create datasource based on configuration
    let datasource = create_datasource(&config).await?;

    // Build and run pipeline
    info!("Starting Carbon pipeline...");
    
    Pipeline::builder()
        .datasource(datasource)
        .instruction(RaydiumAmmV4Decoder::new(), processor)
        .metrics(metrics)
        .build()?
        .run()
        .await?;

    Ok(())
}

fn init_tracing(log_level: &str) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(env_filter)
        .init();

    Ok(())
}

async fn create_datasource(config: &Config) -> Result<Box<dyn Datasource>> {
    // Check if we should use Yellowstone (production)
    #[cfg(feature = "yellowstone")]
    if let (Some(url), Some(token)) = (&config.solana.yellowstone_url, &config.solana.yellowstone_token) {
        info!("Using Yellowstone gRPC datasource");
        
        use carbon_yellowstone_grpc_datasource::{YellowstoneGrpcDatasource, YellowstoneGrpcDatasourceConfig};
        use solana_sdk::{commitment_config::CommitmentLevel, pubkey::Pubkey};
        use std::str::FromStr;
        use std::collections::{HashMap, HashSet};
        
        let raydium_program_id = Pubkey::from_str(&config.indexer.raydium_program_id)?;
        
        let mut transaction_filters = HashMap::new();
        transaction_filters.insert(
            "raydium".to_string(),
            carbon_yellowstone_grpc_datasource::GeyserTransactionFilter {
                accounts: vec![raydium_program_id.to_string()],
                ..Default::default()
            },
        );

        let datasource_config = YellowstoneGrpcDatasourceConfig {
            url: url.clone(),
            x_token: Some(token.clone()),
            commitment: CommitmentLevel::from_str(&config.solana.commitment)
                .unwrap_or(CommitmentLevel::Confirmed),
            account_filters: HashMap::new(),
            transaction_filters,
            entry_filters: Default::default(),
            blocks_filters: Default::default(),
            transactions_status: Default::default(),
            ping_interval: None,
            stream_buffer_size: None,
        };

        let datasource = YellowstoneGrpcDatasource::new(
            datasource_config,
            Arc::new(RwLock::new(HashSet::new())),
        ).await?;

        return Ok(Box::new(datasource));
    }

    // Default to RPC WebSocket for development
    info!("Using RPC WebSocket datasource");
    
    use carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribeDatasource};
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    
    let raydium_program_id = Pubkey::from_str(&config.indexer.raydium_program_id)?;
    
    let filters = Filters {
        programs: vec![raydium_program_id],
        accounts: vec![],
        include_vote_transactions: false,
        include_failed_transactions: false,
    };

    let datasource = RpcBlockSubscribeDatasource::new(
        config.solana.ws_url.clone(),
        filters,
    );

    Ok(Box::new(datasource))
}

async fn process_batch(
    publisher: &MessagePublisher,
    batch: &[SwapEvent],
    events_processed: &Arc<RwLock<u64>>,
    last_event_time: &Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
) {
    info!("Processing batch of {} swap events", batch.len());
    
    if let Err(e) = publisher.publish_batch(batch.to_vec()).await {
        error!("Failed to publish batch: {}", e);
    } else {
        let mut count = events_processed.write().await;
        *count += batch.len() as u64;
        
        let mut last_time = last_event_time.write().await;
        *last_time = Some(chrono::Utc::now());
    }
}
