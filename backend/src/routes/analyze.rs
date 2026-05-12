use axum::{
    Json,
    http::StatusCode,
    extract::State,
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

    let response = AnalyzeTokenResponse {
        token_address: payload.api_key,
        chain: payload.chain,
        message: format!(
            "Analysis pipeline is not implemented yet. Birdeye base url: {}",
            app_state.birdeye_client.base_url()
        ),
    };

    Ok(Json(response))
}