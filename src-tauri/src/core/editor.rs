/*!
 * Editor Engine Module
 * 
 * Manages code editing, syntax highlighting, and editor state.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorEngine {
    pub open_files: HashMap<String, EditorTab>,
    pub active_tab: Option<String>,
    pub editor_settings: EditorSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorTab {
    pub id: String,
    pub path: PathBuf,
    pub content: String,
    pub language: String,
    pub is_dirty: bool,
    pub cursor_position: CursorPosition,
    pub selection: Option<Selection>,
    pub scroll_position: ScrollPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selection {
    pub start: CursorPosition,
    pub end: CursorPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollPosition {
    pub scroll_top: f64,
    pub scroll_left: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: u32,
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub minimap: bool,
    pub bracket_pair_colorization: bool,
    pub format_on_save: bool,
    pub auto_closing_brackets: bool,
    pub auto_closing_quotes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

impl Default for EditorEngine {
    fn default() -> Self {
        Self {
            open_files: HashMap::new(),
            active_tab: None,
            editor_settings: EditorSettings::default(),
        }
    }
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_family: "JetBrains Mono".to_string(),
            font_size: 14,
            tab_size: 4,
            insert_spaces: true,
            word_wrap: false,
            line_numbers: true,
            minimap: true,
            bracket_pair_colorization: true,
            format_on_save: true,
            auto_closing_brackets: true,
            auto_closing_quotes: true,
        }
    }
}

impl EditorEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_file(&mut self, path: PathBuf, content: String) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let language = self.detect_language(&path);
        
        let tab = EditorTab {
            id: id.clone(),
            path,
            content,
            language,
            is_dirty: false,
            cursor_position: CursorPosition { line: 0, column: 0 },
            selection: None,
            scroll_position: ScrollPosition { scroll_top: 0.0, scroll_left: 0.0 },
        };

        self.open_files.insert(id.clone(), tab);
        self.active_tab = Some(id.clone());
        
        Ok(id)
    }

    pub fn close_file(&mut self, id: &str) -> Result<()> {
        self.open_files.remove(id);
        
        if self.active_tab.as_ref() == Some(&id.to_string()) {
            self.active_tab = self.open_files.keys().next().cloned();
        }
        
        Ok(())
    }

    pub fn update_content(&mut self, id: &str, content: String) -> Result<()> {
        if let Some(tab) = self.open_files.get_mut(id) {
            tab.content = content;
            tab.is_dirty = true;
        }
        Ok(())
    }

    pub fn get_content(&self, id: &str) -> Option<&String> {
        self.open_files.get(id).map(|tab| &tab.content)
    }

    pub fn set_cursor_position(&mut self, id: &str, position: CursorPosition) -> Result<()> {
        if let Some(tab) = self.open_files.get_mut(id) {
            tab.cursor_position = position;
        }
        Ok(())
    }

    pub fn set_selection(&mut self, id: &str, selection: Option<Selection>) -> Result<()> {
        if let Some(tab) = self.open_files.get_mut(id) {
            tab.selection = selection;
        }
        Ok(())
    }

    pub fn get_completions(&self, id: &str, _position: CursorPosition) -> Result<Vec<CompletionItem>> {
        // This would integrate with LSP or other completion providers
        // For now, return basic completions
        let mut completions = Vec::new();
        
        if let Some(tab) = self.open_files.get(id) {
            // Basic keyword completions based on language
            match tab.language.as_str() {
                "rust" => {
                    completions.extend(vec![
                        CompletionItem {
                            label: "fn".to_string(),
                            kind: CompletionKind::Keyword,
                            detail: Some("Function".to_string()),
                            documentation: Some("Define a function".to_string()),
                            insert_text: "fn $0() {\n    \n}".to_string(),
                        },
                        CompletionItem {
                            label: "let".to_string(),
                            kind: CompletionKind::Keyword,
                            detail: Some("Variable declaration".to_string()),
                            documentation: Some("Declare a variable".to_string()),
                            insert_text: "let $0 = ".to_string(),
                        },
                    ]);
                }
                "javascript" | "typescript" => {
                    completions.extend(vec![
                        CompletionItem {
                            label: "function".to_string(),
                            kind: CompletionKind::Keyword,
                            detail: Some("Function".to_string()),
                            documentation: Some("Define a function".to_string()),
                            insert_text: "function $0() {\n    \n}".to_string(),
                        },
                        CompletionItem {
                            label: "const".to_string(),
                            kind: CompletionKind::Keyword,
                            detail: Some("Constant declaration".to_string()),
                            documentation: Some("Declare a constant".to_string()),
                            insert_text: "const $0 = ".to_string(),
                        },
                    ]);
                }
                _ => {}
            }
        }
        
        Ok(completions)
    }

    fn detect_language(&self, path: &PathBuf) -> String {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension {
                "rs" => "rust".to_string(),
                "js" => "javascript".to_string(),
                "ts" => "typescript".to_string(),
                "tsx" => "typescript".to_string(),
                "jsx" => "javascript".to_string(),
                "py" => "python".to_string(),
                "html" => "html".to_string(),
                "css" => "css".to_string(),
                "json" => "json".to_string(),
                "toml" => "toml".to_string(),
                "yaml" | "yml" => "yaml".to_string(),
                "md" => "markdown".to_string(),
                "xml" => "xml".to_string(),
                "sql" => "sql".to_string(),
                "sh" => "shell".to_string(),
                "ps1" => "powershell".to_string(),
                _ => "plaintext".to_string(),
            }
        } else {
            "plaintext".to_string()
        }
    }

    pub fn get_open_files(&self) -> Vec<&EditorTab> {
        self.open_files.values().collect()
    }

    pub fn get_active_tab(&self) -> Option<&EditorTab> {
        self.active_tab.as_ref().and_then(|id| self.open_files.get(id))
    }

    pub fn set_active_tab(&mut self, id: &str) -> Result<()> {
        if self.open_files.contains_key(id) {
            self.active_tab = Some(id.to_string());
        }
        Ok(())
    }
}