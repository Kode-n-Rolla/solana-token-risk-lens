use crate::types::risk::RiskComponent;

const MOMENTUM_MAX_SCORE: u8 = 25;

pub fn score_momentum_risk(
    price_change_1h_percent: Option<f64>,
    price_change_4h_percent: Option<f64>,
    price_change_24h_percent: Option<f64>,
    liquidity_usd: Option<f64>,
) -> RiskComponent {
    let mut score = 0;
    let mut flags = Vec::new();

    if let Some(change_1h) = price_change_1h_percent {
        let absolute_change = change_1h.abs();

        if absolute_change >= 25.0 {
            score += 8;
            flags.push("Price moved sharply over the last hour.".to_string());
        } else if absolute_change >= 12.0 {
            score += 4;
            flags.push("Price movement over the last hour is elevated.".to_string());
        }
    }

    if let Some(change_4h) = price_change_4h_percent {
        let absolute_change = change_4h.abs();

        if absolute_change >= 40.0 {
            score += 8;
            flags.push("Price moved sharply over the last 4 hours.".to_string());
        } else if absolute_change >= 20.0 {
            score += 4;
            flags.push("Price movement over the last 4 hours is notable.".to_string());
        }
    }

    if let Some(change_24h) = price_change_24h_percent {
        let absolute_change = change_24h.abs();

        if absolute_change >= 60.0 {
            score += 7;
            flags.push("24h price movement is extreme.".to_string());
        } else if absolute_change >= 30.0 {
            score += 4;
            flags.push("24h price movement is significant.".to_string());
        }
    }

    if let (Some(liquidity), Some(change_24h)) = (liquidity_usd, price_change_24h_percent) {
        if liquidity < 50_000.0 && change_24h.abs() >= 30.0 {
            score += 5;
            flags.push(
                "Strong 24h price movement is occurring in relatively thin liquidity.".to_string(),
            );
        }
    }

    if score > MOMENTUM_MAX_SCORE {
        score = MOMENTUM_MAX_SCORE;
    }

    let level = if score >= 17 {
        "High".to_string()
    } else if score >= 8 {
        "Moderate".to_string()
    } else {
        "Low".to_string()
    };

    let summary = if flags.is_empty() {
        "Recent price movement appears relatively contained across key time windows.".to_string()
    } else {
        "Recent price movement may warrant manual review, especially if volatility is paired with thinner liquidity.".to_string()
    };

    RiskComponent {
        score,
        max_score: MOMENTUM_MAX_SCORE,
        level,
        flags,
        summary,
    }
}
