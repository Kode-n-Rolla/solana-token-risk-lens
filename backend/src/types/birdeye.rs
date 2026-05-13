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
    pub update_unix_time: Option<f64>,
    pub update_human_time: Option<String>,
    pub price_change_24h: Option<f64>,
    pub price_in_native: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverviewExtensions {
    pub description: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub discord: Option<String>,
    pub coingecko_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverviewData {
    pub address: Option<String>,
    pub decimal: Option<u8>,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub market_cap: Option<f64>,
    pub fdv: Option<f64>,
    pub extensions: Option<OverviewExtensions>,
    pub logo_uri: Option<String>,
    pub liquidity: Option<f64>,
    pub last_trade_unix_time: Option<i64>,
    pub last_trade_human_time: Option<String>,
    pub price: Option<f64>,
    pub price_change_1m_percent: Option<f64>,
    pub price_change_5m_percent: Option<f64>,
    pub price_change_30m_percent: Option<f64>,
    pub price_change_1h_percent: Option<f64>,
    pub price_change_2h_percent: Option<f64>,
    pub price_change_4h_percent: Option<f64>,
    pub price_change_8h_percent: Option<f64>,
    pub price_change_24h_percent: Option<f64>,
    pub unique_wallet_24h: Option<u64>,
    pub holder: Option<u64>,
    pub trade_24h: Option<u64>,
    pub sell_24h: Option<u64>,
    pub buy_24h: Option<u64>,
    pub v24h_usd: Option<f64>,
    pub v_buy_24h_usd: Option<f64>,
    pub v_sell_24h_usd: Option<f64>,
    pub total_supply: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub number_markets: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderData {
    pub items: Vec<HolderItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderItem {
    pub amount: String,
    pub decimal: u8,
    pub mint: String,
    pub owner: String,
    pub token_account: String,
    pub ui_amount: f64,
    pub is_scaled_ui_token: bool,
    pub multiplier: Option<f64>,
}