use std::path::Path;
use anyhow::{Result, Context, anyhow};
use crate::utils::copy_dir_recursive;
use crate::models::configs::Config;

async fn run_git(args: &[&str], current_dir: &Path) -> Result<()> {
    let status = tokio::process::Command::new("git")
        .args(args)
        .current_dir(current_dir)
        .status()
        .await
        .context(format!("Failed to execute git {:?}", args))?;
    if !status.success() {
        return Err(anyhow!("Git command failed: {:?}", args));
    }
    Ok(())
}

pub async fn handle() -> Result<()> {
    let mka_dir = Path::new(Config::DIR_NAME);
    if mka_dir.exists() {
        println!("MKA already initialized.");
        return Ok(());
    }

    println!("Initializing MKA from {}...", Config::REPO_URL);

    let temp_dir = Path::new(Config::TEMP_DIR);
    if temp_dir.exists() {
        tokio::fs::remove_dir_all(temp_dir).await?;
    }
    tokio::fs::create_dir_all(temp_dir).await?;

    run_git(&["init"], temp_dir).await?;
    run_git(&["remote", "add", "origin", Config::REPO_URL], temp_dir).await?;
    run_git(&["sparse-checkout", "set", Config::TEMPLATE_DIR], temp_dir).await?;
    
    if let Err(_) = run_git(&["pull", "--depth", "1", "origin", "main"], temp_dir).await {
        run_git(&["pull", "--depth", "1", "origin", "master"], temp_dir).await
            .context("Failed to pull from both 'main' and 'master' branches.")?;
    }

    let source_templates = temp_dir.join(Config::TEMPLATE_DIR);
    if source_templates.exists() {
        copy_dir_recursive(&source_templates, Path::new(".")).await?;
    } else {
        return Err(anyhow!("The repository does not contain a '{}' directory.", Config::TEMPLATE_DIR));
    }

    tokio::fs::remove_dir_all(temp_dir).await?;

    println!("Initialized MKA project structure from templates.");
    Ok(())
}
