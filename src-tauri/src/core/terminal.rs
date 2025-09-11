/*!
 * Terminal Manager Module
 * 
 * Manages terminal sessions and command execution.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use uuid::Uuid;
use anyhow::Result;
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalManager {
    pub terminals: HashMap<String, TerminalSession>,
    pub active_terminal: Option<String>,
    pub default_shell: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub title: String,
    pub working_directory: PathBuf,
    pub shell: String,
    pub is_active: bool,
    pub history: Vec<String>,
    pub output_buffer: Vec<TerminalOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalOutput {
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub output_type: OutputType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputType {
    Stdout,
    Stderr,
    Command,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self {
            terminals: HashMap::new(),
            active_terminal: None,
            default_shell: if cfg!(windows) {
                "powershell.exe".to_string()
            } else {
                "/bin/bash".to_string()
            },
        }
    }
}

impl TerminalManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_terminal(&mut self, working_directory: PathBuf, title: Option<String>) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let title = title.unwrap_or_else(|| format!("Terminal {}", self.terminals.len() + 1));
        
        let session = TerminalSession {
            id: id.clone(),
            title,
            working_directory,
            shell: self.default_shell.clone(),
            is_active: true,
            history: Vec::new(),
            output_buffer: Vec::new(),
        };

        self.terminals.insert(id.clone(), session);
        self.active_terminal = Some(id.clone());
        
        Ok(id)
    }

    pub fn close_terminal(&mut self, id: &str) -> Result<()> {
        self.terminals.remove(id);
        
        if self.active_terminal.as_ref() == Some(&id.to_string()) {
            self.active_terminal = self.terminals.keys().next().cloned();
        }
        
        Ok(())
    }

    pub async fn execute_command(&mut self, terminal_id: &str, command: &str) -> Result<CommandResult> {
        let _start_time = std::time::Instant::now();
        
        // Get working directory before borrowing
        let working_dir = if let Some(session) = self.terminals.get(terminal_id) {
            session.working_directory.clone()
        } else {
            return Err(anyhow::anyhow!("Terminal session not found: {}", terminal_id));
        };
        
        // Execute command
        let result = self.run_command(command, &working_dir).await?;
        
        // Update session with result
        if let Some(session) = self.terminals.get_mut(terminal_id) {
            // Add command to history
            session.history.push(command.to_string());
            
            // Add command to output buffer
            session.output_buffer.push(TerminalOutput {
                content: format!("$ {}", command),
                timestamp: chrono::Utc::now(),
                output_type: OutputType::Command,
            });
            
            // Add output to buffer
            if !result.stdout.is_empty() {
                session.output_buffer.push(TerminalOutput {
                    content: result.stdout.clone(),
                    timestamp: chrono::Utc::now(),
                    output_type: OutputType::Stdout,
                });
            }
            
            if !result.stderr.is_empty() {
                session.output_buffer.push(TerminalOutput {
                    content: result.stderr.clone(),
                    timestamp: chrono::Utc::now(),
                    output_type: OutputType::Stderr,
                });
            }

            // Keep only last 1000 lines of output
            if session.output_buffer.len() > 1000 {
                session.output_buffer.drain(0..session.output_buffer.len() - 1000);
            }

            Ok(result)
        } else {
            Err(anyhow::anyhow!("Terminal not found: {}", terminal_id))
        }
    }

    async fn run_command(&self, command: &str, working_dir: &PathBuf) -> Result<CommandResult> {
        let start_time = std::time::Instant::now();
        
        let mut cmd = if cfg!(windows) {
            let mut cmd = AsyncCommand::new("powershell.exe");
            cmd.arg("-Command").arg(command);
            cmd
        } else {
            let mut cmd = AsyncCommand::new("bash");
            cmd.arg("-c").arg(command);
            cmd
        };

        cmd.current_dir(working_dir)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let child = cmd.spawn()?;
        let output = child.wait_with_output().await?;
        
        let duration = start_time.elapsed();
        
        Ok(CommandResult {
            command: command.to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms: duration.as_millis() as u64,
        })
    }

    pub fn get_terminal_output(&self, terminal_id: &str, lines: Option<usize>) -> Result<Vec<TerminalOutput>> {
        if let Some(session) = self.terminals.get(terminal_id) {
            let output = if let Some(lines) = lines {
                session.output_buffer
                    .iter()
                    .rev()
                    .take(lines)
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect()
            } else {
                session.output_buffer.clone()
            };
            Ok(output)
        } else {
            Err(anyhow::anyhow!("Terminal not found: {}", terminal_id))
        }
    }

    pub fn get_terminal_history(&self, terminal_id: &str) -> Result<Vec<String>> {
        if let Some(session) = self.terminals.get(terminal_id) {
            Ok(session.history.clone())
        } else {
            Err(anyhow::anyhow!("Terminal not found: {}", terminal_id))
        }
    }

    pub fn set_working_directory(&mut self, terminal_id: &str, path: PathBuf) -> Result<()> {
        if let Some(session) = self.terminals.get_mut(terminal_id) {
            session.working_directory = path;
        }
        Ok(())
    }

    pub fn get_working_directory(&self, terminal_id: &str) -> Result<PathBuf> {
        if let Some(session) = self.terminals.get(terminal_id) {
            Ok(session.working_directory.clone())
        } else {
            Err(anyhow::anyhow!("Terminal not found: {}", terminal_id))
        }
    }

    pub fn get_terminals(&self) -> Vec<&TerminalSession> {
        self.terminals.values().collect()
    }

    pub fn get_active_terminal(&self) -> Option<&TerminalSession> {
        self.active_terminal.as_ref().and_then(|id| self.terminals.get(id))
    }

    pub fn set_active_terminal(&mut self, id: &str) -> Result<()> {
        if self.terminals.contains_key(id) {
            // Deactivate current terminal
            if let Some(current_id) = &self.active_terminal {
                if let Some(session) = self.terminals.get_mut(current_id) {
                    session.is_active = false;
                }
            }
            
            // Activate new terminal
            if let Some(session) = self.terminals.get_mut(id) {
                session.is_active = true;
            }
            
            self.active_terminal = Some(id.to_string());
        }
        Ok(())
    }

    pub fn clear_terminal(&mut self, terminal_id: &str) -> Result<()> {
        if let Some(session) = self.terminals.get_mut(terminal_id) {
            session.output_buffer.clear();
        }
        Ok(())
    }
}