# Safe Mode Implementation Walkthrough

## Overview
Added a configurable **safe mode** option that allows you to toggle command execution on/off. When enabled, all command executions (both manual and keyboard shortcuts) will be blocked with a clear error message.

## What Was Changed

### 1. Config Model ([models.rs:L12-16](file:///Users/z/Documents/Code/Self/climgr/src-tauri/src/models.rs#L12-L16))
Created a new `Config` struct to store app-wide settings:
```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub safe_mode: bool,
}
```

### 2. Config Storage ([store.rs:L28-51](file:///Users/z/Documents/Code/Self/climgr/src-tauri/src/store.rs#L28-L51))
Added functions to persist config to `config.json`:
- `get_config()` - Returns default config (`safe_mode: false`) if file doesn't exist
- `save_config()` - Saves config with pretty JSON formatting

### 3. Safe Mode Check ([lib.rs:L8-23](file:///Users/z/Documents/Code/Self/climgr/src-tauri/src/lib.rs#L8-L23))
Modified `run_command_script()` to check safe mode before executing:
```rust
fn run_command_script(app_handle: &AppHandle, script: &str) -> Result<String, String> {
    let config_path = get_config_path(app_handle);
    let config = store::get_config(&config_path)?;
    
    if config.safe_mode {
        return Err("Command execution disabled in safe mode...".to_string());
    }
    // ... execute command
}
```

### 4. New Tauri Commands ([lib.rs:L111-123](file:///Users/z/Documents/Code/Self/climgr/src-tauri/src/lib.rs#L111-L123))
Exposed two new commands to the frontend:
- `get_config` - Retrieves current config
- `update_config` - Updates and persists config

## How to Use

### From Frontend (TypeScript/JavaScript)
```typescript
import { invoke } from '@tauri-apps/api/core';

// Get current config
const config = await invoke('get_config');
console.log(config.safe_mode); // true or false

// Enable safe mode
await invoke('update_config', { 
  config: { safe_mode: true } 
});

// Disable safe mode (allow command execution)
await invoke('update_config', { 
  config: { safe_mode: false } 
});
```

### Behavior
- **Safe Mode OFF (default)**: All commands execute normally with full privileges
- **Safe Mode ON**: All command executions fail with error message:
  ```
  "Command execution disabled in safe mode. Disable safe mode in settings to execute commands."
  ```

## File Locations
- **Commands**: `~/.local/share/app/commands.json` (or platform equivalent)
- **Config**: `~/.local/share/app/config.json` (or platform equivalent)

## Benefits
- ✅ Prevents accidental command execution
- ✅ Quick toggle via frontend UI
- ✅ Preserves ability to run privileged commands when needed
- ✅ Applies to both manual execution and keyboard shortcuts
- ✅ Clean error messages guide users to disable safe mode

## Next Steps
You can now:
1. Add a toggle switch in your frontend settings UI ✅ **DONE**
2. Display safe mode status in the app ✅ **DONE** 
3. Show helpful error messages when commands fail due to safe mode ✅ **DONE**

---

## UI Implementation

### New Component: [SafeModeToggle.tsx](file:///Users/z/Documents/Code/Self/climgr/src/components/SafeModeToggle.tsx)

Created a beautiful toggle switch component with:
- **Visual toggle switch** - Orange when safe mode is ON 🔒, green when OFF ⚡
- **Status labels** - Clear indication of current mode
- **Real-time updates** - Immediately syncs with backend
- **Error handling** - Shows errors if config fails to update
- **Browser preview support** - Works in development mode

### UI Integration ([CommandList.tsx](file:///Users/z/Documents/Code/Self/climgr/src/components/CommandList.tsx#L123-L134))

Added the toggle to the header between the title and "Add Command" button:
```tsx
<div className="flex justify-between items-center mb-6 gap-4">
  <h2>My Commands</h2>
  <div className="flex items-center gap-4">
    <SafeModeToggle />
    <button>Add Command</button>
  </div>
</div>
```

### Type Definitions ([types/index.ts](file:///Users/z/Documents/Code/Self/climgr/src/types/index.ts))
```typescript
export interface Config {
  safe_mode: boolean;
}
```

### Visual Design
- 🔒 **Safe Mode ON**: Orange toggle + "Commands disabled" message
- ⚡ **Active Mode OFF**: Green toggle + "Commands enabled" message
- Smooth transitions and accessible focus states
- Responsive design that works on all screen sizes

## How It Works

1. **User toggles switch** → Frontend calls `update_config` 
2. **Backend saves** to `config.json`
3. **Command execution** checks safe mode before running
4. If safe mode is ON, commands fail with clear error message
5. Toggle updates to reflect new state

## Complete Feature Flow

```
User clicks toggle
    ↓
SafeModeToggle.toggleSafeMode()
    ↓
invoke('update_config', { safe_mode: true })
    ↓
Backend: store::save_config()
    ↓
User tries to run command
    ↓
Backend: run_command_script() checks safe mode
    ↓
If safe mode ON → Return error
If safe mode OFF → Execute command
```

