use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    birdeye::errors::BirdeyeClientError,
    types::birdeye::{BirdeyeResponse, HolderData},
};

use super::client::BirdeyeClient;

impl BirdeyeClient {
    pub async fn get_holders(
        &self,
        api_key: &str,
        token_address: &str,
        limit: u32,
    ) -> Result<BirdeyeResponse<HolderData>, BirdeyeClientError> {
        let mut headers = HeaderMap::new();

        let api_key_header = 
            HeaderValue::from_str(api_key).map_err(|_| BirdeyeClientError::InvalidApiKeyHeader)?;
        
        headers.insert("x-api-key", api_key_header);
        headers.insert("x-chain", HeaderValue::from_static("solana"));

        let url = format!("{}/defi/v3/token/holder", self.base_url());

        let response = self
            .http_client()
            .get(url)
            .headers(headers)
            .query(&[
                ("address", token_address),
                ("offset", "0"),
                ("limit", &limit.to_string()),
                ("ui_amount_mode", "scaled"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BirdeyeClientError::HttpStatus {
                endpoint: "holders",
                status: response.status(),
            });
        }

        let holders_response = response.json::<BirdeyeResponse<HolderData>>().await?;

        Ok(holders_response)
    }
}
