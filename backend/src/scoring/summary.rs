use crate::types::risk::RiskComponent;

pub fn calculate_risk_index(
    holder_risk: Option<&RiskComponent>,
    liquidity_risk: &RiskComponent,
    momentum_risk: &RiskComponent,
    context_risk: &RiskComponent,
) -> u8 {
    let holder_score = holder_risk.map(|risk| risk.score).unwrap_or(0);

    holder_score
        .saturating_add(liquidity_risk.score)
        .saturating_add(momentum_risk.score)
        .saturating_add(context_risk.score)
}

pub fn risk_level_from_score(score: u8) -> String {
    match score {
        0..=25 => "Low observable risk".to_string(),
        26..=50 => "Moderate observable risk".to_string(),
        51..=75 => "High observable risk".to_string(),
        _ => "Severe observable risk".to_string()
    }
}

pub fn generate_analysis_summary(
    risk_level: &str,
    holder_risk: Option<&RiskComponent>,
    liquidity_risk: &RiskComponent,
    momentum_risk: &RiskComponent,
    context_risk: &RiskComponent,
) -> String {
    let mut highlights = Vec::new();

    if let Some(holder_risk) = holder_risk {
        if holder_risk.score > 0 {
            highlights.push("holder concentration");
        }
    }

    if liquidity_risk.score > 0 {
        highlights.push("liquidity conditions");
    }

    if momentum_risk.score > 0 {
        highlights.push("recent price movement");
    }

    if context_risk.score > 0 {
        highlights.push("limited public project context");
    }

    if highlights.is_empty() {
        return format!(
            "{}. No major observable risk signals were triggered by the current Birdeye inputs, but manual verification is still recommended.",
            risk_level
        );
    }

    let joined = highlights.join(", ");

    format!(
        "{}. The strongest observable signals in this analysis relate to {}. Use these signals as prompts for manual review rather than a definitive verdict.",
        risk_level, joined
    )
}
