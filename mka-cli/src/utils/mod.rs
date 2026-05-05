use std::path::Path;
use std::fs;
use anyhow::{Result, Context, anyhow};
use serde_json::Value;
use jsonschema::JSONSchema;

pub fn validate_yaml(content: &str, schema_path: &Path) -> Result<Value> {
    let yaml_value: Value = serde_yaml::from_str(content)
        .context("Failed to parse YAML content")?;
    
    if schema_path.exists() {
        let schema_content = fs::read_to_string(schema_path)?;
        let schema_json: Value = serde_json::from_str(&schema_content)?;
        let compiled = JSONSchema::compile(&schema_json)
            .map_err(|e| anyhow!("Failed to compile schema: {}", e))?;
        
        let result = compiled.validate(&yaml_value);
        if let Err(errors) = result {
            let mut msg = String::from("Validation failed:\n");
            for error in errors {
                msg.push_str(&format!("  - {}: {}\n", error.instance_path, error));
            }
            return Err(anyhow!(msg));
        }
    }
    
    Ok(yaml_value)
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
