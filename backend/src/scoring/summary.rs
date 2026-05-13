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