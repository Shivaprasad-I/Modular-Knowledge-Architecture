use std::path::PathBuf;
use anyhow::Result;
use sha2::{Sha256, Digest};
use crate::utils::find_mka_root;

use std::path::Path;

pub struct Config;
impl Config{
    pub const DIR_NAME: &'static str = ".MKA";

    pub fn get_mka_folder() -> Result<PathBuf> {
        Ok(find_mka_root()?.join(Config::DIR_NAME))
    }

    pub fn get_index_file() -> Result<PathBuf> {
        let config = Config::load_config(None);
        if let Some(ref path_str) = config.index_file {
            return Ok(PathBuf::from(path_str));
        }
        Ok(Config::get_mka_folder()?.join("index.mka.yaml"))
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
        let config = Config::load_config(None);
        if let Some(ref path_str) = config.db_path {
            let path = PathBuf::from(path_str);
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            return Ok(path);
        }
        let db_dir = Config::get_app_data_dir()?.join("databases");
        if !db_dir.exists() {
            std::fs::create_dir_all(&db_dir)?;
        }
        let hash = Config::get_project_hash()?;
        Ok(db_dir.join(format!("{}.db", hash)))
    }

    pub fn get_model_path() -> Result<PathBuf> {
        let config = Config::load_config(None);
        if let Some(ref path_str) = config.model_path {
            let path = PathBuf::from(path_str);
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            return Ok(path);
        }
        let model_dir = Config::get_app_data_dir()?.join("models");
        if !model_dir.exists() {
            std::fs::create_dir_all(&model_dir)?;
        }
        Ok(model_dir.join("all-MiniLM-L6-v2.onnx"))
    }

    pub fn get_tokenizer_path() -> Result<PathBuf> {
        let config = Config::load_config(None);
        if let Some(ref path_str) = config.tokenizer_path {
            let path = PathBuf::from(path_str);
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            return Ok(path);
        }
        let model_dir = Config::get_app_data_dir()?.join("models");
        if !model_dir.exists() {
            std::fs::create_dir_all(&model_dir)?;
        }
        Ok(model_dir.join("tokenizer.json"))
    }

    pub fn get_treesitter_dir() -> PathBuf {
        let config = Config::load_config(None);
        if let Some(ref path_str) = config.treesitter_dir {
            return PathBuf::from(path_str);
        }
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

    pub fn get_user_config_dir() -> Result<PathBuf> {
        let path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find user config directory"))?
            .join("mka");
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        Ok(path)
    }

    pub fn get_user_config_file() -> Result<PathBuf> {
        Ok(Config::get_user_config_dir()?.join("config.yaml"))
    }

    pub fn load_config(mka_folder: Option<&Path>) -> MkaConfig {
        let mut config = MkaConfig::default();

        // 1. Load user-level config
        if let Ok(user_config_path) = Config::get_user_config_file() {
            if user_config_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&user_config_path) {
                    if let Ok(user_cfg) = serde_yaml::from_str::<MkaConfig>(&content) {
                        config = user_cfg;
                    }
                }
            }
        }

        // 2. Load project-level config (override)
        let folder = match mka_folder {
            Some(p) => Some(p.to_path_buf()),
            None => Config::get_mka_folder().ok(),
        };

        if let Some(f) = folder {
            let project_config_path = f.join("config.yaml");
            if project_config_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&project_config_path) {
                    if let Ok(project_cfg) = serde_yaml::from_str::<MkaConfig>(&content) {
                        if project_cfg.parsers_enabled.is_some() {
                            config.parsers_enabled = project_cfg.parsers_enabled;
                        }
                        if project_cfg.repo_url.is_some() {
                            config.repo_url = project_cfg.repo_url;
                        }
                        if project_cfg.template_dir.is_some() {
                            config.template_dir = project_cfg.template_dir;
                        }
                        if project_cfg.temp_dir.is_some() {
                            config.temp_dir = project_cfg.temp_dir;
                        }
                        if project_cfg.model_url.is_some() {
                            config.model_url = project_cfg.model_url;
                        }
                        if project_cfg.tokenizer_url.is_some() {
                            config.tokenizer_url = project_cfg.tokenizer_url;
                        }
                        if project_cfg.treesitter_dir.is_some() {
                            config.treesitter_dir = project_cfg.treesitter_dir;
                        }
                        if project_cfg.model_path.is_some() {
                            config.model_path = project_cfg.model_path;
                        }
                        if project_cfg.tokenizer_path.is_some() {
                            config.tokenizer_path = project_cfg.tokenizer_path;
                        }
                        if project_cfg.db_path.is_some() {
                            config.db_path = project_cfg.db_path;
                        }
                        if project_cfg.index_file.is_some() {
                            config.index_file = project_cfg.index_file;
                        }
                        if project_cfg.schema_file.is_some() {
                            config.schema_file = project_cfg.schema_file;
                        }
                    }
                }
            }
        }

        config
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct MkaConfig {
    pub parsers_enabled: Option<bool>,
    pub repo_url: Option<String>,
    pub template_dir: Option<String>,
    pub temp_dir: Option<String>,
    pub model_url: Option<String>,
    pub tokenizer_url: Option<String>,
    pub treesitter_dir: Option<String>,
    pub model_path: Option<String>,
    pub tokenizer_path: Option<String>,
    pub db_path: Option<String>,
    pub index_file: Option<String>,
    pub schema_file: Option<String>,
}

impl MkaConfig {
    pub fn parsers_enabled(&self) -> bool {
        self.parsers_enabled.unwrap_or(false)
    }

    pub fn repo_url(&self) -> &str {
        self.repo_url.as_deref().unwrap_or("https://github.com/Shivaprasad-I/Modular-Knowledge-Architecture.git")
    }

    pub fn template_dir(&self) -> &str {
        self.template_dir.as_deref().unwrap_or("templates")
    }

    pub fn temp_dir(&self) -> &str {
        self.temp_dir.as_deref().unwrap_or(".mka_temp")
    }

    pub fn model_url(&self) -> &str {
        self.model_url.as_deref().unwrap_or("https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx")
    }

    pub fn tokenizer_url(&self) -> &str {
        self.tokenizer_url.as_deref().unwrap_or("https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/tokenizer.json")
    }
}

#[cfg(test)]
pub static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
mod tests;
