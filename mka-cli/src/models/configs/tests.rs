#[cfg(test)]
mod tests {
    use crate::models::configs::Config;
    use std::env;

    use crate::models::configs::TEST_LOCK;

    #[test]
    fn test_treesitter_dir_default() {
        let _guard = TEST_LOCK.lock().unwrap();
        let original_home = env::var("HOME");
        // Mock HOME
        env::set_var("HOME", "/home/testuser");
        let dir = Config::get_treesitter_dir();
        
        let contains_correct_folder = if cfg!(debug_assertions) {
            dir.to_string_lossy().contains("treesitter-debug")
        } else {
            dir.to_string_lossy().contains("treesitter")
        };

        assert!(contains_correct_folder);
        assert!(dir.to_string_lossy().contains(".mka"));

        // Restore
        if let Ok(val) = original_home {
            env::set_var("HOME", val);
        } else {
            env::remove_var("HOME");
        }
    }

    #[test]
    fn test_load_config_user_level() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let old_xdg = env::var("XDG_CONFIG_HOME");
        let old_appdata = env::var("APPDATA");

        env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        env::set_var("APPDATA", temp_dir.path());

        // Create user config directory and file
        let user_config_dir = temp_dir.path().join("mka");
        std::fs::create_dir_all(&user_config_dir).unwrap();
        std::fs::write(user_config_dir.join("config.yaml"), "parsers_enabled: true\n").unwrap();

        let config = Config::load_config(None);
        assert!(config.parsers_enabled());

        // Restore env
        if let Ok(ref val) = old_xdg {
            env::set_var("XDG_CONFIG_HOME", val);
        } else {
            env::remove_var("XDG_CONFIG_HOME");
        }
        if let Ok(ref val) = old_appdata {
            env::set_var("APPDATA", val);
        } else {
            env::remove_var("APPDATA");
        }
    }

    #[test]
    fn test_config_overrides() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let old_xdg = env::var("XDG_CONFIG_HOME");
        let old_appdata = env::var("APPDATA");

        env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        env::set_var("APPDATA", temp_dir.path());

        // Create user config directory and file
        let user_config_dir = temp_dir.path().join("mka");
        std::fs::create_dir_all(&user_config_dir).unwrap();
        
        let config_yaml = r#"
parsers_enabled: true
repo_url: "https://example.com/custom.git"
template_dir: "custom_templates"
temp_dir: ".custom_temp"
model_url: "https://example.com/model.onnx"
tokenizer_url: "https://example.com/tokenizer.json"
treesitter_dir: "/custom/treesitter"
model_path: "/custom/model.onnx"
tokenizer_path: "/custom/tokenizer.json"
db_path: "/custom/project.db"
index_file: "/custom/index.yaml"
schema_file: "/custom/schema.json"
"#;
        std::fs::write(user_config_dir.join("config.yaml"), config_yaml).unwrap();

        let config = Config::load_config(None);
        assert!(config.parsers_enabled());
        assert_eq!(config.repo_url(), "https://example.com/custom.git");
        assert_eq!(config.template_dir(), "custom_templates");
        assert_eq!(config.temp_dir(), ".custom_temp");
        assert_eq!(config.model_url(), "https://example.com/model.onnx");
        assert_eq!(config.tokenizer_url(), "https://example.com/tokenizer.json");
        
        // Assert the getter paths return overridden values
        assert_eq!(Config::get_index_file().unwrap().to_str().unwrap(), "/custom/index.yaml");
        assert_eq!(Config::get_treesitter_dir().to_str().unwrap(), "/custom/treesitter");

        // Restore env
        if let Ok(ref val) = old_xdg {
            env::set_var("XDG_CONFIG_HOME", val);
        } else {
            env::remove_var("XDG_CONFIG_HOME");
        }
        if let Ok(ref val) = old_appdata {
            env::set_var("APPDATA", val);
        } else {
            env::remove_var("APPDATA");
        }
    }

    #[test]
    fn test_custom_paths_parent_directory_creation() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let old_xdg = env::var("XDG_CONFIG_HOME");
        let old_appdata = env::var("APPDATA");

        env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        env::set_var("APPDATA", temp_dir.path());

        // Create user config directory and file with parent path in temp_dir
        let user_config_dir = temp_dir.path().join("mka");
        std::fs::create_dir_all(&user_config_dir).unwrap();

        let test_parent = temp_dir.path().join("nested_folder");
        let test_db = test_parent.join("test.db");
        let test_model = test_parent.join("test.onnx");
        let test_tokenizer = test_parent.join("test.json");

        assert!(!test_parent.exists());

        let config_yaml = format!(
            "db_path: \"{}\"\nmodel_path: \"{}\"\ntokenizer_path: \"{}\"\n",
            test_db.to_string_lossy().replace('\\', "/"),
            test_model.to_string_lossy().replace('\\', "/"),
            test_tokenizer.to_string_lossy().replace('\\', "/")
        );
        std::fs::write(user_config_dir.join("config.yaml"), config_yaml).unwrap();

        // Calling getters should trigger creation of test_parent
        let db_path = Config::get_db_path().unwrap();
        let model_path = Config::get_model_path().unwrap();
        let tokenizer_path = Config::get_tokenizer_path().unwrap();

        assert_eq!(db_path, test_db);
        assert_eq!(model_path, test_model);
        assert_eq!(tokenizer_path, test_tokenizer);
        assert!(test_parent.exists());

        // Restore env
        if let Ok(ref val) = old_xdg {
            env::set_var("XDG_CONFIG_HOME", val);
        } else {
            env::remove_var("XDG_CONFIG_HOME");
        }
        if let Ok(ref val) = old_appdata {
            env::set_var("APPDATA", val);
        } else {
            env::remove_var("APPDATA");
        }
    }
}
