# Development Journey & Setup Guide

This document describes how this project was generated and the tools used by Gemini 3.7 (OpenCode) to build it.

## 1. Project Generation Stack

The project was built using the following core technologies:

*   **Frontend Framework**: [Next.js](https://nextjs.org/) (v16.1.6)
    *   Using the **App Router** (`src/app`).
    *   Configured for **Static Export** (`output: 'export'` in `next.config.ts`) to work with Tauri.
*   **UI Library**: [React](https://react.dev/) (v19)
*   **Styling**: [Tailwind CSS](https://tailwindcss.com/) (v4)
*   **Desktop Framework**: [Tauri](https://v2.tauri.app/) (v2.0)
    *   **Backend**: Rust (v1.77+)
    *   **IPC**: Tauri Commands for communication between React and Rust.
*   **Language**: TypeScript (Frontend) & Rust (Backend).

## 2. How It Was Built (Step-by-Step)

Gemini 3.7 followed a structured "Subagent-Driven Development" process to build this app:

1.  **Initialization**:
    *   Initialized a standard Next.js app (`npx create-next-app`).
    *   Initialized Tauri (`npm run tauri init`) to wrap the web app in a native window.

2.  **Backend Implementation (Rust)**:
    *   Defined the `Command` data structure in Rust.
    *   Implemented file-based persistence (saving commands to `commands.json` in the app data directory).
    *   Created Tauri commands (`get_commands`, `add_command`, `execute_command`, etc.) to expose this logic to the frontend.
    *   Added **Global Shortcuts** capability using `tauri-plugin-global-shortcut`.

3.  **Frontend Implementation (React/Next.js)**:
    *   Created strictly typed interfaces (`src/types/index.ts`).
    *   Built the **Command List** UI to fetch and display commands.
    *   Built the **Command Form** (Modal) for creating/editing commands.
    *   Implemented the **Execution UI** to run commands and display real-time output.
    *   Ensured all components use the `"use client"` directive for Tauri compatibility.

4.  **Verification**:
    *   Ran Rust unit tests (`cargo test`) to verify backend logic.
    *   Ran `npm run lint` and `npm run build` to ensure the frontend compiles correctly for production.

## 3. Setting Up Your Development Environment

To continue development on this project, follow these steps:

### A. Prerequisites

1.  **Node.js**: Install version 18 or higher (LTS recommended).
    *   Verify: `node -v`
2.  **Rust**: Install via Rustup.
    *   Command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    *   Verify: `cargo --version`
3.  **OS Dependencies**:
    *   **macOS**: Needs Xcode Command Line Tools (`xcode-select --install`).
    *   **Linux**: Needs webkit2gtk and other libs (see [Tauri Linux Guide](https://v2.tauri.app/start/prerequisites/#linux)).
    *   **Windows**: Needs Microsoft Visual Studio C++ Build Tools.

### B. Setup Steps

1.  **Clone the repo**:
    ```bash
    git clone <repository-url>
    cd climgr
    ```

2.  **Install Frontend Dependencies**:
    ```bash
    npm install
    ```

3.  **Install Backend Dependencies**:
    Tauri will automatically handle Rust dependencies, but you can pre-fetch them:
    ```bash
    cd src-tauri
    cargo fetch
    cd ..
    ```

### C. Running the App

*   **Development Mode** (Hot-reloading for both Rust and React):
    ```bash
    npm run tauri dev
    ```

*   **Browser-Only Mode** (Mock data, no native features):
    ```bash
    npm run dev
    ```
    *Note: Since this app relies on Rust for logic, the browser version uses mock data for preview purposes.*

## 4. Key Configuration Files

*   `src-tauri/tauri.conf.json`: Main configuration for the desktop app (window size, permissions, bundle ID).
*   `src-tauri/capabilities/default.json`: Security capabilities (which Tauri commands the frontend is allowed to call).
*   `next.config.ts`: Next.js config (crucial setting: `output: 'export'`).
*   `src-tauri/Cargo.toml`: Rust dependencies.

## 5. Safe Mode Feature Implementation

### Overview
Added a configurable **Safe Mode** feature to prevent accidental command execution while maintaining the ability to execute privileged commands when needed.

### Implementation Details

#### Backend (Rust)

**New Files/Modules:**
- `Config` struct in `src-tauri/src/models.rs` for application settings
- Config storage functions in `src-tauri/src/store.rs`
- Safe mode check in `run_command_script()` function

**Key Changes:**

1. **Config Model** (`models.rs`):
   ```rust
   #[derive(Debug, Serialize, Deserialize, Clone)]
   pub struct Config {
       pub safe_mode: bool,
   }
   ```

2. **Config Storage** (`store.rs`):
   - `get_config(path: &Path) -> Result<Config, String>` - Loads config, returns default if missing
   - `save_config(path: &Path, config: &Config) -> Result<(), String>` - Persists config to JSON

3. **Safe Mode Check** (`lib.rs`):
   - Modified `run_command_script()` to check config before execution
   - Returns error message when safe mode is enabled
   - Applies to both manual execution and keyboard shortcuts

4. **New Tauri Commands**:
   - `get_config` - Retrieves current configuration
   - `update_config` - Updates and persists configuration

**Error Handling Improvements:**
- Changed `get_store_path()` and `get_config_path()` to return `Result` instead of panicking
- Added error propagation with `?` operator throughout
- Provides meaningful error messages instead of crashes

#### Frontend (TypeScript/React)

**New Files:**
- `src/components/SafeModeToggle.tsx` - Toggle switch component with visual indicators

**Key Changes:**

1. **Type Definitions** (`src/types/index.ts`):
   ```typescript
   export interface Config {
     safe_mode: boolean;
   }
   ```

2. **SafeModeToggle Component**:
   - Visual toggle switch (Orange = Safe Mode ON 🔒, Green = Active Mode ⚡)
   - Real-time sync with backend
   - Status labels showing current mode
   - Error handling for config failures
   - Browser preview support

3. **UI Integration** (`CommandList.tsx`):
   - Added toggle to header between title and "Add Command" button
   - Responsive layout with proper spacing

### Testing

**Compile Backend:**

```bash
# Navigate to the Rust backend directory
cd src-tauri

# Check for compilation errors (faster than full build)
cargo check

# Or build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

**Running Tests:**

Due to macOS permission issues with the default temp directory, use a project-local temp directory:

```bash
cd src-tauri

# Create local temp directory (first time only)
mkdir -p .tmp

# Run tests with custom temp directory
TMPDIR=$(pwd)/.tmp cargo test -- --nocapture

# Run tests with output visible (shows println! and test names)
cargo test -- --nocapture

# Run all tests
cargo test

# Run tests for a specific module
cargo test models
cargo test store

# Run a specific test by name
cargo test test_save_and_load_commands

```

**Test Coverage:**
- ✅ `test_config_default` - Config serialization
- ✅ `test_command_struct_serialization` - Command model serialization
- ✅ `test_save_and_load_config` - Config persistence
- ✅ `test_save_and_load_commands` - Command persistence

**Manual Testing:**
1. Launch app: `npm run tauri dev`
2. Toggle safe mode switch in header
3. Try executing a command with safe mode ON (should fail with error)
4. Disable safe mode and execute command (should work)
5. Verify setting persists after app restart

### File Locations

- **Commands**: `~/.local/share/app/commands.json` (or platform equivalent)
- **Config**: `~/.local/share/app/config.json` (or platform equivalent)

### User Features

1. **Safe Mode Toggle**: Visual switch in app header
2. **Status Indicators**: 
   - 🔒 Orange = Safe Mode (commands disabled)
   - ⚡ Green = Active Mode (commands enabled)
3. **Error Messages**: Clear feedback when commands are blocked
4. **Persistence**: Settings saved across app restarts

### Documentation

Full implementation details available in:
- `docs/plans/safe-mode-implementation.md` - Complete walkthrough with code examples
- `README.md` - User-facing feature documentation

## 6. Process Management Feature Implementation

### Overview
Added the ability to terminate long-running processes directly from the UI. This includes support for both standard PID-based termination and custom "Kill Scripts" (e.g., `pkill`).

### Implementation Details

#### Backend (Rust)

**New Components:**
- **ProcessManager**: A thread-safe registry (`Mutex<HashMap<String, u32>>`) that maps Command IDs to their active PIDs.
- **Kill Logic**: 
  - Cross-platform support: Uses `kill -9` on Unix-like systems and `taskkill /F /PID` on Windows.
  - Priority-based termination: Custom kill scripts take precedence over PID-based termination.

**Key Changes:**

1. **Model Update** (`models.rs`):
   Added `kill_script: Option<String>` to the `Command` struct.

2. **Process Registry** (`lib.rs`):
   ```rust
   struct ProcessManager {
       processes: Mutex<HashMap<String, u32>>,
   }
   ```

3. **Tauri Commands**:
   - `execute_command`: Now registers the spawned PID in the `ProcessManager`.
   - `kill_command`: Checks for a custom script first, then falls back to killing the registered PID.

#### Frontend (TypeScript/React)

**Key Changes:**

1. **UI Feedback**:
   - The "Run" button transforms into a "Kill Running..." button when a command is active.
   - Execution state now tracks `loading` status to toggle UI elements.

2. **Command Form**:
   - Added a "Kill Script" field to allow users to specify custom termination commands (e.g., `pkill -f "my-server"`).

3. **Safety Measures**:
   - "Edit", "Delete", and "Copy" buttons are disabled while a command is running to prevent race conditions or inconsistent states.

### Testing

**Backend Tests:**
- Updated `test_save_and_load_commands` to verify `kill_script` serialization.
- Verified compilation and state management with `cargo check`.

**Manual Testing:**
1. Create a long-running command (e.g., `sleep 60`).
2. Run the command and verify the button changes to "Kill Running...".
3. Click "Kill" and verify the process terminates.
4. Add a custom kill script (e.g., `pkill sleep`) and verify it takes precedence.


