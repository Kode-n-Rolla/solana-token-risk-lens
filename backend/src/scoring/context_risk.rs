use crate::types::{birdeye::OverviewExtensions, risk::RiskComponent};

const CONTEXT_MAX_SCORE: u8 = 10;

pub fn score_context_risk(
    extensions: Option<&OverviewExtensions>,
) -> RiskComponent {
    let mut score = 0;
    let mut flags = Vec::new();

    let link_count = extensions.map(count_project_links).unwrap_or(0);

    match link_count {
        0 => {
            score += 8;
            flags.push(
                "Birdeye did not return public project links for this token.".to_string(),
            );
        }
        1 => {
            score += 4;
            flags.push(
                "Birdeye returned only limited public project link coverage.".to_string(),
            );
        }
        _ => {}
    }

    let level = if score >= 7 {
        "High".to_string()
    } else if score >= 4 {
        "Moderate".to_string()
    } else {
        "Low".to_string()
    };

    let summary = if score == 0 {
        "Birdeye returned multiple public project links for manual review.".to_string()
    } else {
        "Public project context appears limited in Birdeye data. Manually verify official website, X/Twitter, Discord or Telegram, and documentation.".to_string()
    };

    RiskComponent {
        score,
        max_score: CONTEXT_MAX_SCORE,
        level,
        flags,
        summary,
    }
}

fn count_project_links(extensions: &OverviewExtensions) -> u8 {
    let mut count = 0;

    if extensions.website.is_some() {
        count += 1;
    }

    if extensions.twitter.is_some() {
        count += 1;
    }

    if extensions.discord.is_some() {
        count += 1;
    }

    count
}
