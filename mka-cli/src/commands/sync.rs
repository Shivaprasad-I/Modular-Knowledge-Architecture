use std::path::Path;
use std::fs;
use anyhow::Result;
use crate::models::{MkaIndex, TriggerMap};
use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};

use crate::utils::find_mka_root;
use crate::models::configs::Config;

pub fn handle() -> Result<()> {
    let mka_root = find_mka_root()?;
    let mka_folder = Config::get_mka_folder()?;
    let index_path = Config::get_index_file()?;
    let index_content = fs::read_to_string(&index_path)?;
    let index: MkaIndex = serde_yaml::from_str(&index_content)?;

    let mut loader = DynamicLanguageLoader::new();

    for summary in &index.trigger_maps {
        let map_path = mka_folder.join(&summary.path);
        let map_content = fs::read_to_string(&map_path)?;
        let mut trigger_map: TriggerMap = serde_yaml::from_str(&map_content)?;

        let mut changed = false;
        println!("Checking Trigger Map: {}", trigger_map.id);

        for node in &mut trigger_map.trigger_nodes {
            if node.trigger_map.is_some() {
                continue; // Skip cross-references
            }

            let file_path_str = node.file.as_deref().unwrap_or("");
            let file_path = mka_root.join(file_path_str);
            let method_name = node.method.as_deref().unwrap_or("main");

            if file_path_str.is_empty() || !file_path.exists() {
                println!("  Node {}::{} is broken. Searching for healing...", file_path_str, method_name);
                if let Some(new_file) = heal_path(&mka_root, file_path_str) {
                    println!("    Healed to: {}", new_file);
                    node.file = Some(new_file);
                    changed = true;
                } else {
                    println!("    Failed to heal node {}::{}.", file_path_str, method_name);
                }
            } else {
                // Verify method exists
                let source = fs::read_to_string(&file_path)?;
                let lang_name = crate::utils::get_language_from_path(&file_path).unwrap_or("text");
                if let Ok(lang) = loader.load_language(lang_name) {
                    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);
                    if analyzer.get_method_signature(method_name).is_err() {
                        println!("  Method {} not found in {}. Searching...", method_name, file_path_str);
                        // Future: implement method healing
                    }
                }
            }
        }

        if changed {
            let updated_content = serde_yaml::to_string(&trigger_map)?;
            fs::write(&map_path, updated_content)?;
            println!("  Updated trigger map file: {}", summary.path);
        }
    }

    let updated_index = serde_yaml::to_string(&index)?;
    fs::write(&index_path, updated_index)?;

    Ok(())
}

fn heal_path(mka_root: &Path, old_path: &str) -> Option<String> {
    let path = Path::new(old_path);
    let filename = path.file_name()?.to_str()?;

    // Search for filename relative to the MKA root
    for entry in walkdir::WalkDir::new(mka_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str() == Some(filename)) 
    {
        // Strip the absolute mka_root prefix to store relative path in YAML
        if let Ok(rel_path) = entry.path().strip_prefix(mka_root) {
            return Some(rel_path.to_string_lossy().to_string());
        }
    }

    None
}

#[cfg(test)]
#[path = "sync_tests.rs"]
mod sync_tests;
