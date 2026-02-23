pub mod models;
pub mod store;

use crate::models::{Command, Config};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

struct ProcessManager {
    processes: Mutex<HashMap<String, u32>>,
}

fn run_command_script(
    app_handle: &AppHandle,
    command_id: &str,
    script: &str,
) -> Result<String, String> {
    // Check safe mode
    let config_path = get_config_path(app_handle)?;
    let config = store::get_config(&config_path)?;

    if config.safe_mode {
        return Err("Command execution disabled in safe mode. Disable safe mode in settings to execute commands.".to_string());
    }

    log::info!("Executing script for command {}: {}", command_id, script);

    let child = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    let pid = child.id();

    {
        let state = app_handle.state::<ProcessManager>();
        state
            .processes
            .lock()
            .unwrap()
            .insert(command_id.to_string(), pid);
    }

    let wait_result = child.wait_with_output();

    {
        let state = app_handle.state::<ProcessManager>();
        state.processes.lock().unwrap().remove(command_id);
    }

    let output = wait_result.map_err(|e| format!("Failed to wait for command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(format!("{}{}", stdout, stderr))
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
    let command_id_clone = command_id.clone();

    tauri::async_runtime::spawn_blocking(move || {
        run_command_script(&app_handle_clone, &command_id_clone, &script)
    })
    .await
    .map_err(|e| format!("Failed to execute command task: {}", e))?
}

/// Kills a running command by its ID.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle
/// * `command_id` - The unique identifier of the command to kill
///
/// # Returns
///
/// * `Ok(())` - Command was successfully killed or was not running
/// * `Err(String)` - Error message if killing failed
#[tauri::command]
fn kill_command(app_handle: AppHandle, state: State<ProcessManager>, command_id: String) -> Result<(), String> {
    // 1. Try custom kill script if it exists
    let path = get_store_path(&app_handle)?;
    let commands = store::get_commands(&path)?;
    
    if let Some(command) = commands.iter().find(|c| c.id == command_id) {
        if let Some(kill_script) = &command.kill_script {
            if !kill_script.trim().is_empty() {
                log::info!("Executing custom kill script for command {}: {}", command_id, kill_script);
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(kill_script)
                    .output()
                    .map_err(|e| format!("Failed to execute kill script: {}", e))?;
                
                if !output.status.success() {
                    log::warn!("Kill script exited with error: {}", String::from_utf8_lossy(&output.stderr));
                }
                // We return Ok here because the script was executed. 
                // The process manager will clean up the PID if/when the main process dies.
                return Ok(());
            }
        }
    }

    // 2. Fallback to PID-based kill
    let pid = {
        let procs = state.processes.lock().unwrap();
        procs.get(&command_id).copied()
    };

    if let Some(pid) = pid {
        log::info!("Killing process {} for command {}", pid, command_id);

        #[cfg(unix)]
        {
            let output = std::process::Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .map_err(|e| format!("Failed to execute kill command: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Kill command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        #[cfg(windows)]
        {
            let output = std::process::Command::new("taskkill")
                .arg("/F")
                .arg("/PID")
                .arg(pid.to_string())
                .output()
                .map_err(|e| format!("Failed to execute taskkill command: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Taskkill command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        // The process removal from the map will happen in the run_command_script thread
        // when wait_with_output returns.
    }

    Ok(())
}

fn get_store_path(app: &AppHandle) -> Result<PathBuf, String> {
    // Check if a custom path is set in the config
    if let Ok(config_path) = get_config_path(app) {
        if let Ok(config) = store::get_config(&config_path) {
            if let Some(path_str) = config.commands_path {
                let expanded_path = store::expand_path(&path_str);
                return Ok(PathBuf::from(expanded_path));
            }
        }
    }

    // Fallback to default location
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

/// Ensures the storage directory exists.
#[tauri::command]
fn ensure_storage_directory(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = get_store_path(&app_handle)?;
    log::info!("Attempting to create storage directory for path: {:?}", path);
    println!("Debug: Attempting to create storage directory for path: {:?}", path);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            log::info!("Parent directory does not exist, creating: {:?}", parent);
            println!("Debug: Creating parent directory: {:?}", parent);
            
            // Try std::fs first
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::warn!("std::fs::create_dir_all failed: {}, attempting mkdir fallback...", e);
                println!("Debug: std::fs::create_dir_all failed: {}, attempting mkdir fallback...", e);
                
                // Fallback to mkdir -p command
                let output = std::process::Command::new("mkdir")
                    .arg("-p")
                    .arg(parent)
                    .output()
                    .map_err(|e| format!("Failed to execute mkdir command: {}", e))?;
                
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to create directory via mkdir: {}", stderr));
                }
            }
        } else {
            log::info!("Parent directory already exists: {:?}", parent);
            println!("Debug: Parent directory already exists: {:?}", parent);
        }
    }
    Ok(path.to_string_lossy().to_string())
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
                                            if let Err(e) = run_command_script(
                                                &app_handle,
                                                &command.id,
                                                &command.script,
                                            ) {
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
        .manage(ProcessManager {
            processes: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_commands,
            add_command,
            update_command,
            delete_command,
            execute_command,
            kill_command,
            get_config,
            update_config,
            ensure_storage_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
