use crate::shard::config::ShardConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub shards: Vec<ShardConfig>,
}

pub fn load_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path).expect("Cannot read config file");
    serde_yaml::from_str(&content).expect("Invalid config format")
}
