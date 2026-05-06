#[cfg(test)]
mod tests {
    use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};

    #[test]
    fn test_extract_method_signature_rust() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("rust").expect("Rust parser should be installed for tests");
        
        let source = r#"
            fn calculate_sum(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "rust".to_string(), source.to_string());
        let sig = analyzer.get_method_signature("calculate_sum").unwrap();
        assert!(sig.contains("fn calculate_sum(a: i32, b: i32) -> i32"));
    }

    #[test]
    fn test_minify_logic_rust_spacing_and_comments() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("rust").expect("Rust parser should be installed for tests");
        
        let source = r#"
            fn test_func() {
                // Leading comment
                let x = 1;
                if true {
                    let y = 2; // Trailing comment
                }
            }
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "rust".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("test_func").unwrap();
        
        // 1. Comments removed
        assert!(!minified.contains("Leading comment"));
        assert!(!minified.contains("Trailing comment"));
        
        // 2. Spacing removed (Rust is not indentation sensitive)
        // Internal lines like "let x = 1;" should not have leading spaces
        for line in minified.lines() {
            assert!(!line.starts_with("    "), "Line '{}' should not have leading spaces in Rust", line);
        }
    }

    #[test]
    fn test_minify_logic_python_preserves_indentation() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("python").expect("Python parser should be installed for tests");
        
        let source = r#"
def my_function():
    # Comment
    x = 10
    if x > 5:
        print("Hello")
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "python".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("my_function").unwrap();
        
        // 1. Comments removed
        assert!(!minified.contains("# Comment"));
        
        // 2. Indentation preserved (Python IS indentation sensitive)
        assert!(minified.contains("if x > 5:"));
        let lines: Vec<&str> = minified.lines().collect();
        let if_idx = lines.iter().position(|l| l.contains("if x > 5:")).unwrap();
        let print_idx = lines.iter().position(|l| l.contains("print(\"Hello\")")).unwrap();
        
        let if_indent = lines[if_idx].chars().take_while(|c| c.is_whitespace()).count();
        let print_indent = lines[print_idx].chars().take_while(|c| c.is_whitespace()).count();
        
        assert!(print_indent > if_indent, "Python must preserve relative indentation");
    }
}
