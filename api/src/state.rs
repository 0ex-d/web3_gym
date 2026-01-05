use crate::config::AppConfig;
use redis::aio::ConnectionManager;
use sui_sdk::SuiClient;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub sui: SuiClient,
    pub redis: ConnectionManager,
}
