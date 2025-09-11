/*!
 * Debugger Engine Module
 * 
 * Manages debugging sessions, breakpoints, and variable inspection.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggerEngine {
    pub active_sessions: HashMap<String, DebugSession>,
    pub breakpoints: HashMap<String, Vec<Breakpoint>>,
    pub debugger_settings: DebuggerSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSession {
    pub id: String,
    pub name: String,
    pub program_path: PathBuf,
    pub working_directory: PathBuf,
    pub arguments: Vec<String>,
    pub environment: HashMap<String, String>,
    pub status: DebugStatus,
    pub current_frame: Option<StackFrame>,
    pub call_stack: Vec<StackFrame>,
    pub variables: HashMap<String, Variable>,
    pub threads: Vec<Thread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: String,
    pub file_path: PathBuf,
    pub line: u32,
    pub column: Option<u32>,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub is_enabled: bool,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub id: u32,
    pub name: String,
    pub file_path: PathBuf,
    pub line: u32,
    pub column: u32,
    pub variables: HashMap<String, Variable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub type_name: String,
    pub is_expanded: bool,
    pub children: Vec<Variable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
    pub state: ThreadState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugStatus {
    NotStarted,
    Running,
    Paused,
    Stopped,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadState {
    Running,
    Stopped,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggerSettings {
    pub auto_attach: bool,
    pub break_on_exceptions: bool,
    pub break_on_unhandled_exceptions: bool,
    pub show_return_value: bool,
    pub enable_step_filtering: bool,
    pub max_string_length: usize,
    pub max_array_elements: usize,
}

impl Default for DebuggerEngine {
    fn default() -> Self {
        Self {
            active_sessions: HashMap::new(),
            breakpoints: HashMap::new(),
            debugger_settings: DebuggerSettings::default(),
        }
    }
}

impl Default for DebuggerSettings {
    fn default() -> Self {
        Self {
            auto_attach: false,
            break_on_exceptions: true,
            break_on_unhandled_exceptions: true,
            show_return_value: true,
            enable_step_filtering: true,
            max_string_length: 1000,
            max_array_elements: 100,
        }
    }
}

impl DebuggerEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_debug_session(
        &mut self,
        name: String,
        program_path: PathBuf,
        working_directory: PathBuf,
        arguments: Vec<String>,
    ) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        
        let session = DebugSession {
            id: id.clone(),
            name,
            program_path,
            working_directory,
            arguments,
            environment: HashMap::new(),
            status: DebugStatus::NotStarted,
            current_frame: None,
            call_stack: Vec::new(),
            variables: HashMap::new(),
            threads: Vec::new(),
        };

        self.active_sessions.insert(id.clone(), session);
        Ok(id)
    }

    pub fn stop_debug_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            session.status = DebugStatus::Stopped;
        }
        Ok(())
    }

    pub fn set_breakpoint(
        &mut self,
        file_path: PathBuf,
        line: u32,
        column: Option<u32>,
        condition: Option<String>,
    ) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let file_key = file_path.to_string_lossy().to_string();
        
        let breakpoint = Breakpoint {
            id: id.clone(),
            file_path,
            line,
            column,
            condition,
            hit_count: 0,
            is_enabled: true,
            is_verified: false,
        };

        self.breakpoints
            .entry(file_key)
            .or_insert_with(Vec::new)
            .push(breakpoint);

        Ok(id)
    }

    pub fn remove_breakpoint(&mut self, breakpoint_id: &str) -> Result<()> {
        for breakpoints in self.breakpoints.values_mut() {
            breakpoints.retain(|bp| bp.id != breakpoint_id);
        }
        Ok(())
    }

    pub fn toggle_breakpoint(&mut self, breakpoint_id: &str) -> Result<()> {
        for breakpoints in self.breakpoints.values_mut() {
            if let Some(breakpoint) = breakpoints.iter_mut().find(|bp| bp.id == breakpoint_id) {
                breakpoint.is_enabled = !breakpoint.is_enabled;
            }
        }
        Ok(())
    }

    pub fn get_breakpoints(&self, file_path: &PathBuf) -> Vec<&Breakpoint> {
        let file_key = file_path.to_string_lossy().to_string();
        self.breakpoints
            .get(&file_key)
            .map(|bps| bps.iter().collect())
            .unwrap_or_default()
    }

    pub fn continue_execution(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            session.status = DebugStatus::Running;
            // In a real implementation, this would send a continue command to the debugger
        }
        Ok(())
    }

    pub fn pause_execution(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            session.status = DebugStatus::Paused;
            // In a real implementation, this would send a pause command to the debugger
        }
        Ok(())
    }

    pub fn step_over(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            // In a real implementation, this would send a step over command to the debugger
            // For now, we'll just simulate updating the current frame
            if let Some(frame) = &mut session.current_frame {
                frame.line += 1;
            }
        }
        Ok(())
    }

    pub fn step_into(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            // In a real implementation, this would send a step into command to the debugger
        }
        Ok(())
    }

    pub fn step_out(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            // In a real implementation, this would send a step out command to the debugger
        }
        Ok(())
    }

    pub fn get_call_stack(&self, session_id: &str) -> Result<Vec<StackFrame>> {
        if let Some(session) = self.active_sessions.get(session_id) {
            Ok(session.call_stack.clone())
        } else {
            Err(anyhow::anyhow!("Debug session not found: {}", session_id))
        }
    }

    pub fn get_variables(&self, session_id: &str, frame_id: Option<u32>) -> Result<HashMap<String, Variable>> {
        if let Some(session) = self.active_sessions.get(session_id) {
            if let Some(frame_id) = frame_id {
                // Get variables for specific frame
                if let Some(frame) = session.call_stack.iter().find(|f| f.id == frame_id) {
                    Ok(frame.variables.clone())
                } else {
                    Ok(HashMap::new())
                }
            } else {
                // Get variables for current frame
                Ok(session.variables.clone())
            }
        } else {
            Err(anyhow::anyhow!("Debug session not found: {}", session_id))
        }
    }

    pub fn evaluate_expression(&self, _session_id: &str, expression: &str) -> Result<Variable> {
        // In a real implementation, this would send an evaluate command to the debugger
        // For now, return a mock variable
        Ok(Variable {
            name: expression.to_string(),
            value: "mock_value".to_string(),
            type_name: "unknown".to_string(),
            is_expanded: false,
            children: Vec::new(),
        })
    }

    pub fn get_threads(&self, session_id: &str) -> Result<Vec<Thread>> {
        if let Some(session) = self.active_sessions.get(session_id) {
            Ok(session.threads.clone())
        } else {
            Err(anyhow::anyhow!("Debug session not found: {}", session_id))
        }
    }

    pub fn set_current_thread(&mut self, session_id: &str, thread_id: u32) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            for thread in &mut session.threads {
                thread.is_active = thread.id == thread_id;
            }
        }
        Ok(())
    }

    pub fn get_active_sessions(&self) -> Vec<&DebugSession> {
        self.active_sessions.values().collect()
    }

    pub fn get_session(&self, session_id: &str) -> Option<&DebugSession> {
        self.active_sessions.get(session_id)
    }
}