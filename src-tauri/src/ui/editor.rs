/*!
 * Editor UI Commands
 * 
 * Tauri command handlers for editor operations.
 */

use tauri::State;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::core::editor::{CursorPosition, Selection};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorState {
    pub open_files: Vec<EditorTabInfo>,
    pub active_tab: Option<String>,
    pub settings: EditorSettingsInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorTabInfo {
    pub id: String,
    pub path: String,
    pub content: String,
    pub language: String,
    pub is_dirty: bool,
    pub cursor_position: CursorPositionInfo,
    pub selection: Option<SelectionInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CursorPositionInfo {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectionInfo {
    pub start: CursorPositionInfo,
    pub end: CursorPositionInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorSettingsInfo {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub completions: Vec<CompletionItemInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionItemInfo {
    pub label: String,
    pub kind: String,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: String,
}

#[tauri::command]
pub async fn get_editor_content(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<String, String> {
    let editor = state.editor.read().await;
    
    if let Some(tab) = editor.open_files.values().find(|tab| tab.path.to_string_lossy() == file_path) {
        Ok(tab.content.clone())
    } else {
        Err("File not open in editor".to_string())
    }
}

#[tauri::command]
pub async fn set_editor_content(
    state: State<'_, AppState>,
    file_path: String,
    content: String,
) -> Result<(), String> {
    let mut editor = state.editor.write().await;
    
    if let Some((_, tab)) = editor.open_files.iter_mut().find(|(_, tab)| tab.path.to_string_lossy() == file_path) {
        tab.content = content;
        tab.is_dirty = true;
        Ok(())
    } else {
        Err("File not open in editor".to_string())
    }
}

#[tauri::command]
pub async fn get_completions(
    state: State<'_, AppState>,
    request: CompletionRequest,
) -> Result<CompletionResponse, String> {
    let editor = state.editor.read().await;
    let file_path = PathBuf::from(&request.file_path);
    
    if let Some(tab) = editor.open_files.values().find(|tab| tab.path == file_path) {
        let position = CursorPosition {
            line: request.line,
            column: request.column,
        };
        
        let completions = editor.get_completions(&tab.id, position)
            .map_err(|e| format!("Failed to get completions: {}", e))?;
        
        let completion_infos: Vec<CompletionItemInfo> = completions.into_iter().map(|comp| CompletionItemInfo {
            label: comp.label,
            kind: format!("{:?}", comp.kind),
            detail: comp.detail,
            documentation: comp.documentation,
            insert_text: comp.insert_text,
        }).collect();
        
        Ok(CompletionResponse {
            completions: completion_infos,
        })
    } else {
        Err("File not open in editor".to_string())
    }
}

#[tauri::command]
pub async fn get_editor_state(
    state: State<'_, AppState>,
) -> Result<EditorState, String> {
    let editor = state.editor.read().await;
    
    let open_files: Vec<EditorTabInfo> = editor.open_files.values().map(|tab| EditorTabInfo {
        id: tab.id.clone(),
        path: tab.path.to_string_lossy().to_string(),
        content: tab.content.clone(),
        language: tab.language.clone(),
        is_dirty: tab.is_dirty,
        cursor_position: CursorPositionInfo {
            line: tab.cursor_position.line,
            column: tab.cursor_position.column,
        },
        selection: tab.selection.as_ref().map(|sel| SelectionInfo {
            start: CursorPositionInfo {
                line: sel.start.line,
                column: sel.start.column,
            },
            end: CursorPositionInfo {
                line: sel.end.line,
                column: sel.end.column,
            },
        }),
    }).collect();
    
    let settings = EditorSettingsInfo {
        font_family: editor.editor_settings.font_family.clone(),
        font_size: editor.editor_settings.font_size,
        tab_size: editor.editor_settings.tab_size,
        insert_spaces: editor.editor_settings.insert_spaces,
        word_wrap: editor.editor_settings.word_wrap,
        line_numbers: editor.editor_settings.line_numbers,
        minimap: editor.editor_settings.minimap,
        bracket_pair_colorization: editor.editor_settings.bracket_pair_colorization,
        format_on_save: editor.editor_settings.format_on_save,
        auto_closing_brackets: editor.editor_settings.auto_closing_brackets,
        auto_closing_quotes: editor.editor_settings.auto_closing_quotes,
    };
    
    Ok(EditorState {
        open_files,
        active_tab: editor.active_tab.clone(),
        settings,
    })
}

#[tauri::command]
pub async fn set_cursor_position(
    state: State<'_, AppState>,
    file_path: String,
    line: u32,
    column: u32,
) -> Result<(), String> {
    let mut editor = state.editor.write().await;
    let file_path = PathBuf::from(&file_path);
    
    if let Some((_, tab)) = editor.open_files.iter().find(|(_, tab)| tab.path == file_path) {
        let position = CursorPosition { line, column };
        let tab_id = tab.id.clone();
        editor.set_cursor_position(&tab_id, position)
            .map_err(|e| format!("Failed to set cursor position: {}", e))?;
        Ok(())
    } else {
        Err("File not open in editor".to_string())
    }
}

#[tauri::command]
pub async fn set_selection(
    state: State<'_, AppState>,
    file_path: String,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
) -> Result<(), String> {
    let mut editor = state.editor.write().await;
    let file_path = PathBuf::from(&file_path);
    
    if let Some((_, tab)) = editor.open_files.iter().find(|(_, tab)| tab.path == file_path) {
        let selection = Selection {
            start: CursorPosition {
                line: start_line,
                column: start_column,
            },
            end: CursorPosition {
                line: end_line,
                column: end_column,
            },
        };
        let tab_id = tab.id.clone();
        editor.set_selection(&tab_id, Some(selection))
            .map_err(|e| format!("Failed to set selection: {}", e))?;
        Ok(())
    } else {
        Err("File not open in editor".to_string())
    }
}

#[tauri::command]
pub async fn clear_selection(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<(), String> {
    let mut editor = state.editor.write().await;
    let file_path = PathBuf::from(&file_path);
    
    if let Some((tab_id, _)) = editor.open_files.iter().find(|(_, tab)| tab.path == file_path) {
        let tab_id = tab_id.clone();
        editor.set_selection(&tab_id, None)
            .map_err(|e| format!("Failed to clear selection: {}", e))?;
        Ok(())
    } else {
        Err("File not open in editor".to_string())
    }
}

#[tauri::command]
pub async fn set_active_tab(
    state: State<'_, AppState>,
    tab_id: String,
) -> Result<(), String> {
    let mut editor = state.editor.write().await;
    editor.set_active_tab(&tab_id)
        .map_err(|e| format!("Failed to set active tab: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn close_tab(
    state: State<'_, AppState>,
    tab_id: String,
) -> Result<(), String> {
    let mut editor = state.editor.write().await;
    editor.close_file(&tab_id)
        .map_err(|e| format!("Failed to close tab: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn update_editor_settings(
    state: State<'_, AppState>,
    settings: EditorSettingsInfo,
) -> Result<(), String> {
    let mut editor = state.editor.write().await;
    
    editor.editor_settings.font_family = settings.font_family;
    editor.editor_settings.font_size = settings.font_size;
    editor.editor_settings.tab_size = settings.tab_size;
    editor.editor_settings.insert_spaces = settings.insert_spaces;
    editor.editor_settings.word_wrap = settings.word_wrap;
    editor.editor_settings.line_numbers = settings.line_numbers;
    editor.editor_settings.minimap = settings.minimap;
    editor.editor_settings.bracket_pair_colorization = settings.bracket_pair_colorization;
    editor.editor_settings.format_on_save = settings.format_on_save;
    editor.editor_settings.auto_closing_brackets = settings.auto_closing_brackets;
    editor.editor_settings.auto_closing_quotes = settings.auto_closing_quotes;
    
    Ok(())
}