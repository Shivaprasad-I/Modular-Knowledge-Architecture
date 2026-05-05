use std::path::Path;
use std::fs;
use anyhow::Result;
use crate::models::{MkaIndex, Workflow};
use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};
use walkdir::WalkDir;

pub fn handle() -> Result<()> {
    println!("Syncing MKA documentation...");
    let index_path = Path::new(".MKA/index.mka.yaml");
    let content = fs::read_to_string(index_path)?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;
    let mut loader = DynamicLanguageLoader::new();

    for summary in &index.workflows {
        let workflow_path = Path::new(".MKA").join(&summary.path);
        let workflow_content = fs::read_to_string(&workflow_path)?;
        let mut workflow: Workflow = serde_yaml::from_str(&workflow_content)?;
        let mut modified = false;

        println!("Checking workflow: {}", workflow.id);

        for node in &mut workflow.nodes {
            let file_path = Path::new(&node.file);
            let method_name = node.method.as_deref().unwrap_or("main");

            let exists = if file_path.exists() {
                let source = fs::read_to_string(file_path)?;
                let ext = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let lang_name = match ext {
                    "rs" => "rust",
                    "ts" | "tsx" | "js" | "jsx" => "typescript",
                    "py" => "python",
                    _ => "unknown",
                };

                if lang_name != "unknown" {
                    if let Ok(lang) = loader.load_language(lang_name) {
                        let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                        analyzer.get_method_signature(method_name).is_ok()
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if !exists {
                println!("  Node {}::{} is broken. Searching for healing...", node.file, method_name);
                if let Some(new_file) = find_method_in_project(method_name, &mut loader)? {
                    println!("    Healed! Found at {}", new_file);
                    node.file = new_file;
                    modified = true;
                } else {
                    println!("    Failed to heal node {}::{}.", node.file, method_name);
                }
            }
        }

        if modified {
            let updated_content = serde_yaml::to_string(&workflow)?;
            fs::write(&workflow_path, updated_content)?;
            println!("  Updated workflow file: {}", summary.path);
        }
    }

    Ok(())
}

fn find_method_in_project(method_name: &str, loader: &mut DynamicLanguageLoader) -> Result<Option<String>> {
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let lang_name = match ext {
                "rs" => "rust",
                "ts" | "tsx" | "js" | "jsx" => "typescript",
                "py" => "python",
                _ => continue,
            };

            if let Ok(lang) = loader.load_language(lang_name) {
                if let Ok(source) = fs::read_to_string(path) {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    if analyzer.get_method_signature(method_name).is_ok() {
                        return Ok(Some(path.to_string_lossy().into_owned()));
                    }
                }
            }
        }
    }

    Ok(None)
}
