use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub solana: SolanaConfig,
    pub rabbitmq: RabbitMQConfig,
    pub indexer: IndexerConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub yellowstone_url: Option<String>,
    pub yellowstone_token: Option<String>,
    pub commitment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMQConfig {
    pub url: String,
    pub exchange: String,
    pub routing_key: String,
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerConfig {
    pub raydium_program_id: String,
    pub batch_size: usize,
    pub start_slot: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub health_check_port: u16,
    pub metrics_port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            solana: SolanaConfig {
                rpc_url: env::var("SOLANA_RPC_URL")
                    .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
                ws_url: env::var("SOLANA_WS_URL")
                    .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
                yellowstone_url: env::var("YELLOWSTONE_URL").ok(),
                yellowstone_token: env::var("YELLOWSTONE_TOKEN").ok(),
                commitment: env::var("COMMITMENT").unwrap_or_else(|_| "confirmed".to_string()),
            },
            rabbitmq: RabbitMQConfig {
                url: env::var("RABBITMQ_URL")
                    .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string()),
                exchange: env::var("RABBITMQ_EXCHANGE")
                    .unwrap_or_else(|_| "solana-swaps".to_string()),
                routing_key: env::var("RABBITMQ_ROUTING_KEY")
                    .unwrap_or_else(|_| "raydium.swap".to_string()),
                queue_name: env::var("RABBITMQ_QUEUE")
                    .unwrap_or_else(|_| "raydium-swaps".to_string()),
            },
            indexer: IndexerConfig {
                raydium_program_id: env::var("RAYDIUM_PROGRAM_ID")
                    .unwrap_or_else(|_| "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()),
                batch_size: env::var("BATCH_SIZE")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .unwrap_or(100),
                start_slot: env::var("START_SLOT").ok().and_then(|s| s.parse().ok()),
            },
            monitoring: MonitoringConfig {
                health_check_port: env::var("HEALTH_CHECK_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                metrics_port: env::var("METRICS_PORT")
                    .unwrap_or_else(|_| "9090".to_string())
                    .parse()
                    .unwrap_or(9090),
                log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            },
        })
    }
}