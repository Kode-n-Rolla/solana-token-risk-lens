use crate::types::risk::RiskComponent;

const LIQUIDITY_MAX_SCORE: u8 = 35;

pub fn score_liquidity_risk(
    liquidity_usd: Option<f64>,
    price_change_24h_percent: Option<f64>,
) -> RiskComponent {
    let mut score = 0;
    let mut flags = Vec::new();

    match liquidity_usd {
        None => {
            score += 12;
            flags.push("Liquidity data was unavailable from Birdeye.".to_string());
        }
        Some(liquidity) if liquidity < 10_000.0 => {
            score += 20;
            flags.push("Liquidity is very low.".to_string());
        }
        Some(liquidity) if liquidity < 50_000.0 => {
            score += 12;
            flags.push("Liquidity is limited.".to_string());
        }
        Some(liquidity) if liquidity < 200_000.0 => {
            score += 6;
            flags.push("Liquidity is modeerate and worth reviewing.".to_string());
        }
        Some(_) => {}
    }

    if let (Some(liquidity), Some(price_change)) = (liquidity_usd, price_change_24h_percent) {
        let absolute_change = price_change.abs();

        if liquidity < 50_000.0 && absolute_change >= 30.0 {
            score += 10;
            flags.push("Large 24h price movement is occuring alongside limited liquidity".to_string());
        } else if liquidity < 200_00.0 && absolute_change >= 20.0 {
            score += 5;
            flags.push("Noticeable price movement is occuring with only moderate liquidity".to_string());
        }
    }

    if score > LIQUIDITY_MAX_SCORE {
        score = LIQUIDITY_MAX_SCORE;
    }

    let level = if score >= 24 {
        "High".to_string()
    } else if score >= 12 {
        "Moderate".to_string()
    } else {
        "Low".to_string()
    };

    let summary = if flags.is_empty() {
        "Observed liquidity appears relatively supportive for current trading activity.".to_string()
    } else {
        "Liquidity conditions may warrant manual review, especially if price moves sharply in thin markets.".to_string()
    
    };

    RiskComponent {
        score,
        max_score: LIQUIDITY_MAX_SCORE,
        level,
        flags,
        summary
    }
}