# CLI Command Manager

A cross-platform desktop application built with **Tauri**, **Next.js**, and **Rust** to manage and execute your frequently used CLI commands with a modern UI and global shortcuts.

## Features

-   **Command Management**: Create, edit, and delete shell commands.
-   **Quick Execution**: Run commands directly from the UI and see the output in real-time.
-   **Process Management**: Kill long-running processes directly from the UI.
-   **Custom Kill Scripts**: Specify a custom command (e.g., `pkill`) to stop specific processes, with automatic PID-based fallback.
-   **Safe Mode Toggle**: Enable/disable command execution with visual indicators (🔒 Safe Mode / ⚡ Active Mode).
-   **Global Shortcuts**: Assign keyboard shortcuts to trigger commands even when the app is in the background.
-   **Modern UI**: Clean interface with Dark Mode support using Tailwind CSS.
-   **Secure**: Runs locally on your machine with Rust-backed execution.

## Prerequisites

Before getting started, ensure you have the following installed:

-   **Node.js** (v18 or newer)
-   **Rust** (latest stable) - [Install Rust](https://www.rust-lang.org/tools/install)
-   **Tauri Prerequisites** (OS-specific dependencies) - [Follow the Tauri Guide](https://v2.tauri.app/start/prerequisites/)

## Getting Started

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/zzuse/climgr.git
    cd climgr
    ```

2.  **Install dependencies**:
    ```bash
    npm install
    ```

3.  **Run in Development Mode**:
    This will start the Next.js frontend and the Tauri application window.
    ```bash
    npm run tauri dev
    ```

## Building for Production

To create a distributable application bundle (e.g., `.app`, `.dmg`, `.exe`, `.deb`):

1.  **Build the application**:
    ```bash
    npm run tauri build
    ```

2.  **Locate the bundle**:
    After the build completes, you can find the installers in:
    -   **macOS**: `src-tauri/target/release/bundle/macos/` or `dmg/`
    -   **Windows**: `src-tauri/target/release/bundle/msi/` or `nsis/`
    -   **Linux**: `src-tauri/target/release/bundle/deb/` or `appimage/`

## Usage

### Managing Commands
-   Click **"Add Command"** to create a new entry.
-   **Name**: A friendly name for the command (e.g., "List Files").
-   **Script**: The actual shell script to run (e.g., `ls -la`).
-   **Kill Script**: (Optional) Custom command to stop the process (e.g., `pkill -f server`). If empty, it uses PID termination.
-   **Shortcut**: (Optional) Global hotkey (e.g., `Cmd+Shift+L`).

### Running & Stopping Commands
-   Click the **"Run"** button on any command card.
-   The output (stdout/stderr) will appear in a collapsible section below the command.
-   For long-running processes, click the **"Kill Running..."** button to stop the execution.

### Safe Mode
-   Use the **Safe Mode toggle** in the header to control command execution:
    -   🔒 **Safe Mode ON** (Orange): All commands are disabled for safety
    -   ⚡ **Active Mode** (Green): Commands can be executed normally
-   This is useful when you want to prevent accidental execution of privileged commands.
-   The setting persists across app restarts.

### Shortcuts
-   Shortcuts registered in the app work globally. Note that if a shortcut is already used by the system or another app, it might conflict.

## Project Structure

-   `src/` - Next.js frontend code (React components, pages, styles).
-   `src-tauri/` - Rust backend code (Tauri configuration, commands, system integration).
-   `src-tauri/tauri.conf.json` - Tauri configuration file.

## Technologies

-   [Tauri v2](https://v2.tauri.app)
-   [Next.js](https://nextjs.org) (App Router)
-   [React](https://react.dev)
-   [Tailwind CSS](https://tailwindcss.com)
-   [Rust](https://www.rust-lang.org)

## License

MIT
