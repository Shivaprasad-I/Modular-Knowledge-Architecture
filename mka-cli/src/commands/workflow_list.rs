use anyhow::{Result, anyhow};
use crate::models::{MkaIndex, configs::Config};

pub async fn handle() -> Result<()> {
    let output = get_workflows_toon().await?;
    println!("{}", output);
    Ok(())
}

pub async fn get_workflows_toon() -> Result<String> {
    let index_path = Config::get_index_file()?;
    if !index_path.exists() {
        return Err(anyhow!("Error: {} not found. Run 'mka init' first.", index_path.display()));
    }

    let content = tokio::fs::read_to_string(index_path).await?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    let mut output = String::from("@mka:workflows\n");
    for summary in index.workflows {
        output.push_str(&format!("- [{}]: {}\n", summary.id, summary.intent));
    }

    Ok(output)
}
