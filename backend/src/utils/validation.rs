use crate::types::api::AnalyzeTokenRequest;

const MIN_HOLDER_LIMIT: u32 = 1;
const MAX_HOLDER_LIMIT: u32 = 100;

pub fn validate_analyze_token_requst(request: &AnalyzeTokenRequest) -> Result<(), &'static str> {
    if request.api_key.trim().is_empty() {
        return Err("Birdeye API key is required");
    };

    if request.token_address.trim().is_empty() {
        return Err("Solana token address is required");
    }

    if request.chain.trim().is_empty() {
        return Err("Chain is required");
    }

    if request.chain != "solana" {
        return Err("Only the solana chain is supported");
    }

    if request.options.holder_limit < MIN_HOLDER_LIMIT
        || request.options.holder_limit > MAX_HOLDER_LIMIT 
    {
        return Err("holderLimit must be between 1 and 100");
    }

    Ok(())
}