use axum::{
    extract::State,
    Json,
    http::StatusCode,
};

use crate::{
    birdeye::errors::BirdeyeClientError,
    types::{
        api::{AnalyzeTokenRequest, AnalyzeTokenResponse, ApiErrorResponse},
        app::AppState,
    },
    utils::validation::validate_analyze_token_requst,
};


pub async fn analyze_token(
    State(app_state): State<AppState>,
    Json(payload): Json<AnalyzeTokenRequest>,
) -> Result<Json<AnalyzeTokenResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    validate_analyze_token_requst(&payload).map_err(|err| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse { error: err.to_string(), }),
        )
    })?;

    let price_response = app_state
        .birdeye_client
        .get_price(&payload.api_key, &payload.token_address)
        .await
        .map_err(|err| match err {
            BirdeyeClientError::InvalidApiKeyHeader => (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse {
                    error: "The provided API key could not be used in the Birdeye request".to_string(),
                }),
            ),
            BirdeyeClientError::HttpStatus(status) => (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorResponse {
                    error: format!("Birdeye returned an unsuccessful status {}", status)
                }),
            ),
            BirdeyeClientError::Request(_) => (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorResponse {
                    error: "Failed to communicate with Birdeye or parse its response".to_string(),
                }),
            ),
        })?;

    let message = match price_response.data {
        Some(price_data) => format!(
            "price fetched. value={:?}, liquidity={:?}",
            price_data.value, price_data.liquidity
        ),
        None => "Birdeye returned no price data".to_string(),
    };

    let response = AnalyzeTokenResponse {
        token_address: payload.token_address,
        chain: payload.chain,
        message
    };

    Ok(Json(response))
}