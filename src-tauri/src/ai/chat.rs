/*!
 * Chat Engine Module
 * 
 * Manages AI chat conversations and message handling.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEngine {
    pub sessions: HashMap<String, ChatSession>,
    pub active_session: Option<String>,
    pub chat_settings: ChatSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub model_used: Option<String>,
    pub context_type: ContextType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: MessageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub model_used: Option<String>,
    pub tokens_used: Option<u32>,
    pub generation_time_ms: Option<u64>,
    pub context_files: Vec<String>,
    pub code_blocks: Vec<CodeBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    pub language: String,
    pub content: String,
    pub start_line: Option<u32>,
    pub end_line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextType {
    General,
    CodeReview,
    Debugging,
    Documentation,
    Refactoring,
    Testing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSettings {
    pub max_context_length: u32,
    pub include_file_context: bool,
    pub include_project_context: bool,
    pub auto_save_sessions: bool,
    pub session_timeout_minutes: u32,
    pub max_messages_per_session: usize,
}

impl Default for ChatEngine {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
            active_session: None,
            chat_settings: ChatSettings::default(),
        }
    }
}

impl Default for ChatSettings {
    fn default() -> Self {
        Self {
            max_context_length: 4096,
            include_file_context: true,
            include_project_context: true,
            auto_save_sessions: true,
            session_timeout_minutes: 60,
            max_messages_per_session: 1000,
        }
    }
}

impl ChatEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_session(&mut self, title: String, context_type: ContextType) -> String {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let session = ChatSession {
            id: id.clone(),
            title,
            messages: Vec::new(),
            created_at: now,
            last_activity: now,
            model_used: None,
            context_type,
        };

        self.sessions.insert(id.clone(), session);
        self.active_session = Some(id.clone());
        
        id
    }

    pub fn close_session(&mut self, session_id: &str) -> Result<()> {
        self.sessions.remove(session_id);
        
        if self.active_session.as_ref() == Some(&session_id.to_string()) {
            self.active_session = self.sessions.keys().next().cloned();
        }
        
        Ok(())
    }

    pub fn add_message(&mut self, session_id: &str, role: MessageRole, content: String, metadata: MessageMetadata) -> Result<String> {
        let message_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let message = ChatMessage {
            id: message_id.clone(),
            role,
            content,
            timestamp: now,
            metadata,
        };

        if let Some(session) = self.sessions.get_mut(session_id) {
            session.messages.push(message);
            session.last_activity = now;
            
            // Trim messages if exceeding limit
            if session.messages.len() > self.chat_settings.max_messages_per_session {
                session.messages.drain(0..session.messages.len() - self.chat_settings.max_messages_per_session);
            }
        } else {
            return Err(anyhow::anyhow!("Session not found: {}", session_id));
        }
        
        Ok(message_id)
    }

    pub fn get_session_messages(&self, session_id: &str, limit: Option<usize>) -> Result<Vec<ChatMessage>> {
        if let Some(session) = self.sessions.get(session_id) {
            let messages = if let Some(limit) = limit {
                session.messages
                    .iter()
                    .rev()
                    .take(limit)
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect()
            } else {
                session.messages.clone()
            };
            Ok(messages)
        } else {
            Err(anyhow::anyhow!("Session not found: {}", session_id))
        }
    }

    pub fn get_active_session(&self) -> Option<&ChatSession> {
        self.active_session.as_ref().and_then(|id| self.sessions.get(id))
    }

    pub fn set_active_session(&mut self, session_id: &str) -> Result<()> {
        if self.sessions.contains_key(session_id) {
            self.active_session = Some(session_id.to_string());
        } else {
            return Err(anyhow::anyhow!("Session not found: {}", session_id));
        }
        Ok(())
    }

    pub fn get_sessions(&self) -> Vec<&ChatSession> {
        self.sessions.values().collect()
    }

    pub fn get_session(&self, session_id: &str) -> Option<&ChatSession> {
        self.sessions.get(session_id)
    }

    pub fn update_session_title(&mut self, session_id: &str, title: String) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.title = title;
        } else {
            return Err(anyhow::anyhow!("Session not found: {}", session_id));
        }
        Ok(())
    }

    pub fn clear_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.messages.clear();
        } else {
            return Err(anyhow::anyhow!("Session not found: {}", session_id));
        }
        Ok(())
    }

    pub fn export_session(&self, session_id: &str) -> Result<String> {
        if let Some(session) = self.sessions.get(session_id) {
            Ok(serde_json::to_string_pretty(session)?)
        } else {
            Err(anyhow::anyhow!("Session not found: {}", session_id))
        }
    }

    pub fn import_session(&mut self, session_data: &str) -> Result<String> {
        let session: ChatSession = serde_json::from_str(session_data)?;
        let id = session.id.clone();
        self.sessions.insert(id.clone(), session);
        Ok(id)
    }

    pub fn cleanup_old_sessions(&mut self) -> Result<()> {
        let cutoff = Utc::now() - chrono::Duration::minutes(self.chat_settings.session_timeout_minutes as i64);
        
        self.sessions.retain(|_, session| {
            session.last_activity > cutoff
        });
        
        Ok(())
    }

    pub fn get_conversation_context(&self, session_id: &str, max_tokens: u32) -> Result<String> {
        if let Some(session) = self.sessions.get(session_id) {
            let mut context = String::new();
            let mut token_count = 0;
            
            // Add system context if available
            if let Some(system_msg) = session.messages.iter().find(|m| matches!(m.role, MessageRole::System)) {
                context.push_str(&format!("System: {}\n\n", system_msg.content));
                token_count += estimate_tokens(&system_msg.content);
            }
            
            // Add recent messages
            for message in session.messages.iter().rev() {
                let message_text = match &message.role {
                    MessageRole::User => format!("User: {}", message.content),
                    MessageRole::Assistant => format!("Assistant: {}", message.content),
                    MessageRole::System => continue, // Already handled above
                };
                
                let message_tokens = estimate_tokens(&message_text);
                if token_count + message_tokens > max_tokens {
                    break;
                }
                
                context.insert_str(0, &format!("{}\n\n", message_text));
                token_count += message_tokens;
            }
            
            Ok(context)
        } else {
            Err(anyhow::anyhow!("Session not found: {}", session_id))
        }
    }

    pub fn extract_code_blocks(&self, content: &str) -> Vec<CodeBlock> {
        let mut code_blocks = Vec::new();
        let mut lines = content.lines().enumerate();
        
        while let Some((line_num, line)) = lines.next() {
            if line.trim().starts_with("```") {
                let language = line.trim().trim_start_matches("```").to_string();
                let mut code_content = String::new();
                let start_line = line_num as u32;
                
                while let Some((_, code_line)) = lines.next() {
                    if code_line.trim() == "```" {
                        break;
                    }
                    code_content.push_str(code_line);
                    code_content.push('\n');
                }
                
                code_blocks.push(CodeBlock {
                    language,
                    content: code_content.trim().to_string(),
                    start_line: Some(start_line),
                    end_line: Some(line_num as u32),
                });
            }
        }
        
        code_blocks
    }
}

fn estimate_tokens(text: &str) -> u32 {
    // Simple token estimation: roughly 4 characters per token
    (text.len() as f32 / 4.0).ceil() as u32
}