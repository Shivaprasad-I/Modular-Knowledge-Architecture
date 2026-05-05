use std::path::Path;
use std::fs;
use anyhow::{Result, Context, anyhow};
use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};
use crate::utils::get_language_from_path;

pub fn handle(path: &str, method: &str) -> Result<()> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Err(anyhow!("File not found: {}", path));
    }

    let lang_name = get_language_from_path(file_path)
        .context(format!("Unsupported file extension: {:?}", file_path.extension()))?;
    let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or(lang_name);

    let source = fs::read_to_string(file_path)?;
    let mut loader = DynamicLanguageLoader::new();
    
    let lang = loader.load_language(lang_name)?;
    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);

    let minified = analyzer.get_minified_logic(method)?;
    
    println!("@mka:snippet:{}:{}", path, method);
    println!("```{}", extension);
    println!("{}", minified);
    println!("```");

    Ok(())
}
