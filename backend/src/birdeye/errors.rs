use thiserror::Error;

#[derive(Debug, Error)]
pub enum BirdeyeClientError {
    #[error("invalid API key header")]
    InvalidApiKeyHeader,

    #[error("request to Birdeye failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Birdeye returned unsuccessful status for {endpoint}: {status}")]
    HttpStatus {
        endpoint: &'static str,
        status: reqwest::StatusCode,
    },
}