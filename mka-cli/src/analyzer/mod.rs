use tree_sitter::{Parser, Query, QueryCursor};
use libloading::{Library, Symbol};
use anyhow::{Result, Context, anyhow};
use regex::Regex;
use streaming_iterator::StreamingIterator;
use crate::models::configs::Config;

pub struct DynamicLanguageLoader {
    libs: Vec<Library>,
}

impl DynamicLanguageLoader {
    pub fn new() -> Self {
        Self { libs: Vec::new() }
    }

    pub fn load_language(&mut self, lang_name: &str) -> Result<tree_sitter::Language> {
        let ext = std::env::consts::DLL_EXTENSION;
        let path = Config::get_treesitter_dir().join(format!("{}.{}", lang_name, ext));

        if path.exists() {
            unsafe {
                let lib = Library::new(&path)
                    .map_err(|e| anyhow!("Failed to load library {:?}: {}", path, e))?;
                let symbol_name = format!("tree_sitter_{}", lang_name.replace('-', "_"));
                let func: Symbol<unsafe extern "C" fn() -> tree_sitter::Language> = lib.get(symbol_name.as_bytes())
                    .map_err(|e| anyhow!("Failed to find symbol {} in {:?}: {}", symbol_name, path, e))?;
                let lang = func();
                self.libs.push(lib);
                return Ok(lang);
            }
        }

        Err(anyhow!(
            "Tree-sitter parser for '{}' not found at {:?}. Please run 'mka install {}'.",
            lang_name, path, lang_name
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
            format!("(method_definition (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(function_definition (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(method_declaration name: (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(method_declaration (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(constructor_declaration name: (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(constructor_declaration (identifier) @name (#eq? @name \"{}\"))", method_name),
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
            format!("(method_definition (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(function_definition (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(method_declaration name: (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(method_declaration (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(constructor_declaration name: (identifier) @name (#eq? @name \"{}\"))", method_name),
            format!("(constructor_declaration (identifier) @name (#eq? @name \"{}\"))", method_name),
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

        let processed_lines: Vec<String> = if self.lang_name == "python" {
            let min_indent = lines.iter()
                .find(|line| !line.trim().is_empty())
                .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
                .unwrap_or(0);
                
            lines.iter()
                .map(|line| {
                    if line.trim().is_empty() {
                        String::new()
                    } else if line.chars().take_while(|c| c.is_whitespace()).count() >= min_indent {
                        let (idx, _) = line.char_indices().nth(min_indent).unwrap_or((line.len(), ' '));
                        line[idx..].to_string()
                    } else {
                        line.to_string()
                    }
                })
                .collect()
        } else {
            lines.iter()
                .map(|line| line.trim_start().to_string())
                .collect()
        };

        let original_lines: Vec<&str> = self.source_code.lines().collect();
        let mut current_orig_line = node.start_position().row;

        let mapped_lines = processed_lines.iter().map(|line| {
            if line.trim().is_empty() {
                return String::new();
            }
            
            let trimmed = line.trim();
            let mut found_line = current_orig_line;
            for i in current_orig_line..original_lines.len() {
                if original_lines[i].contains(trimmed) {
                    found_line = i;
                    break;
                }
            }
            current_orig_line = found_line;
            
            format!("{} {}", found_line + 1, line)
        }).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n");

        Ok(mapped_lines.trim().to_string())
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
}

#[cfg(test)]
mod tests;
