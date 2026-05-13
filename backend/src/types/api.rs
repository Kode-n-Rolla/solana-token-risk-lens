use serde::{Deserialize, Serialize};

use crate::{
    scoring::holders::HolderConcentrationMetrics,
    types::risk::RiskComponent
};

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
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub logo_uri: Option<String>,
    pub price: Option<f64>,
    pub liquidity: Option<f64>,
    pub risk_index: u8,
    pub risk_level: String,
    pub summary: String,
    pub holder_metrics: Option<HolderConcentrationMetrics>,
    pub holder_risk: Option<RiskComponent>,
    pub liquidity_risk: RiskComponent,
    pub momentum_risk: RiskComponent,
    pub context_risk: RiskComponent,
    pub red_flags: Vec<String>,
    pub manual_checks: Vec<String>,
    pub data_sources: Vec<DataSourceStatus>,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceStatus {
    pub source: String,
    pub status: String,
    pub detail: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceProbeResponse {
    pub source: String,
    pub status: String,
    pub detail: Option<String>,
    pub token_address: String,
    pub chain: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldersProbeResponse {
    pub source: String,
    pub status: String,
    pub detail: Option<String>,
    pub token_address: String,
    pub chain: String,
    pub holder_count: usize,
    pub top_holder_owner: Option<String>,
    pub top_holder_ui_amount: Option<f64>,
    pub top1_percent: Option<f64>,
    pub top5_percent: Option<f64>,
    pub top10_percent: Option<f64>,
    pub risk_component: Option<RiskComponent>,
    pub message: String,
}