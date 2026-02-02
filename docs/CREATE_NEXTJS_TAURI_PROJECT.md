# Creating a Next.js + Tauri Desktop Application

This guide documents the complete process of creating a desktop application using Next.js (React) for the frontend and Tauri (Rust) for the backend.

---

## Prerequisites

Before starting, ensure you have:

1. **Node.js** (v18+)
   ```bash
   node --version  # Should be 18.x or higher
   ```

2. **Rust** (latest stable)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   cargo --version
   rustc --version
   ```

3. **OS-Specific Dependencies**
   - **macOS**: Xcode Command Line Tools
     ```bash
     xcode-select --install
     ```
   - **Linux**: webkit2gtk and build essentials
     ```bash
     # Ubuntu/Debian
     sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libayatana-appindicator3-dev librsvg2-dev
     ```
   - **Windows**: Microsoft Visual Studio C++ Build Tools

---

## Step 1: Create Next.js App

```bash
# Create a new Next.js app with TypeScript
npx create-next-app@latest my-tauri-app --typescript --tailwind --eslint --app --src-dir --import-alias "@/*"

# Navigate to project
cd my-tauri-app
```

### Options Explained:
- `--typescript`: Use TypeScript
- `--tailwind`: Include Tailwind CSS
- `--eslint`: Include ESLint
- `--app`: Use the App Router (recommended)
- `--src-dir`: Put source files in `src/` directory
- `--import-alias "@/*"`: Enable absolute imports from `@`

---

## Step 2: Configure Next.js for Tauri

Tauri requires a **static export** from Next.js. Update your `next.config.ts`:

```typescript
// next.config.ts
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: 'export',  // Required for Tauri
  images: {
    unoptimized: true,  // Required for static export
  },
};

export default nextConfig;
```

### Why Static Export?
- Tauri serves files from the filesystem, not a Node.js server
- Static export generates plain HTML/CSS/JS files
- No server-side rendering (use client-side only)

---

## Step 3: Initialize Tauri

```bash
# Install Tauri CLI
npm install -D @tauri-apps/cli

# need add tauri into package.json
# Initialize Tauri in your project
npm run tauri init
```

### During `tauri init`, answer the prompts:
- **App name**: `my-tauri-app`
- **Window title**: `My Tauri App`
- **Frontend dev URL**: `http://localhost:3000`
- **Frontend build command**: `npm run build`
- **Frontend dev command**: `npm run dev`
- **Frontend dist directory**: `out` (Next.js static export directory)

This creates a `src-tauri/` directory with:
```
src-tauri/
├── Cargo.toml          # Rust dependencies
├── tauri.conf.json     # Tauri configuration
├── src/
│   ├── main.rs         # Entry point
│   └── lib.rs          # Main logic (Tauri commands)
└── capabilities/
    └── default.json    # Security permissions
```

---

## Step 4: Install Tauri API

```bash
# Install Tauri JavaScript API
npm install @tauri-apps/api
```

This allows your frontend to communicate with Rust:
```typescript
import { invoke } from '@tauri-apps/api/core';

// Call a Rust function
const result = await invoke('my_rust_command', { arg1: 'value' });
```

---

## Step 5: Configure Tauri

Edit `src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "My Tauri App",
  "version": "0.1.0",
  "identifier": "com.example.mytauriapp",
  "build": {
    "frontendDist": "../out",
    "devUrl": "http://localhost:3000",
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev"
  },
  "app": {
    "windows": [
      {
        "title": "My Tauri App",
        "width": 1024,
        "height": 768,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

---

## Step 6: Create Tauri Commands (Rust Backend)

Edit `src-tauri/src/lib.rs`:

```rust
// Import necessary modules
use tauri::Manager;

// Define a Tauri command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Tauri.", name)
}

// Another command example
#[tauri::command]
fn get_data() -> Vec<String> {
    vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()]
}

// Main Tauri run function
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Any setup logic here
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Edit `src-tauri/src/main.rs`:

```rust
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
}
```

---

## Step 7: Use Tauri Commands in React

Create a component that calls Rust functions:

```tsx
// src/app/page.tsx
"use client";

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function Home() {
  const [greeting, setGreeting] = useState('');
  const [items, setItems] = useState<string[]>([]);
  const [name, setName] = useState('');

  // Load data on mount
  useEffect(() => {
    async function loadData() {
      try {
        const data = await invoke<string[]>('get_data');
        setItems(data);
      } catch (error) {
        console.error('Error loading data:', error);
      }
    }
    loadData();
  }, []);

  // Call greet command
  const handleGreet = async () => {
    const result = await invoke<string>('greet', { name });
    setGreeting(result);
  };

  return (
    <main className="p-8">
      <h1 className="text-2xl font-bold mb-4">My Tauri App</h1>
      
      <div className="mb-4">
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Enter your name"
          className="border p-2 rounded mr-2"
        />
        <button 
          onClick={handleGreet}
          className="bg-blue-500 text-white px-4 py-2 rounded"
        >
          Greet
        </button>
      </div>
      
      {greeting && <p className="text-green-600 mb-4">{greeting}</p>}
      
      <h2 className="text-xl font-bold mb-2">Items from Rust:</h2>
      <ul className="list-disc pl-6">
        {items.map((item, index) => (
          <li key={index}>{item}</li>
        ))}
      </ul>
    </main>
  );
}
```

**Important**: Add `"use client"` directive for components that use Tauri API.

---

## Step 8: Add Scripts to package.json

```json
{
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "eslint",
    "tauri": "tauri"
  }
}
```

---

## Step 9: Run in Development Mode

```bash
# Start the app (runs both Next.js and Tauri)
npm run tauri dev
```

This will:
1. Start the Next.js dev server (hot reload)
2. Compile the Rust backend
3. Open a native window pointing to the frontend

---

## Step 10: Build for Production

```bash
# Create production build
npm run tauri build
```

Output locations:
- **macOS**: `src-tauri/target/release/bundle/macos/` and `.dmg`
- **Windows**: `src-tauri/target/release/bundle/msi/` and `.exe`
- **Linux**: `src-tauri/target/release/bundle/deb/` and `.AppImage`

---

## Project Structure

```
my-tauri-app/
├── package.json              # Node dependencies
├── next.config.ts            # Next.js config (output: 'export')
├── tsconfig.json             # TypeScript config
├── tailwind.config.ts        # Tailwind config
├── src/
│   ├── app/                  # Next.js App Router
│   │   ├── layout.tsx        # Root layout
│   │   ├── page.tsx          # Home page
│   │   └── globals.css       # Global styles
│   ├── components/           # React components
│   └── types/                # TypeScript types
└── src-tauri/
    ├── Cargo.toml            # Rust dependencies
    ├── tauri.conf.json       # Tauri configuration
    ├── capabilities/
    │   └── default.json      # Security permissions
    └── src/
        ├── main.rs           # Entry point
        └── lib.rs            # Tauri commands
```

---

## Adding Tauri Plugins

Tauri v2 uses plugins for extended functionality:

### Global Shortcuts
```bash
# Add to Cargo.toml
npm run tauri add global-shortcut
```

```rust
// In lib.rs
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

.plugin(tauri_plugin_global_shortcut::Builder::new().build())
```

### File System Access
```bash
npm run tauri add fs
```

### Logging
```bash
npm run tauri add log
```

---

## Security: Capabilities

Tauri v2 uses a capability-based security model. Edit `src-tauri/capabilities/default.json`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for desktop",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-close",
    "core:window:allow-minimize",
    "core:window:allow-maximize",
    "global-shortcut:default"
  ]
}
```

---

## Common Patterns

### Data Models (Rust)
```rust
// src-tauri/src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub value: Option<String>,
}
```

### Data Persistence (JSON File)
```rust
// src-tauri/src/store.rs
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn load_data<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}

pub fn save_data<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), String> {
    let file = File::create(path).map_err(|e| e.to_string())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data).map_err(|e| e.to_string())
}
```

### TypeScript Types (Mirror Rust)
```typescript
// src/types/index.ts
export interface Item {
  id: string;
  name: string;
  value?: string;
}
```

---

## Troubleshooting

### "Cannot find module '@tauri-apps/api'"
```bash
npm install @tauri-apps/api
```

### "output: 'export' not working"
Ensure Next.js Image component uses `unoptimized: true`:
```typescript
<Image src="/image.png" alt="" width={100} height={100} unoptimized />
```

### Rust compilation errors
```bash
cd src-tauri
cargo check    # Check for errors
cargo build    # Build debug
```

### Hot reload not working
- Frontend: Should auto-reload
- Backend: Tauri auto-rebuilds Rust on save

---

## Summary

### Key Concepts:
1. **Next.js** provides the UI (static export for Tauri)
2. **Tauri** provides the native wrapper and Rust backend
3. **invoke()** bridges JavaScript ↔ Rust communication
4. **Tauri commands** are Rust functions exposed to the frontend
5. **Capabilities** define security permissions

### Workflow:
1. Design UI in React/Next.js
2. Implement backend logic in Rust
3. Expose Rust functions via `#[tauri::command]`
4. Call from frontend using `invoke()`
5. Test with `npm run tauri dev`
6. Build with `npm run tauri build`

This architecture gives you:
- ✅ Modern React frontend with hot reload
- ✅ Rust backend for performance and security
- ✅ Native desktop app (not Electron)
- ✅ Small bundle size (~5-10MB vs 100MB+ for Electron)
- ✅ Cross-platform (macOS, Windows, Linux)
