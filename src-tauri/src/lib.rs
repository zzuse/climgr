pub mod models;
pub mod store;

use crate::models::{Command, Config};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

fn run_command_script(app_handle: &AppHandle, script: &str) -> Result<String, String> {
    // Check safe mode
    let config_path = get_config_path(app_handle)?;
    let config = store::get_config(&config_path)?;

    if config.safe_mode {
        return Err("Command execution disabled in safe mode. Disable safe mode in settings to execute commands.".to_string());
    }

    log::info!("Executing script: {}", script);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            Ok(format!("{}{}", stdout, stderr))
        }
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}

/// Executes a command by its ID.
///
/// This Tauri command looks up a command by its unique ID, checks if safe mode is enabled,
/// and executes the associated shell script if allowed.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle for accessing app data directories
/// * `command_id` - The unique identifier of the command to execute
///
/// # Returns
///
/// * `Ok(String)` - Combined stdout and stderr output from the command execution
/// * `Err(String)` - Error message if:
///   - Safe mode is enabled (command execution disabled)
///   - Command ID not found in the stored commands
///   - Failed to access app data directory
///   - Failed to read commands from storage
///   - Script execution failed
///
/// # Example
///
/// ```javascript
/// // From the frontend
/// import { invoke } from '@tauri-apps/api/core';
///
/// try {
///   const output = await invoke('execute_command', {
///     commandId: '123e4567-e89b-12d3-a456-426614174000'
///   });
///   console.log('Command output:', output);
/// } catch (error) {
///   console.error('Execution failed:', error);
/// }
/// ```
///
/// # Security
///
/// This function checks the safe mode configuration before executing any command.
/// If safe mode is enabled, execution will fail with an appropriate error message.
#[tauri::command]
async fn execute_command(app_handle: tauri::AppHandle, command_id: String) -> Result<String, String> {
    let path = get_store_path(&app_handle)?;
    let commands = store::get_commands(&path)?;

    let command = commands
        .iter()
        .find(|c| c.id == command_id)
        .ok_or_else(|| String::from("Command not found"))?;

    let script = command.script.clone();
    let app_handle_clone = app_handle.clone();

    tauri::async_runtime::spawn_blocking(move || run_command_script(&app_handle_clone, &script))
        .await
        .map_err(|e| format!("Failed to execute command task: {}", e))?
}

fn get_store_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("commands.json"))
}

fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("config.json"))
}

fn refresh_shortcuts(app_handle: &tauri::AppHandle) -> Result<(), String> {
    app_handle
        .global_shortcut()
        .unregister_all()
        .map_err(|e| e.to_string())?;

    let path = get_store_path(app_handle)?;
    // Ignore errors reading store, maybe empty
    if let Ok(commands) = store::get_commands(&path) {
        for command in commands {
            if let Some(shortcut) = command.shortcut {
                if !shortcut.trim().is_empty() {
                    // Best effort registration
                    if let Err(e) = app_handle.global_shortcut().register(shortcut.as_str()) {
                        log::error!("Failed to register shortcut '{}': {}", shortcut, e);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Retrieves all stored commands.
///
/// This Tauri command fetches all commands from persistent storage.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle for accessing app data directories
///
/// # Returns
///
/// * `Ok(Vec<Command>)` - A vector containing all stored commands
/// * `Err(String)` - Error message if:
///   - Failed to access app data directory
///   - Failed to read from storage file
///   - JSON parsing failed
///
/// # Example
///
/// ```javascript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const commands = await invoke('get_commands');
/// console.log('All commands:', commands);
/// ```
#[tauri::command]
fn get_commands(app_handle: tauri::AppHandle) -> Result<Vec<Command>, String> {
    let path = get_store_path(&app_handle)?;
    store::get_commands(&path)
}

/// Adds a new command to storage.
///
/// This Tauri command creates a new command entry and updates the global shortcuts.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle for accessing app data directories
/// * `command` - The command object to add (must have a unique ID)
///
/// # Returns
///
/// * `Ok(())` - Command was successfully added and shortcuts refreshed
/// * `Err(String)` - Error message if:
///   - Failed to access app data directory
///   - Failed to read existing commands
///   - Failed to save updated commands
///   - Failed to refresh global shortcuts
///
/// # Example
///
/// ```javascript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const newCommand = {
///   id: crypto.randomUUID(),
///   name: 'List Files',
///   script: 'ls -la',
///   description: 'List all files',
///   shortcut: 'Ctrl+L'
/// };
///
/// await invoke('add_command', { command: newCommand });
/// ```
///
/// # Note
///
/// This function does not check for duplicate IDs. Ensure the ID is unique before calling.
#[tauri::command]
fn add_command(app_handle: tauri::AppHandle, command: Command) -> Result<(), String> {
    let path = get_store_path(&app_handle)?;
    let mut commands = store::get_commands(&path)?;
    commands.push(command);
    store::save_commands(&path, &commands)?;
    refresh_shortcuts(&app_handle)
}

/// Updates an existing command.
///
/// This Tauri command finds a command by its ID and updates it with new values,
/// then refreshes global shortcuts.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle for accessing app data directories
/// * `command` - The updated command object (ID must match an existing command)
///
/// # Returns
///
/// * `Ok(())` - Command was successfully updated and shortcuts refreshed
/// * `Err(String)` - Error message if:
///   - Command with the given ID not found
///   - Failed to access app data directory
///   - Failed to read existing commands
///   - Failed to save updated commands
///   - Failed to refresh global shortcuts
///
/// # Example
///
/// ```javascript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const updatedCommand = {
///   id: '123e4567-e89b-12d3-a456-426614174000',
///   name: 'List Files (Updated)',
///   script: 'ls -lah',
///   description: 'List all files with human-readable sizes',
///   shortcut: 'Cmd+L'
/// };
///
/// await invoke('update_command', { command: updatedCommand });
/// ```
#[tauri::command]
fn update_command(app_handle: tauri::AppHandle, command: Command) -> Result<(), String> {
    let path = get_store_path(&app_handle)?;
    let mut commands = store::get_commands(&path)?;
    if let Some(index) = commands.iter().position(|c| c.id == command.id) {
        commands[index] = command;
        store::save_commands(&path, &commands)?;
        refresh_shortcuts(&app_handle)
    } else {
        Err("Command not found".to_string())
    }
}

/// Deletes a command by its ID.
///
/// This Tauri command removes a command from storage and updates global shortcuts.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle for accessing app data directories
/// * `id` - The unique identifier of the command to delete
///
/// # Returns
///
/// * `Ok(())` - Command was successfully deleted and shortcuts refreshed
/// * `Err(String)` - Error message if:
///   - Failed to access app data directory
///   - Failed to read existing commands
///   - Failed to save updated commands
///   - Failed to refresh global shortcuts
///
/// # Example
///
/// ```javascript
/// import { invoke } from '@tauri-apps/api/core';
///
/// await invoke('delete_command', {
///   id: '123e4567-e89b-12d3-a456-426614174000'
/// });
/// ```
///
/// # Note
///
/// If the command ID doesn't exist, this function still succeeds (idempotent operation).
#[tauri::command]
fn delete_command(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_store_path(&app_handle)?;
    let mut commands = store::get_commands(&path)?;
    commands.retain(|c| c.id != id);
    store::save_commands(&path, &commands)?;
    refresh_shortcuts(&app_handle)
}

/// Retrieves the current application configuration.
///
/// This Tauri command fetches the app configuration, including safe mode status.
/// If no config file exists, returns default configuration (safe_mode: false).
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle for accessing app data directories
///
/// # Returns
///
/// * `Ok(Config)` - The current configuration object
/// * `Err(String)` - Error message if:
///   - Failed to access app data directory
///   - Failed to parse configuration file
///
/// # Example
///
/// ```javascript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const config = await invoke('get_config');
/// console.log('Safe mode:', config.safe_mode);
/// ```
#[tauri::command]
fn get_config(app_handle: tauri::AppHandle) -> Result<Config, String> {
    let path = get_config_path(&app_handle)?;
    store::get_config(&path)
}

/// Updates the application configuration.
///
/// This Tauri command saves the provided configuration to persistent storage.
/// Changes take effect immediately for subsequent command executions.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle for accessing app data directories
/// * `config` - The new configuration object to save
///
/// # Returns
///
/// * `Ok(())` - Configuration was successfully saved
/// * `Err(String)` - Error message if:
///   - Failed to access app data directory
///   - Failed to write configuration file
///   - Failed to serialize configuration to JSON
///
/// # Example
///
/// ```javascript
/// import { invoke } from '@tauri-apps/api/core';
///
/// await invoke('update_config', {
///   config: { safe_mode: true }
/// });
/// ```
///
/// # Note
///
/// When safe mode is enabled, all command executions (manual and shortcuts) will be blocked.
#[tauri::command]
fn update_config(app_handle: tauri::AppHandle, config: Config) -> Result<(), String> {
    let path = get_config_path(&app_handle)?;
    store::save_config(&path, &config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            #[cfg(desktop)]
            {
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(|app_handle, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                let shortcut_str = shortcut.to_string();
                                if let Ok(path) = get_store_path(app_handle) {
                                    if let Ok(commands) = store::get_commands(&path) {
                                        if let Some(command) = commands.iter().find(|c| {
                                            c.shortcut.as_deref() == Some(shortcut_str.as_str())
                                        }) {
                                            if let Err(e) =
                                                run_command_script(&app_handle, &command.script)
                                            {
                                                log::error!(
                                                    "Failed to execute shortcut command: {}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                refresh_shortcuts(app.handle())?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_commands,
            add_command,
            update_command,
            delete_command,
            execute_command,
            get_config,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
