use axum::{extract::Json, http::StatusCode};

use crate::types::api::{
    AnalyzeTokenRequest,
    AnalyzeTokenResponse,
};


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