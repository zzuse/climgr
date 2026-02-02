import { describe, it, expect } from 'vitest';
import type { Command, Config } from '../index';

describe('Type Definitions', () => {
    describe('Command type', () => {
        it('should create a valid Command with all required fields', () => {
            const command: Command = {
                id: '123e4567-e89b-12d3-a456-426614174000',
                name: 'Test Command',
                script: 'echo "Hello World"',
            };

            expect(command.id).toBe('123e4567-e89b-12d3-a456-426614174000');
            expect(command.name).toBe('Test Command');
            expect(command.script).toBe('echo "Hello World"');
            expect(command.shortcut).toBeUndefined();
            expect(command.description).toBeUndefined();
        });

        it('should create a valid Command with optional fields', () => {
            const command: Command = {
                id: '123',
                name: 'Full Command',
                script: 'ls -la',
                shortcut: 'Ctrl+L',
                description: 'List all files',
            };

            expect(command.shortcut).toBe('Ctrl+L');
            expect(command.description).toBe('List all files');
        });

        it('should allow undefined optional fields', () => {
            const command: Command = {
                id: '456',
                name: 'Minimal Command',
                script: 'pwd',
                shortcut: undefined,
                description: undefined,
            };

            expect(command.shortcut).toBeUndefined();
            expect(command.description).toBeUndefined();
        });
    });

    describe('Config type', () => {
        it('should create a valid Config with safe_mode false', () => {
            const config: Config = {
                safe_mode: false,
            };

            expect(config.safe_mode).toBe(false);
        });

        it('should create a valid Config with safe_mode true', () => {
            const config: Config = {
                safe_mode: true,
            };

            expect(config.safe_mode).toBe(true);
        });
    });
});
