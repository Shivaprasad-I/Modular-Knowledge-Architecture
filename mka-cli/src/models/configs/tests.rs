#[cfg(test)]
mod tests {
    use crate::models::configs::Config;
    use std::env;

    #[test]
    fn test_treesitter_dir_default() {
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
}
