use std::path::Path;
use anyhow::{Result, Context, anyhow};
use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};
use crate::utils::get_language_from_path;

pub async fn handle(path: &str, method: &str) -> Result<()> {
    let output = get_method_toon(path, method).await?;
    println!("{}", output);
    Ok(())
}

pub async fn get_method_toon(path: &str, method: &str) -> Result<String> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Err(anyhow!("File not found: {}", path));
    }

    let lang_name = get_language_from_path(file_path)
        .context(format!("Unsupported file extension: {:?}", file_path.extension()))?;
    let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or(lang_name);

    let source = tokio::fs::read_to_string(file_path).await?;
    let mut loader = DynamicLanguageLoader::new();
    
    let lang = loader.load_language(lang_name)?;
    let analyzer = SourceAnalyzer::new(lang, lang_name.to_string(), source);

    let minified = analyzer.get_minified_logic(method)?;
    
    let mut output = format!("@mka:snippet:{}:{}\n", path, method);
    output.push_str(&format!("```{}\n", extension));
    output.push_str(&minified);
    output.push_str("\n```");

    Ok(output)
}
