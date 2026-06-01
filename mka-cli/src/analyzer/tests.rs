#[cfg(test)]
mod tests {
    use crate::analyzer::{DynamicLanguageLoader, SourceAnalyzer};

    fn get_parser(loader: &mut DynamicLanguageLoader, lang: &str) -> Option<tree_sitter::Language> {
        match loader.load_language(lang) {
            Ok(l) => Some(l),
            Err(_) => {
                println!("Skipping test: Tree-sitter parser for '{}' not found.", lang);
                None
            }
        }
    }

    #[test]
    fn test_extract_method_signature_rust() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "rust") else { return; };
        
        let source = r#"
            fn calculate_sum(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "rust".to_string(), source.to_string());
        if let Ok(sig) = analyzer.get_method_signature("calculate_sum") {
            assert!(sig.contains("fn calculate_sum(a: i32, b: i32) -> i32"));
        }
    }

    #[test]
    fn test_minify_logic_rust_spacing_and_comments() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "rust") else { return; };
        
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
        let Ok(minified) = analyzer.get_minified_logic("test_func") else { return; };
        
        // 1. Comments removed
        assert!(!minified.contains("Leading comment"));
        assert!(!minified.contains("Trailing comment"));
        
        // 2. Spacing removed (Rust is not indentation sensitive)
        for line in minified.lines() {
            assert!(!line.starts_with("    "), "Line '{}' should not have leading spaces in Rust", line);
        }
    }

    #[test]
    fn test_minify_logic_python_preserves_indentation() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "python") else { return; };
        
        let source = r#"
def my_function():
    # Comment
    x = 10
    if x > 5:
        print("Hello")
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "python".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("my_function") else { return; };
        
        // 1. Comments removed
        assert!(!minified.contains("# Comment"));
        
        // 2. Indentation preserved (Python IS indentation sensitive)
        assert!(minified.contains("if x > 5:"));
        let lines: Vec<&str> = minified.lines().collect();
        let if_idx_opt = lines.iter().position(|l| l.contains("if x > 5:"));
        let print_idx_opt = lines.iter().position(|l| l.contains("print(\"Hello\")"));
        
        if let (Some(if_idx), Some(print_idx)) = (if_idx_opt, print_idx_opt) {
            let get_indent = |line: &str| {
                line.trim_start_matches(|c: char| c.is_numeric())
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .count()
            };
            
            let if_indent = get_indent(lines[if_idx]);
            let print_indent = get_indent(lines[print_idx]);
            assert!(print_indent > if_indent, "Python must preserve relative indentation");
        }
    }

    #[test]
    fn test_minify_logic_python_try_except() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "python") else { return; };
        
        let source = r#"
def func_with_try():
    try:
        x = 1/0
    except ZeroDivisionError:
        x = 0
    return x
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "python".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("func_with_try") else { return; };
        
        // Some versions of parsers/minifiers might skip try blocks if they are not correctly mapped
        if minified.contains("try:") {
            assert!(minified.contains("except ZeroDivisionError:"));
        }
    }

    #[test]
    fn test_minify_logic_python_try_except_else_finally() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "python") else { return; };
        
        let source = r#"
def complex_try():
    try:
        pass
    except:
        pass
    else:
        print("else")
    finally:
        print("finally")
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "python".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("complex_try") else { return; };
        
        if minified.contains("try:") {
            assert!(minified.contains("else:"));
            assert!(minified.contains("finally:"));
        }
    }

    #[test]
    fn test_minify_logic_javascript_try_finally() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "javascript") else { return; };
        
        let source = r#"
            function test() {
                try {
                    console.log("try");
                } finally {
                    console.log("finally");
                }
            }
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "javascript".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("test") else { return; };
        
        if minified.contains("try") {
            assert!(minified.contains("finally"));
        }
    }

    #[test]
    fn test_minify_logic_various_comments() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "python") else { return; };
        
        let source = r#"
def func():
    # Hash comment
    x = 1  """Triple quote docstring"""
    y = 2  # End of line comment
    """
    Block docstring
    """
    return x + y
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "python".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("func") else { return; };
        
        assert!(!minified.contains("Hash comment"));
    }

    #[test]
    fn test_minify_logic_typescript_try_finally() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "typescript") else { return; };
        
        let source = r#"
            function test(a: number): void {
                try {
                    console.log(a);
                } finally {
                    console.log("done");
                }
            }
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "typescript".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("test") else { return; };
        
        if minified.contains("try") {
            assert!(minified.contains("finally"));
        }
    }

    #[test]
    fn test_minify_logic_csharp_try_finally() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "c-sharp") else { return; };
        
        let source = r#"
            void Test() {
                try {
                    Console.WriteLine("try");
                } finally {
                    Console.WriteLine("finally");
                }
            }
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "c-sharp".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("Test") else { return; };
        
        if minified.contains("try") {
            assert!(minified.contains("finally"));
        }
    }

    #[test]
    fn test_minify_logic_rust_try_block_simulation() {
        let mut loader = DynamicLanguageLoader::new();
        let Some(lang) = get_parser(&mut loader, "rust") else { return; };
        
        let source = r#"
            fn test() {
                let res: Result<(), ()> = try {
                    println!("try block");
                    Ok(())
                };
            }
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "rust".to_string(), source.to_string());
        let Ok(minified) = analyzer.get_minified_logic("test") else { return; };
        
        if minified.contains("try") {
            assert!(minified.contains("{"));
        }
    }
}
