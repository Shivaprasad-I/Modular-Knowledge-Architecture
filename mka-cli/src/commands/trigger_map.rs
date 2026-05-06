use std::fs;
use anyhow::{Result, Context};
use crate::models::{MkaIndex, TriggerMap};
use crate::utils::{validate_yaml, find_mka_root};
use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};
use crate::models::configs::Config;

pub fn handle(id: &str, snippets: bool) -> Result<()> {
    let index_path = Config::get_index_file()?;
    let content = fs::read_to_string(&index_path)?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    let map_summary = index.trigger_maps.iter()
        .find(|w| w.id == id)
        .context(format!("Trigger Map '{}' not found in index.", id))?;

    let map_path = Config::get_mka_folder()?.join(&map_summary.path);
    let map_content = fs::read_to_string(&map_path)?;
    
    let schema_path = Config::get_schema_file()?;
    validate_yaml(&map_content, &schema_path)?;
    
    let trigger_map: TriggerMap = serde_yaml::from_str(&map_content)?;
    let project_root = find_mka_root()?;

    if snippets {
        println!("# @mka:trigger-map:{}", trigger_map.id);
        println!("**intent:** {}\n", trigger_map.intent);

        let mut loader = DynamicLanguageLoader::new();
        for node in trigger_map.trigger_nodes {
            if let Some(ref ref_id) = node.trigger_map {
                println!("### trigger-map: {}", ref_id);
                if let Some(note) = &node.note {
                    println!("**note:** {}", note);
                }
                println!();
                continue;
            }

            let file_path_str = node.file.as_deref().unwrap_or("[MISSING FILE]");
            let file_path = project_root.join(file_path_str);
            if !file_path.exists() {
                println!("### file: {} [NOT FOUND]\n", file_path_str);
                continue;
            }

            let source = fs::read_to_string(&file_path)?;
            let lang_name = crate::utils::get_language_from_path(&file_path).unwrap_or("text");
            let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or(lang_name);

            match loader.load_language(lang_name) {
                Ok(lang) => {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    let method_name = node.method.as_deref().unwrap_or("main");
                    
                    match analyzer.get_method_signature(method_name) {
                        Ok(_sig) => {
                            println!("### file: {}", file_path_str);
                            if let Some(note) = &node.note {
                                println!("**note:** {}", note);
                            }
                            
                            if let Ok(minified) = analyzer.get_minified_logic(method_name) {
                                println!("**snippet:**");
                                println!("```{}", extension);
                                println!("{}", minified);
                                println!("```\n");
                            }
                        }
                        Err(e) => println!("### file: {} [ERROR: {}]\n", file_path_str, e),
                    }
                }
                Err(e) => println!("### file: {} [ERROR: {}]\n", file_path_str, e),
            }
        }
    } else {
        let mut map_obj = serde_json::Map::new();
        map_obj.insert("intent".to_string(), serde_json::json!(trigger_map.intent));

        let mut loader = DynamicLanguageLoader::new();
        let mut nodes_array = Vec::new();

        for node in trigger_map.trigger_nodes {
            let mut node_obj = serde_json::Map::new();
            
            if let Some(ref ref_id) = node.trigger_map {
                node_obj.insert("trigger_map".to_string(), serde_json::json!(ref_id));
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

            let source = fs::read_to_string(&file_path)?;
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

        map_obj.insert("trigger_nodes".to_string(), serde_json::json!(nodes_array));

        let value = serde_json::json!({
            format!("@mka:trigger-map:{}", trigger_map.id): map_obj
        });

        println!("{}", toon::encode(&value, None));
    }

    Ok(())
}
