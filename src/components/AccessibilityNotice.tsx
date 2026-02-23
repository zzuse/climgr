"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Config } from "../types";

export default function AccessibilityNotice() {
  const [isMac, setIsMac] = useState(false);
  const [isVisible, setIsVisible] = useState(false);
  const [config, setConfig] = useState<Config | null>(null);

  useEffect(() => {
    const init = async () => {
      try {
        // 1. Check if it's macOS
        const isMacResult = await invoke<boolean>("is_macos");
        setIsMac(isMacResult);
        
        if (!isMacResult) return;

        // 2. Fetch config to check if dismissed
        const currentConfig = await invoke<Config>("get_config");
        setConfig(currentConfig);
        
        // Show only if not dismissed
        if (!currentConfig.accessibility_notice_dismissed) {
          setIsVisible(true);
        }
      } catch (err) {
        console.error("Failed to initialize AccessibilityNotice:", err);
      }
    };
    init();
  }, []);

  const handleOpenSettings = async () => {
    try {
      await invoke("open_accessibility_settings");
    } catch (err) {
      console.error("Failed to open settings:", err);
      alert("Failed to open Accessibility settings. Please open System Settings manually.");
    }
  };

  const handleDismiss = async () => {
    setIsVisible(false);
    if (config) {
      try {
        const newConfig = { ...config, accessibility_notice_dismissed: true };
        await invoke("update_config", { config: newConfig });
        setConfig(newConfig);
      } catch (err) {
        console.error("Failed to save dismissal state:", err);
      }
    }
  };

  if (!isMac || !isVisible) return null;

  return (
    <div className="mb-6 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div className="flex gap-3">
        <div className="mt-0.5 text-blue-600 dark:text-blue-400">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={2} stroke="currentColor" className="w-5 h-5">
            <path strokeLinecap="round" strokeLinejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
          </svg>
        </div>
        <div>
          <h4 className="text-sm font-semibold text-blue-900 dark:text-blue-100">Accessibility Permissions Required</h4>
          <p className="text-sm text-blue-800 dark:text-blue-200 mt-1">
            To execute system commands via shortcuts, please add <strong>climgr</strong> to 
            <span className="font-medium"> Privacy & Security → Accessibility</span>.
          </p>
        </div>
      </div>
      <div className="flex items-center gap-2">
        <button
          onClick={handleOpenSettings}
          className="whitespace-nowrap px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white text-xs font-medium rounded transition-colors"
        >
          Open Settings
        </button>
        <button
          onClick={handleDismiss}
          className="p-1.5 text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200 transition-colors"
          title="Dismiss"
        >
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={2} stroke="currentColor" className="w-4 h-4">
            <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>
  );
}
