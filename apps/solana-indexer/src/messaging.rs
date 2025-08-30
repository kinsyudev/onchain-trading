use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PumpfunTradeEvent {
    pub signature: String,
    pub slot: u64,
    pub timestamp: u64,
    #[serde(rename = "programId")]
    pub program_id: String,
    pub mint: String,
    #[serde(rename = "solAmount")]
    pub sol_amount: String,
    #[serde(rename = "tokenAmount")]
    pub token_amount: String,
    #[serde(rename = "isBuy")]
    pub is_buy: bool,
    pub trader: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaydiumTradeEvent {
    pub signature: String,
    pub slot: u64,
    pub timestamp: u64,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "swapType")]
    pub swap_type: String, // "swapBaseIn" or "swapBaseOut"
    #[serde(rename = "poolId")]
    pub pool_id: String,
    pub user: String,
    // Optional fields based on swap type
    #[serde(rename = "amountIn", skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<String>,
    #[serde(rename = "minimumAmountOut", skip_serializing_if = "Option::is_none")]
    pub minimum_amount_out: Option<String>,
    #[serde(rename = "maxAmountIn", skip_serializing_if = "Option::is_none")]
    pub max_amount_in: Option<String>,
    #[serde(rename = "amountOut", skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<String>,
}

pub struct RabbitMQPublisher {
    client: reqwest::Client,
    rabbitmq_url: String,
}

impl RabbitMQPublisher {
    pub fn new(rabbitmq_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            rabbitmq_url,
        }
    }

    pub async fn publish_pumpfun_trade(&self, event: PumpfunTradeEvent) -> eyre::Result<()> {
        self.publish_to_queue("pumpfun-trades", &event).await
    }

    pub async fn publish_raydium_trade(&self, event: RaydiumTradeEvent) -> eyre::Result<()> {
        self.publish_to_queue("raydium-trades", &event).await
    }

    async fn publish_to_queue<T: Serialize>(&self, queue: &str, message: &T) -> eyre::Result<()> {
        let payload = serde_json::to_string(message)?;
        
        // Simple HTTP API approach for RabbitMQ publishing
        // In production, you might want to use a proper AMQP client
        let mut body = HashMap::<String, serde_json::Value>::new();
        body.insert("properties".to_string(), serde_json::Value::Object(serde_json::Map::new()));
        body.insert("routing_key".to_string(), serde_json::Value::String(queue.to_string()));
        body.insert("payload".to_string(), serde_json::Value::String(payload));
        body.insert("payload_encoding".to_string(), serde_json::Value::String("string".to_string()));

        let response = self.client
            .post(&format!("{}/api/exchanges/%2F/amq.default/publish", self.rabbitmq_url))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(eyre::eyre!("Failed to publish message: {}", response.status()));
        }

        log::debug!("Published message to queue: {}", queue);
        Ok(())
    }
}
