use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use crate::models::{MkaIndex, Workflow};
use crate::utils::validate_yaml;
use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};
use crate::models::configs::Config;

pub fn handle(id: &str, view: bool) -> Result<()> {
    let index_path = Path::new(Config::INDEX_FILE);
    let content = fs::read_to_string(index_path)?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    let workflow_summary = index.workflows.iter()
        .find(|w| w.id == id)
        .context(format!("Feature '{}' not found in index.", id))?;

    let workflow_path = Path::new(Config::MAIN_FOLDER).join(&workflow_summary.path);
    let workflow_content = fs::read_to_string(&workflow_path)?;
    
    let schema_path = Path::new(Config::SCHEMA_FILE);
    validate_yaml(&workflow_content, schema_path)?;
    
    let workflow: Workflow = serde_yaml::from_str(&workflow_content)?;

    if view {
        println!("# @mka:feature:{}", workflow.id);
        println!("**intent:** {}\n", workflow.intent);

        let mut loader = DynamicLanguageLoader::new();
        for node in workflow.nodes {
            let file_path = Path::new(&node.file);
            if !file_path.exists() {
                println!("### file: {} [NOT FOUND]\n", node.file);
                continue;
            }

            let source = fs::read_to_string(file_path)?;
            let lang_name = crate::utils::get_language_from_path(file_path).unwrap_or("text");

            match loader.load_language(lang_name) {
                Ok(lang) => {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    let method_name = node.method.as_deref().unwrap_or("main");
                    
                    match analyzer.get_method_signature(method_name) {
                        Ok(sig) => {
                            println!("### file: {}", node.file);
                            if let Some(note) = &node.note {
                                println!("**note:** {}", note);
                            }
                            
                            if let Ok(minified) = analyzer.get_minified_logic(method_name) {
                                println!("**snippet:**");
                                println!("```{}", lang_name);
                                println!("{}", minified);
                                println!("```\n");
                            }

                            if let Ok(models) = analyzer.detect_models(&sig) {
                                for (m_name, m_sig) in models {
                                    println!("**model {}:** `{}`", m_name, m_sig);
                                }
                            }
                        }
                        Err(e) => println!("### file: {} [ERROR: {}]\n", node.file, e),
                    }
                }
                Err(e) => println!("### file: {} [ERROR: {}]\n", node.file, e),
            }
        }
    } else {
        let mut feature_obj = serde_json::Map::new();
        feature_obj.insert("intent".to_string(), serde_json::json!(workflow.intent));

        let mut loader = DynamicLanguageLoader::new();
        let mut nodes_array = Vec::new();

        for node in workflow.nodes {
            let mut node_obj = serde_json::Map::new();
            node_obj.insert("file".to_string(), serde_json::json!(node.file));

            let file_path = Path::new(&node.file);
            if !file_path.exists() {
                node_obj.insert("error".to_string(), serde_json::json!("NOT FOUND"));
                nodes_array.push(serde_json::json!(node_obj));
                continue;
            }

            let source = fs::read_to_string(file_path)?;
            let lang_name = crate::utils::get_language_from_path(file_path).unwrap_or("text");

            if lang_name == "text" {
                node_obj.insert("error".to_string(), serde_json::json!("UNSUPPORTED EXTENSION"));
                nodes_array.push(serde_json::json!(node_obj));
                continue;
            }

            match loader.load_language(lang_name) {
                Ok(lang) => {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    let method_name = node.method.as_deref().unwrap_or("main");
                    
                    match analyzer.get_method_signature(method_name) {
                        Ok(sig) => {
                            if let Some(note) = &node.note {
                                node_obj.insert("note".to_string(), serde_json::json!(note));
                            }
                            node_obj.insert("sig".to_string(), serde_json::json!(sig));
                        }
                        Err(e) => {
                            node_obj.insert("error".to_string(), serde_json::json!(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    node_obj.insert("error".to_string(), serde_json::json!(e.to_string()));
                }
            }
            nodes_array.push(serde_json::json!(node_obj));
        }

        feature_obj.insert("nodes".to_string(), serde_json::json!(nodes_array));

        let value = serde_json::json!({
            format!("@mka:feature:{}", workflow.id): feature_obj
        });

        println!("{}", toon::encode(&value, None));
    }

    Ok(())
}
