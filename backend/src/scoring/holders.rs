use serde::Serialize;

use crate::types::birdeye::HolderItem;

#[derive(Debug, Clone, Serialize)]
pub struct HolderConcentrationMetrics {
    pub top1_percent: f64,
    pub top5_percent: f64,
    pub top10_percent: f64,
}

pub fn calculate_holder_concentration(
    holders: &[HolderItem],
    total_supply: f64,
) -> Option<HolderConcentrationMetrics> {
    if holders.is_empty() || total_supply <= 0.0 {
        return None;
    }

    let top1_sum = sum_top_ui_amounts(holders, 1);
    let top5_sum = sum_top_ui_amounts(holders, 5);
    let top10_sum = sum_top_ui_amounts(holders, 10);

    Some(HolderConcentrationMetrics {
        top1_percent: percentage_of_supply(top1_sum, total_supply),
        top5_percent: percentage_of_supply(top5_sum, total_supply),
        top10_percent: percentage_of_supply(top10_sum, total_supply),        
    })
}

fn sum_top_ui_amounts(holders: &[HolderItem], count: usize) -> f64 {
    holders.iter().take(count).map(|holder| holder.ui_amount).sum()
}

fn percentage_of_supply(amount: f64, total_supply: f64) -> f64 {
    (amount / total_supply) * 100.0
}