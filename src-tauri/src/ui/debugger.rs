/*!
 * Debugger UI Commands
 * 
 * Tauri command handlers for debugger operations.
 */

use tauri::State;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugSessionInfo {
    pub id: String,
    pub name: String,
    pub program_path: String,
    pub working_directory: String,
    pub status: String,
    pub current_frame: Option<StackFrameInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrameInfo {
    pub id: u32,
    pub name: String,
    pub file_path: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreakpointInfo {
    pub id: String,
    pub file_path: String,
    pub line: u32,
    pub column: Option<u32>,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub is_enabled: bool,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDebugRequest {
    pub name: String,
    pub program_path: String,
    pub working_directory: String,
    pub arguments: Vec<String>,
}

#[tauri::command]
pub async fn start_debug_session(
    state: State<'_, AppState>,
    request: StartDebugRequest,
) -> Result<String, String> {
    let mut debugger = state.debugger.write().await;
    
    let program_path = PathBuf::from(&request.program_path);
    let working_directory = PathBuf::from(&request.working_directory);
    
    let session_id = debugger.start_debug_session(
        request.name,
        program_path,
        working_directory,
        request.arguments,
    ).map_err(|e| format!("Failed to start debug session: {}", e))?;
    
    Ok(session_id)
}


#[tauri::command]
pub async fn set_breakpoint(
    state: State<'_, AppState>,
    file_path: String,
    line: u32,
    column: Option<u32>,
    condition: Option<String>,
) -> Result<String, String> {
    let mut debugger = state.debugger.write().await;
    let path = PathBuf::from(&file_path);
    
    let breakpoint_id = debugger.set_breakpoint(path, line, column, condition)
        .map_err(|e| format!("Failed to set breakpoint: {}", e))?;
    
    Ok(breakpoint_id)
}


#[tauri::command]
pub async fn continue_execution(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<(), String> {
    let mut debugger = state.debugger.write().await;
    debugger.continue_execution(&session_id)
        .map_err(|e| format!("Failed to continue execution: {}", e))?;
    Ok(())
}


#[tauri::command]
pub async fn step_over(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<(), String> {
    let mut debugger = state.debugger.write().await;
    debugger.step_over(&session_id)
        .map_err(|e| format!("Failed to step over: {}", e))?;
    Ok(())
}



