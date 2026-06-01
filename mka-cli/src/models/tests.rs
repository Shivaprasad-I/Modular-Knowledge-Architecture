#[cfg(test)]
mod tests {
    use crate::models::{MkaIndex, Workflow, WorkflowSummary, WorkflowNode};

    #[test]
    fn test_index_serialization() {
        let index = MkaIndex {
            project: "test-project".to_string(),
            version: 1.0,
            workflows: vec![
                WorkflowSummary {
                    id: "workflow-1".to_string(),
                    intent: "First workflow".to_string(),
                    path: "maps/workflow-1.mka.yaml".to_string(),
                }
            ],
        };

        let yaml = serde_yaml::to_string(&index).unwrap();
        assert!(yaml.contains("project: test-project"));
        assert!(yaml.contains("workflows:"));
        assert!(yaml.contains("id: workflow-1"));

        let deserialized: MkaIndex = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.project, "test-project");
        assert_eq!(deserialized.workflows.len(), 1);
    }

    #[test]
    fn test_workflow_serialization() {
        let map = Workflow {
            id: "sync".to_string(),
            intent: "Sync knowledge".to_string(),
            workflow_nodes: vec![
                WorkflowNode {
                    file: Some("src/main.rs".to_string()),
                    workflow: None,
                    method: Some("main".to_string()),
                    note: Some("Entry point".to_string()),
                }
            ],
            validation: None,
        };

        let yaml = serde_yaml::to_string(&map).unwrap();
        assert!(yaml.contains("id: sync"));
        assert!(yaml.contains("workflow_nodes:"));
        assert!(yaml.contains("method: main"));

        let deserialized: Workflow = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.id, "sync");
        assert_eq!(deserialized.workflow_nodes.len(), 1);
    }
}
