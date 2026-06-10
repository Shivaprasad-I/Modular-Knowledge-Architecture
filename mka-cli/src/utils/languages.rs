use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct LanguageMapping {
    pub extensions: &'static [&'static str],
    pub name: &'static str,
}

pub struct LanguageRegistry;

impl LanguageRegistry {
    pub const MAPPINGS: &'static [LanguageMapping] = &[
        LanguageMapping { extensions: &["rs"], name: "rust" },
        LanguageMapping { extensions: &["ts", "tsx", "js", "jsx"], name: "typescript" },
        LanguageMapping { extensions: &["py"], name: "python" },
        LanguageMapping { extensions: &["cs"], name: "c-sharp" },
        LanguageMapping { extensions: &["go"], name: "go" },
        LanguageMapping { extensions: &["c"], name: "c" },
        LanguageMapping { extensions: &["cpp", "cc", "cxx", "h", "hpp"], name: "cpp" },
        LanguageMapping { extensions: &["java"], name: "java" },
        LanguageMapping { extensions: &["rb"], name: "ruby" },
        LanguageMapping { extensions: &["php"], name: "php" },
        LanguageMapping { extensions: &["kt", "kts"], name: "kotlin" },
        LanguageMapping { extensions: &["swift"], name: "swift" },
        LanguageMapping { extensions: &["scala"], name: "scala" },
        LanguageMapping { extensions: &["sh", "bash", "zsh"], name: "bash" },
        LanguageMapping { extensions: &["sql"], name: "sql" },
        LanguageMapping { extensions: &["html", "htm"], name: "html" },
        LanguageMapping { extensions: &["css"], name: "css" },
        LanguageMapping { extensions: &["json"], name: "json" },
        LanguageMapping { extensions: &["yaml", "yml"], name: "yaml" },
        LanguageMapping { extensions: &["toml"], name: "toml" },
        LanguageMapping { extensions: &["md"], name: "markdown" },
        LanguageMapping { extensions: &["r", "R"], name: "r" },
        LanguageMapping { extensions: &["dart"], name: "dart" },
        LanguageMapping { extensions: &["elixir", "ex", "exs"], name: "elixir" },
        LanguageMapping { extensions: &["elm"], name: "elm" },
        LanguageMapping { extensions: &["hs"], name: "haskell" },
        LanguageMapping { extensions: &["lua"], name: "lua" },
        LanguageMapping { extensions: &["ml", "mli"], name: "ocaml" },
        LanguageMapping { extensions: &["pl", "pm"], name: "perl" },
        LanguageMapping { extensions: &["rkt"], name: "racket" },
        LanguageMapping { extensions: &["zig"], name: "zig" },
        LanguageMapping { extensions: &["jl"], name: "julia" },
        LanguageMapping { extensions: &["sol"], name: "solidity" },
        LanguageMapping { extensions: &["mk"], name: "make" },
        LanguageMapping { extensions: &["cmake"], name: "cmake" },
        LanguageMapping { extensions: &["xml"], name: "xml" },
    ];

    pub fn get_language_from_path(path: &Path) -> Option<&'static str> {
        let ext = path.extension()?.to_str()?;
        let ext_lower = ext.to_lowercase();
        Self::MAPPINGS.iter()
            .find(|mapping| mapping.extensions.iter().any(|&e| e.to_lowercase() == ext_lower))
            .map(|mapping| mapping.name)
    }
}
