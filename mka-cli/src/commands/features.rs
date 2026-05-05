use std::path::Path;
use std::fs;
use anyhow::{Result, anyhow};
use crate::models::{MkaIndex, configs::Config};

pub fn handle() -> Result<()> {
    let index_path = Path::new(Config::INDEX_FILE);
    if !index_path.exists() {
        return Err(anyhow!("Error: {} not found. Run 'mka init' first.", Config::INDEX_FILE));
    }

    let content = fs::read_to_string(index_path)?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    println!("@mka:features");
    for workflow in index.workflows {
        println!("- [{}]: {}", workflow.id, workflow.intent);
    }

    Ok(())
}
