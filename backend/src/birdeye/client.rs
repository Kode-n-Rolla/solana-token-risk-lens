use reqwest::Client;

#[derive(Clone)]
pub struct BirdeyeClient {
    http_client: Client,
    base_url: String,
}

impl BirdeyeClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            base_url: "https://public-api.birdeye.so".to_string(),
        }
    }

    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}