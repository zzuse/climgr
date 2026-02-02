# CLI Manager - Developer Commands Cheat Sheet

## 🚀 Quick Start

```bash
# First time setup
npm install

# Start development (recommended)
npm run tauri dev
```

---

## 📦 Frontend Commands

### Development
```bash
# Browser preview (mock data, no Tauri)
npm run dev

# Full app with Tauri backend (recommended)
npm run tauri dev
```

### Code Quality
```bash
# Type checking
npx tsc --noEmit

# Linting
npm run lint

# Fix linting issues
npm run lint -- --fix
```

### Building
```bash
# Build for production (frontend only)
npm run build

# Test production build locally
npm run start
```

---

## 🦀 Backend Commands (Rust)

### Development
```bash
cd src-tauri

# Quick syntax check (fast)
cargo check

# Build debug version
cargo build

# Build optimized release
cargo build --release
```

### Testing
```bash
cd src-tauri

# Run all tests
TMPDIR=$(pwd)/.tmp cargo test

# Run tests with output
TMPDIR=$(pwd)/.tmp cargo test -- --nocapture

# Run specific test
TMPDIR=$(pwd)/.tmp cargo test test_save_and_load_commands
```

### Documentation
```bash
cd src-tauri

# Generate and open documentation
cargo doc --open
```

---

## 📱 Building Desktop App

### Development Build (Faster)
```bash
npm run tauri build -- --debug
```

### Production Build (Optimized)
```bash
npm run tauri build
```

**Output locations:**
- **macOS**: `src-tauri/target/release/bundle/macos/`
- **DMG**: `src-tauri/target/release/bundle/dmg/`

---

## 🧪 Testing

### Backend Tests (Rust)
```bash
cd src-tauri
mkdir -p .tmp
TMPDIR=$(pwd)/.tmp cargo test -- --nocapture
```
✅ **4 tests** - All passing

### Frontend Tests
❌ **Not implemented yet**

See `docs/TESTING_GUIDE.md` for setup instructions.

---

## 🔍 Debugging

### Check TypeScript Types
```bash
npx tsc --noEmit
```

### Check Rust Compilation
```bash
cd src-tauri
cargo check
```

### View Logs (Development)
- Frontend: Check browser console (F12)
- Backend: Logs appear in terminal running `npm run tauri dev`

---

## 📊 Project Structure

```
climgr/
├── src/                    # Frontend (Next.js/React)
│   ├── app/               # Next.js pages
│   ├── components/        # React components
│   └── types/            # TypeScript definitions
├── src-tauri/             # Backend (Rust/Tauri)
│   ├── src/              # Rust source
│   │   ├── lib.rs       # Main logic + Tauri commands
│   │   ├── models.rs    # Data structures
│   │   └── store.rs     # Persistence layer
│   └── Cargo.toml        # Rust dependencies
├── docs/                  # Documentation
└── package.json          # Node dependencies
```

---

## 🆘 Common Issues

### Permission Error During Tests
```bash
# Use project-local temp directory
cd src-tauri
mkdir -p .tmp
TMPDIR=$(pwd)/.tmp cargo test
```

### Node Module Permission Issues
If you see EPERM errors:
1. Check terminal has Full Disk Access
2. Try: `sudo chown -R $(whoami) node_modules`

### Hot Reload Not Working
- Restart `npm run tauri dev`
- Clear `.next` cache: `rm -rf .next`

---

## 📚 More Documentation

- `README.md` - User guide and features
- `docs/DEVELOPMENT_JOURNEY.md` - Implementation details
- `docs/plans/safe-mode-implementation.md` - Safe mode feature walkthrough
- `docs/CODE_REVIEW_FIXES.md` - Recent improvements
- `docs/TESTING_GUIDE.md` - Testing setup (TODO)

---

## 💡 Tips

- Use `npm run tauri dev` for day-to-day development
- Run `cargo check` frequently to catch Rust errors early
- Run `npx tsc --noEmit` to check TypeScript before committing
- Backend tests cover storage and serialization
- Safe mode protects against accidental command execution
