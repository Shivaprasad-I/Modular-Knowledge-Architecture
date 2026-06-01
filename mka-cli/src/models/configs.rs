use std::path::PathBuf;
use anyhow::Result;
use sha2::{Sha256, Digest};
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

    pub fn get_app_data_dir() -> Result<PathBuf> {
        let path = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find application data directory"))?
            .join("mka");
        
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        Ok(path)
    }

    pub fn get_project_hash() -> Result<String> {
        let root = find_mka_root()?;
        let path_str = root.to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn get_db_path() -> Result<PathBuf> {
        let db_dir = Config::get_app_data_dir()?.join("databases");
        if !db_dir.exists() {
            std::fs::create_dir_all(&db_dir)?;
        }
        let hash = Config::get_project_hash()?;
        Ok(db_dir.join(format!("{}.db", hash)))
    }

    pub fn get_model_path() -> Result<PathBuf> {
        let model_dir = Config::get_app_data_dir()?.join("models");
        if !model_dir.exists() {
            std::fs::create_dir_all(&model_dir)?;
        }
        Ok(model_dir.join("all-MiniLM-L6-v2.onnx"))
    }

    pub fn get_tokenizer_path() -> Result<PathBuf> {
        let model_dir = Config::get_app_data_dir()?.join("models");
        if !model_dir.exists() {
            std::fs::create_dir_all(&model_dir)?;
        }
        Ok(model_dir.join("tokenizer.json"))
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
