use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as Base64Engine;
use chrono::Utc;
use redis::AsyncCommands;
use sui_sdk::json::SuiJsonValue;
use sui_sdk::types::crypto::ToFromBytes;
use sui_sdk::types::signature::GenericSignature;

use crate::errors::ApiError;
use crate::handlers::utils::{ensure_membership_owner, parse_address, parse_object_id};
use crate::model::{EntryPrepareRequest, EntryPrepareResponse};
use crate::state::AppState;
use tracing::{debug, info};

const SIGNATURE_VALIDITY_WINDOW_SECS: i64 = 60;

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

pub async fn entry_prepare(
    State(state): State<Arc<AppState>>,
    Json(req): Json<EntryPrepareRequest>,
) -> Result<Json<EntryPrepareResponse>, ApiError> {
    let sender_address = parse_address(&req.address)?;
    let membership_id = parse_object_id(&req.membership_id)?;

    // mitigate replay attack
    let now = Utc::now().timestamp();
    if (now - req.timestamp).abs() > SIGNATURE_VALIDITY_WINDOW_SECS {
        return Err(ApiError::BadRequest(
            "Request expired. Please try again.".to_string(),
        ));
    }

    // verify signature
    let message = format!("Gym entry at {}", req.timestamp);
    debug!("Checking signature on message: {message:?}");
    let signature_bytes = Base64Engine
        .decode(req.signature.as_bytes())
        .map_err(|_| ApiError::bad_request("invalid signature encoding"))?;
    let signature = GenericSignature::from_bytes(&signature_bytes)
        .map_err(|_| ApiError::bad_request("invalid signature"))?;

    sui_sdk::verify_personal_message_signature::verify_personal_message_signature(
        signature,
        message.as_bytes(),
        sender_address,
        None,
    )
    .await
    .map_err(|_| ApiError::Unauthorized("signature verification failed".to_string()))?;

    ensure_membership_owner(&state, membership_id, sender_address).await?;

    let rate_limit_key = format!("ratelimit:entry:{}", sender_address);
    let mut redis = state.redis.clone();
    if redis.exists(&rate_limit_key).await? {
        return Err(ApiError::TooManyRequests(
            "Please wait 10 seconds".to_string(),
        ));
    }

    let tx_data = state
        .sui
        .transaction_builder()
        .move_call(
            sender_address,
            state.config.package_id,
            &state.config.module,
            &state.config.verify_fn,
            vec![],
            vec![
                SuiJsonValue::from_object_id(membership_id),
                SuiJsonValue::from_object_id(state.config.clock_id),
            ],
            None,
            state.config.gas_budget,
            None,
        )
        .await
        .map_err(|e| ApiError::InternalServer(format!("Failed to build tx: {}", e)))?;

    let tx_bytes = bcs::to_bytes(&tx_data).map_err(ApiError::Bcs)?;
    let tx_bytes_b64 = Base64Engine.encode(&tx_bytes);
    let digest = tx_data.digest().to_string();
    let expires_at = Utc::now().timestamp() + state.config.tx_ttl_secs as i64;

    let cache_prepared_key = format!("prepared:{}:{}", sender_address, digest);
    let _: () = redis.set_ex(&cache_prepared_key, "1", 300).await?;

    info!(
        event = "entry_prepared",
        address = %sender_address,
        membership_id = %membership_id,
        digest = %digest,
    );

    Ok(Json(EntryPrepareResponse {
        tx_bytes_b64,
        digest,
        expires_at,
    }))
}
