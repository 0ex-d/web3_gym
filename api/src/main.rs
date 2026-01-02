use std::str::FromStr;

use chrono::{DateTime, Utc};
use std::{
    sync::Arc,
    time::{self, Duration},
};

use tokio::sync::broadcast;
use tracing::{Level, info};
use tracing_subscriber;

use sui_sdk::SuiClientBuilder;
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::types::base_types::ObjectID;

const PKG_ID: &str = "0x4f68c8030c478aa13367981d12f3c90545c03cc30a15768cc03e8c8d85617a16";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let log_level = std::env::var("RUST_LOG")
        .map(|lev| lev.parse().expect("invalid RUST_LOG, change to eg 'info'"))
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let sui_rpc_url =
        std::env::var("RPC_URL").unwrap_or("https://fullnode.testnet.sui.io:443".to_owned());

    let sui_client = SuiClientBuilder::default().build(sui_rpc_url).await?;
    info!("Sui testnet version: {}", sui_client.api_version());

    // info!(
    //     "available_rpc_methods: {:?}",
    //     sui_client.available_rpc_methods()
    // );

    let pkg_id = ObjectID::from_str(PKG_ID)?;

    let pkg = sui_client
        .read_api()
        .get_normalized_move_modules_by_package(pkg_id)
        .await?;
    info!("{:#?}", pkg);

    // let chain_id = sui_client.read_api().get_chain_identifier().await?;

    // write EDA code
    // let gm = GymManager::run();

    Ok(())
}
