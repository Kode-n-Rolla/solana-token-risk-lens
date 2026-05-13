use crate::{
    scoring::holders::HolderConcentrationMetrics,
    types::risk::RiskComponent,
};

const HOLDERS_MAX_SCORE: u8 = 30;

pub fn score_holder_concentration(
    metrics: &HolderConcentrationMetrics,
) -> RiskComponent {
    let mut score = 0;
    let mut flags = Vec::new();

    if metrics.top1_percent >= 20.0 {
        score += 10;
        flags.push("Top holder controls a large share of total supply.".to_string());
    } else if metrics.top1_percent >= 10.0 {
        score += 6;
        flags.push("Top holder concentration is elevated.".to_string());
    } else if metrics.top1_percent >= 5.0 {
        score += 3;
        flags.push("Top holder concentration is worth reviewing.".to_string());
    }

    if metrics.top5_percent >= 50.0 {
        score += 8;
        flags.push("Top 5 holders control a large share of supply.".to_string());
    } else if metrics.top5_percent >= 30.0 {
        score += 5;
        flags.push("Top 5 holder concentration is notable.".to_string());
    } else if metrics.top5_percent >= 20.0 {
        score += 3;
        flags.push("Top 5 holders control a meaningful share of supply.".to_string());
    }

    if metrics.top10_percent >= 70.0 {
        score += 7;
        flags.push("Top 10 holders control most of the supply.".to_string());
    } else if metrics.top10_percent >= 50.0 {
        score += 4;
        flags.push("Top 10 holder concentration is elevated.".to_string());
    } else if metrics.top10_percent >= 40.0 {
        score += 2;
        flags.push("Top 10 holders represent a sizable share of supply.".to_string());
    }

    let level = if score >= 19 {
        "High".to_string()
    } else if score >= 9 {
        "Moderate".to_string()
    } else {
        "Low".to_string()
    };

    let summary = if flags.is_empty() {
        "Holder concentration appears limited based on the sampled top holders.".to_string()
    } else {
        "Holder concentration may warrant manual review. Large holders can include exchanges, pools, lockers, or project-controlled wallets.".to_string()
    };

    RiskComponent {
        score,
        max_score: HOLDERS_MAX_SCORE,
        level,
        flags,
        summary,
    }
}
