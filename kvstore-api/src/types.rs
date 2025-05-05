use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct SetRequest {
    pub value: String,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub prefix: String,
}