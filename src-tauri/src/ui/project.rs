/*!
 * Project UI Commands
 * 
 * Tauri command handlers for project management operations.
 */

use tauri::State;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, error, warn};

use crate::core::project::ProjectType;
use crate::database::{ProjectRecord};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub project_type: String,
    pub language: String,
    pub created_at: i64,
    pub last_opened: String, // Changed from i64 to String to match frontend
    pub settings: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub files: Vec<FileInfo>,
    pub directories: Vec<DirectoryInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: i64,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryInfo {
    pub name: String,
    pub path: String,
    pub files_count: usize,
    pub subdirectories_count: usize,
}

#[tauri::command]
pub async fn test_project_connection() -> Result<String, String> {
    info!("Test connection called");
    Ok("Connection working".to_string())
}

#[tauri::command]
pub async fn open_project(
    state: State<'_, AppState>,
    path: String,
) -> Result<ProjectInfo, String> {
    info!("Opening project: {}", path);
    
    let project_path = PathBuf::from(&path);
    
    // Validate path exists
    if !project_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    // Open project in project manager
    let mut project_manager = state.projects.write().await;
    let project = project_manager.open_project(project_path.clone())
        .map_err(|e| {
            error!("Failed to open project: {}", e);
            e.to_string()
        })?;
    
    info!("Project opened successfully: {}", project.name);
    
    // Save to database (non-blocking, don't fail if this fails)
    let project_record = ProjectRecord {
        id: project.id.to_string(),
        name: project.name.clone(),
        path: project.path.to_string_lossy().to_string(),
        project_type: format!("{:?}", project.project_type),
        language: format!("{:?}", project.project_type),
        created_at: project.last_opened.timestamp(),
        last_opened: project.last_opened.timestamp(),
        settings: serde_json::to_string(&project.settings).unwrap_or_default(),
    };
    
    // Try to save to database, but don't fail the whole operation if it fails
    if let Err(e) = {
        let database = state.database.read().await;
        database.insert_project(&project_record).await
    } {
        warn!("Failed to save project to database: {}", e);
        // Continue anyway - the project is still opened successfully
    }
    
    Ok(ProjectInfo {
        id: project.id.to_string(),
        name: project.name,
        path: project.path.to_string_lossy().to_string(),
        project_type: format!("{:?}", project.project_type),
        language: format!("{:?}", project.project_type),
        created_at: project.last_opened.timestamp(),
        last_opened: project.last_opened.to_rfc3339(), // Convert to string
        settings: serde_json::to_string(&project.settings).unwrap_or_default(),
    })
}

#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    path: String,
    project_type: String,
) -> Result<ProjectInfo, String> {
    let project_path = PathBuf::from(&path);
    let project_type_enum = match project_type.as_str() {
        "rust" => ProjectType::Rust,
        "javascript" => ProjectType::JavaScript,
        "typescript" => ProjectType::TypeScript,
        "python" => ProjectType::Python,
        _ => ProjectType::General,
    };
    
    // Create project in project manager
    let mut project_manager = state.projects.write().await;
    let project = project_manager.create_project(project_path.clone(), project_type_enum)
        .map_err(|e| e.to_string())?;
    
    // Save to database
    let project_record = ProjectRecord {
        id: project.id.to_string(),
        name: project.name.clone(),
        path: project.path.to_string_lossy().to_string(),
        project_type: format!("{:?}", project.project_type),
        language: format!("{:?}", project.project_type),
        created_at: project.last_opened.timestamp(),
        last_opened: project.last_opened.timestamp(),
        settings: serde_json::to_string(&project.settings).unwrap_or_default(),
    };
    
    let database = state.database.read().await;
    database.insert_project(&project_record).await
        .map_err(|e| e.to_string())?;
    
    Ok(ProjectInfo {
        id: project.id.to_string(),
        name: project.name,
        path: project.path.to_string_lossy().to_string(),
        project_type: format!("{:?}", project.project_type),
        language: format!("{:?}", project.project_type),
        created_at: project.last_opened.timestamp(),
        last_opened: project.last_opened.to_rfc3339(), // Convert to string
        settings: serde_json::to_string(&project.settings).unwrap_or_default(),
    })
}

#[tauri::command]
pub async fn get_project_structure(
    _state: State<'_, AppState>,
    project_path: String,
) -> Result<ProjectStructure, String> {
    let path = PathBuf::from(&project_path);
    
    if !path.exists() {
        return Err("Project path does not exist".to_string());
    }
    
    let mut files = Vec::new();
    let mut directories = Vec::new();
    
    collect_project_structure(&path, &path, &mut files, &mut directories)
        .map_err(|e| e.to_string())?;
    
    Ok(ProjectStructure { files, directories })
}

fn collect_project_structure(
    root_path: &PathBuf,
    current_path: &PathBuf,
    files: &mut Vec<FileInfo>,
    directories: &mut Vec<DirectoryInfo>,
) -> Result<()> {
    if !current_path.is_dir() {
        return Ok(());
    }
    
    let mut file_count = 0;
    let mut subdir_count = 0;
    
    for entry in std::fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            file_count += 1;
            
            let metadata = std::fs::metadata(&path)?;
            let relative_path = path.strip_prefix(root_path)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            
            let language = detect_language(&path);
            
            files.push(FileInfo {
                name: path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string(),
                path: relative_path,
                size: metadata.len(),
                modified: metadata.modified()?
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                language,
            });
        } else if path.is_dir() {
            subdir_count += 1;
            
            let relative_path = path.strip_prefix(root_path)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            
            directories.push(DirectoryInfo {
                name: path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string(),
                path: relative_path,
                files_count: 0, // Will be calculated recursively
                subdirectories_count: 0, // Will be calculated recursively
            });
            
            // Recursively collect subdirectory contents
            collect_project_structure(root_path, &path, files, directories)?;
        }
    }
    
    // Update directory counts
    if let Some(dir_info) = directories.last_mut() {
        dir_info.files_count = file_count;
        dir_info.subdirectories_count = subdir_count;
    }
    
    Ok(())
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

#[tauri::command]
pub async fn get_recent_projects(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<ProjectInfo>, String> {
    let database = state.database.read().await;
    let limit = limit.unwrap_or(10);
    
    let projects = database.get_recent_projects(limit).await
        .map_err(|e| e.to_string())?;
    
    let project_infos: Vec<ProjectInfo> = projects.into_iter().map(|p| ProjectInfo {
        id: p.id,
        name: p.name,
        path: p.path,
        project_type: p.project_type,
        language: p.language,
        created_at: p.created_at,
        last_opened: chrono::DateTime::from_timestamp(p.last_opened, 0)
            .unwrap_or_default()
            .to_rfc3339(),
        settings: p.settings,
    }).collect();
    
    Ok(project_infos)
}