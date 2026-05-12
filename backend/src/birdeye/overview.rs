use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    birdeye::errors::BirdeyeClientError,
    types::birdeye::{BirdeyeResponse, OverviewData},
};

use super::client::BirdeyeClient;

impl BirdeyeClient {
    pub async fn get_overview(
        &self,
        api_key: &str,
        token_address: &str
    ) -> Result<BirdeyeResponse<OverviewData>, BirdeyeClientError> {
        let mut headers = HeaderMap::new();

        let api_key_header = HeaderValue::from_str(api_key)
            .map_err(|_| BirdeyeClientError::InvalidApiKeyHeader)?;

        headers.insert("x-api-key", api_key_header);
        headers.insert("x-chain", HeaderValue::from_static("solana"));

        let url = format!("{}/defi/token_overview", self.base_url());

        let response = self
            .http_client()
            .get(url)
            .headers(headers)
            .query(&[("address", token_address)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BirdeyeClientError::HttpStatus {
                endpoint: "overview",
                status: response.status(),
            });
        }

        let overview_response = response.json::<BirdeyeResponse<OverviewData>>().await?;

        Ok(overview_response)
    }
}