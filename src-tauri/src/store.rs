use crate::models::{Command, Config};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Expands `~` to the user's home directory.
///
/// If the path starts with `~`, it replaces it with the value of the `HOME` environment variable.
/// If `HOME` is not set, it returns the original string.
pub fn expand_path(path_str: &str) -> String {
    if path_str.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            if let Some(stripped) = path_str.strip_prefix('~') {
                return format!("{}{}", home, stripped);
            }
        }
    }
    path_str.to_string()
}

/// Retrieves all commands from persistent storage.
///
/// Returns an empty vector if the file doesn't exist. This allows the app to start
/// with no commands and add them later.
///
/// # Arguments
///
/// * `path` - Path to the commands JSON file
///
/// # Returns
///
/// * `Ok(Vec<Command>)` - Vector of commands (empty if file doesn't exist)
/// * `Err(String)` - Error if file cannot be read or JSON is invalid
pub fn get_commands(path: &Path) -> Result<Vec<Command>, String> {
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let commands: Vec<Command> = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(commands)
}

/// Saves commands to persistent storage.
///
/// Creates the parent directory if it doesn't exist. Writes commands as
/// pretty-printed JSON for human readability.
///
/// # Arguments
///
/// * `path` - Path where the commands JSON file should be saved
/// * `commands` - Slice of commands to save
///
/// # Returns
///
/// * `Ok(())` - Commands were successfully saved
/// * `Err(String)` - Error if directory creation or file write fails
pub fn save_commands(path: &Path, commands: &[Command]) -> Result<(), String> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let file = File::create(path).map_err(|e| e.to_string())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, commands).map_err(|e| e.to_string())?;
    Ok(())
}

/// Retrieves application configuration from storage.
///
/// Returns default configuration (safe_mode: false) if the file doesn't exist.
/// This allows the app to run with sensible defaults on first launch.
///
/// # Arguments
///
/// * `path` - Path to the config JSON file
///
/// # Returns
///
/// * `Ok(Config)` - Configuration object (default if file doesn't exist)
/// * `Err(String)` - Error if file cannot be read or JSON is invalid
pub fn get_config(path: &Path) -> Result<Config, String> {
    if !path.exists() {
        // Return default config if file doesn't exist
        return Ok(Config {
            safe_mode: false,
            commands_path: None,
        });
    }

    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(config)
}

/// Saves application configuration to persistent storage.
///
/// Creates the parent directory if it doesn't exist. Writes config as
/// pretty-printed JSON for human readability.
///
/// # Arguments
///
/// * `path` - Path where the config JSON file should be saved
/// * `config` - Configuration object to save
///
/// # Returns
///
/// * `Ok(())` - Configuration was successfully saved
/// * `Err(String)` - Error if directory creation or file write fails
pub fn save_config(path: &Path, config: &Config) -> Result<(), String> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let file = File::create(path).map_err(|e| e.to_string())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, config).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Command;
    use std::fs;

    #[test]
    fn test_save_and_load_commands() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_store_tdd.json");

        // Ensure clean state
        if file_path.exists() {
            let _ = fs::remove_file(&file_path);
        }

        // Test loading non-existent file returns empty
        let initial_load =
            get_commands(&file_path).expect("Should return empty list for missing file");
        assert!(initial_load.is_empty());

        let commands = vec![
            Command {
                id: "1".to_string(),
                name: "Test 1".to_string(),
                script: "echo 1".to_string(),
                kill_script: None,
                shortcut: None,
                description: None,
            },
            Command {
                id: "2".to_string(),
                name: "Test 2".to_string(),
                script: "echo 2".to_string(),
                kill_script: Some("pkill 2".to_string()),
                shortcut: Some("Ctrl+2".to_string()),
                description: Some("Description".to_string()),
            },
        ];

        save_commands(&file_path, &commands).expect("Failed to save commands");

        let loaded = get_commands(&file_path).expect("Failed to load commands");

        assert_eq!(
            commands.len(),
            loaded.len(),
            "Loaded commands length mismatch"
        );
        assert_eq!(commands[0].id, loaded[0].id);
        assert_eq!(commands[0].name, loaded[0].name);
        assert_eq!(commands[1].id, loaded[1].id);
        assert_eq!(commands[1].name, loaded[1].name);

        // Cleanup
        if file_path.exists() {
            let _ = fs::remove_file(&file_path);
        }
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_config.json");

        // Cleanup before test
        if file_path.exists() {
            let _ = fs::remove_file(&file_path);
        }

        // Test loading non-existent file returns default
        let default_config = get_config(&file_path).expect("Should return default config");
        assert_eq!(default_config.safe_mode, false);

        // Save a config with safe mode enabled
        let config = Config {
            safe_mode: true,
            commands_path: None,
        };
        save_config(&file_path, &config).expect("Failed to save config");

        // Load and verify
        let loaded = get_config(&file_path).expect("Failed to load config");
        assert_eq!(loaded.safe_mode, true);

        // Cleanup
        if file_path.exists() {
            let _ = fs::remove_file(&file_path);
        }
    }

    #[test]
    fn test_expand_path() {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        
        let path = "~/test/file.json";
        let expanded = expand_path(path);
        
        assert!(expanded.starts_with(&home));
        
        // This check depends on whether HOME ends with a slash, so we should be careful.
        // Usually HOME is /Users/z without trailing slash.
        // expanded should be /Users/z/test/file.json
        // Or if HOME is /, expanded is //test/file.json which is valid.
        
        let expected_suffix = if home.ends_with('/') {
            "test/file.json"
        } else {
            "/test/file.json"
        };
        
        assert!(expanded.ends_with(expected_suffix));
        
        let absolute_path = "/usr/bin/test";
        let not_expanded = expand_path(absolute_path);
        assert_eq!(not_expanded, absolute_path);

        // Test multiple tildes - only the first should be expanded
        let multi_tilde = "~/Documents/file~backup.json";
        let expanded_multi = expand_path(multi_tilde);
        // Should start with HOME
        assert!(expanded_multi.starts_with(&home));
        // Should contain the second tilde
        assert!(expanded_multi.contains("file~backup.json"));
        // Should NOT contain HOME twice (unless HOME contains a tilde, which is rare)
        let home_count = expanded_multi.matches(&home).count();
        assert_eq!(home_count, 1);
    }

    #[test]
    fn test_ensure_directory_creation() {
        let temp_dir = std::env::temp_dir();
        let test_subdir = temp_dir.join("climgr_test_subdir");
        let file_path = test_subdir.join("commands.json");
        
        // Ensure clean state
        if test_subdir.exists() {
            let _ = fs::remove_dir_all(&test_subdir);
        }
        
        // Try saving commands - this should create the directory (which is what ensure_storage_directory relies on effectively)
        let commands = vec![];
        save_commands(&file_path, &commands).expect("Should create directory and save");
        
        assert!(test_subdir.exists());
        assert!(file_path.exists());
        
        // Cleanup
        if test_subdir.exists() {
            let _ = fs::remove_dir_all(&test_subdir);
        }
    }
}
