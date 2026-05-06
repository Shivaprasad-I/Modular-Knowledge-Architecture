#[cfg(test)]
mod tests {
    use crate::models::{MkaIndex, TriggerMap, TriggerMapSummary, TriggerNode};

    #[test]
    fn test_index_serialization() {
        let index = MkaIndex {
            project: "test-project".to_string(),
            version: 1.0,
            trigger_maps: vec![
                TriggerMapSummary {
                    id: "action-1".to_string(),
                    intent: "First action".to_string(),
                    path: "maps/action-1.mka.yaml".to_string(),
                }
            ],
        };

        let yaml = serde_yaml::to_string(&index).unwrap();
        assert!(yaml.contains("project: test-project"));
        assert!(yaml.contains("trigger_maps:"));
        assert!(yaml.contains("id: action-1"));

        let deserialized: MkaIndex = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.project, "test-project");
        assert_eq!(deserialized.trigger_maps.len(), 1);
    }

    #[test]
    fn test_trigger_map_serialization() {
        let map = TriggerMap {
            id: "sync".to_string(),
            intent: "Sync knowledge".to_string(),
            trigger_nodes: vec![
                TriggerNode {
                    file: Some("src/main.rs".to_string()),
                    trigger_map: None,
                    method: Some("main".to_string()),
                    note: Some("Entry point".to_string()),
                }
            ],
            validation: None,
        };

        let yaml = serde_yaml::to_string(&map).unwrap();
        assert!(yaml.contains("id: sync"));
        assert!(yaml.contains("trigger_nodes:"));
        assert!(yaml.contains("method: main"));

        let deserialized: TriggerMap = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.id, "sync");
        assert_eq!(deserialized.trigger_nodes.len(), 1);
    }
}
