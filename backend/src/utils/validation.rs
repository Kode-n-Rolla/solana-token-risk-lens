use crate::types::api::AnalyzeTokenRequest;

pub fn validate_analyze_token_requst(request: &AnalyzeTokenRequest) -> Result<(), &'static str> {
    if request.api_key.trim().is_empty() {
        return Err("API key is required");
    };

    if request.token_address.trim().is_empty() {
        return Err("Token address is required");
    }

    if request.chain.trim().is_empty() {
        return Err("Chain is required");
    }

    Ok(())
}