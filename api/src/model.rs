use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChallengeRequest {
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub nonce: String,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub address: String,
    pub nonce: String,
    pub signature_b64: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct EnterPrepareRequest {
    pub address: String,
    pub membership_id: String,
}

#[derive(Debug, Serialize)]
pub struct EnterPrepareResponse {
    pub tx_bytes_b64: String,
    pub digest: String,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct EnterSubmitRequest {
    pub address: String,
    pub tx_bytes_b64: String,
    pub signature_b64: String,
}

#[derive(Debug, Serialize)]
pub struct EnterSubmitResponse {
    pub tx_digest: String,
}
