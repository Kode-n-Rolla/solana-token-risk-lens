use reqwest::header::{HeaderMap, HeaderValue};

use crate::types::birdeye::{BirdeyeResponse, PriceData};

use super::client::BirdeyeClient;

impl BirdeyeClient {
    pub async fn get_price(
        &self,
        api_key: &str,
        token_address: &str,
    ) -> Result<BirdeyeResponse<PriceData>, reqwest::Error> {
        let mut headers= HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(api_key).unwrap());
        headers.insert("x-chain", HeaderValue::from_static("solana"));

        let url = format!("{}/defi/price", self.base_url());

        let response = self
            .http_client()
            .get(url)
            .headers(headers)
            .query(&[("address", token_address)])
            .send()
            .await?;

        response.json::<BirdeyeResponse<PriceData>>().await
    }
}