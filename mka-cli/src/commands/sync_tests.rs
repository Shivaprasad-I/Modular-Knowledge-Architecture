#[cfg(test)]
mod tests {
    use crate::models::{MkaIndex, TriggerMap, TriggerMapSummary, TriggerNode};
    use crate::models::configs::Config;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_sync_heals_path() {
        let dir = tempdir().unwrap();
        let mka_dir = dir.path().join(Config::DIR_NAME);
        fs::create_dir_all(&mka_dir).unwrap();
        let maps_dir = mka_dir.join("TriggerMaps");
        fs::create_dir_all(&maps_dir).unwrap();

        // Create a file that will be "moved"
        let old_path = dir.path().join("src/old_dir/logic.rs");
        fs::create_dir_all(old_path.parent().unwrap()).unwrap();
        fs::write(&old_path, "fn main() {}").unwrap();

        // Create the trigger map pointing to the OLD path
        let map_id = "test-action";
        let map_path = maps_dir.join(format!("{}.mka.yaml", map_id));
        let trigger_map = TriggerMap {
            id: map_id.to_string(),
            intent: "Test healing".to_string(),
            trigger_nodes: vec![
                TriggerNode {
                    file: Some("src/old_dir/logic.rs".to_string()),
                    trigger_map: None,
                    method: Some("main".to_string()),
                    note: None,
                }
            ],
            validation: None,
        };
        fs::write(&map_path, serde_yaml::to_string(&trigger_map).unwrap()).unwrap();

        // Create the index
        let index = MkaIndex {
            project: "test".to_string(),
            version: 1.0,
            trigger_maps: vec![
                TriggerMapSummary {
                    id: map_id.to_string(),
                    intent: "Test healing".to_string(),
                    path: format!("TriggerMaps/{}.mka.yaml", map_id),
                }
            ],
        };
        fs::write(mka_dir.join("index.mka.yaml"), serde_yaml::to_string(&index).unwrap()).unwrap();

        // Move the file to a NEW path
        let new_path = dir.path().join("src/new_dir/logic.rs");
        fs::create_dir_all(new_path.parent().unwrap()).unwrap();
        fs::rename(&old_path, &new_path).unwrap();

        // Run sync (we need to change directory to the temp dir)
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();
        
        // We'll call the handle function directly
        // Note: This requires that the handle function uses relative paths to .MKA
        crate::commands::sync::handle().unwrap();

        // Verify the map was updated
        let updated_map_content = fs::read_to_string(&map_path).unwrap();
        let updated_map: TriggerMap = serde_yaml::from_str(&updated_map_content).unwrap();
        
        assert_eq!(updated_map.trigger_nodes[0].file.as_deref(), Some("src/new_dir/logic.rs"));

        std::env::set_current_dir(original_dir).unwrap();
    }
}
