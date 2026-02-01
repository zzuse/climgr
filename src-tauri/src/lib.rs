pub mod models;

use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

fn run_command_script(script: &str) -> Result<String, String> {
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

#[tauri::command]
fn execute_command(app_handle: tauri::AppHandle, command_id: String) -> Result<String, String> {
    let path = get_store_path(&app_handle);
    let commands = store::get_commands(&path)?;

    let command = commands
        .iter()
        .find(|c| c.id == command_id)
        .ok_or_else(|| String::from("Command not found"))?;

    run_command_script(&command.script)
}

pub mod store;

use crate::models::Command;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn get_store_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("commands.json")
}

fn refresh_shortcuts(app_handle: &tauri::AppHandle) -> Result<(), String> {
    app_handle
        .global_shortcut()
        .unregister_all()
        .map_err(|e| e.to_string())?;

    let path = get_store_path(app_handle);
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

#[tauri::command]
fn get_commands(app_handle: tauri::AppHandle) -> Result<Vec<Command>, String> {
    let path = get_store_path(&app_handle);
    store::get_commands(&path)
}

#[tauri::command]
fn add_command(app_handle: tauri::AppHandle, command: Command) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = store::get_commands(&path)?;
    commands.push(command);
    store::save_commands(&path, &commands)?;
    refresh_shortcuts(&app_handle)
}

#[tauri::command]
fn update_command(app_handle: tauri::AppHandle, command: Command) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = store::get_commands(&path)?;
    if let Some(index) = commands.iter().position(|c| c.id == command.id) {
        commands[index] = command;
        store::save_commands(&path, &commands)?;
        refresh_shortcuts(&app_handle)
    } else {
        Err("Command not found".to_string())
    }
}

#[tauri::command]
fn delete_command(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = store::get_commands(&path)?;
    commands.retain(|c| c.id != id);
    store::save_commands(&path, &commands)?;
    refresh_shortcuts(&app_handle)
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
                                let path = get_store_path(app_handle);
                                if let Ok(commands) = store::get_commands(&path) {
                                    if let Some(command) = commands.iter().find(|c| {
                                        c.shortcut.as_deref() == Some(shortcut_str.as_str())
                                    }) {
                                        if let Err(e) = run_command_script(&command.script) {
                                            log::error!(
                                                "Failed to execute shortcut command: {}",
                                                e
                                            );
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
            execute_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
