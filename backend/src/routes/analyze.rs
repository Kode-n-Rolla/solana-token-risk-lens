use axum::{Json, http::StatusCode};

use crate::{
    types::api::{
        AnalyzeTokenRequest,
        AnalyzeTokenResponse,
        ApiErrorResponse,
    },
    utils::validation::validate_analyze_token_requst,
};


pub async fn analyze_token(
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
        message: "Analysis pipeline is not implemented yet".to_string(),
    };

    Ok(Json(response))
}