"use client";

import { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Config } from '../types';

export default function SafeModeToggle() {
    const [config, setConfig] = useState<Config | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const loadConfig = useCallback(async () => {
        try {
            const result = await invoke<Config>('get_config');
            setConfig(result);
            setError(null);
        } catch (err) {
            console.error('Failed to load config:', err);
            setError('Failed to load settings');
            // Fallback for browser preview
            if (typeof window !== 'undefined' && !window.__TAURI_INTERNALS__) {
                setConfig({ safe_mode: false });
                setError(null);
            }
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        loadConfig();
    }, [loadConfig]);

    const toggleSafeMode = useCallback(async () => {
        if (!config) return;

        const newConfig: Config = { safe_mode: !config.safe_mode };

        try {
            setLoading(true);
            await invoke('update_config', { config: newConfig });
            setConfig(newConfig);
            setError(null);
        } catch (err) {
            console.error('Failed to update config:', err);
            setError('Failed to update settings');
            // Fallback for browser preview
            if (typeof window !== 'undefined' && !window.__TAURI_INTERNALS__) {
                setConfig(newConfig);
                setError(null);
            }
        } finally {
            setLoading(false);
        }
    }, [config]);

    if (loading && !config) {
        return null;
    }

    return (
        <div className="flex items-center gap-3">
            <div className="flex items-center gap-2">
                <button
                    onClick={toggleSafeMode}
                    disabled={loading}
                    className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 ${config?.safe_mode ? 'bg-orange-500' : 'bg-green-600'
                        }`}
                    role="switch"
                    aria-checked={config?.safe_mode}
                    aria-label={config?.safe_mode ? 'Disable safe mode' : 'Enable safe mode'}
                >
                    <span
                        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${config?.safe_mode ? 'translate-x-6' : 'translate-x-1'
                            }`}
                    />
                </button>
                <div className="flex flex-col">
                    <span className="text-sm font-medium dark:text-white">
                        {config?.safe_mode ? '🔒 Safe Mode' : '⚡ Active Mode'}
                    </span>
                    <span className="text-xs text-zinc-500 dark:text-zinc-400">
                        {loading ? 'Updating...' : (config?.safe_mode ? 'Commands disabled' : 'Commands enabled')}
                    </span>
                </div>
            </div>
            {error && (
                <span className="text-xs text-red-500">{error}</span>
            )}
        </div>
    );
}
