# CLI Command Manager - Remaining Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the remaining features of the CLI Command Manager: UI for editing/adding commands, execution logic, shortcuts, and final polish.

**Architecture:**
- **Frontend:** React components (`CommandForm`, `CommandList`) using Tailwind CSS for UI.
- **Backend:** Existing Rust functions exposed via Tauri commands.
- **Shortcuts:** Tauri global shortcut plugin.

**Tech Stack:** React, TypeScript, Tailwind CSS, Rust, Tauri.

---

### Task 1: UI for Adding/Editing Commands

**Files:**
- Create: `src/components/CommandForm.tsx`
- Modify: `src/components/CommandList.tsx`
- Modify: `src/types/index.ts` (if needed)

**Step 1: Create CommandForm component**

Create `src/components/CommandForm.tsx`. This component should:
- Accept props: `initialData?` (Command), `onSubmit` (function), `onCancel` (function).
- Render a form with fields:
    - Name (text, required)
    - Script (textarea, required)
    - Description (text, optional)
    - Shortcut (text, optional)
- Handle form state.
- On submit, validate and call `onSubmit` with the command object.

**Step 2: Integrate CommandForm into CommandList**

Modify `src/components/CommandList.tsx`:
- Add state for `editingCommand` (Command | null) and `isCreating` (boolean).
- Add "Add Command" button to the header.
- Add "Edit" and "Delete" buttons to each command item.
- When "Add" is clicked, show `CommandForm` (empty).
- When "Edit" is clicked, show `CommandForm` with `editingCommand` data.
- Implement `handleSave` function:
    - If creating, generate ID (use `crypto.randomUUID()`) and call `invoke("add_command", { command })`.
    - If editing, call `invoke("update_command", { command })`.
    - Refresh list after success.
    - Close form.
- Implement `handleDelete` function:
    - Call `invoke("delete_command", { id })`.
    - Refresh list.

**Step 3: Verify**

- Run the app.
- Test adding a command.
- Test editing a command.
- Test deleting a command.

---

### Task 2: Execution Logic & UI

**Files:**
- Modify: `src-tauri/src/lib.rs` (Review execution logic)
- Modify: `src/components/CommandList.tsx`

**Step 1: Review Rust Execution Logic**
*Note: `execute_command` is already implemented in `lib.rs`, utilizing `sh -c`. This is sufficient for MVP.*

**Step 2: Add Execute Button to UI**

Modify `src/components/CommandList.tsx`:
- Add a "Run" button to each command.
- Implement `handleExecute(id: string)`:
    - Call `invoke("execute_command", { commandId: id })`.
    - Display the result (stdout/stderr) in a toast or a modal, or a simpler console log for now. Let's add a simple "Output" area below the command card if it was just run, or use a basic `alert` for success/failure to start, then upgrade to a non-blocking UI element.
    - *Plan:* Add a collapsible "Last Output" section to the command card that opens when execution finishes.

**Step 3: Verify**
- Create a command `echo "Hello"`.
- Click Run.
- Verify output is displayed.

---

### Task 3: Global Shortcuts

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/capabilities/default.json` (if needed for permissions)

**Step 1: Add dependencies**
*Note: `tauri-plugin-global-shortcut` might need to be added.*
- Check `src-tauri/Cargo.toml`. If missing, add `tauri-plugin-global-shortcut = "2"`.
- Run `cargo add tauri-plugin-global-shortcut` in `src-tauri`.

**Step 2: Register Shortcuts in Rust**
- Modify `src-tauri/src/lib.rs`:
    - Initialize the plugin in `setup`.
    - When `add_command` or `update_command` is called, register the shortcut if present.
    - *Complexity Warning:* Managing dynamic shortcuts in Tauri v2 can be tricky.
    - *Alternative:* simpler approach for MVP: Re-register all shortcuts on app launch and whenever commands change.
    - Let's implement a helper `register_all_shortcuts(app_handle)` that clears and re-registers everything based on the store. Call this in `setup` and after `add/update/delete`.

**Step 3: Handle Shortcut Events**
- In `setup`, configure the shortcut handler to lookup the command by shortcut string and execute it.

---

### Task 4: Polish & Verification

**Files:**
- Modify: `src/app/globals.css`
- Modify: `src/components/*`

**Step 1: Verify on macOS**
- Ensure paths and execution work (handled by `sh -c`).

**Step 2: Style Polish**
- Apply glassmorphism effects (backdrop-blur) to the CommandForm modal.
- Ensure Dark Mode looks consistent.

**Step 3: Final Build Test**
- Run `npm run tauri build` to ensure release build works.
