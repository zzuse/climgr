"use client";

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Command } from '../types';

interface CommandFormProps {
  commandToEdit?: Command | null;
  onSuccess: () => void;
  onCancel: () => void;
}

export default function CommandForm({ commandToEdit, onSuccess, onCancel }: CommandFormProps) {
  const [name, setName] = useState(commandToEdit?.name || '');
  const [script, setScript] = useState(commandToEdit?.script || '');
  const [killScript, setKillScript] = useState(commandToEdit?.kill_script || '');
  const [description, setDescription] = useState(commandToEdit?.description || '');
  const [shortcut, setShortcut] = useState(commandToEdit?.shortcut || '');
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!name.trim()) {
      setError("Name is required");
      return;
    }
    if (!script.trim()) {
      setError("Script is required");
      return;
    }

    try {
      setIsSubmitting(true);
      if (commandToEdit) {
        const updatedCommand: Command = {
          ...commandToEdit,
          name,
          script,
          kill_script: killScript || undefined,
          description: description || undefined,
          shortcut: shortcut || undefined,
        };
        await invoke('update_command', { command: updatedCommand });
      } else {
        const newCommand: Command = {
            id: crypto.randomUUID(),
            name,
            script,
            kill_script: killScript || undefined,
            description: description || undefined,
            shortcut: shortcut || undefined,
        };
        await invoke('add_command', { command: newCommand });
      }
      onSuccess();
    } catch (err) {
      console.error("Failed to save command:", err);
      setError("Failed to save command. " + String(err));
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4 z-50">
      <div className="bg-white dark:bg-zinc-900 rounded-lg p-6 w-full max-w-md shadow-xl border dark:border-zinc-800">
        <h2 className="text-xl font-bold mb-4 dark:text-white">
          {commandToEdit ? 'Edit Command' : 'Add New Command'}
        </h2>
        
        {error && (
            <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-2 rounded mb-4 text-sm">
                {error}
            </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="command-name" className="block text-sm font-medium mb-1 dark:text-zinc-300">Name *</label>
            <input
              id="command-name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full p-2 border rounded dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
              placeholder="e.g., List Files"
              disabled={isSubmitting}
            />
          </div>

          <div>
            <label htmlFor="command-script" className="block text-sm font-medium mb-1 dark:text-zinc-300">Script *</label>
            <textarea
              id="command-script"
              value={script}
              onChange={(e) => setScript(e.target.value)}
              className="w-full p-2 border rounded dark:bg-zinc-800 dark:border-zinc-700 dark:text-white font-mono text-sm h-24"
              placeholder="e.g., ls -la"
              disabled={isSubmitting}
            />
          </div>

          <div>
            <label htmlFor="command-kill-script" className="block text-sm font-medium mb-1 dark:text-zinc-300">Kill Script</label>
            <input
              id="command-kill-script"
              type="text"
              value={killScript}
              onChange={(e) => setKillScript(e.target.value)}
              className="w-full p-2 border rounded dark:bg-zinc-800 dark:border-zinc-700 dark:text-white font-mono text-sm"
              placeholder="e.g., pkill -f my-process"
              disabled={isSubmitting}
            />
            <p className="text-[10px] text-zinc-500 mt-1">Optional: Custom command to stop the process. Falls back to PID kill if empty.</p>
          </div>

          <div>
            <label htmlFor="command-description" className="block text-sm font-medium mb-1 dark:text-zinc-300">Description</label>
            <input
              id="command-description"
              type="text"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full p-2 border rounded dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
              placeholder="Optional description"
              disabled={isSubmitting}
            />
          </div>

          <div>
            <label htmlFor="command-shortcut" className="block text-sm font-medium mb-1 dark:text-zinc-300">Shortcut</label>
            <input
              id="command-shortcut"
              type="text"
              value={shortcut}
              onChange={(e) => setShortcut(e.target.value)}
              className="w-full p-2 border rounded dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
              placeholder="e.g., Ctrl+L"
              disabled={isSubmitting}
            />
          </div>

          <div className="flex justify-end gap-2 mt-6">
            <button
              type="button"
              onClick={onCancel}
              className="px-4 py-2 text-sm text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white"
              disabled={isSubmitting}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              disabled={isSubmitting}
            >
              {isSubmitting ? 'Saving...' : (commandToEdit ? 'Update' : 'Create')}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
