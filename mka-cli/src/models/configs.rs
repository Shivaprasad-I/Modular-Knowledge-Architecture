use std::path::PathBuf;
use anyhow::Result;
use crate::utils::find_mka_root;

pub struct Config;
impl Config{
    pub const DIR_NAME: &'static str = ".MKA";
    pub const TEMPLATE_DIR: &'static str = "templates";
    pub const REPO_URL: &'static str = "https://github.com/Shivaprasad-I/Modular-Knowledge-Architecture.git";
    pub const TEMP_DIR: &'static str = ".mka_temp";

    pub fn get_mka_folder() -> Result<PathBuf> {
        Ok(find_mka_root()?.join(Config::DIR_NAME))
    }

    pub fn get_index_file() -> Result<PathBuf> {
        Ok(Config::get_mka_folder()?.join("index.mka.yaml"))
    }

    pub fn get_schema_file() -> Result<PathBuf> {
        Ok(Config::get_mka_folder()?.join("schema.json"))
    }

    pub fn get_treesitter_dir() -> PathBuf {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        
        let folder_name = if cfg!(debug_assertions) {
            "treesitter-debug"
        } else {
            "treesitter"
        };

        PathBuf::from(home).join(".mka").join(folder_name)
    }
}

#[cfg(test)]
mod tests;
