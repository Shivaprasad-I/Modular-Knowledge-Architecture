pub mod enums;
pub mod configs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MkaIndex {
    pub project: String,
    pub version: f32,
    pub trigger_maps: Vec<TriggerMapSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerMapSummary {
    pub id: String,
    pub intent: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerMap {
    pub id: String,
    pub intent: String,
    pub trigger_nodes: Vec<TriggerNode>,
    pub validation: Option<Validation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerNode {
    pub file: Option<String>,
    pub trigger_map: Option<String>,
    pub method: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validation {
    pub test_file: Option<String>,
}

#[cfg(test)]
mod tests;
