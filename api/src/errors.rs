use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn bad_request(message: &str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.to_owned(),
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: message.to_owned(),
        }
    }

    pub fn from_redis(err: redis::RedisError) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            message: format!("redis error: {err}"),
        }
    }

    pub fn from_sui(err: sui_sdk::error::Error) -> Self {
        Self {
            status: StatusCode::BAD_GATEWAY,
            message: format!("sui error: {err}"),
        }
    }

    pub fn from_anyhow(err: anyhow::Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("error: {err}"),
        }
    }

    pub fn from_bcs(err: bcs::Error) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: format!("bcs error: {err}"),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self::from_anyhow(err)
    }
}

impl From<sui_sdk::error::Error> for ApiError {
    fn from(err: sui_sdk::error::Error) -> Self {
        Self::from_sui(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(serde_json::json!({
            "error": self.message,
        }));
        (self.status, body).into_response()
    }
}
