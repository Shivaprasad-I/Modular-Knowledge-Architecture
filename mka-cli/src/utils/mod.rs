use std::path::{Path, PathBuf};
use anyhow::{Result, Context, anyhow};
use serde_json::Value;
use jsonschema::JSONSchema;
use crate::models::configs::Config;

pub fn find_mka_root() -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;
    
    loop {
        if current_dir.join(Config::DIR_NAME).exists() {
            return Ok(current_dir);
        }
        
        if !current_dir.pop() {
            return Err(anyhow!("fatal: not an MKA repository (or any of the parent directories): {}", Config::DIR_NAME));
        }
    }
}

pub async fn validate_yaml(content: &str, schema_path: &Path) -> Result<Value> {
    let yaml_value: Value = serde_yaml::from_str(content)
        .context("Failed to parse YAML content")?;
    
    if schema_path.exists() {
        let schema_content = tokio::fs::read_to_string(schema_path).await?;
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

#[async_recursion::async_recursion]
pub async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    tokio::fs::create_dir_all(dst).await?;
    let mut dir = tokio::fs::read_dir(src).await?;
    while let Some(entry) = dir.next_entry().await? {
        let file_type = entry.file_type().await?;
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dst.join(entry.file_name())).await?;
        } else {
            tokio::fs::copy(entry.path(), dst.join(entry.file_name())).await?;
        }
    }
    Ok(())
}

pub fn get_language_from_path(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "rs" => Some("rust"),
        "ts" | "tsx" | "js" | "jsx" => Some("typescript"),
        "py" => Some("python"),
        "cs" => Some("c-sharp"),
        "go" => Some("go"),
        "c" => Some("c"),
        "cpp" | "cc" | "cxx" => Some("cpp"),
        "java" => Some("java"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        _ => None,
    }
}

#[cfg(test)]
mod tests;

