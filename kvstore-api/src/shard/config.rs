use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ShardConfig {
    pub id: usize,
    pub addr: String,
    pub range_start: String,
    pub range_end: String,
}