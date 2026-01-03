use std::str::FromStr;
use std::time::Duration;
use url::Url;

use eyre::WrapErr;
use sui_sdk::types::SUI_CLOCK_OBJECT_ID;
use sui_sdk::types::base_types::ObjectID;

pub const RPC_TIMEOUT_ERR_SLEEP_RETRY_PERIOD_MS: Duration = Duration::from_millis(10_000);
pub const MAX_CONCURRENT_REQUESTS: usize = 1_000;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub rpc_url: String,
    pub package_id: ObjectID,
    pub module: String,
    pub verify_fn: String,
    pub clock_id: ObjectID,
    pub gas_budget: u64,
    pub challenge_ttl_secs: usize,
    pub verify_ttl_secs: usize,
    pub tx_ttl_secs: usize,
    pub redis_url: String,
    pub bind_addr: String,
}

impl AppConfig {
    pub fn from_env() -> eyre::Result<Self> {
        let rpc_url = std::env::var("RPC_URL")
            .unwrap_or_else(|_| "https://fullnode.testnet.sui.io:443".to_owned());
        let rpc_url: Url = rpc_url.parse()?;
        let package_id = std::env::var("PACKAGE_ID")
            .wrap_err("PACKAGE_ID is required")?
            .parse()
            .wrap_err("invalid PACKAGE_ID")?;
        let module = std::env::var("MODULE").unwrap_or_else(|_| "gym".to_owned());
        let verify_fn =
            std::env::var("VERIFY_FN").unwrap_or_else(|_| "verify_and_enter".to_owned());
        let clock_id = std::env::var("CLOCK_ID")
            .ok()
            .map(|val| ObjectID::from_str(&val))
            .transpose()
            .wrap_err("invalid CLOCK_ID")?
            .unwrap_or(SUI_CLOCK_OBJECT_ID);
        let gas_budget = std::env::var("GAS_BUDGET")
            .ok()
            .and_then(|val| val.parse::<u64>().ok())
            .unwrap_or(30_000_000);
        let challenge_ttl_secs = std::env::var("CHALLENGE_TTL_SECS")
            .ok()
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap_or(120);
        let verify_ttl_secs = std::env::var("VERIFY_TTL_SECS")
            .ok()
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap_or(90);
        let tx_ttl_secs = std::env::var("TX_TTL_SECS")
            .ok()
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap_or(90);

        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_owned());
        let redis_url: Url = redis_url.parse()?;

        let ip_addr = std::env::var("IP_ADDR").unwrap_or_else(|_| "0.0.0.0".to_owned());
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
        let bind_addr: String = format!("{}:{}", ip_addr, port).parse()?;

        Ok(Self {
            rpc_url: rpc_url.to_string(),
            package_id,
            module,
            verify_fn,
            clock_id,
            gas_budget,
            challenge_ttl_secs,
            verify_ttl_secs,
            tx_ttl_secs,
            redis_url:redis_url.to_string(),
            bind_addr,
        })
    }
}
