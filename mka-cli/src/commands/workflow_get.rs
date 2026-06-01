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
    let index_path = Config::get_index_file()?;
    let content = tokio::fs::read_to_string(&index_path).await?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    let map_summary = index.workflows.iter()
        .find(|w| w.id == id)
        .context(format!("Workflow '{}' not found in index.", id))?;

    let map_path = Config::get_mka_folder()?.join(&map_summary.path);
    let map_content = tokio::fs::read_to_string(&map_path).await?;
    
    let schema_path = Config::get_schema_file()?;
    validate_yaml(&map_content, &schema_path).await?;
    
    let workflow: Workflow = serde_yaml::from_str(&map_content)?;
    let project_root = find_mka_root()?;

    let mut output = String::new();

    if snippets {
        output.push_str(&format!("# @mka:workflow:{}\n", workflow.id));
        output.push_str(&format!("**intent:** {}\n\n", workflow.intent));

        let mut loader = DynamicLanguageLoader::new();
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
                output.push_str(&format!("### file: {} [NOT FOUND]\n\n", file_path_str));
                continue;
            }

            let source = tokio::fs::read_to_string(&file_path).await?;
            let lang_name = crate::utils::get_language_from_path(&file_path).unwrap_or("text");
            let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or(lang_name);

            match loader.load_language(lang_name) {
                Ok(lang) => {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    let method_name = node.method.as_deref().unwrap_or("main");
                    
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
                        Err(e) => output.push_str(&format!("### file: {} [ERROR: {}]\n\n", file_path_str, e)),
                    }
                }
                Err(e) => output.push_str(&format!("### file: {} [ERROR: {}]\n\n", file_path_str, e)),
            }
        }
    } else {
        let mut map_obj = serde_json::Map::new();
        map_obj.insert("intent".to_string(), serde_json::json!(workflow.intent));

        let mut loader = DynamicLanguageLoader::new();
        let mut nodes_array = Vec::new();

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
                node_obj.insert("error".to_string(), serde_json::json!("NOT FOUND"));
                nodes_array.push(serde_json::json!(node_obj));
                continue;
            }

            let source = tokio::fs::read_to_string(&file_path).await?;
            let lang_name = crate::utils::get_language_from_path(&file_path).unwrap_or("text");

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

        map_obj.insert("workflow_nodes".to_string(), serde_json::json!(nodes_array));

        let value = serde_json::json!({
            format!("@mka:workflow:{}", workflow.id): map_obj
        });

        output.push_str(&toon::encode(&value, None));
    }

    Ok(output)
}
