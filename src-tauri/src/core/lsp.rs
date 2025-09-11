/*!
 * Language Server Protocol Manager Module
 * 
 * Manages LSP connections and language-specific features.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use anyhow::Result;
use tower_lsp::lsp_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageServerManager {
    pub active_servers: HashMap<String, LanguageServer>,
    pub server_configs: HashMap<String, ServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageServer {
    pub id: String,
    pub name: String,
    pub language: String,
    pub status: ServerStatus,
    pub capabilities: ServerCapabilities,
    pub root_uri: Option<String>,
    pub workspace_folders: Vec<WorkspaceFolder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub language: String,
    pub command: String,
    pub args: Vec<String>,
    pub options: HashMap<String, serde_json::Value>,
    pub initialization_options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerStatus {
    NotStarted,
    Starting,
    Running,
    Stopped,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub text_document_sync: Option<TextDocumentSyncCapability>,
    pub completion_provider: Option<CompletionOptions>,
    pub hover_provider: Option<bool>,
    pub signature_help_provider: Option<SignatureHelpOptions>,
    pub definition_provider: Option<bool>,
    pub references_provider: Option<bool>,
    pub document_highlight_provider: Option<bool>,
    pub document_symbol_provider: Option<bool>,
    pub workspace_symbol_provider: Option<bool>,
    pub code_action_provider: Option<CodeActionProviderCapability>,
    pub code_lens_provider: Option<CodeLensOptions>,
    pub document_formatting_provider: Option<bool>,
    pub document_range_formatting_provider: Option<bool>,
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    pub rename_provider: Option<RenameOptions>,
    pub document_link_provider: Option<DocumentLinkOptions>,
    pub color_provider: Option<ColorProviderCapability>,
    pub folding_range_provider: Option<FoldingRangeProviderCapability>,
    pub execute_command_provider: Option<ExecuteCommandOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String,
}

impl Default for LanguageServerManager {
    fn default() -> Self {
        Self {
            active_servers: HashMap::new(),
            server_configs: Self::get_default_server_configs(),
        }
    }
}

impl LanguageServerManager {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_default_server_configs() -> HashMap<String, ServerConfig> {
        let mut configs = HashMap::new();

        // Rust Language Server (rust-analyzer)
        configs.insert("rust".to_string(), ServerConfig {
            name: "rust-analyzer".to_string(),
            language: "rust".to_string(),
            command: "rust-analyzer".to_string(),
            args: vec![],
            options: HashMap::new(),
            initialization_options: HashMap::new(),
        });

        // TypeScript/JavaScript Language Server
        configs.insert("typescript".to_string(), ServerConfig {
            name: "typescript-language-server".to_string(),
            language: "typescript".to_string(),
            command: "typescript-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            options: HashMap::new(),
            initialization_options: HashMap::new(),
        });

        configs.insert("javascript".to_string(), ServerConfig {
            name: "typescript-language-server".to_string(),
            language: "javascript".to_string(),
            command: "typescript-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            options: HashMap::new(),
            initialization_options: HashMap::new(),
        });

        // Python Language Server (pylsp)
        configs.insert("python".to_string(), ServerConfig {
            name: "pylsp".to_string(),
            language: "python".to_string(),
            command: "pylsp".to_string(),
            args: vec![],
            options: HashMap::new(),
            initialization_options: HashMap::new(),
        });

        // HTML Language Server
        configs.insert("html".to_string(), ServerConfig {
            name: "html-language-server".to_string(),
            language: "html".to_string(),
            command: "html-languageserver".to_string(),
            args: vec!["--stdio".to_string()],
            options: HashMap::new(),
            initialization_options: HashMap::new(),
        });

        // CSS Language Server
        configs.insert("css".to_string(), ServerConfig {
            name: "css-language-server".to_string(),
            language: "css".to_string(),
            command: "css-languageserver".to_string(),
            args: vec!["--stdio".to_string()],
            options: HashMap::new(),
            initialization_options: HashMap::new(),
        });

        configs
    }

    pub async fn start_server(&mut self, language: &str, workspace_root: PathBuf) -> Result<String> {
        let config = self.server_configs.get(language)
            .ok_or_else(|| anyhow::anyhow!("No server config found for language: {}", language))?;

        let id = Uuid::new_v4().to_string();
        let server = LanguageServer {
            id: id.clone(),
            name: config.name.clone(),
            language: language.to_string(),
            status: ServerStatus::Starting,
            capabilities: ServerCapabilities::default(),
            root_uri: Some(format!("file://{}", workspace_root.to_string_lossy())),
            workspace_folders: vec![WorkspaceFolder {
                uri: format!("file://{}", workspace_root.to_string_lossy()),
                name: workspace_root.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("workspace")
                    .to_string(),
            }],
        };

        self.active_servers.insert(id.clone(), server);
        
        // In a real implementation, this would start the actual LSP server process
        // For now, we'll just mark it as running
        if let Some(server) = self.active_servers.get_mut(&id) {
            server.status = ServerStatus::Running;
        }

        Ok(id)
    }

    pub fn stop_server(&mut self, server_id: &str) -> Result<()> {
        if let Some(server) = self.active_servers.get_mut(server_id) {
            server.status = ServerStatus::Stopped;
        }
        Ok(())
    }

    pub fn get_completions(&self, _server_id: &str, _file_path: &PathBuf, _position: Position) -> Result<Vec<CompletionItem>> {
        // In a real implementation, this would send a completion request to the LSP server
        // For now, return empty completions
        Ok(vec![])
    }

    pub fn get_hover(&self, _server_id: &str, _file_path: &PathBuf, _position: Position) -> Result<Option<Hover>> {
        // In a real implementation, this would send a hover request to the LSP server
        Ok(None)
    }

    pub fn get_definition(&self, _server_id: &str, _file_path: &PathBuf, _position: Position) -> Result<Vec<Location>> {
        // In a real implementation, this would send a definition request to the LSP server
        Ok(vec![])
    }

    pub fn get_references(&self, _server_id: &str, _file_path: &PathBuf, _position: Position) -> Result<Vec<Location>> {
        // In a real implementation, this would send a references request to the LSP server
        Ok(vec![])
    }

    pub fn get_document_symbols(&self, _server_id: &str, _file_path: &PathBuf) -> Result<Vec<DocumentSymbol>> {
        // In a real implementation, this would send a document symbols request to the LSP server
        Ok(vec![])
    }

    pub fn get_workspace_symbols(&self, _server_id: &str, _query: &str) -> Result<Vec<SymbolInformation>> {
        // In a real implementation, this would send a workspace symbols request to the LSP server
        Ok(vec![])
    }

    pub fn get_code_actions(&self, _server_id: &str, _file_path: &PathBuf, _range: Range) -> Result<Vec<CodeAction>> {
        // In a real implementation, this would send a code actions request to the LSP server
        Ok(vec![])
    }

    pub fn format_document(&self, _server_id: &str, _file_path: &PathBuf) -> Result<Vec<TextEdit>> {
        // In a real implementation, this would send a document formatting request to the LSP server
        Ok(vec![])
    }

    pub fn format_range(&self, _server_id: &str, _file_path: &PathBuf, _range: Range) -> Result<Vec<TextEdit>> {
        // In a real implementation, this would send a range formatting request to the LSP server
        Ok(vec![])
    }

    pub fn rename_symbol(&self, _server_id: &str, _file_path: &PathBuf, _position: Position, _new_name: &str) -> Result<WorkspaceEdit> {
        // In a real implementation, this would send a rename request to the LSP server
        Ok(WorkspaceEdit {
            changes: None,
            document_changes: None,
            change_annotations: None,
        })
    }

    pub fn get_servers_for_language(&self, language: &str) -> Vec<&LanguageServer> {
        self.active_servers
            .values()
            .filter(|server| server.language == language)
            .collect()
    }

    pub fn get_active_servers(&self) -> Vec<&LanguageServer> {
        self.active_servers.values().collect()
    }

    pub fn get_server(&self, server_id: &str) -> Option<&LanguageServer> {
        self.active_servers.get(server_id)
    }

    pub fn add_server_config(&mut self, config: ServerConfig) {
        self.server_configs.insert(config.language.clone(), config);
    }

    pub fn remove_server_config(&mut self, language: &str) {
        self.server_configs.remove(language);
    }

    pub fn get_server_configs(&self) -> Vec<&ServerConfig> {
        self.server_configs.values().collect()
    }
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            text_document_sync: None,
            completion_provider: None,
            hover_provider: None,
            signature_help_provider: None,
            definition_provider: None,
            references_provider: None,
            document_highlight_provider: None,
            document_symbol_provider: None,
            workspace_symbol_provider: None,
            code_action_provider: None,
            code_lens_provider: None,
            document_formatting_provider: None,
            document_range_formatting_provider: None,
            document_on_type_formatting_provider: None,
            rename_provider: None,
            document_link_provider: None,
            color_provider: None,
            folding_range_provider: None,
            execute_command_provider: None,
        }
    }
}