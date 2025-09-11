export interface Project {
  id: string;
  name: string;
  path: string;
  project_type: 'rust' | 'javascript' | 'typescript' | 'python' | 'general';
  last_opened: string;
  settings?: ProjectSettings;
}

export interface ProjectSettings {
  auto_save: boolean;
  format_on_save: boolean;
  git_integration: boolean;
  lsp_enabled: boolean;
}

export interface FileNode {
  name: string;
  path: string;
  is_directory: boolean;
  children?: FileNode[];
}

export interface ProjectStructure {
  files: FileNode[];
}