use std::path::Path;
use std::fs;
use std::process::Command;
use anyhow::{Result, Context, anyhow};
use crate::utils::copy_dir_recursive;

const REPO_URL: &str = "https://github.com/Shivaprasad-I/Modular-Knowledge-Architecture.git";

pub fn handle() -> Result<()> {
    let mka_dir = Path::new(".MKA");
    if mka_dir.exists() {
        println!("MKA already initialized.");
        return Ok(());
    }

    println!("Initializing MKA from {}...", REPO_URL);

    let temp_dir = Path::new(".mka_temp");
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
    run(&["remote", "add", "origin", REPO_URL])?;
    run(&["sparse-checkout", "set", ".MKA"])?;
    
    if let Err(_) = run(&["pull", "--depth", "1", "origin", "main"]) {
        run(&["pull", "--depth", "1", "origin", "master"])
            .context("Failed to pull from both 'main' and 'master' branches.")?;
    }

    let source_mka = temp_dir.join(".MKA");
    if source_mka.exists() {
        copy_dir_recursive(&source_mka, mka_dir)?;
    } else {
        return Err(anyhow!("The repository does not contain a .MKA directory."));
    }

    fs::remove_dir_all(temp_dir)?;

    println!("Initialized .MKA directory structure via sparse-checkout.");
    Ok(())
}
