/*!
 * Git UI Commands
 * 
 * Tauri command handlers for Git operations.
 */

use tauri::State;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepositoryInfo {
    pub path: String,
    pub current_branch: String,
    pub remote_branches: Vec<String>,
    pub local_branches: Vec<String>,
    pub status: GitStatusInfo,
    pub is_dirty: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatusInfo {
    pub modified_files: Vec<FileStatusInfo>,
    pub staged_files: Vec<FileStatusInfo>,
    pub untracked_files: Vec<FileStatusInfo>,
    pub deleted_files: Vec<FileStatusInfo>,
    pub renamed_files: Vec<FileStatusInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatusInfo {
    pub path: String,
    pub status: String,
    pub diff: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitRequest {
    pub message: String,
    pub author_name: String,
    pub author_email: String,
}

#[tauri::command]
pub async fn get_git_status(
    state: State<'_, AppState>,
    repository_path: String,
) -> Result<GitStatusInfo, String> {
    let git_manager = state.git.read().await;
    let path = PathBuf::from(&repository_path);
    
    let status = git_manager.get_repository_status(&path)
        .map_err(|e| format!("Failed to get git status: {}", e))?;
    
    let status_info = GitStatusInfo {
        modified_files: status.modified_files.into_iter().map(|f| FileStatusInfo {
            path: f.path.to_string_lossy().to_string(),
            status: format!("{:?}", f.status),
            diff: f.diff,
        }).collect(),
        staged_files: status.staged_files.into_iter().map(|f| FileStatusInfo {
            path: f.path.to_string_lossy().to_string(),
            status: format!("{:?}", f.status),
            diff: f.diff,
        }).collect(),
        untracked_files: status.untracked_files.into_iter().map(|f| FileStatusInfo {
            path: f.path.to_string_lossy().to_string(),
            status: format!("{:?}", f.status),
            diff: f.diff,
        }).collect(),
        deleted_files: status.deleted_files.into_iter().map(|f| FileStatusInfo {
            path: f.path.to_string_lossy().to_string(),
            status: format!("{:?}", f.status),
            diff: f.diff,
        }).collect(),
        renamed_files: status.renamed_files.into_iter().map(|f| FileStatusInfo {
            path: f.path.to_string_lossy().to_string(),
            status: format!("{:?}", f.status),
            diff: f.diff,
        }).collect(),
    };
    
    Ok(status_info)
}

#[tauri::command]
pub async fn stage_changes(
    state: State<'_, AppState>,
    repository_path: String,
    file_path: String,
) -> Result<(), String> {
    let git_manager = state.git.read().await;
    let repo_path = PathBuf::from(&repository_path);
    let file_path = PathBuf::from(&file_path);
    
    git_manager.stage_file(&repo_path, &file_path)
        .map_err(|e| format!("Failed to stage file: {}", e))?;
    Ok(())
}


#[tauri::command]
pub async fn commit_changes(
    state: State<'_, AppState>,
    repository_path: String,
    request: CommitRequest,
) -> Result<String, String> {
    let git_manager = state.git.read().await;
    let repo_path = PathBuf::from(&repository_path);
    
    let commit_hash = git_manager.commit_changes(
        &repo_path,
        &request.message,
        &request.author_name,
        &request.author_email,
    ).map_err(|e| format!("Failed to commit changes: {}", e))?;
    
    Ok(commit_hash)
}

#[tauri::command]
pub async fn push_changes(
    state: State<'_, AppState>,
    repository_path: String,
    remote_name: String,
    branch_name: String,
) -> Result<(), String> {
    let git_manager = state.git.read().await;
    let repo_path = PathBuf::from(&repository_path);
    
    git_manager.push_changes(&repo_path, &remote_name, &branch_name)
        .map_err(|e| format!("Failed to push changes: {}", e))?;
    Ok(())
}





