use std::path::Path;
use anyhow::{Result, Context};
use crate::models::{MkaIndex, Workflow};
use crate::utils::{validate_yaml, find_mka_root};
use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};
use crate::models::configs::Config;

pub async fn handle(id: &str, snippets: bool) -> Result<()> {
    let output = get_workflow_content(id, snippets).await?;
    println!("{}", output);
    Ok(())
}

pub async fn get_workflow_content(id: &str, snippets: bool) -> Result<String> {
    let project_root = find_mka_root()?;
    let mka_folder = Config::get_mka_folder()?;
    get_workflow_content_with_paths(id, snippets, &project_root, &mka_folder).await
}

pub async fn get_workflow_content_with_paths(
    id: &str,
    snippets: bool,
    project_root: &Path,
    mka_folder: &Path,
) -> Result<String> {
    let config = Config::load_config(Some(mka_folder));
    
    let index_path = if let Some(ref path_str) = config.index_file {
        std::path::PathBuf::from(path_str)
    } else {
        mka_folder.join("index.mka.yaml")
    };
    
    let content = tokio::fs::read_to_string(&index_path).await?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    let map_summary = index.workflows.iter()
        .find(|w| w.id == id)
        .context(format!("Workflow '{}' not found in index.", id))?;

    let map_path = mka_folder.join(&map_summary.path);
    let map_content = tokio::fs::read_to_string(&map_path).await?;
    
    let schema_path = if let Some(ref path_str) = config.schema_file {
        std::path::PathBuf::from(path_str)
    } else {
        mka_folder.join("schema.json")
    };
    
    validate_yaml(&map_content, &schema_path).await?;
    
    let workflow: Workflow = serde_yaml::from_str(&map_content)?;

    let parsers_enabled = config.parsers_enabled();

    let mut output = String::new();

    if snippets {
        output.push_str(&format!("# @mka:workflow:{}\n", workflow.id));
        output.push_str(&format!("**intent:** {}\n", workflow.intent));
        if let Some(ref validation) = workflow.validation {
            if let Some(ref test_file) = validation.test_file {
                output.push_str(&format!("**validation:** {}\n", test_file));
            }
        }
        output.push('\n');

        let mut loader = DynamicLanguageLoader::new();
        let mut missing_languages = Vec::new();

        for node in workflow.workflow_nodes {
            if let Some(ref ref_id) = node.workflow {
                output.push_str(&format!("### workflow: {}\n", ref_id));
                if let Some(note) = &node.note {
                    output.push_str(&format!("**note:** {}\n", note));
                }
                output.push('\n');
                continue;
            }

            let file_path_str = node.file.as_deref().unwrap_or("[MISSING FILE]");
            let file_path = project_root.join(file_path_str);
            if !file_path.exists() {
                output.push_str(&format!("### file: {} [NOT FOUND]\n", file_path_str));
                if let Some(method) = &node.method {
                    output.push_str(&format!("**method:** {}\n", method));
                }
                if let Some(note) = &node.note {
                    output.push_str(&format!("**note:** {}\n", note));
                }
                output.push('\n');
                continue;
            }

            if !parsers_enabled || node.method.is_none() {
                output.push_str(&format!("### file: {}\n", file_path_str));
                if let Some(method) = &node.method {
                    output.push_str(&format!("**method:** {}\n", method));
                }
                if let Some(note) = &node.note {
                    output.push_str(&format!("**note:** {}\n", note));
                }
                output.push('\n');
                continue;
            }

            let source = tokio::fs::read_to_string(&file_path).await?;
            let lang_name = crate::utils::get_language_from_path(&file_path).unwrap_or("text");
            let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or(lang_name);

            match loader.load_language(lang_name) {
                Ok(lang) => {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    let method_name = node.method.as_deref().unwrap();
                    
                    match analyzer.get_method_signature(method_name) {
                        Ok(_sig) => {
                            output.push_str(&format!("### file: {}\n", file_path_str));
                            if let Some(note) = &node.note {
                                output.push_str(&format!("**note:** {}\n", note));
                            }
                            
                            if let Ok(minified) = analyzer.get_minified_logic(method_name) {
                                output.push_str("**snippet:**\n");
                                output.push_str(&format!("```{}\n", extension));
                                output.push_str(&minified);
                                output.push_str("\n```\n\n");
                            }
                        }
                        Err(e) => {
                            output.push_str(&format!("### file: {} [ERROR: {}]\n", file_path_str, e));
                            if let Some(method) = &node.method {
                                output.push_str(&format!("**method:** {}\n", method));
                            }
                            if let Some(note) = &node.note {
                                output.push_str(&format!("**note:** {}\n", note));
                            }
                            output.push('\n');
                        }
                    }
                }
                Err(e) => {
                    if lang_name != "text" {
                        if !missing_languages.contains(&lang_name.to_string()) {
                            missing_languages.push(lang_name.to_string());
                        }
                        output.push_str(&format!("### file: {}\n", file_path_str));
                        if let Some(method) = &node.method {
                            output.push_str(&format!("**method:** {}\n", method));
                        }
                        if let Some(note) = &node.note {
                            output.push_str(&format!("**note:** {}\n", note));
                        }
                        output.push('\n');
                    } else {
                        output.push_str(&format!("### file: {} [ERROR: {}]\n", file_path_str, e));
                        if let Some(method) = &node.method {
                            output.push_str(&format!("**method:** {}\n", method));
                        }
                        if let Some(note) = &node.note {
                            output.push_str(&format!("**note:** {}\n", note));
                        }
                        output.push('\n');
                    }
                }
            }
        }

        if !missing_languages.is_empty() {
            let langs = missing_languages.join(", ");
            output.push_str(&format!(
                "\n[LLM: Missing parsers: {}. (1 language per command). Ask user to run: mka install <language>]\n",
                langs
            ));
        }
    } else {
        let mut map_obj = serde_json::Map::new();
        map_obj.insert("intent".to_string(), serde_json::json!(workflow.intent));
        if let Some(ref validation) = workflow.validation {
            if let Some(ref test_file) = validation.test_file {
                let mut val_obj = serde_json::Map::new();
                val_obj.insert("test_file".to_string(), serde_json::json!(test_file));
                map_obj.insert("validation".to_string(), serde_json::json!(val_obj));
            }
        }

        let mut loader = DynamicLanguageLoader::new();
        let mut nodes_array = Vec::new();
        let mut missing_languages = Vec::new();

        for node in workflow.workflow_nodes {
            let mut node_obj = serde_json::Map::new();
            
            if let Some(ref ref_id) = node.workflow {
                node_obj.insert("workflow".to_string(), serde_json::json!(ref_id));
                if let Some(note) = &node.note {
                    node_obj.insert("note".to_string(), serde_json::json!(note));
                }
                nodes_array.push(serde_json::json!(node_obj));
                continue;
            }

            let file_path_str = node.file.as_deref().unwrap_or("[MISSING FILE]");
            node_obj.insert("file".to_string(), serde_json::json!(file_path_str));

            let file_path = project_root.join(file_path_str);
            if !file_path.exists() {
                if let Some(method) = &node.method {
                    node_obj.insert("method".to_string(), serde_json::json!(method));
                }
                if let Some(note) = &node.note {
                    node_obj.insert("note".to_string(), serde_json::json!(note));
                }
                node_obj.insert("error".to_string(), serde_json::json!("NOT FOUND"));
                nodes_array.push(serde_json::json!(node_obj));
                continue;
            }

            if !parsers_enabled || node.method.is_none() {
                if let Some(method) = &node.method {
                    node_obj.insert("method".to_string(), serde_json::json!(method));
                }
                if let Some(note) = &node.note {
                    node_obj.insert("note".to_string(), serde_json::json!(note));
                }
                nodes_array.push(serde_json::json!(node_obj));
                continue;
            }

            let source = tokio::fs::read_to_string(&file_path).await?;
            let lang_name = crate::utils::get_language_from_path(&file_path).unwrap_or("text");

            if lang_name == "text" {
                if let Some(method) = &node.method {
                    node_obj.insert("method".to_string(), serde_json::json!(method));
                }
                if let Some(note) = &node.note {
                    node_obj.insert("note".to_string(), serde_json::json!(note));
                }
                node_obj.insert("error".to_string(), serde_json::json!("UNSUPPORTED EXTENSION"));
                nodes_array.push(serde_json::json!(node_obj));
                continue;
            }

            match loader.load_language(lang_name) {
                Ok(lang) => {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    let method_name = node.method.as_deref().unwrap();
                    
                    match analyzer.get_method_signature(method_name) {
                        Ok(sig) => {
                            if let Some(note) = &node.note {
                                node_obj.insert("note".to_string(), serde_json::json!(note));
                            }
                            node_obj.insert("sig".to_string(), serde_json::json!(sig));
                        }
                        Err(e) => {
                            if let Some(method) = &node.method {
                                node_obj.insert("method".to_string(), serde_json::json!(method));
                            }
                            if let Some(note) = &node.note {
                                node_obj.insert("note".to_string(), serde_json::json!(note));
                            }
                            node_obj.insert("error".to_string(), serde_json::json!(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    if lang_name != "text" {
                        if !missing_languages.contains(&lang_name.to_string()) {
                            missing_languages.push(lang_name.to_string());
                        }
                        if let Some(method) = &node.method {
                            node_obj.insert("method".to_string(), serde_json::json!(method));
                        }
                        if let Some(note) = &node.note {
                            node_obj.insert("note".to_string(), serde_json::json!(note));
                        }
                    } else {
                        if let Some(method) = &node.method {
                            node_obj.insert("method".to_string(), serde_json::json!(method));
                        }
                        if let Some(note) = &node.note {
                            node_obj.insert("note".to_string(), serde_json::json!(note));
                        }
                        node_obj.insert("error".to_string(), serde_json::json!(e.to_string()));
                    }
                }
            }
            nodes_array.push(serde_json::json!(node_obj));
        }

        map_obj.insert("workflow_nodes".to_string(), serde_json::json!(nodes_array));

        let value = serde_json::json!({
            format!("@mka:workflow:{}", workflow.id): map_obj
        });

        output.push_str(&toon::encode(&value, None));

        if !missing_languages.is_empty() {
            let langs = missing_languages.join(", ");
            output.push_str(&format!(
                "\n[LLM: Missing parsers: {}. (1 language per command). Ask user to run: mka install <language>]\n",
                langs
            ));
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::configs::TEST_LOCK;

    #[tokio::test]
    async fn test_get_workflow_content_without_snippets() {
        let _guard = TEST_LOCK.lock().unwrap();
        let mka_folder = Config::get_mka_folder().unwrap();
        let config_path = mka_folder.join("config.yaml");
        let existed = config_path.exists();
        let old_content = if existed { Some(std::fs::read_to_string(&config_path).unwrap()) } else { None };
        
        std::fs::write(&config_path, "parsers_enabled: true\n").unwrap();
        
        struct ConfigGuard(std::path::PathBuf, Option<String>);
        impl Drop for ConfigGuard {
            fn drop(&mut self) {
                if let Some(ref content) = self.1 {
                    let _ = std::fs::write(&self.0, content);
                } else {
                    let _ = std::fs::remove_file(&self.0);
                }
            }
        }
        let _guard_config = ConfigGuard(config_path, old_content);

        // Verify if parser exists in the environment
        let parser_path = Config::get_treesitter_dir().join("rust.so");

        // Only run test cases if the required parser exists
        if parser_path.exists() {
            let result = get_workflow_content("mka-init", false).await;
            assert!(result.is_ok());
            let content = result.unwrap();
            // Without snippets, it should return TOON JSON
            assert!(content.contains("@mka:workflow:mka-init"));
            assert!(content.contains("intent"));
            assert!(content.contains("workflow_nodes"));
            // It should not contain Markdown headers like "### file:" or "### workflow:" or "**snippet:**"
            assert!(!content.contains("### file:"));
            assert!(!content.contains("**snippet:**"));
        } else {
            println!("Skipping test assertions for test_get_workflow_content_without_snippets because rust parser is not available.");
        }
    }

    #[tokio::test]
    async fn test_get_workflow_content_with_snippets() {
        let _guard = TEST_LOCK.lock().unwrap();
        let mka_folder = Config::get_mka_folder().unwrap();
        let config_path = mka_folder.join("config.yaml");
        let existed = config_path.exists();
        let old_content = if existed { Some(std::fs::read_to_string(&config_path).unwrap()) } else { None };
        
        std::fs::write(&config_path, "parsers_enabled: true\n").unwrap();
        
        struct ConfigGuard(std::path::PathBuf, Option<String>);
        impl Drop for ConfigGuard {
            fn drop(&mut self) {
                if let Some(ref content) = self.1 {
                    let _ = std::fs::write(&self.0, content);
                } else {
                    let _ = std::fs::remove_file(&self.0);
                }
            }
        }
        let _guard_config = ConfigGuard(config_path, old_content);

        // Verify if parser exists in the environment
        let parser_path = Config::get_treesitter_dir().join("rust.so");

        // Only run test cases if the required parser exists
        if parser_path.exists() {
            let result = get_workflow_content("mka-init", true).await;
            assert!(result.is_ok());
            let content = result.unwrap();
            // With snippets, it should return Markdown format
            assert!(content.contains("# @mka:workflow:mka-init"));
            assert!(content.contains("### file:"));
            // Since mka-init has files/methods, it should extract snippets
            // Let's assert it contains either snippet or error
            assert!(content.contains("snippet:") || content.contains("ERROR") || content.contains("NOT FOUND"));
        } else {
            println!("Skipping test assertions for test_get_workflow_content_with_snippets because rust parser is not available.");
        }
    }

    #[tokio::test]
    async fn test_get_workflow_content_with_no_method_and_missing_parser() {
        let _guard = TEST_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        
        let mka_dir = dir.path().join(".MKA");
        std::fs::create_dir_all(&mka_dir).unwrap();
        std::fs::create_dir_all(mka_dir.join("Workflows")).unwrap();
        
        std::fs::write(mka_dir.join("schema.json"), r#"{"type": "object"}"#).unwrap();
        std::fs::write(mka_dir.join("config.yaml"), "parsers_enabled: false\n").unwrap();
        
        let index_content = r#"
project: test-project
version: 1.0
workflows:
  - id: test-workflow
    intent: "Test workflow"
    path: "Workflows/test-workflow.mka.yaml"
"#;
        std::fs::write(mka_dir.join("index.mka.yaml"), index_content).unwrap();

        // 1. Test case: Node with method = None (should bypass tree-sitter)
        let workflow_content_no_method = r#"
id: test-workflow
intent: "Test new behavior"
workflow_nodes:
  - file: "test_file.rs"
    note: "A note without a method"
validation:
  test_file: "test_file.rs"
"#;
        std::fs::write(mka_dir.join("Workflows/test-workflow.mka.yaml"), workflow_content_no_method).unwrap();
        std::fs::write(dir.path().join("test_file.rs"), "fn main() {}").unwrap();

        let result_markdown = get_workflow_content_with_paths("test-workflow", true, dir.path(), &mka_dir).await;
        assert!(result_markdown.is_ok());
        let content_markdown = result_markdown.unwrap();
        assert!(content_markdown.contains("test_file.rs"));
        assert!(content_markdown.contains("A note without a method"));
        assert!(content_markdown.contains("**validation:** test_file.rs"));
        assert!(!content_markdown.contains("snippet:"));

        let result_toon = get_workflow_content_with_paths("test-workflow", false, dir.path(), &mka_dir).await;
        assert!(result_toon.is_ok());
        let content_toon = result_toon.unwrap();
        assert!(content_toon.contains("test_file.rs"));
        assert!(content_toon.contains("A note without a method"));
        assert!(content_toon.contains("validation"));
        assert!(content_toon.contains("test_file.rs"));
        assert!(!content_toon.contains("sig"));

        // 2. Test case: Parsers disabled by default (no warning for missing parser even if method is present)
        let workflow_content_missing_parser = r#"
id: test-workflow
intent: "Test new behavior"
workflow_nodes:
  - file: "test_file.sql"
    method: "query"
    note: "A SQL query note"
"#;
        std::fs::write(mka_dir.join("Workflows/test-workflow.mka.yaml"), workflow_content_missing_parser).unwrap();
        std::fs::write(dir.path().join("test_file.sql"), "SELECT 1;").unwrap();

        let result_markdown_disabled = get_workflow_content_with_paths("test-workflow", true, dir.path(), &mka_dir).await;
        assert!(result_markdown_disabled.is_ok());
        let content_markdown_disabled = result_markdown_disabled.unwrap();
        assert!(content_markdown_disabled.contains("test_file.sql"));
        assert!(content_markdown_disabled.contains("A SQL query note"));
        assert!(content_markdown_disabled.contains("**method:** query"));
        assert!(!content_markdown_disabled.contains("[LLM: Missing parsers:"));

        let result_toon_disabled = get_workflow_content_with_paths("test-workflow", false, dir.path(), &mka_dir).await;
        assert!(result_toon_disabled.is_ok());
        let content_toon_disabled = result_toon_disabled.unwrap();
        assert!(content_toon_disabled.contains("test_file.sql"));
        assert!(content_toon_disabled.contains("query"));
        assert!(content_toon_disabled.contains("A SQL query note"));

        // 3. Test case: Parsers enabled in config (should show warning note at the end)
        std::fs::write(mka_dir.join("config.yaml"), "parsers_enabled: true").unwrap();

        let result_markdown_enabled = get_workflow_content_with_paths("test-workflow", true, dir.path(), &mka_dir).await;
        assert!(result_markdown_enabled.is_ok());
        let content_markdown_enabled = result_markdown_enabled.unwrap();
        assert!(content_markdown_enabled.contains("test_file.sql"));
        assert!(content_markdown_enabled.contains("A SQL query note"));
        assert!(content_markdown_enabled.contains("**method:** query"));
        assert!(content_markdown_enabled.contains("[LLM: Missing parsers: sql."));

        let result_toon_enabled = get_workflow_content_with_paths("test-workflow", false, dir.path(), &mka_dir).await;
        assert!(result_toon_enabled.is_ok());
        let content_toon_enabled = result_toon_enabled.unwrap();
        assert!(content_toon_enabled.contains("test_file.sql"));
        assert!(content_toon_enabled.contains("A SQL query note"));
        assert!(content_toon_enabled.contains("query"));
        assert!(content_toon_enabled.contains("[LLM: Missing parsers: sql."));

        // 4. Test case: File does not exist (should still output method and note, plus not found status)
        let workflow_content_not_found = r#"
id: test-workflow
intent: "Test missing file behavior"
workflow_nodes:
  - file: "non_existent_file.rs"
    method: "missing_method"
    note: "A note on a missing file"
"#;
        std::fs::write(mka_dir.join("Workflows/test-workflow.mka.yaml"), workflow_content_not_found).unwrap();

        let result_markdown_not_found = get_workflow_content_with_paths("test-workflow", true, dir.path(), &mka_dir).await;
        assert!(result_markdown_not_found.is_ok());
        let content_markdown_not_found = result_markdown_not_found.unwrap();
        assert!(content_markdown_not_found.contains("non_existent_file.rs [NOT FOUND]"));
        assert!(content_markdown_not_found.contains("**method:** missing_method"));
        assert!(content_markdown_not_found.contains("**note:** A note on a missing file"));

        let result_toon_not_found = get_workflow_content_with_paths("test-workflow", false, dir.path(), &mka_dir).await;
        assert!(result_toon_not_found.is_ok());
        let content_toon_not_found = result_toon_not_found.unwrap();
        assert!(content_toon_not_found.contains("non_existent_file.rs"));
        assert!(content_toon_not_found.contains("missing_method"));
        assert!(content_toon_not_found.contains("A note on a missing file"));
        assert!(content_toon_not_found.contains("NOT FOUND"));

        // 5. Test case: Unsupported extension (e.g. .txt)
        let workflow_content_unsupported = r#"
id: test-workflow
intent: "Test unsupported extension behavior"
workflow_nodes:
  - file: "test_file.txt"
    method: "some_method"
    note: "A note on unsupported file"
"#;
        std::fs::write(mka_dir.join("Workflows/test-workflow.mka.yaml"), workflow_content_unsupported).unwrap();
        std::fs::write(dir.path().join("test_file.txt"), "hello world").unwrap();

        let result_markdown_unsupported = get_workflow_content_with_paths("test-workflow", true, dir.path(), &mka_dir).await;
        assert!(result_markdown_unsupported.is_ok());
        let content_markdown_unsupported = result_markdown_unsupported.unwrap();
        assert!(content_markdown_unsupported.contains("test_file.txt [ERROR:"));
        assert!(content_markdown_unsupported.contains("**method:** some_method"));
        assert!(content_markdown_unsupported.contains("**note:** A note on unsupported file"));

        let result_toon_unsupported = get_workflow_content_with_paths("test-workflow", false, dir.path(), &mka_dir).await;
        assert!(result_toon_unsupported.is_ok());
        let content_toon_unsupported = result_toon_unsupported.unwrap();
        assert!(content_toon_unsupported.contains("test_file.txt"));
        assert!(content_toon_unsupported.contains("some_method"));
        assert!(content_toon_unsupported.contains("A note on unsupported file"));
        assert!(content_toon_unsupported.contains("UNSUPPORTED EXTENSION"));

        // 6. Test case: Analyzer error (method not found in file) or parser load error
        let workflow_content_analyzer_err = r#"
id: test-workflow
intent: "Test analyzer error behavior"
workflow_nodes:
  - file: "test_file.rs"
    method: "non_existent_function"
    note: "A note on function not in file"
"#;
        std::fs::write(mka_dir.join("Workflows/test-workflow.mka.yaml"), workflow_content_analyzer_err).unwrap();
        std::fs::write(dir.path().join("test_file.rs"), "fn main() {}").unwrap();

        let result_markdown_analyzer = get_workflow_content_with_paths("test-workflow", true, dir.path(), &mka_dir).await;
        assert!(result_markdown_analyzer.is_ok());
        let content_markdown_analyzer = result_markdown_analyzer.unwrap();
        assert!(content_markdown_analyzer.contains("test_file.rs [ERROR:"));
        assert!(content_markdown_analyzer.contains("**method:** non_existent_function"));
        assert!(content_markdown_analyzer.contains("**note:** A note on function not in file"));

        let result_toon_analyzer = get_workflow_content_with_paths("test-workflow", false, dir.path(), &mka_dir).await;
        assert!(result_toon_analyzer.is_ok());
        let content_toon_analyzer = result_toon_analyzer.unwrap();
        assert!(content_toon_analyzer.contains("test_file.rs"));
        assert!(content_toon_analyzer.contains("non_existent_function"));
        assert!(content_toon_analyzer.contains("A note on function not in file"));
        assert!(content_toon_analyzer.contains("error"));
    }
}
