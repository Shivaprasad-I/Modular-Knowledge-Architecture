#[cfg(test)]
mod tests {
    use crate::utils::{get_language_from_path, validate_yaml};
    use std::path::Path;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_get_language_from_path() {
        assert_eq!(get_language_from_path(Path::new("test.rs")), Some("rust"));
        assert_eq!(get_language_from_path(Path::new("TEST.RS")), Some("rust"));
        assert_eq!(get_language_from_path(Path::new("test.py")), Some("python"));
        assert_eq!(get_language_from_path(Path::new("test.ts")), Some("typescript"));
        assert_eq!(get_language_from_path(Path::new("test.js")), Some("typescript"));
        assert_eq!(get_language_from_path(Path::new("test.kt")), Some("kotlin"));
        assert_eq!(get_language_from_path(Path::new("test.cpp")), Some("cpp"));
        assert_eq!(get_language_from_path(Path::new("test.sh")), Some("bash"));
        assert_eq!(get_language_from_path(Path::new("test.html")), Some("html"));
        assert_eq!(get_language_from_path(Path::new("test.unknown")), None);
        assert_eq!(get_language_from_path(Path::new("test")), None);
    }

    #[tokio::test]
    async fn test_validate_yaml_no_schema() {
        let content = "id: test\nintent: test intent\nworkflow_nodes: []";
        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("non_existent_schema.json");
        
        let result = validate_yaml(content, &schema_path).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_yaml_with_schema() {
        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("schema.json");
        let schema_content = r#"{
            "type": "object",
            "required": ["id"],
            "properties": {
                "id": { "type": "string" }
            }
        }"#;
        fs::write(&schema_path, schema_content).unwrap();

        // Valid YAML
        let valid_content = "id: test_id";
        assert!(validate_yaml(valid_content, &schema_path).await.is_ok());

        // Invalid YAML (missing required id)
        let invalid_content = "not_id: test_id";
        assert!(validate_yaml(invalid_content, &schema_path).await.is_err());
    }
}
