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
