"use client";

import { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Config } from '../types';

export default function StorageSettings() {
    const [config, setConfig] = useState<Config | null>(null);
    const [path, setPath] = useState('');
    const [loading, setLoading] = useState(true);
    const [message, setMessage] = useState<{ type: 'success' | 'error', text: string } | null>(null);

    const loadConfig = useCallback(async () => {
        try {
            const result = await invoke<Config>('get_config');
            setConfig(result);
            setPath(result.commands_path || '');
            setMessage(null);
        } catch (err) {
            console.error('Failed to load config:', err);
            setMessage({ type: 'error', text: 'Failed to load settings' });
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        loadConfig();
    }, [loadConfig]);

const ICLOUD_PATH = '~/Library/Mobile Documents/com~apple~CloudDocs/climgr/commands.json';

    const handleSave = async () => {
        if (!config) return;

        // If path is empty, set to undefined to use default
        const newPath = path.trim() === '' ? undefined : path.trim();
        const newConfig: Config = { ...config, commands_path: newPath };

        try {
            setLoading(true);
            await invoke('update_config', { config: newConfig });
            
            // Ensure directory exists for the new path
            try {
                const resolvedPath = await invoke<string>('ensure_storage_directory');
                console.log('Storage directory ensured at:', resolvedPath);
            } catch (dirErr) {
                console.error('Failed to create storage directory:', dirErr);
                // Don't fail the whole operation, but warn
                setMessage({ type: 'error', text: 'Settings saved, but failed to create directory: ' + dirErr });
                return;
            }

            setConfig(newConfig);
            setMessage({ type: 'success', text: 'Storage path updated & directory created!' });
            
            // Reload to apply changes
            window.location.reload(); 
        } catch (err) {
            console.error('Failed to update config:', err);
            setMessage({ type: 'error', text: 'Failed to update storage path' });
        } finally {
            setLoading(false);
        }
    };

    const useICloud = () => {
        setPath(ICLOUD_PATH);
    };

    const useDefault = () => {
        setPath('');
    };

    const isICloud = path === ICLOUD_PATH;

    const [isMac, setIsMac] = useState(false);
    useEffect(() => {
        const checkPlatform = async () => {
            try {
                const result = await invoke<boolean>("is_macos");
                setIsMac(result);
            } catch (err) {
                console.error("Failed to check platform:", err);
            }
        };
        checkPlatform();
    }, []);

    const handleOpenAccessibility = async () => {
        try {
            await invoke("open_accessibility_settings");
        } catch (err) {
            console.error("Failed to open accessibility settings:", err);
        }
    };

    const handleResetNotice = async () => {
        if (!config) return;
        try {
            setLoading(true);
            const newConfig = { ...config, accessibility_notice_dismissed: false };
            await invoke('update_config', { config: newConfig });
            setConfig(newConfig);
            setMessage({ type: 'success', text: 'Accessibility notice has been reset.' });
        } catch (err) {
            console.error('Failed to reset notice:', err);
            setMessage({ type: 'error', text: 'Failed to reset notice' });
        } finally {
            setLoading(false);
        }
    };

    if (loading && !config) return <div className="text-sm text-zinc-500">Loading settings...</div>;

    return (
        <div className="p-4 bg-zinc-50 dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 space-y-6">
            <div className="space-y-4">
                <h3 className="font-semibold text-lg dark:text-white">Storage Settings</h3>
                
                <div className="space-y-2">
                    <label className="block text-sm font-medium text-zinc-700 dark:text-zinc-300">
                        Commands File Path
                    </label>
                    <div className="flex gap-2">
                        <input
                            type="text"
                            value={path}
                            onChange={(e) => setPath(e.target.value)}
                            placeholder="Default (App Data)"
                            readOnly={isICloud}
                            className={`flex-1 rounded-md border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-900 px-3 py-2 text-sm focus:border-blue-500 focus:ring-blue-500 dark:text-white ${
                                isICloud ? 'opacity-75 cursor-not-allowed bg-zinc-100 dark:bg-zinc-800' : ''
                            }`}
                        />
                    </div>
                    {isICloud && (
                        <p className="text-xs text-blue-600 dark:text-blue-400 font-medium">
                            ✓ Using iCloud Drive (climgr/commands.json)
                        </p>
                    )}
                    <p className="text-xs text-zinc-500 dark:text-zinc-400">
                        {isICloud 
                            ? "Commands will be synced via iCloud Drive. This folder is accessible in Finder."
                            : "Leave empty to use default application storage. Use `~` for home directory."}
                    </p>
                </div>

                <div className="flex flex-wrap gap-2">
                    <button
                        onClick={useICloud}
                        disabled={isICloud}
                        className={`px-3 py-1.5 text-xs font-medium rounded-md transition-colors ${
                            isICloud 
                            ? 'bg-blue-600 text-white' 
                            : 'text-blue-700 bg-blue-100 hover:bg-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:hover:bg-blue-900/50'
                        }`}
                    >
                        Use iCloud Drive
                    </button>
                    <button
                        onClick={useDefault}
                        className="px-3 py-1.5 text-xs font-medium text-zinc-700 bg-zinc-100 rounded-md hover:bg-zinc-200 dark:bg-zinc-700 dark:text-zinc-300 dark:hover:bg-zinc-600 transition-colors"
                    >
                        Reset to Default
                    </button>
                </div>

                <div className="pt-2 flex justify-end">
                    <button
                        onClick={handleSave}
                        disabled={loading}
                        className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 transition-colors"
                    >
                        {loading ? 'Saving...' : 'Save & Reload'}
                    </button>
                </div>
            </div>

            {isMac && (
                <div className="pt-6 border-t border-zinc-200 dark:border-zinc-700 space-y-3">
                    <h3 className="font-semibold text-lg dark:text-white">System Permissions</h3>
                    <p className="text-sm text-zinc-600 dark:text-zinc-400">
                        To execute system commands via global shortcuts on macOS, you must add <strong>climgr</strong> to the Accessibility list in System Settings.
                    </p>
                    <div className="flex flex-wrap gap-2">
                        <button
                            onClick={handleOpenAccessibility}
                            className="px-3 py-1.5 text-xs font-medium text-blue-700 bg-blue-100 rounded-md hover:bg-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:hover:bg-blue-900/50 transition-colors"
                        >
                            Open Accessibility Settings
                        </button>
                        {config?.accessibility_notice_dismissed && (
                            <button
                                onClick={handleResetNotice}
                                className="px-3 py-1.5 text-xs font-medium text-zinc-700 bg-zinc-100 rounded-md hover:bg-zinc-200 dark:bg-zinc-700 dark:text-zinc-300 dark:hover:bg-zinc-600 transition-colors"
                            >
                                Restore Notice
                            </button>
                        )}
                    </div>
                </div>
            )}

            {message && (
                <div className={`text-sm p-2 rounded ${
                    message.type === 'success' ? 'bg-green-50 text-green-700 dark:bg-green-900/20 dark:text-green-400' : 'bg-red-50 text-red-700 dark:bg-red-900/20 dark:text-red-400'
                }`}>
                    {message.text}
                </div>
            )}
        </div>
    );
}
