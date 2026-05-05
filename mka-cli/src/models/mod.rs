pub mod enums;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MkaIndex {
    pub project: String,
    pub version: f32,
    pub workflows: Vec<WorkflowSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowSummary {
    pub id: String,
    pub intent: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub intent: String,
    pub nodes: Vec<Node>,
    pub validation: Option<Validation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub file: String,
    pub method: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validation {
    pub test_file: Option<String>,
}
