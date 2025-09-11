/*!
 * File UI Commands
 * 
 * Tauri command handlers for file operations.
 */

use tauri::State;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub content: String,
    pub language: String,
    pub size: u64,
    pub modified: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub children: Option<Vec<FileNode>>,
    pub expanded: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileOperation {
    pub success: bool,
    pub message: String,
    pub file_path: String,
}

#[tauri::command]
pub async fn open_file(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<FileContent, String> {
    let path = PathBuf::from(&file_path);
    
    if !path.exists() {
        return Err("File does not exist".to_string());
    }
    
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    let language = detect_language(&path);
    
    // Open file in editor
    let mut editor = state.editor.write().await;
    let _tab_id = editor.open_file(path.clone(), content.clone())
        .map_err(|e| format!("Failed to open file in editor: {}", e))?;
    
    Ok(FileContent {
        path: file_path,
        content,
        language,
        size: metadata.len(),
        modified: metadata.modified()
            .map_err(|e| format!("Failed to get modification time: {}", e))?
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    })
}

#[tauri::command]
pub async fn save_file(
    state: State<'_, AppState>,
    file_path: String,
    content: String,
) -> Result<FileOperation, String> {
    let path = PathBuf::from(&file_path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    
    // Write content to file
    std::fs::write(&path, &content)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    // Update editor tab
    let mut editor = state.editor.write().await;
    if let Some(tab) = editor.open_files.values_mut().find(|tab| tab.path == path) {
        tab.content = content;
        tab.is_dirty = false;
    }
    
    Ok(FileOperation {
        success: true,
        message: "File saved successfully".to_string(),
        file_path,
    })
}

#[tauri::command]
pub async fn create_file(
    state: State<'_, AppState>,
    file_path: String,
    initial_content: Option<String>,
) -> Result<FileOperation, String> {
    let path = PathBuf::from(&file_path);
    
    if path.exists() {
        return Err("File already exists".to_string());
    }
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    
    let content = initial_content.unwrap_or_default();
    
    // Create the file
    std::fs::write(&path, &content)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    // Open file in editor
    let mut editor = state.editor.write().await;
    editor.open_file(path.clone(), content)
        .map_err(|e| format!("Failed to open file in editor: {}", e))?;
    
    Ok(FileOperation {
        success: true,
        message: "File created successfully".to_string(),
        file_path,
    })
}

#[tauri::command]
pub async fn delete_file(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<FileOperation, String> {
    let path = PathBuf::from(&file_path);
    
    if !path.exists() {
        return Err("File does not exist".to_string());
    }
    
    // Remove file from editor if open
    let mut editor = state.editor.write().await;
    if let Some(tab_id) = editor.open_files.iter().find(|(_, tab)| tab.path == path).map(|(id, _)| id.clone()) {
        editor.close_file(&tab_id)
            .map_err(|e| format!("Failed to close file in editor: {}", e))?;
    }
    
    // Delete the file
    std::fs::remove_file(&path)
        .map_err(|e| format!("Failed to delete file: {}", e))?;
    
    Ok(FileOperation {
        success: true,
        message: "File deleted successfully".to_string(),
        file_path,
    })
}

#[tauri::command]
pub async fn get_file_info(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<FileContent, String> {
    let path = PathBuf::from(&file_path);
    
    if !path.exists() {
        return Err("File does not exist".to_string());
    }
    
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    let language = detect_language(&path);
    
    Ok(FileContent {
        path: file_path,
        content,
        language,
        size: metadata.len(),
        modified: metadata.modified()
            .map_err(|e| format!("Failed to get modification time: {}", e))?
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    })
}

#[tauri::command]
pub async fn list_directory(
    _state: State<'_, AppState>,
    directory_path: String,
) -> Result<Vec<FileNode>, String> {
    let path = PathBuf::from(&directory_path);
    
    if !path.exists() {
        return Err("Directory does not exist".to_string());
    }
    
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }
    
    let mut files = Vec::new();
    
    for entry in std::fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let entry_path = entry.path();
        let entry_name = entry_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let is_directory = entry_path.is_dir();
        
        files.push(FileNode {
            name: entry_name,
            path: entry_path.to_string_lossy().to_string(),
            is_directory,
            children: None, // We'll load children on demand
            expanded: Some(false),
        });
    }
    
    // Sort directories first, then files, alphabetically
    files.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    
    Ok(files)
}

fn detect_language(file_path: &PathBuf) -> String {
    if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
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
            _ => "text".to_string(),
        }
    } else {
        "text".to_string()
    }
}