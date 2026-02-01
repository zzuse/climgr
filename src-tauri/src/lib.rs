pub mod models;
pub mod store;

use crate::models::Command;
use crate::store::{get_commands as load_store, save_commands as persist_store};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn get_store_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("commands.json")
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
    store::save_commands(&path, &commands)
}

#[tauri::command]
fn update_command(app_handle: tauri::AppHandle, command: Command) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = store::get_commands(&path)?;
    if let Some(index) = commands.iter().position(|c| c.id == command.id) {
        commands[index] = command;
        store::save_commands(&path, &commands)?;
        Ok(())
    } else {
        Err("Command not found".to_string())
    }
}

#[tauri::command]
fn delete_command(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = store::get_commands(&path)?;
    commands.retain(|c| c.id != id);
    store::save_commands(&path, &commands)
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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_commands,
            add_command,
            update_command,
            delete_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
