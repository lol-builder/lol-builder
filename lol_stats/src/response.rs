use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemFile {
    pub data: serde_json::Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub gold: Gold,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Gold {
    pub base: usize,
    pub purchasable: bool,
    pub total: usize,
    pub sell: usize,
}
