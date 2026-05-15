use crate::types::api::AnalyzeTokenRequest;

const MIN_HOLDER_LIMIT: u32 = 1;
const MAX_HOLDER_LIMIT: u32 = 100;
const SOLANA_ADDRESS_MIN_LEN: usize = 32;
const SOLANA_ADDRESS_MAX_LEN: usize = 44;
const BASE58_ALPHABET: &str =
    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn validate_analyze_token_request(request: &AnalyzeTokenRequest) -> Result<(), &'static str> {
    if request.api_key.trim().is_empty() {
        return Err("Birdeye API key is required");
    }

    if request.token_address.trim().is_empty() {
        return Err("Solana token address is required");
    }

    if request.chain.trim().is_empty() {
        return Err("Chain is required");
    }

    if request.chain != "solana" {
        return Err("Only the solana chain is supported");
    }

    if !request.options.include_holders {
        return Err("Holder analysis must remain enabled in this MVP");
    }

    if request.options.holder_limit < MIN_HOLDER_LIMIT
        || request.options.holder_limit > MAX_HOLDER_LIMIT
    {
        return Err("holderLimit must be between 1 and 100");
    }

    if !looks_like_solana_address(request.token_address.trim()) {
        return Err("Token address does not match the expected Solana address format");
    }

    Ok(())
}

fn looks_like_solana_address(address: &str) -> bool {
    let length = address.len();

    if !(SOLANA_ADDRESS_MIN_LEN..=SOLANA_ADDRESS_MAX_LEN).contains(&length) {
        return false;
    }

    address.chars().all(|ch| BASE58_ALPHABET.contains(ch))
}
