use axum::{
    extract::State,
    Json,
    http::StatusCode,
};

use crate::{
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
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse{
                    error: "Failed to fetch price data from Birdeye".to_string(),
                })
            )
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