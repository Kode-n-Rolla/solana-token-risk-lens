use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirdeyeResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceData {
    pub value: Option<f64>,
    pub liquidity: Option<f64>,
    pub update_unix_time: Option<f64>,
    pub update_human_time: Option<String>,
    pub price_change_24h_percent: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverviewData {
    pub address: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub logo_uri: Option<String>,
}