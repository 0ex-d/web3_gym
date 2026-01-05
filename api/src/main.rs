use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::routing::{get, post};
use tokio::time::Instant;
use tracing::{debug, info};

use api::config::{AppConfig, MAX_CONCURRENT_REQUESTS, RPC_TIMEOUT_ERR_SLEEP_RETRY_PERIOD_MS};
use api::handlers::routes::{entry_prepare, health};
use api::state::AppState;
use redis::aio::ConnectionManager;
use sui_sdk::SuiClientBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenvy::dotenv().expect("to load .env file");

    let log_level = std::env::var("RUST_LOG")
        .map(|lev| lev.parse().expect("invalid RUST_LOG, change to eg 'info'"))
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let config = AppConfig::from_env()?;
    let start_ts = Instant::now();
    let sui_client = SuiClientBuilder::default()
        .request_timeout(RPC_TIMEOUT_ERR_SLEEP_RETRY_PERIOD_MS)
        .max_concurrent_requests(MAX_CONCURRENT_REQUESTS)
        .build(config.rpc_url.clone())
        .await
        .expect("can't connect to Sui RPC {:?}");
    info!(
        "Sui RPC version: {} connected!. Took {:?}",
        sui_client.api_version(),
        Duration::from(start_ts.elapsed())
    );

    debug!("Starting up redis: {}", config.redis_url);

    let redis_client = redis::Client::open(config.redis_url.clone())?;
    let redis: ConnectionManager = redis_client
        .get_connection_manager()
        .await
        .expect("redis conn failed");

    let state = Arc::new(AppState {
        config,
        sui: sui_client,
        redis,
    });

    let bind_addr = state.config.bind_addr.clone();
    let app = Router::new()
        .route("/health", get(health))
        .route("/entry/prepare", post(entry_prepare))
        .with_state(state);

    debug!("Starting up API server: {bind_addr}");
    let start_ts = Instant::now();

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!(
        "API listening on {}. Took {:?}",
        bind_addr,
        Duration::from(start_ts.elapsed())
    );
    axum::serve(listener, app).await?;

    Ok(())
}
