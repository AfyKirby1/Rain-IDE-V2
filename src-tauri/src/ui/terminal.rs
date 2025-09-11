/*!
 * Terminal UI Commands
 * 
 * Tauri command handlers for terminal operations.
 */

use tauri::State;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalInfo {
    pub id: String,
    pub title: String,
    pub working_directory: String,
    pub shell: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalOutput {
    pub content: String,
    pub timestamp: i64,
    pub output_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub terminal_id: String,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub command: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[tauri::command]
pub async fn create_terminal(
    state: State<'_, AppState>,
    working_directory: String,
    title: Option<String>,
) -> Result<String, String> {
    let mut terminal_manager = state.terminal.write().await;
    let path = PathBuf::from(&working_directory);
    
    let terminal_id = terminal_manager.create_terminal(path, title)
        .map_err(|e| format!("Failed to create terminal: {}", e))?;
    
    Ok(terminal_id)
}

#[tauri::command]
pub async fn execute_command(
    state: State<'_, AppState>,
    request: CommandRequest,
) -> Result<CommandResult, String> {
    let mut terminal_manager = state.terminal.write().await;
    
    let result = terminal_manager.execute_command(&request.terminal_id, &request.command).await
        .map_err(|e| format!("Failed to execute command: {}", e))?;
    
    Ok(CommandResult {
        command: result.command,
        exit_code: result.exit_code,
        stdout: result.stdout,
        stderr: result.stderr,
        duration_ms: result.duration_ms,
    })
}

#[tauri::command]
pub async fn get_terminal_output(
    state: State<'_, AppState>,
    terminal_id: String,
    lines: Option<usize>,
) -> Result<Vec<TerminalOutput>, String> {
    let terminal_manager = state.terminal.read().await;
    
    let output = terminal_manager.get_terminal_output(&terminal_id, lines)
        .map_err(|e| format!("Failed to get terminal output: {}", e))?;
    
    let terminal_outputs: Vec<TerminalOutput> = output.into_iter().map(|out| TerminalOutput {
        content: out.content,
        timestamp: out.timestamp.timestamp(),
        output_type: format!("{:?}", out.output_type),
    }).collect();
    
    Ok(terminal_outputs)
}



