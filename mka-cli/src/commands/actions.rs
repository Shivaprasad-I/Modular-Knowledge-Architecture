use std::fs;
use anyhow::{Result, anyhow};
use crate::models::{MkaIndex, configs::Config};

pub fn handle() -> Result<()> {
    let index_path = Config::get_index_file()?;
    if !index_path.exists() {
        return Err(anyhow!("Error: {} not found. Run 'mka init' first.", index_path.display()));
    }

    let content = fs::read_to_string(index_path)?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    println!("@mka:actions");
    for summary in index.trigger_maps {
        println!("- [{}]: {}", summary.id, summary.intent);
    }

    Ok(())
}
