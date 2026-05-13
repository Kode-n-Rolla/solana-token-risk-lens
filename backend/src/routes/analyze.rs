use axum::{
    Json, extract::State, http::StatusCode
};

use tokio::time::{sleep, Duration};

use crate::{
    birdeye::errors::BirdeyeClientError,
    scoring::holders::calculate_holder_concentration,
    types::{
        api::{
            AnalyzeTokenRequest, AnalyzeTokenResponse, ApiErrorResponse, DataSourceStatus,
            HoldersProbeResponse, SourceProbeResponse,
        },
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

    // Help with bypass 429 status code, when few request at the same second
    sleep(Duration::from_secs(3)).await;

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
        liquidity: overview_data.as_ref().and_then(|data| data.liquidity),
        data_sources,
        message: "Basic token data fetched from available Birdeye sources".to_string(),
    };

    Ok(Json(response))
}

pub async fn probe_price(
    State(app_state): State<AppState>,
    Json(payload): Json<AnalyzeTokenRequest>,
) -> Result<Json<SourceProbeResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    validate_analyze_token_request(&payload).map_err(validatation_error_response)?;

    let result = app_state
        .birdeye_client
        .get_price(&payload.api_key, &payload.token_address)
        .await;

    Ok(Json(build_probe_response(
        "price",
        result.map(|response| response.data.is_some()),
        &payload.token_address,
        &payload.chain,
    )))
}

pub async fn probe_overview(
    State(app_state): State<AppState>,
    Json(payload): Json<AnalyzeTokenRequest>,
) -> Result<Json<SourceProbeResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    validate_analyze_token_request(&payload).map_err(validatation_error_response)?;

    let result = app_state
        .birdeye_client
        .get_overview(&payload.api_key, &payload.token_address)
        .await;

    Ok(Json(build_probe_response(
        "overview",
        result.map(|response| response.data.is_some()),
        &payload.token_address,
        &payload.chain,
    )))
}

pub async fn probe_holders(
    State(app_state): State<AppState>,
    Json(payload): Json<AnalyzeTokenRequest>,
) -> Result<Json<HoldersProbeResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    validate_analyze_token_request(&payload).map_err(validatation_error_response)?;

    let overview_result = app_state
        .birdeye_client
        .get_overview(&payload.api_key, &payload.token_address)
        .await;

    let holders_result = app_state
        .birdeye_client
        .get_holders(
            &payload.api_key,
            &payload.token_address,
            payload.options.holder_limit,
        )
        .await;

    let response = match (overview_result, holders_result) {
        (Ok(overview_response), Ok(holders_response)) => {
            let holders = holders_response.data.map(|data| data.items).unwrap_or_default();
            let top_holder = holders.first();

            let concentration = overview_response
                .data
                .as_ref()
                .and_then(|overview| overview.total_supply)
                .and_then(|total_supply| {
                    calculate_holder_concentration(&holders, total_supply)
                });

            HoldersProbeResponse {
                source: "holders".to_string(),
                status: "ok".to_string(),
                detail: None,
                token_address: payload.token_address,
                chain: payload.chain,
                holder_count: holders.len(),
                top_holder_owner: top_holder.map(|holder| holder.owner.clone()),
                top_holder_ui_amount: top_holder.map(|holder| holder.ui_amount),
                top1_percent: concentration.as_ref().map(|metrics| metrics.top1_percent),
                top5_percent: concentration.as_ref().map(|metrics| metrics.top5_percent),
                top10_percent: concentration.as_ref().map(|metrics| metrics.top10_percent),
                message: "Standalone holders probe completed".to_string(),
            }
        }
        (Err(error), _) => HoldersProbeResponse {
            source: "holders".to_string(),
            status: "error".to_string(),
            detail: Some(format!(
                "Overview request failed before concentration analysis: {}",
                format_birdeye_error(&error)
            )),
            token_address: payload.token_address,
            chain: payload.chain,
            holder_count: 0,
            top_holder_owner: None,
            top_holder_ui_amount: None,
            top1_percent: None,
            top5_percent: None,
            top10_percent: None,
            message: "Standalone holders probe completed".to_string(),
        },
        (_, Err(error)) => HoldersProbeResponse {
            source: "holders".to_string(),
            status: "error".to_string(),
            detail: Some(format_birdeye_error(&error)),
            token_address: payload.token_address,
            chain: payload.chain,
            holder_count: 0,
            top_holder_owner: None,
            top_holder_ui_amount: None,
            top1_percent: None,
            top5_percent: None,
            top10_percent: None,
            message: "Standalone holders probe completed".to_string(),
        },
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
            format!("Failed to communicate with Birdeye or parse its response: {}", error)
        }
        BirdeyeClientError::Json(error) => {
            format!("Failed to decode Birdeye JSON: {}", error)
        },
    }        
}

fn build_probe_response(
    source: &str,
    result: Result<bool, BirdeyeClientError>,
    token_address: &str,
    chain: &str,
) -> SourceProbeResponse {
    match result {
        Ok(has_data) => SourceProbeResponse {
            source: source.to_string(),
            status: "ok".to_string(),
            detail: if has_data {
                None
            } else {
                Some("Birdeye request succeeded but returned no data".to_string())
            },
            token_address: token_address.to_string(),
            chain: chain.to_string(),
            message: format!("Standalone {} probe completed", source),
        },
        Err(error) => SourceProbeResponse {
            source: source.to_string(),
            status: "error".to_string(),
            detail: Some(format_birdeye_error(&error)),
            token_address: token_address.to_string(),
            chain: chain.to_string(),
            message: format!("Standalone {} probe completed", source),
        },
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
