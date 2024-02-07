use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemFile {
    pub data: serde_json::Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub name: String,
}
