use std::process::Command;
use std::path::{PathBuf};
use anyhow::{Result, anyhow, Context};
use std::fs;

pub fn handle(language: &str) -> Result<()> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    
    let target_dir = if cfg!(windows) {
        match std::env::var("LOCALAPPDATA") {
            Ok(local_app_data) => PathBuf::from(local_app_data).join("tree-sitter").join("lib"),
            Err(_) => PathBuf::from(home).join(".cache").join("tree-sitter").join("lib"),
        }
    } else {
        PathBuf::from(home).join(".cache").join("tree-sitter").join("lib")
    };

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let temp_dir = std::env::temp_dir().join(format!("mka-ts-{}", language));
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    fs::create_dir_all(&temp_dir)?;

    println!("Cloning tree-sitter-{} grammar...", language);
    let repo_url = format!("https://github.com/tree-sitter/tree-sitter-{}", language);
    let status = Command::new("git")
        .args(&["clone", "--depth", "1", &repo_url, "."])
        .current_dir(&temp_dir)
        .status()
        .context("Failed to execute git clone. Is git installed?")?;

    if !status.success() {
        return Err(anyhow!("Failed to clone repository: {}", repo_url));
    }

    println!("Compiling parser...");
    let ext = std::env::consts::DLL_EXTENSION;
    let output_file = target_dir.join(format!("{}.{}", language, ext));

    let mut files = vec!["src/parser.c".to_string()];
    let mut is_cpp = false;

    if temp_dir.join("src/scanner.c").exists() {
        files.push("src/scanner.c".to_string());
    } else if temp_dir.join("src/scanner.cc").exists() {
        files.push("src/scanner.cc".to_string());
        is_cpp = true;
    }

    let status = if cfg!(windows) {
        // Try to detect compiler
        if Command::new("gcc").arg("--version").status().is_ok() {
            let mut cmd = Command::new(if is_cpp { "g++" } else { "gcc" });
            cmd.args(&["-O3", "-shared", "-fPIC", "-I./src"]);
            for f in &files { cmd.arg(f); }
            cmd.arg("-o").arg(&output_file);
            cmd.current_dir(&temp_dir).status()
        } else {
            let mut cmd = Command::new("cl.exe");
            cmd.args(&["/LD", "/Isrc", "/O2"]);
            for f in &files { cmd.arg(f); }
            cmd.arg(format!("/Fe:{}", output_file.to_string_lossy()));
            cmd.current_dir(&temp_dir).status()
        }
    } else {
        let mut cmd = Command::new(if is_cpp { "g++" } else { "gcc" });
        cmd.args(&["-O3", "-shared", "-fPIC", "-I./src"]);
        for f in &files { cmd.arg(f); }
        cmd.arg("-o").arg(&output_file);
        cmd.current_dir(&temp_dir).status()
    }.context("Failed to execute C/C++ compiler. Ensure gcc, g++, or cl.exe is in your PATH.")?;

    if !status.success() {
        return Err(anyhow!("Compilation failed."));
    }

    let _ = fs::remove_dir_all(&temp_dir);
    println!("Successfully installed {} parser to {:?}", language, output_file);

    Ok(())
}
