use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskComponent {
    pub score: u8,
    pub max_score: u8,
    pub level: String,
    pub flags: Vec<String>,
    pub summary: String,
}