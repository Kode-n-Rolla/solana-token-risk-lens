use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    birdeye::errors::BirdeyeClientError,
    types::birdeye::{BirdeyeResponse, PriceData},
};

use super::client::BirdeyeClient;

impl BirdeyeClient {
    pub async fn get_price(
        &self,
        api_key: &str,
        token_address: &str,
    ) -> Result<BirdeyeResponse<PriceData>, BirdeyeClientError> {
        let mut headers= HeaderMap::new();

        let api_key_header = 
            HeaderValue::from_str(api_key).map_err(|_| BirdeyeClientError::InvalidApiKeyHeader)?;

        headers.insert("x-api-key", api_key_header);
        headers.insert("x-chain", HeaderValue::from_static("solana"));

        let url = format!("{}/defi/price", self.base_url());

        let response = self
            .http_client()
            .get(url)
            .headers(headers)
            .query(&[("address", token_address)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BirdeyeClientError::HttpStatus(response.status()));
        }

        let price_respone = response.json::<BirdeyeResponse<PriceData>>().await?;

        Ok(price_respone)
    }
}