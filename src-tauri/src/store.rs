use crate::models::{Command, Config};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn get_commands(path: &Path) -> Result<Vec<Command>, String> {
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let commands: Vec<Command> = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(commands)
}

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

pub fn get_config(path: &Path) -> Result<Config, String> {
    if !path.exists() {
        // Return default config if file doesn't exist
        return Ok(Config { safe_mode: false });
    }

    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(config)
}

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
                shortcut: None,
                description: None,
            },
            Command {
                id: "2".to_string(),
                name: "Test 2".to_string(),
                script: "echo 2".to_string(),
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
        let config = Config { safe_mode: true };
        save_config(&file_path, &config).expect("Failed to save config");

        // Load and verify
        let loaded = get_config(&file_path).expect("Failed to load config");
        assert_eq!(loaded.safe_mode, true);

        // Cleanup
        if file_path.exists() {
            let _ = fs::remove_file(&file_path);
        }
    }
}
