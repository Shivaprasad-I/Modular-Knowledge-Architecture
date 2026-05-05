use tree_sitter::{Parser, Query, QueryCursor};
use libloading::{Library, Symbol};
use std::path::Path;
use anyhow::{Result, Context, anyhow};
use regex::Regex;
use streaming_iterator::StreamingIterator;

pub struct DynamicLanguageLoader {
    libs: Vec<Library>,
}

impl DynamicLanguageLoader {
    pub fn new() -> Self {
        Self { libs: Vec::new() }
    }

    pub fn load_language(&mut self, lang_name: &str) -> Result<tree_sitter::Language> {
        let ext = std::env::consts::DLL_EXTENSION;
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        
        let mut possible_paths = vec![
            format!("{}/.local/share/nvim/lazy/nvim-treesitter/parser/{}.{}", home, lang_name, ext), // Neovim (lazy.nvim)
            format!("{}/.local/share/nvim/site/pack/packer/start/nvim-treesitter/parser/{}.{}", home, lang_name, ext), // Neovim (packer.nvim)
            format!("{}/.cache/tree-sitter/lib/{}.{}", home, lang_name, ext),
            format!("./parsers/{}.{}", lang_name, ext),
        ];

        // Windows-specific Neovim paths
        if cfg!(windows) {
            if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                possible_paths.push(format!("{}/nvim-data/lazy/nvim-treesitter/parser/{}.{}", local_app_data, lang_name, ext));
            }
        } else {
            possible_paths.push(format!("/usr/local/lib/tree-sitter-{}.{}", lang_name, ext));
        }

        for path in possible_paths {
            if Path::new(&path).exists() {
                unsafe {
                    let lib = Library::new(&path)
                        .map_err(|e| anyhow!("Failed to load library {}: {}", path, e))?;
                    let symbol_name = format!("tree_sitter_{}", lang_name.replace('-', "_"));
                    let func: Symbol<unsafe extern "C" fn() -> tree_sitter::Language> = lib.get(symbol_name.as_bytes())
                        .map_err(|e| anyhow!("Failed to find symbol {} in {}: {}", symbol_name, path, e))?;
                    let lang = func();
                    self.libs.push(lib);
                    return Ok(lang);
                }
            }
        }

        Err(anyhow!(
            "Tree-sitter parser for '{}' not found. Please run 'mka install {}' or install it to a standard location.",
            lang_name, lang_name
        ))
    }
}

pub struct SourceAnalyzer {
    language: tree_sitter::Language,
    lang_name: String,
    source_code: String,
}

impl SourceAnalyzer {
    pub fn new(language: tree_sitter::Language, lang_name: String, source_code: String) -> Self {
        Self { language, lang_name, source_code }
    }

    pub fn get_method_signature(&self, method_name: &str) -> Result<String> {
        let mut parser = Parser::new();
        parser.set_language(&self.language)?;
        let tree = parser.parse(&self.source_code, None)
            .context("Failed to parse source code")?;

        let query_strs = vec![
            format!("(function_item (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(function_declaration (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(method_definition (property_identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(function_definition (identifier) @name (#eq? @name \"{}\"))", method_name),
        ];

        let mut cursor = QueryCursor::new();
        for q_str in &query_strs {
            if let Ok(query) = Query::new(&self.language, q_str) {
                let mut matches = cursor.matches(&query, tree.root_node(), self.source_code.as_bytes());
                while let Some(m) = matches.next() {
                    for capture in m.captures {
                        let node = capture.node.parent().unwrap_or(capture.node);
                        let sig = node.utf8_text(self.source_code.as_bytes())?;
                        if let Some(first_line) = sig.lines().next() {
                            return Ok(first_line.trim().to_string());
                        }
                    }
                }
            }
        }

        Err(anyhow!("Method '{}' not found via Tree-sitter", method_name))
    }

    pub fn get_minified_logic(&self, method_name: &str) -> Result<String> {
        let mut parser = Parser::new();
        parser.set_language(&self.language)?;
        let tree = parser.parse(&self.source_code, None)
            .context("Failed to parse source code")?;

        let query_strs = vec![
            format!("(function_item (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(function_declaration (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(method_definition (property_identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(function_definition (identifier) @name (#eq? @name \"{}\"))", method_name),
        ];

        let mut cursor = QueryCursor::new();
        for q_str in &query_strs {
            if let Ok(query) = Query::new(&self.language, q_str) {
                let mut matches = cursor.matches(&query, tree.root_node(), self.source_code.as_bytes());
                while let Some(m) = matches.next() {
                    if let Some(capture) = m.captures.iter().next() {
                        let node = capture.node.parent().unwrap_or(capture.node);
                        return self.minify_node(node);
                    }
                }
            }
        }

        Ok("[Method body not found]".to_string())
    }

    fn minify_node(&self, node: tree_sitter::Node) -> Result<String> {
        let mut result = String::new();
        let mut current_pos = node.start_byte();
        let end_pos = node.end_byte();

        self.traverse_and_minify(node, &mut result, &mut current_pos, end_pos)?;
        
        let re_empty_lines = Regex::new(r"\n\s*\n")?;
        let collapsed = re_empty_lines.replace_all(&result, "\n").to_string();
        
        let lines: Vec<&str> = collapsed.lines().collect();

        if self.lang_name == "python" {
            let min_indent = lines.iter()
                .find(|line| !line.trim().is_empty())
                .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
                .unwrap_or(0);
                
            let unindented = lines.iter()
                .map(|line| {
                    if line.trim().is_empty() {
                        ""
                    } else if line.chars().take_while(|c| c.is_whitespace()).count() >= min_indent {
                        let (idx, _) = line.char_indices().nth(min_indent).unwrap_or((line.len(), ' '));
                        &line[idx..]
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            Ok(unindented.trim().to_string())
        } else {
            // Trim ALL leading spaces for non-indentation sensitive languages
            let untabbed = lines.iter()
                .map(|line| line.trim_start())
                .collect::<Vec<_>>()
                .join("\n");
            Ok(untabbed.trim().to_string())
        }
    }

    fn traverse_and_minify(
        &self, 
        node: tree_sitter::Node, 
        result: &mut String, 
        current_pos: &mut usize,
        _end_boundary: usize
    ) -> Result<()> {
        let kind = node.kind();
        
        if kind.contains("comment") {
            *current_pos = node.end_byte();
            return Ok(());
        }

        if ["try_statement", "try_stmt", "handle_clause"].contains(&kind) {
            result.push_str(" /* -- error handling -- */ ");
            *current_pos = node.end_byte();
            return Ok(());
        }

        let text = node.utf8_text(self.source_code.as_bytes())?;
        if text.contains("@mka:ignore-start") {
            if let Some(ignore_end) = text.find("@mka:ignore-end") {
                result.push_str(" /* -- logic omitted -- */ ");
                *current_pos = node.start_byte() + ignore_end + "@mka:ignore-end".len();
                return Ok(());
            }
        }

        if node.child_count() == 0 {
            if node.start_byte() >= *current_pos {
                if let Ok(skipped_str) = std::str::from_utf8(&self.source_code.as_bytes()[*current_pos..node.start_byte()]) {
                    result.push_str(skipped_str);
                }
                result.push_str(node.utf8_text(self.source_code.as_bytes())?);
                *current_pos = node.end_byte();
            }
        } else {
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                self.traverse_and_minify(child, result, current_pos, _end_boundary)?;
            }
        }

        Ok(())
    }

    pub fn detect_models(&self, signature: &str) -> Result<Vec<(String, String)>> {
        let mut models = Vec::new();
        let re_type = Regex::new(r"\b([A-Z][a-zA-Z0-0_]*)\b")?;
        
        for cap in re_type.captures_iter(signature) {
            let model_name = &cap[1];
            if ["Result", "Option", "String", "Vec", "Self", "ClapParser", "Subcommand", "Library", "Symbol", "JSONSchema", "Value", "TsParser", "Query", "QueryCursor"].contains(&model_name) {
                continue;
            }

            if let Ok(model_sig) = self.extract_model_signature(model_name) {
                models.push((model_name.to_string(), model_sig));
            }
        }
        
        Ok(models)
    }

    fn extract_model_signature(&self, model_name: &str) -> Result<String> {
        let query_strs = vec![
            format!("(struct_item (identifier) @name (#eq? @name \"{}\"))", model_name),
            format!("(enum_item (identifier) @name (#eq? @name \"{}\"))", model_name),
            format!("(interface_declaration (identifier) @name (#eq? @name \"{}\"))", model_name),
            format!("(class_declaration (identifier) @name (#eq? @name \"{}\"))", model_name),
            format!("(type_alias_declaration (identifier) @name (#eq? @name \"{}\"))", model_name),
        ];

        let mut parser = Parser::new();
        parser.set_language(&self.language)?;
        let tree = parser.parse(&self.source_code, None).context("Failed to parse")?;
        let mut cursor = QueryCursor::new();

        for q_str in &query_strs {
            if let Ok(query) = Query::new(&self.language, q_str) {
                let mut matches = cursor.matches(&query, tree.root_node(), self.source_code.as_bytes());
                while let Some(m) = matches.next() {
                    for capture in m.captures {
                        let node = capture.node.parent().unwrap_or(capture.node);
                        let text = node.utf8_text(self.source_code.as_bytes())?;
                        if let Some(first_line) = text.lines().next() {
                            return Ok(first_line.trim().to_string());
                        }
                    }
                }
            }
        }

        Err(anyhow!("Model '{}' not found", model_name))
    }
}
