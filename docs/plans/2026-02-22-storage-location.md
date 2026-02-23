# Storage Location Feature

## Goal
Allow users to customize where their commands are stored, specifically enabling storage in iCloud Drive for synchronization across devices.

## Implementation Details

### Backend (Rust)
1.  **Config Update:** Added `commands_path: Option<String>` to `Config` struct in `src-tauri/src/models.rs`.
2.  **Storage Logic:** Updated `get_store_path` in `src-tauri/src/lib.rs` to:
    - Check `config.commands_path` first.
    - Expand `~` to the user's home directory.
    - Fallback to default application data directory if not set.
3.  **Default Config:** Updated `store.rs` to initialize `commands_path` as `None`.

### Frontend (React)
1.  **Type Definition:** Updated `src/types/index.ts` to include `commands_path`.
2.  **UI Component:** Created `src/components/StorageSettings.tsx` to manage storage settings.
    - Provides a text input for custom paths.
    - Includes a "Use iCloud Drive" shortcut button that sets the path to `~/Library/Mobile Documents/com~apple~CloudDocs/climgr/commands.json`.
    - Includes a "Reset to Default" button.
3.  **Integration:** Added a settings toggle button to `src/components/CommandList.tsx` to display the storage settings panel.

## Usage
1.  Click the "Settings" (gear) icon in the main view.
2.  Click "Use iCloud Drive" to set the storage path to iCloud.
3.  Click "Save & Reload".
4.  Commands will now be loaded from and saved to the new location.

## Notes
- The application must have permission to access the specified directory.
- On macOS, `~/Library/Mobile Documents/com~apple~CloudDocs/` corresponds to the root of iCloud Drive.
