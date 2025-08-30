use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::messaging::MessagePublisher;

#[derive(Clone)]
pub struct HealthState {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub events_processed: Arc<RwLock<u64>>,
    pub last_event_time: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
    pub publisher: MessagePublisher,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    uptime_seconds: i64,
    events_processed: u64,
    last_event_time: Option<chrono::DateTime<chrono::Utc>>,
    rabbitmq_connected: bool,
}

async fn health_check(State(state): State<HealthState>) -> impl IntoResponse {
    let events_processed = *state.events_processed.read().await;
    let last_event_time = *state.last_event_time.read().await;
    let uptime = chrono::Utc::now() - state.start_time;
    
    let rabbitmq_connected = state.publisher.health_check().await.is_ok();

    let response = HealthResponse {
        status: if rabbitmq_connected { "healthy" } else { "degraded" }.to_string(),
        uptime_seconds: uptime.num_seconds(),
        events_processed,
        last_event_time,
        rabbitmq_connected,
    };

    if rabbitmq_connected {
        (StatusCode::OK, Json(response))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(response))
    }
}

#[derive(Serialize)]
struct MetricsResponse {
    events_per_minute: f64,
    average_processing_time_ms: f64,
    queue_depth: u64,
}

async fn metrics(State(state): State<HealthState>) -> impl IntoResponse {
    let events_processed = *state.events_processed.read().await;
    let uptime_minutes = (chrono::Utc::now() - state.start_time).num_seconds() as f64 / 60.0;
    
    let response = MetricsResponse {
        events_per_minute: if uptime_minutes > 0.0 {
            events_processed as f64 / uptime_minutes
        } else {
            0.0
        },
        average_processing_time_ms: 0.0, // TODO: Implement timing
        queue_depth: 0, // TODO: Get from RabbitMQ
    };

    Json(response)
}

pub fn create_health_router(state: HealthState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics))
        .with_state(state)
}

pub async fn start_health_server(port: u16, state: HealthState) {
    let app = create_health_router(state);
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Starting health check server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}