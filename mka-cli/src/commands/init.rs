use std::path::Path;
use anyhow::{Result, Context, anyhow};
use crate::utils::copy_dir_recursive;
use crate::models::configs::Config;
use dialoguer::{theme::ColorfulTheme, MultiSelect};

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

async fn configure_mcp_server(path: &Path, needs_trust: bool) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let mut config_val = if path.exists() {
        let content = tokio::fs::read_to_string(path).await?;
        serde_json::from_str::<serde_json::Value>(&content)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    if !config_val.is_object() {
        config_val = serde_json::json!({});
    }

    let mcp_servers = config_val
        .as_object_mut()
        .unwrap()
        .entry("mcpServers")
        .or_insert_with(|| serde_json::json!({}));

    if !mcp_servers.is_object() {
        *mcp_servers = serde_json::json!({});
    }

    let mut mka_server = serde_json::json!({
        "command": "mka",
        "args": ["mcp"]
    });

    if needs_trust {
        mka_server.as_object_mut().unwrap().insert(
            "trust".to_string(),
            serde_json::json!(true)
        );
    }

    mcp_servers
        .as_object_mut()
        .unwrap()
        .insert("mka".to_string(), mka_server);

    let pretty_content = serde_json::to_string_pretty(&config_val)?;
    tokio::fs::write(path, pretty_content).await?;
    Ok(())
}

pub async fn handle() -> Result<()> {
    let mka_dir = Path::new(Config::DIR_NAME);
    if mka_dir.exists() {
        println!("MKA already initialized.");
        return Ok(());
    }

    let config = Config::load_config(None);
    let repo_url = config.repo_url();
    let template_dir = config.template_dir();
    let temp_dir_name = config.temp_dir();

    println!("Initializing MKA from {}...", repo_url);

    let temp_dir = Path::new(temp_dir_name);
    if temp_dir.exists() {
        tokio::fs::remove_dir_all(temp_dir).await?;
    }
    tokio::fs::create_dir_all(temp_dir).await?;

    run_git(&["init"], temp_dir).await?;
    run_git(&["remote", "add", "origin", repo_url], temp_dir).await?;
    run_git(&["sparse-checkout", "set", template_dir], temp_dir).await?;
    
    if let Err(_) = run_git(&["pull", "--depth", "1", "origin", "main"], temp_dir).await {
        run_git(&["pull", "--depth", "1", "origin", "master"], temp_dir).await
            .context("Failed to pull from both 'main' and 'master' branches.")?;
    }

    let source_templates = temp_dir.join(template_dir);
    if source_templates.exists() {
        copy_dir_recursive(&source_templates, Path::new(".")).await?;
    } else {
        return Err(anyhow!("The repository does not contain a '{}' directory.", template_dir));
    }

    tokio::fs::remove_dir_all(temp_dir).await?;

    println!("Initialized MKA project structure from templates.");

    // Prompt user to configure MCP servers
    let home = dirs::home_dir();
    let config_dir = dirs::config_dir();

    let agy_config_path = home.as_ref().map(|h| h.join(".gemini/antigravity-cli/mcp_config.json"));
    let claude_config_path = config_dir.as_ref().map(|c| c.join("Claude/claude_desktop_config.json"));
    let gemini_settings_path = home.as_ref().map(|h| h.join(".gemini/settings.json"));

    println!("\nWould you like to configure MKA's MCP server for your AI assistants/CLIs?");
    let choices = &[
        "Antigravity CLI (agy) - ~/.gemini/antigravity-cli/mcp_config.json",
        "Claude Desktop - ~/Library/Application Support/Claude/claude_desktop_config.json",
        "Gemini CLI (gemini) - ~/.gemini/settings.json",
    ];

    let defaults = &[true, false, false];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tools to configure (Space to select/deselect, Enter to confirm)")
        .items(&choices[..])
        .defaults(&defaults[..])
        .interact_opt()?;

    if let Some(selected) = selections {
        for index in selected {
            match index {
                0 => {
                    if let Some(ref path) = agy_config_path {
                        match configure_mcp_server(path, true).await {
                            Ok(_) => println!("✔ Successfully configured MKA MCP for Antigravity CLI."),
                            Err(e) => println!("✘ Failed to configure Antigravity CLI: {}", e),
                        }
                    }
                }
                1 => {
                    if let Some(ref path) = claude_config_path {
                        match configure_mcp_server(path, false).await {
                            Ok(_) => println!("✔ Successfully configured MKA MCP for Claude Desktop."),
                            Err(e) => println!("✘ Failed to configure Claude Desktop: {}", e),
                        }
                    }
                }
                2 => {
                    if let Some(ref path) = gemini_settings_path {
                        match configure_mcp_server(path, true).await {
                            Ok(_) => println!("✔ Successfully configured MKA MCP for Gemini CLI."),
                            Err(e) => println!("✘ Failed to configure Gemini CLI: {}", e),
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_configure_mcp_server_new_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("mcp_config.json");

        assert!(!config_path.exists());

        // Configure with needs_trust = true
        let result = configure_mcp_server(&config_path, true).await;
        assert!(result.is_ok());
        assert!(config_path.exists());

        // Read and parse
        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(val["mcpServers"]["mka"]["command"], "mka");
        assert_eq!(val["mcpServers"]["mka"]["args"][0], "mcp");
        assert_eq!(val["mcpServers"]["mka"]["trust"], true);
    }

    #[tokio::test]
    async fn test_configure_mcp_server_existing_file_merge() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("mcp_config.json");

        // Write existing config with other keys and other servers
        let existing_content = r#"{
            "theme": "dark",
            "mcpServers": {
                "existing_server": {
                    "command": "node",
                    "args": ["server.js"]
                }
            }
        }"#;
        tokio::fs::write(&config_path, existing_content).await.unwrap();

        // Configure with needs_trust = false
        let result = configure_mcp_server(&config_path, false).await;
        assert!(result.is_ok());

        // Read and parse
        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Check that existing settings are preserved
        assert_eq!(val["theme"], "dark");
        assert_eq!(val["mcpServers"]["existing_server"]["command"], "node");
        assert_eq!(val["mcpServers"]["existing_server"]["args"][0], "server.js");

        // Check that mka was added
        assert_eq!(val["mcpServers"]["mka"]["command"], "mka");
        assert_eq!(val["mcpServers"]["mka"]["args"][0], "mcp");
        assert_eq!(val["mcpServers"]["mka"]["trust"], serde_json::Value::Null); // not set since needs_trust = false
    }

    #[tokio::test]
    async fn test_configure_mcp_server_merge_no_mcpservers_key() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("mcp_config.json");

        // Existing config has other root keys but no mcpServers key
        let existing_content = r#"{
            "editor.fontSize": 14,
            "telemetry.enabled": false
        }"#;
        tokio::fs::write(&config_path, existing_content).await.unwrap();

        let result = configure_mcp_server(&config_path, true).await;
        assert!(result.is_ok());

        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Check root-level settings are preserved
        assert_eq!(val["editor.fontSize"], 14);
        assert_eq!(val["telemetry.enabled"], false);

        // Check mka is successfully added
        assert_eq!(val["mcpServers"]["mka"]["command"], "mka");
        assert_eq!(val["mcpServers"]["mka"]["args"][0], "mcp");
        assert_eq!(val["mcpServers"]["mka"]["trust"], true);
    }

    #[tokio::test]
    async fn test_configure_mcp_server_merge_invalid_mcpservers_type() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("mcp_config.json");

        // Existing config has mcpServers but as a string instead of an object
        let existing_content = r#"{
            "user": "shivu",
            "mcpServers": "disabled_for_now"
        }"#;
        tokio::fs::write(&config_path, existing_content).await.unwrap();

        let result = configure_mcp_server(&config_path, true).await;
        assert!(result.is_ok());

        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Check user is preserved
        assert_eq!(val["user"], "shivu");

        // Check mcpServers was healed to an object and mka was added
        assert_eq!(val["mcpServers"]["mka"]["command"], "mka");
        assert_eq!(val["mcpServers"]["mka"]["trust"], true);
    }

    #[tokio::test]
    async fn test_configure_mcp_server_merge_existing_mka() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("mcp_config.json");

        // Existing config has an outdated mka server config under mcpServers, plus another server
        let existing_content = r#"{
            "mcpServers": {
                "existing_server": {
                    "command": "node"
                },
                "mka": {
                    "command": "old-mka-path",
                    "args": ["old-arg"],
                    "trust": false
                }
            }
        }"#;
        tokio::fs::write(&config_path, existing_content).await.unwrap();

        // Configure with needs_trust = true
        let result = configure_mcp_server(&config_path, true).await;
        assert!(result.is_ok());

        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Check other server is preserved
        assert_eq!(val["mcpServers"]["existing_server"]["command"], "node");

        // Check mka is updated to the correct configuration
        assert_eq!(val["mcpServers"]["mka"]["command"], "mka");
        assert_eq!(val["mcpServers"]["mka"]["args"][0], "mcp");
        assert_eq!(val["mcpServers"]["mka"]["trust"], true);
    }

    #[tokio::test]
    async fn test_configure_mcp_server_merge_complex_nested() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("mcp_config.json");

        // Existing config has complex nested JSON objects
        let existing_content = r#"{
            "nested": {
                "array": [1, 2, {"key": "val"}],
                "boolean": true,
                "null_val": null
            }
        }"#;
        tokio::fs::write(&config_path, existing_content).await.unwrap();

        let result = configure_mcp_server(&config_path, false).await;
        assert!(result.is_ok());

        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Check nested structure is fully preserved
        assert_eq!(val["nested"]["boolean"], true);
        assert_eq!(val["nested"]["array"][2]["key"], "val");
        assert!(val["nested"]["null_val"].is_null());

        // Check mka was added
        assert_eq!(val["mcpServers"]["mka"]["command"], "mka");
    }

    #[tokio::test]
    async fn test_configure_mcp_server_invalid_json() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("mcp_config.json");

        // Write invalid JSON
        tokio::fs::write(&config_path, "{invalid_json").await.unwrap();

        // Configure
        let result = configure_mcp_server(&config_path, true).await;
        assert!(result.is_ok());

        // Read and parse
        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Check that invalid json was overwritten/healed and contains mka
        assert_eq!(val["mcpServers"]["mka"]["command"], "mka");
        assert_eq!(val["mcpServers"]["mka"]["args"][0], "mcp");
    }
}

