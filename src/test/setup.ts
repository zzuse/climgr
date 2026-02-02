import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri API for testing
const mockInvoke = vi.fn();

vi.mock('@tauri-apps/api/core', () => ({
    invoke: mockInvoke,
}));

// Setup global window for Tauri detection
if (typeof window !== 'undefined') {
    Object.defineProperty(window, '__TAURI_INTERNALS__', {
        value: undefined,
        writable: true,
    });
}

// Export mock for tests to use
export { mockInvoke };
