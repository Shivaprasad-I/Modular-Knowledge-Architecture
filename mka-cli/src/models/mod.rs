pub mod enums;
pub mod configs;
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
    pub workflow_nodes: Vec<WorkflowNode>,
    pub validation: Option<Validation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub file: Option<String>,
    pub workflow: Option<String>,
    pub method: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validation {
    pub test_file: Option<String>,
}

#[cfg(test)]
mod tests;
