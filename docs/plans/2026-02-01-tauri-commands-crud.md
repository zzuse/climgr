# Tauri Commands (CRUD) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement Tauri commands for CRUD operations on `Command` objects and register them in the Tauri application.

**Architecture:** Create `tauri::command` handlers in `src-tauri/src/lib.rs`. Use a helper function to resolve the storage path using `AppHandle`. These handlers will wrap the existing `store` logic.

**Tech Stack:** Rust, Tauri v2.

---

### Task 1: Add Dependencies and Helper in lib.rs

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Import necessary modules and define `get_store_path`**

```rust
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
```

**Step 2: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: add get_store_path helper and imports"
```

### Task 2: Implement `get_commands` and `add_command`

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Implement the command handlers**

```rust
#[tauri::command]
fn get_commands(app_handle: AppHandle) -> Result<Vec<Command>, String> {
    let path = get_store_path(&app_handle);
    load_store(&path)
}

#[tauri::command]
fn add_command(app_handle: AppHandle, command: Command) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = load_store(&path)?;
    commands.push(command);
    persist_store(&path, &commands)
}
```

**Step 2: Register commands in `run()`**

```rust
.invoke_handler(tauri::generate_handler![get_commands, add_command])
```

**Step 3: Verify with `cargo check`**

Run: `cd src-tauri && cargo check`
Expected: Success

**Step 4: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: implement get_commands and add_command Tauri commands"
```

### Task 3: Implement `update_command` and `delete_command`

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Implement the remaining command handlers**

```rust
#[tauri::command]
fn update_command(app_handle: AppHandle, command: Command) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = load_store(&path)?;
    if let Some(index) = commands.iter().position(|c| c.id == command.id) {
        commands[index] = command;
        persist_store(&path, &commands)
    } else {
        Err(format!("Command with id {} not found", command.id))
    }
}

#[tauri::command]
fn delete_command(app_handle: AppHandle, id: String) -> Result<(), String> {
    let path = get_store_path(&app_handle);
    let mut commands = load_store(&path)?;
    let initial_len = commands.len();
    commands.retain(|c| c.id != id);
    if commands.len() < initial_len {
        persist_store(&path, &commands)
    } else {
        Err(format!("Command with id {} not found", id))
    }
}
```

**Step 2: Register new commands in `run()`**

```rust
.invoke_handler(tauri::generate_handler![
    get_commands,
    add_command,
    update_command,
    delete_command
])
```

**Step 3: Verify with `cargo check`**

Run: `cd src-tauri && cargo check`
Expected: Success

**Step 4: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: implement update_command and delete_command Tauri commands"
```

### Task 4: Final Verification

**Files:**
- Run commands

**Step 1: Run `cargo check` for the whole project**

Run: `cd src-tauri && cargo check`
Expected: Success

**Step 2: Final Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: complete Task 4 Tauri Commands (CRUD)"
```
