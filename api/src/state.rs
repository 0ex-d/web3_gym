use redis::aio::ConnectionManager;
use sui_sdk::SuiClient;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub sui: SuiClient,
    pub redis: ConnectionManager,
}
