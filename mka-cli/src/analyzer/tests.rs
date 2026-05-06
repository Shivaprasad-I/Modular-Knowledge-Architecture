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
        println!("Minified output:\n{}", minified);
        
        // 1. Comments removed
        assert!(!minified.contains("# Comment"));
        
        // 2. Indentation preserved (Python IS indentation sensitive)
        assert!(minified.contains("if x > 5:"));
        let lines: Vec<&str> = minified.lines().collect();
        let if_idx = lines.iter().position(|l| l.contains("if x > 5:")).unwrap();
        let print_idx = lines.iter().position(|l| l.contains("print(\"Hello\")")).unwrap();
        
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

    #[test]
    fn test_minify_logic_python_try_except() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("python").expect("Python parser should be installed for tests");
        
        let source = r#"
def func_with_try():
    try:
        x = 1/0
    except ZeroDivisionError:
        x = 0
    return x
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "python".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("func_with_try").unwrap();
        
        println!("Minified output:\n{}", minified);
        
        // After fix, it should contain the logic
        assert!(minified.contains("x = 1/0"));
        // It might still contain the try/except keywords if I don't skip them specifically
    }

    #[test]
    fn test_minify_logic_python_try_except_else_finally() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("python").expect("Python parser should be installed for tests");
        
        let source = r#"
def complex_try():
    try:
        x = 1
    except:
        x = 2
    else:
        x = 3
    finally:
        cleanup()
    return x
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "python".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("complex_try").unwrap();
        
        println!("Minified output:\n{}", minified);
        
        assert!(minified.contains("x = 1"));
        assert!(minified.contains("x = 3"));
        assert!(minified.contains("cleanup()"));
        assert!(!minified.contains("x = 2"));
        assert!(!minified.contains("except"));
        assert!(!minified.contains("finally"));
        assert!(!minified.contains("else:"));
    }

    #[test]
    fn test_minify_logic_javascript_try_finally() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("javascript").expect("JS parser should be installed for tests");
        
        let source = r#"
function testFunc() {
    try {
        console.log("logic");
    } finally {
        console.log("cleanup");
    }
}
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "javascript".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("testFunc").unwrap();
        
        println!("Minified output:\n{}", minified);
        
        assert!(minified.contains("console.log(\"logic\")"));
        assert!(minified.contains("console.log(\"cleanup\")"));
        assert!(!minified.contains("try"));
        assert!(!minified.contains("finally"));
    }

    #[test]
    fn test_minify_logic_various_comments() {
        let mut loader = DynamicLanguageLoader::new();
        
        // Test Python inline and block-like comments (Python doesn't have true block comments, but multi-line strings are often used)
        let lang_py = loader.load_language("python").unwrap();
        let source_py = r#"
def py_func():
    x = 1 # inline comment
    """
    multi-line 
    docstring
    """
    return x
        "#;
        let analyzer_py = SourceAnalyzer::new(lang_py, "python".to_string(), source_py.to_string());
        let minified_py = analyzer_py.get_minified_logic("py_func").unwrap();
        println!("Minified Python with comments:\n{}", minified_py);
        assert!(!minified_py.contains("inline comment"));
        assert!(!minified_py.contains("multi-line"));
        assert!(!minified_py.contains("#"));

        // Test JavaScript inline and multi-line block comments
        let lang_js = loader.load_language("javascript").unwrap();
        let source_js = r#"
function jsFunc() {
    let x = 1; // inline
    /* 
       multi-line
       block 
    */
    return x; /* trailing */
}
        "#;
        let analyzer_js = SourceAnalyzer::new(lang_js, "javascript".to_string(), source_js.to_string());
        let minified_js = analyzer_js.get_minified_logic("jsFunc").unwrap();
        assert!(!minified_js.contains("inline"));
        assert!(!minified_js.contains("multi-line"));
        assert!(!minified_js.contains("trailing"));
        assert!(!minified_js.contains("/*"));
        assert!(!minified_js.contains("*/"));
        assert!(!minified_js.contains("//"));
    }

    #[test]
    fn test_minify_logic_csharp_try_finally() {
        let mut loader = DynamicLanguageLoader::new();
        // The user installed c-sharp
        let lang = loader.load_language("c-sharp").expect("C# parser should be installed");
        
        let source = r#"
class Test {
    void MyMethod() {
        try {
            DoWork(); // code
        } finally {
            Cleanup();
        }
    }
}
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "c-sharp".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("MyMethod").unwrap();
        
        println!("Minified C# output:\n{}", minified);
        
        assert!(minified.contains("DoWork()"));
        assert!(minified.contains("Cleanup()"));
        assert!(!minified.contains("try"));
        assert!(!minified.contains("finally"));
        assert!(!minified.contains("// code"));
    }

    #[test]
    fn test_minify_logic_rust_try_block_simulation() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("rust").expect("Rust parser should be installed");
        
        // Rust doesn't have a standard 'try' statement in the same way (it has try blocks in nightly or ? operator)
        // But we can test its comment removal and general block handling.
        let source = r#"
fn test_func() {
    /* multi-line 
       comment */
    let x = 5; // inline
    x
}
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "rust".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("test_func").unwrap();
        
        println!("Minified Rust output:\n{}", minified);
        
        assert!(minified.contains("let x = 5;"));
        assert!(!minified.contains("multi-line"));
        assert!(!minified.contains("inline"));
    }

    #[test]
    fn test_minify_logic_typescript_try_finally() {
        let mut loader = DynamicLanguageLoader::new();
        let lang = loader.load_language("typescript").expect("TypeScript parser should be installed");
        
        let source = r#"
function tsFunc() {
    try {
        const x: number = 1;
    } finally {
        cleanup();
    }
}
        "#;
        
        let analyzer = SourceAnalyzer::new(lang, "typescript".to_string(), source.to_string());
        let minified = analyzer.get_minified_logic("tsFunc").unwrap();
        
        println!("Minified TypeScript output:\n{}", minified);
        
        assert!(minified.contains("const x: number = 1"));
        assert!(minified.contains("cleanup()"));
        assert!(!minified.contains("try"));
        assert!(!minified.contains("finally"));
    }
}
