"use client";

import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Command } from "../types";
import CommandForm from "./CommandForm";
import SafeModeToggle from "./SafeModeToggle";

type ExecutionState = {
  loading: boolean;
  output: string | null;
  error: string | null;
};

export default function CommandList() {
  const [commands, setCommands] = useState<Command[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingCommand, setEditingCommand] = useState<Command | null>(null);
  const [executionStates, setExecutionStates] = useState<Record<string, ExecutionState>>({});

  const fetchCommands = useCallback(async () => {
    try {
      const result = await invoke<Command[]>("get_commands");
      setCommands(result);
      setError(null);
    } catch (err) {
      console.error("Failed to fetch commands:", err);
      setError("Failed to load commands. Are you running in Tauri?");

      // Fallback for development/browser preview
      if (typeof window !== 'undefined' && !window.__TAURI_INTERNALS__) {
        console.log("Using mock data for browser preview");
        setTimeout(() => {
          setCommands([
            { id: "1", name: "List Files", script: "ls -la", description: "List all files including hidden ones", shortcut: "Cmd+L" },
            { id: "2", name: "Check Node Version", script: "node -v", description: "Check current node version" }
          ]);
          setError(null);
        }, 0);
      }
    }
  }, []);

  useEffect(() => {
    fetchCommands();
  }, [fetchCommands]);

  const handleAdd = () => {
    setEditingCommand(null);
    setIsFormOpen(true);
  };

  const handleEdit = (cmd: Command) => {
    setEditingCommand(cmd);
    setIsFormOpen(true);
  };

  const handleDelete = async (id: string) => {
    if (!window.confirm("Are you sure you want to delete this command?")) return;
    try {
      await invoke("delete_command", { id });
      fetchCommands();
    } catch (err) {
      console.error("Failed to delete command:", err);
      // Mock behavior
      if (typeof window !== 'undefined' && !window.__TAURI_INTERNALS__) {
        setCommands(prev => prev.filter(c => c.id !== id));
      }
    }
  };

  const handleRun = async (cmd: Command) => {
    setExecutionStates(prev => ({
      ...prev,
      [cmd.id]: { loading: true, output: null, error: null }
    }));

    try {
      const result = await invoke<string>("execute_command", { commandId: cmd.id });
      setExecutionStates(prev => ({
        ...prev,
        [cmd.id]: { loading: false, output: result, error: null }
      }));
    } catch (err) {
      console.error("Failed to execute command:", err);

      // Mock behavior for browser
      if (typeof window !== 'undefined' && !window.__TAURI_INTERNALS__) {
        console.log("Mocking execution for", cmd.name);
        setTimeout(() => {
          setExecutionStates(prev => ({
            ...prev,
            [cmd.id]: {
              loading: false,
              output: `Mock output for: ${cmd.script}\n\nFiles found:\n- file1.txt\n- file2.js\n\n(Executed successfully)`,
              error: null
            }
          }));
        }, 1000);
        return;
      }

      setExecutionStates(prev => ({
        ...prev,
        [cmd.id]: { loading: false, output: null, error: String(err) }
      }));
    }
  };

  const handleFormSuccess = () => {
    setIsFormOpen(false);
    fetchCommands();
  };

  return (
    <div className="w-full max-w-2xl mx-auto p-4">
      <div className="flex justify-between items-center mb-6 gap-4">
        <h2 className="text-2xl font-bold dark:text-white">My Commands</h2>
        <div className="flex items-center gap-4">
          <SafeModeToggle />
          <button
            onClick={handleAdd}
            className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded text-sm font-medium transition-colors"
          >
            Add Command
          </button>
        </div>
      </div>

      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4" role="alert">
          <span className="block sm:inline">{error}</span>
        </div>
      )}
      <div className="space-y-4">
        {commands.map((cmd) => (
          <div
            key={cmd.id}
            className="p-4 border rounded-lg shadow-sm bg-white dark:bg-zinc-800 dark:border-zinc-700 hover:shadow-md transition-shadow"
          >
            <div className="flex justify-between items-start">
              <h3 className="font-semibold text-lg dark:text-zinc-100">{cmd.name}</h3>
              <div className="flex items-center gap-2">
                {cmd.shortcut && (
                  <span className="text-xs font-mono bg-zinc-100 dark:bg-zinc-700 px-2 py-1 rounded dark:text-zinc-300 border dark:border-zinc-600">
                    {cmd.shortcut}
                  </span>
                )}
              </div>
            </div>
            {cmd.description && (
              <p className="text-sm text-zinc-500 dark:text-zinc-400 mt-1">{cmd.description}</p>
            )}
            <div className="mt-3">
              <code className="block w-full text-xs bg-zinc-50 dark:bg-black p-3 rounded border dark:border-zinc-800 dark:text-zinc-300 font-mono overflow-x-auto">
                {cmd.script}
              </code>
            </div>

            <div className="mt-4 flex justify-end gap-2">
              <button
                onClick={() => handleRun(cmd)}
                disabled={executionStates[cmd.id]?.loading}
                className="px-3 py-1 text-sm bg-green-600 hover:bg-green-700 text-white rounded disabled:opacity-50 transition-colors flex items-center gap-1"
              >
                {executionStates[cmd.id]?.loading ? (
                  <>
                    <svg className="animate-spin -ml-1 mr-1 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Running...
                  </>
                ) : (
                  <>
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    Run
                  </>
                )}
              </button>
              <button
                onClick={() => handleEdit(cmd)}
                className="px-3 py-1 text-sm text-zinc-600 dark:text-zinc-400 hover:text-blue-600 dark:hover:text-blue-400 border border-zinc-200 dark:border-zinc-700 rounded hover:border-blue-300 dark:hover:border-blue-700"
              >
                Edit
              </button>
              <button
                onClick={() => handleDelete(cmd.id)}
                className="px-3 py-1 text-sm text-zinc-600 dark:text-zinc-400 hover:text-red-600 dark:hover:text-red-400 border border-zinc-200 dark:border-zinc-700 rounded hover:border-red-300 dark:hover:border-red-700"
              >
                Delete
              </button>
            </div>

            {executionStates[cmd.id] && (executionStates[cmd.id].output || executionStates[cmd.id].error) && (
              <div className="mt-3 border-t border-zinc-100 dark:border-zinc-700 pt-3">
                <div className="flex justify-between items-center mb-2">
                  <span className="text-xs font-semibold text-zinc-500 uppercase tracking-wider">Output</span>
                  <button
                    onClick={() => setExecutionStates(prev => {
                      const newState = { ...prev };
                      delete newState[cmd.id];
                      return newState;
                    })}
                    className="text-xs text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-300"
                  >
                    Dismiss
                  </button>
                </div>
                {executionStates[cmd.id].error ? (
                  <div className="text-sm text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400 p-2 rounded">
                    Error: {executionStates[cmd.id].error}
                  </div>
                ) : (
                  <pre className="text-xs bg-zinc-900 text-green-400 p-3 rounded font-mono overflow-x-auto whitespace-pre-wrap max-h-64 overflow-y-auto">
                    {executionStates[cmd.id].output}
                  </pre>
                )}
              </div>
            )}

          </div>
        ))}
        {commands.length === 0 && !error && (
          <div className="text-center py-12 border-2 border-dashed border-zinc-200 dark:border-zinc-700 rounded-lg">
            <p className="text-zinc-500 dark:text-zinc-400">No commands found.</p>
          </div>
        )}
      </div>

      {isFormOpen && (
        <CommandForm
          key={editingCommand ? editingCommand.id : 'new-command'}
          commandToEdit={editingCommand}
          onSuccess={handleFormSuccess}
          onCancel={() => setIsFormOpen(false)}
        />
      )}
    </div>
  );
}
