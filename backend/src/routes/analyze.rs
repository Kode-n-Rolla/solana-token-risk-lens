use axum::{
    Json, extract::State, http::StatusCode, response
};

use crate::{
    birdeye::{errors::BirdeyeClientError, price},
    types::{
        api::{AnalyzeTokenRequest, AnalyzeTokenResponse, ApiErrorResponse, DataSourceStatus},
        app::AppState,
    },
    utils::validation::validate_analyze_token_request,
};


pub async fn analyze_token(
    State(app_state): State<AppState>,
    Json(payload): Json<AnalyzeTokenRequest>,
) -> Result<Json<AnalyzeTokenResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    validate_analyze_token_request(&payload).map_err(validatation_error_response)?;

    let price_result = app_state
        .birdeye_client
        .get_price(&payload.api_key, &payload.token_address)
        .await;

    let overview_result = app_state
        .birdeye_client
        .get_overview(&payload.api_key, &payload.token_address)
        .await;

    let mut data_sources = Vec::new();

    let price_data = match price_result {
        Ok(response) => {
            data_sources.push(DataSourceStatus {
                source: "price".to_string(),
                status: "ok".to_string(),
                detail: None,
            });

            response.data
        }
        Err(error) => {
            data_sources.push(data_source_error_status("price", &error));
            None
        }
    };

    let overview_data = match overview_result {
        Ok(response) => {
            data_sources.push(DataSourceStatus {
                source: "overview".to_string(),
                status: "ok".to_string(),
                detail: None,
            });

            response.data
        }
        Err(error) => {
            data_sources.push(data_source_error_status("overview", &error));
            None
        }
    };

    if price_data.is_none() && overview_data.is_none() {
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorResponse {
                error: "Birdeye data was unavailable for this token request".to_string(),
            }),
        ));
    }

    let response = AnalyzeTokenResponse {
        token_address: payload.token_address,
        chain: payload.chain,
        name: overview_data.as_ref().and_then(|data| data.name.clone()),
        symbol: overview_data.as_ref().and_then(|data| data.symbol.clone()),
        logo_uri: overview_data.as_ref().and_then(|data| data.logo_uri.clone()),
        price: price_data.as_ref().and_then(|data| data.value),
        liquidity: price_data.as_ref().and_then(|data| data.liquidity),
        data_sources,
        message: "Basic token data fetched from available Birdeye sources".to_string(),
    };

    Ok(Json(response))
}

fn validatation_error_response(message: &'static str) -> (StatusCode, Json<ApiErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiErrorResponse { error: message.to_string() })
    )
}

fn data_source_error_status(source: &str, error: &BirdeyeClientError) -> DataSourceStatus {
    DataSourceStatus {
        source: source.to_string(),
        status: "error".to_string(),
        detail: Some(format_birdeye_error(error)),
    }
}

fn format_birdeye_error(error: &BirdeyeClientError) -> String {
    match error {
        BirdeyeClientError::InvalidApiKeyHeader => {
            "The provided API key could not be used in the Birdeye request".to_string()
        }
        BirdeyeClientError::HttpStatus { endpoint, status } => {
            format!(
                "Birdeye endpoint {} returned an unsuccessful status: {}",
                endpoint, status
            )
        }
        BirdeyeClientError::Request(_) => {
            "Failed to communicate with Birdeye or parse its response".to_string()
        }
    }        
}

/*
fn birdeye_error_response(error: BirdeyeClientError) -> (StatusCode, Json<ApiErrorResponse>) {
    match error {
        BirdeyeClientError::InvalidApiKeyHeader => (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse {
                error: "The provided API key could not be used in the Birdeye request".to_string(),
            }),
        ),
        BirdeyeClientError::HttpStatus {endpoint, status} => (
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorResponse {
                error: format!("Birdeye endpoint {} returned an unsuccessful status {}", endpoint, status)
            }),
        ),
        BirdeyeClientError::Request(_) => (
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorResponse {
                error: "Failed to communicate with Birdeye or parse its response".to_string(),
            }),
        ),
    }
} */