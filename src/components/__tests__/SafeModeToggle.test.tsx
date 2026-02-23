import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import SafeModeToggle from '../SafeModeToggle';
import { invoke } from '@tauri-apps/api/core';

// Mock the Tauri invoke function
vi.mock('@tauri-apps/api/core');

describe('SafeModeToggle Component', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    it('should render the toggle button', async () => {
        vi.mocked(invoke).mockResolvedValue({ safe_mode: false, accessibility_notice_dismissed: false });

        render(<SafeModeToggle />);

        await waitFor(() => {
            expect(screen.getByRole('switch')).toBeInTheDocument();
        });
    });

    it('should display "Active Mode" when safe mode is off', async () => {
        vi.mocked(invoke).mockResolvedValue({ safe_mode: false, accessibility_notice_dismissed: false });

        render(<SafeModeToggle />);

        await waitFor(() => {
            expect(screen.getByText('⚡ Active Mode')).toBeInTheDocument();
            expect(screen.getByText('Commands enabled')).toBeInTheDocument();
        });
    });

    it('should display "Safe Mode" when safe mode is on', async () => {
        vi.mocked(invoke).mockResolvedValue({ safe_mode: true, accessibility_notice_dismissed: false });

        render(<SafeModeToggle />);

        await waitFor(() => {
            expect(screen.getByText('🔒 Safe Mode')).toBeInTheDocument();
            expect(screen.getByText('Commands disabled')).toBeInTheDocument();
        });
    });

    it('should have correct aria-label for accessibility', async () => {
        vi.mocked(invoke).mockResolvedValue({ safe_mode: false, accessibility_notice_dismissed: false });

        render(<SafeModeToggle />);

        await waitFor(() => {
            const button = screen.getByRole('switch');
            expect(button).toHaveAttribute('aria-label', 'Enable safe mode');
        });
    });

    it('should toggle safe mode when clicked', async () => {
        const user = userEvent.setup();

        // Initial state: safe mode off
        vi.mocked(invoke).mockResolvedValueOnce({ safe_mode: false, accessibility_notice_dismissed: false });
        // After toggle: safe mode on
        vi.mocked(invoke).mockResolvedValueOnce(undefined);

        render(<SafeModeToggle />);

        // Wait for initial load
        await waitFor(() => {
            expect(screen.getByText('⚡ Active Mode')).toBeInTheDocument();
        });

        // Click the toggle
        const toggleButton = screen.getByRole('switch');
        await user.click(toggleButton);

        // Verify update_config was called
        expect(invoke).toHaveBeenCalledWith('update_config', {
            config: { safe_mode: true, accessibility_notice_dismissed: false },
        });
    });

    it('should show loading state during toggle', async () => {
        const user = userEvent.setup();

        vi.mocked(invoke).mockResolvedValueOnce({ safe_mode: false, accessibility_notice_dismissed: false });

        // Make update slow to see loading state
        let resolveUpdate: () => void;
        const updatePromise = new Promise<void>((resolve) => {
            resolveUpdate = resolve;
        });
        vi.mocked(invoke).mockReturnValueOnce(updatePromise as unknown as Promise<unknown>);

        render(<SafeModeToggle />);

        await waitFor(() => {
            expect(screen.getByText('Commands enabled')).toBeInTheDocument();
        });

        const toggleButton = screen.getByRole('switch');
        await user.click(toggleButton);

        // Should show "Updating..." while loading
        expect(screen.getByText('Updating...')).toBeInTheDocument();

        // Resolve the update
        resolveUpdate!();

        await waitFor(() => {
            expect(screen.queryByText('Updating...')).not.toBeInTheDocument();
        });
    });

    it('should call get_config on mount', async () => {
        vi.mocked(invoke).mockResolvedValue({ safe_mode: false, accessibility_notice_dismissed: false });

        render(<SafeModeToggle />);

        await waitFor(() => {
            expect(invoke).toHaveBeenCalledWith('get_config');
        });
    });

    it('should handle error gracefully with browser fallback', async () => {
        const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => { });

        vi.mocked(invoke).mockRejectedValue(new Error('Failed to load'));

        render(<SafeModeToggle />);

        // In browser mode (no Tauri), component falls back to default config
        // and clears the error, so we should see the default safe_mode: false state
        await waitFor(() => {
            expect(screen.getByText('⚡ Active Mode')).toBeInTheDocument();
        });

        // Verify error was logged
        expect(consoleErrorSpy).toHaveBeenCalledWith(
            'Failed to load config:',
            expect.any(Error)
        );

        consoleErrorSpy.mockRestore();
    });

    it('should have correct CSS classes based on state', async () => {
        vi.mocked(invoke).mockResolvedValue({ safe_mode: true, accessibility_notice_dismissed: false });

        render(<SafeModeToggle />);

        await waitFor(() => {
            const button = screen.getByRole('switch');
            expect(button.className).toContain('bg-orange-500');
        });
    });
});
