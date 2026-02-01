# CLI Command Manager - Implementation Status & Plan (Next.js Edition)

> **Context:** This plan replaces the outdated `task.md` and `implementation.md`. It reflects the actual Next.js + Tauri architecture.

## Technology Stack
- **Backend:** Rust (Tauri)
- **Frontend:** Next.js (App Router) + React + TailwindCSS
- **Storage:** JSON file (local app data)

## Implementation Status

### ✅ Completed
1.  **Project Initialization**
    - Tauri initialized with Next.js (`src/app`).
    - TailwindCSS configured.
    - Rust backend configured (`tauri.conf.json`).

2.  **Backend (Rust)**
    - `Command` struct defined (`src-tauri/src/models.rs`).
    - Store logic implemented (`src-tauri/src/store.rs`).
    - Tauri commands exposed (`get_commands`, `add_command`, `update_command`, `delete_command`, `execute_command`).
    - Unit tests passing for models and store.

3.  **Frontend (UI)**
    - `src/app/page.tsx` setup.
    - `src/components/CommandList.tsx` implemented (Lists commands, uses `use client`).
    - `src/types/index.ts` defined.

---

## Remaining Tasks

### Task 6: UI for Adding/Editing Commands
**Goal:** Allow users to create and modify commands via a form modal.

*   **Create `src/components/CommandForm.tsx`**
    *   **Directives:** Must use `"use client"`.
    *   **Props:** `initialData?` (Command), `onSubmit` (function), `onCancel` (function).
    *   **Fields:** Name, Script (textarea), Description, Shortcut.
    *   **Validation:** Basic required fields.

*   **Update `src/components/CommandList.tsx`**
    *   Add "Add Command" button.
    *   Add "Edit" and "Delete" buttons to items.
    *   Manage modal state (`isOpen`, `editingCommand`).
    *   Connect to `invoke("add_command")` and `invoke("update_command")`.
    *   Implement `handleDelete` with `invoke("delete_command")`.

### Task 8 & 9: Execution Logic & UI
**Goal:** Run commands from the UI and see output.

*   **Backend:** `execute_command` is already implemented (Task 4).
*   **Update `src/components/CommandList.tsx`**
    *   Add "Run" button.
    *   Call `invoke("execute_command", { id })`.
    *   Display stdout/stderr output (Toast or collapsible section).

### Task 10-12: Global Shortcuts
**Goal:** Trigger commands via keyboard even when app is unfocused.

*   **Backend (`src-tauri/src/lib.rs`)**
    *   Integrate `tauri-plugin-global-shortcut`.
    *   Implement logic to register/unregister shortcuts based on the Command store.
    *   Handle shortcut events to trigger the associated script.

### Task 13-14: Polish & Verification
**Goal:** Production-ready quality.

*   **Style:** Dark mode verification, glassmorphism on modals.
*   **Build:** Verify `npm run tauri build` works (Next.js static export).
*   **OS:** Verify on macOS (permissions, paths).
