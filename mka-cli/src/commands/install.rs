use anyhow::{Result, anyhow, Context};
use crate::models::configs::Config;

pub async fn handle(language: Option<&str>, list: bool) -> Result<()> {
    if list {
        let mut supported_langs: Vec<&str> = crate::utils::languages::LanguageRegistry::MAPPINGS.iter()
            .map(|m| m.name)
            .collect();
        supported_langs.sort();
        supported_langs.dedup();
        println!("Supported tree-sitter parsers:");
        for lang in supported_langs {
            println!("  - {}", lang);
        }
        return Ok(());
    }

    let language = language.ok_or_else(|| anyhow!("Language name is required when not listing supported parsers."))?;
    let target_dir = Config::get_treesitter_dir();

    if !target_dir.exists() {
        tokio::fs::create_dir_all(&target_dir).await?;
    }

    let temp_dir = std::env::temp_dir().join(format!("mka-ts-{}", language));
    if temp_dir.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
    tokio::fs::create_dir_all(&temp_dir).await?;

    println!("Cloning tree-sitter-{} grammar...", language);
    let repo_url = format!("https://github.com/tree-sitter/tree-sitter-{}", language);
    let status = tokio::process::Command::new("git")
        .args(&["clone", "--depth", "1", &repo_url, "."])
        .current_dir(&temp_dir)
        .status()
        .await
        .context("Failed to execute git clone. Is git installed?")?;

    if !status.success() {
        return Err(anyhow!("Failed to clone repository: {}", repo_url));
    }

    // Discover the correct source directory (handle monorepos like typescript)
    let mut build_dir = temp_dir.clone();
    if !temp_dir.join("src/parser.c").exists() {
        let sub_dir = temp_dir.join(language);
        if sub_dir.join("src/parser.c").exists() {
            build_dir = sub_dir;
        }
    }

    if !build_dir.join("src/parser.c").exists() {
        return Err(anyhow!("Could not find src/parser.c in repository. This grammar might use a non-standard structure."));
    }

    println!("Compiling parser...");
    let output_file = target_dir.join(format!("{}.{}", language, "so"));

    let mut files = vec!["src/parser.c".to_string()];
    let mut is_cpp = false;

    if build_dir.join("src/scanner.c").exists() {
        files.push("src/scanner.c".to_string());
    } else if build_dir.join("src/scanner.cc").exists() {
        files.push("src/scanner.cc".to_string());
        is_cpp = true;
    }

    let status = if cfg!(windows) {
        // Try to detect compiler
        if tokio::process::Command::new("gcc").arg("--version").status().await.is_ok() {
            let mut cmd = tokio::process::Command::new(if is_cpp { "g++" } else { "gcc" });
            cmd.args(&["-O3", "-shared", "-fPIC", "-I./src"]);
            for f in &files { cmd.arg(f); }
            cmd.arg("-o").arg(&output_file);
            cmd.current_dir(&build_dir).status().await
        } else {
            let mut cmd = tokio::process::Command::new("cl.exe");
            cmd.args(&["/LD", "/Isrc", "/O2"]);
            for f in &files { cmd.arg(f); }
            cmd.arg(format!("/Fe:{}", output_file.to_string_lossy()));
            cmd.current_dir(&build_dir).status().await
        }
    } else {
        let mut cmd = tokio::process::Command::new(if is_cpp { "g++" } else { "gcc" });
        cmd.args(&["-O3", "-shared", "-fPIC", "-I./src"]);
        for f in &files { cmd.arg(f); }
        cmd.arg("-o").arg(&output_file);
        cmd.current_dir(&build_dir).status().await
    }.context("Failed to execute C/C++ compiler. Ensure gcc, g++, or cl.exe is in your PATH.")?;

    if !status.success() {
        return Err(anyhow!("Compilation failed."));
    }

    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    println!("Successfully installed {} parser to {:?}", language, output_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_install_list_supported() {
        let result = handle(None, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_install_requires_language_when_no_list() {
        let result = handle(None, false).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Language name is required when not listing supported parsers.");
    }
}
