use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("sui error: {0}")]
    Sui(#[from] sui_sdk::error::Error),

    #[error("bcs error: {0}")]
    Bcs(#[from] bcs::Error),

    #[error("internal error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl ApiError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Redis(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::Sui(_) => StatusCode::BAD_GATEWAY,
            Self::Bcs(_) => StatusCode::BAD_REQUEST,
            Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(serde_json::json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}