use std::path::Path;
use std::fs;
use std::process::Command;
use anyhow::{Result, Context, anyhow};
use crate::utils::copy_dir_recursive;
use crate::models::configs::Config;

pub fn handle() -> Result<()> {
    let mka_dir = Path::new(Config::DIR_NAME);
    if mka_dir.exists() {
        println!("MKA already initialized.");
        return Ok(());
    }

    println!("Initializing MKA from {}...", Config::REPO_URL);

    let temp_dir = Path::new(Config::TEMP_DIR);
    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir)?;
    }
    fs::create_dir_all(temp_dir)?;

    let run = |args: &[&str]| -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .current_dir(temp_dir)
            .status()
            .context(format!("Failed to execute git {:?}", args))?;
        if !status.success() {
            return Err(anyhow!("Git command failed: {:?}", args));
        }
        Ok(())
    };

    run(&["init"])?;
    run(&["remote", "add", "origin", Config::REPO_URL])?;
    run(&["sparse-checkout", "set", Config::TEMPLATE_DIR])?;
    
    if let Err(_) = run(&["pull", "--depth", "1", "origin", "main"]) {
        run(&["pull", "--depth", "1", "origin", "master"])
            .context("Failed to pull from both 'main' and 'master' branches.")?;
    }

    let source_templates = temp_dir.join(Config::TEMPLATE_DIR);
    if source_templates.exists() {
        copy_dir_recursive(&source_templates, Path::new("."))?;
    } else {
        return Err(anyhow!("The repository does not contain a '{}' directory.", Config::TEMPLATE_DIR));
    }

    fs::remove_dir_all(temp_dir)?;

    println!("Initialized MKA project structure from templates.");
    Ok(())
}
