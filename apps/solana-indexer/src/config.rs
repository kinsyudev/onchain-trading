use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub geyser_url: String,
    pub x_token: Option<String>,
    pub rpc_url: String,
    pub rpc_ws_url: String,
    pub rabbitmq_url: String,
    pub data_source: DataSourceType,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    Geyser,
    Rpc,
    Both,
}

impl Config {
    pub fn from_env() -> eyre::Result<Self> {
        dotenv::dotenv().ok();
        
        let data_source = match env::var("DATA_SOURCE")
            .unwrap_or_else(|_| "geyser".to_string())
            .to_lowercase()
            .as_str()
        {
            "rpc" => DataSourceType::Rpc,
            "both" => DataSourceType::Both,
            _ => DataSourceType::Geyser,
        };

        Ok(Config {
            geyser_url: env::var("GEYSER_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            x_token: env::var("X_TOKEN").ok(),
            rpc_url: env::var("RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            rpc_ws_url: env::var("RPC_WS_URL")
                .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
            rabbitmq_url: env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "http://guest:guest@localhost:15672".to_string()),
            data_source,
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }

    pub fn validate(&self) -> eyre::Result<()> {
        // Basic URL validation
        if !self.rpc_url.starts_with("http://") && !self.rpc_url.starts_with("https://") {
            return Err(eyre::eyre!("RPC_URL must be a valid HTTP/HTTPS URL"));
        }

        if !self.rpc_ws_url.starts_with("ws://") && !self.rpc_ws_url.starts_with("wss://") {
            return Err(eyre::eyre!("RPC_WS_URL must be a valid WebSocket URL"));
        }

        match self.data_source {
            DataSourceType::Geyser | DataSourceType::Both => {
                if self.geyser_url.is_empty() {
                    return Err(eyre::eyre!("GEYSER_URL is required when using Geyser data source"));
                }
            }
            _ => {}
        }

        Ok(())
    }
}
