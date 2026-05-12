use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeTokenRequest {
    pub api_key: String,
    pub token_address: String,
    pub chain: String,
    pub options: AnalyzeOptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeOptions {
    pub include_holders: bool,
    pub holder_limit: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeTokenResponse {
    pub token_address: String,
    pub chain: String,
    pub message: String,
}

pub async fn analyze_token(
    Json(payload): Json<AnalyzeTokenRequest>,
) -> Result<Json<AnalyzeTokenResponse>, StatusCode> {
    if payload.api_key.trim().is_empty() || payload.token_address.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let response = AnalyzeTokenResponse {
        token_address: payload.api_key,
        chain: payload.chain,
        message: "Analysis pipeline is not implemented yet".to_string(),
    };

    Ok(Json(response))
}