# CLI Command Manager - Implementation Plan

## User Review Required
> [!IMPORTANT]
> **Technology Stack Proposal**:
> - **Backend**: Rust (Tauri) - High performance, cross-platform (macOS ARM/Intel), secure system access.
> - **Frontend**: React + Vite + TailwindCSS - Modern, responsive UI, easy to implement dynamic features like glassmorphism.
> - **Storage**: JSON file (local app data) or SQLite. For simplicity and portability, JSON is recommended initially.
>
> Please confirm if this stack is acceptable.

## Proposed Changes

### Project Initialization
#### [NEW] [cli-command-manager]
- Create a new Tauri project using `npm create tauri-app`.
- Configure for macOS (Bundle ID, initial window settings).

### Backend (Rust / Tauri)
#### [NEW] [src-tauri/src/main.rs]
- Define `Command` struct: `id`, `name`, `script`, `shortcut`, `description`.
- Implement Tauri Commands:
    - `get_commands()`: Read from storage.
    - `add_command(cmd)`: Write to storage.
    - `update_command(cmd)`: Update storage.
    - `delete_command(id)`: Remove from storage.
    - `execute_command(id)`: Spawns a shell process to run the script.
- Register global shortcuts using `tauri-plugin-global-shortcut`.

#### [NEW] [src-tauri/src/store.rs]
- Implement file-based persistence (loading/saving `Vec<Command>` to `app_data_dir`).

### Frontend (React)
#### [NEW] [src/App.tsx]
- Main layout: Sidebar (optional) + Command List.
- "Add Command" Button -> Modal Form.

#### [NEW] [src/components/CommandList.tsx]
- Render list of commands with "Run", "Edit", "Delete" actions.
- Execute button triggers `execute_command` on backend.

#### [NEW] [src/components/CommandForm.tsx]
- Form for Name, Script, Shortcut.

## Verification Plan

### Automated Tests
- **Rust Unit Tests**:
    - Test command CRUD logic in isolation (mock storage).
    - `cargo test` in `src-tauri`.

### Manual Verification
1.  **Build & Run**:
    - Run `npm run tauri dev`.
    - Verify app launches on macOS.
2.  **CRUD Flow**:
    - **Create**: Add a new command `echo "Hello World"`. verify it appears in list.
    - **Read**: Restart app and verify command persists.
    - **Update**: Change script to `echo "Updated"`.
    - **Delete**: Remove the command.
3.  **Execution**:
    - Run the `echo "Hello World"` command. Verify output is captured/shown or system effect occurs (e.g. `open .`).
4.  **Shortcuts**:
    - Assign a global shortcut (e.g., `Cmd+Shift+K`).
    - Focus another app, press `Cmd+Shift+K`, verify command runs.
