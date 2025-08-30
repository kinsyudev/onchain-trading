use anyhow::{Context, Result};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    ExchangeKind,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::config::RabbitMQConfig;
use crate::models::SwapEvent;

#[derive(Clone)]
pub struct MessagePublisher {
    channel: Arc<Mutex<Channel>>,
    config: RabbitMQConfig,
}

impl MessagePublisher {
    pub async fn new(config: RabbitMQConfig) -> Result<Self> {
        let connection = Connection::connect(&config.url, ConnectionProperties::default())
            .await
            .context("Failed to connect to RabbitMQ")?;

        let channel = connection
            .create_channel()
            .await
            .context("Failed to create channel")?;

        // Declare exchange
        channel
            .exchange_declare(
                &config.exchange,
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .context("Failed to declare exchange")?;

        // Declare queue
        channel
            .queue_declare(
                &config.queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .context("Failed to declare queue")?;

        // Bind queue to exchange
        channel
            .queue_bind(
                &config.queue_name,
                &config.exchange,
                &config.routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to bind queue")?;

        info!(
            "Connected to RabbitMQ - Exchange: {}, Queue: {}",
            config.exchange, config.queue_name
        );

        Ok(Self {
            channel: Arc::new(Mutex::new(channel)),
            config,
        })
    }

    pub async fn publish_swap_event(&self, event: SwapEvent) -> Result<()> {
        let payload = serde_json::to_vec(&event).context("Failed to serialize event")?;

        let channel = self.channel.lock().await;

        let confirm = channel
            .basic_publish(
                &self.config.exchange,
                &self.config.routing_key,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2), // Persistent
            )
            .await
            .context("Failed to publish message")?
            .await
            .context("Failed to confirm message")?;

        if confirm.is_ack() {
            info!(
                "Published swap event - Signature: {}, Pool: {}",
                event.data.signature, event.data.pool_id
            );
        } else {
            warn!(
                "Message not acknowledged - Signature: {}",
                event.data.signature
            );
        }

        Ok(())
    }

    pub async fn publish_batch(&self, events: Vec<SwapEvent>) -> Result<()> {
        for event in events {
            if let Err(e) = self.publish_swap_event(event.clone()).await {
                error!(
                    "Failed to publish event {}: {}",
                    event.data.signature, e
                );
                // Continue with other events even if one fails
            }
        }
        Ok(())
    }

    pub async fn health_check(&self) -> Result<()> {
        let channel = self.channel.lock().await;
        let status = channel.status();
        
        if status.connected() {
            Ok(())
        } else {
            anyhow::bail!("RabbitMQ channel is not connected")
        }
    }
}