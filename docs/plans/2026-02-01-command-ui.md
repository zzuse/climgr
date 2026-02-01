# Command UI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a UI for adding and editing commands in the CLI Manager.

**Architecture:** 
- `CommandForm` component for input, using React state and Tauri invokes.
- `CommandList` component updated to manage form visibility and list updates.
- Tailwind CSS for styling.

**Tech Stack:** React, Next.js, Tauri, Tailwind CSS.

---

### Task 1: Finalize CommandForm.tsx

**Files:**
- Modify: `src/components/CommandForm.tsx`

**Step 1: Add "use client" directive**
- Add `"use client";` to the top of `src/components/CommandForm.tsx`.

**Step 2: Review and Verify**
- Verify import statements and logic match requirements (already checked, looks good).
- Ensure `crypto.randomUUID()` is available (it is in modern browsers/environments).

### Task 2: Update CommandList.tsx

**Files:**
- Modify: `src/components/CommandList.tsx`

**Step 1: Add State for Form**
- Add `isFormOpen` (boolean) and `editingCommand` (Command | null) state variables.

**Step 2: Implement Handlers**
- `handleAdd`: Set `editingCommand` to null, `isFormOpen` to true.
- `handleEdit(cmd)`: Set `editingCommand` to cmd, `isFormOpen` to true.
- `handleDelete(id)`: Invoke `delete_command` with id, then refresh list.
- `handleFormSuccess`: Refresh list, set `isFormOpen` to false.
- `handleFormCancel`: Set `isFormOpen` to false.

**Step 3: Update UI**
- Add "Add Command" button in the header (next to "My Commands").
- Add "Edit" and "Delete" buttons to each command card (e.g., in a row at the bottom or top-right).
- Conditionally render `<CommandForm>` when `isFormOpen` is true.

**Step 4: Verify Integration**
- Ensure `invoke` calls are correct.

