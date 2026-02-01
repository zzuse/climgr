export interface Command {
  id: string;
  name: string;
  script: string;
  shortcut?: string | null;
  description?: string | null;
}
