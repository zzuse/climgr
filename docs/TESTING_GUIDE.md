# Frontend Testing Guide

## Current Status

❌ **No tests currently exist** in the frontend.

## Adding Tests (Recommended)

### 1. Install Testing Dependencies

```bash
npm install -D vitest @testing-library/react @testing-library/jest-dom jsdom
```

### 2. Create Test Configuration

Create `vitest.config.ts`:

```typescript
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
```

### 3. Create Setup File

Create `src/test/setup.ts`:

```typescript
import '@testing-library/jest-dom';

// Mock Tauri API
global.window = Object.create(window);
Object.defineProperty(window, '__TAURI_INTERNALS__', {
  value: undefined,
  writable: true,
});
```

### 4. Add Test Script to package.json

```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  }
}
```

### 5. Write Example Tests

#### Example: `src/types/__tests__/index.test.ts`

```typescript
import { describe, it, expect } from 'vitest';
import type { Command, Config } from '../index';

describe('Type definitions', () => {
  it('should create valid Command object', () => {
    const command: Command = {
      id: '123',
      name: 'Test',
      script: 'echo test',
    };
    
    expect(command.id).toBe('123');
    expect(command.shortcut).toBeUndefined();
  });

  it('should create valid Config object', () => {
    const config: Config = {
      safe_mode: true,
    };
    
    expect(config.safe_mode).toBe(true);
  });
});
```

#### Example: `src/components/__tests__/SafeModeToggle.test.tsx`

```typescript
import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import SafeModeToggle from '../SafeModeToggle';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue({ safe_mode: false }),
}));

describe('SafeModeToggle', () => {
  it('should render toggle button', async () => {
    render(<SafeModeToggle />);
    
    // Component loads config async, so wait for it
    await screen.findByRole('switch');
    
    expect(screen.getByRole('switch')).toBeInTheDocument();
  });
});
```

### 6. Run Tests

```bash
# Run all tests
npm test

# Run with UI
npm run test:ui

# Run with coverage report
npm run test:coverage

# Run tests in watch mode
npm test -- --watch
```

## Integration Tests (Backend + Frontend)

For full integration tests, you'd need to:

1. Mock the Tauri bridge
2. Test Tauri commands
3. Test the full app workflow

This is more complex and typically done via end-to-end testing with tools like:
- Playwright
- Cypress

## Recommendation

Start with unit tests for:
1. **Components** - UI rendering and interactions
2. **Types** - Type validation
3. **Utilities** - Helper functions (if any)

The backend (Rust) already has good test coverage with `cargo test`.
