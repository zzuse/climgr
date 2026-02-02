export interface Command {
  id: string;
  name: string;
  script: string;
  shortcut?: string;
  description?: string;
}

export interface Config {
  safe_mode: boolean;
}
