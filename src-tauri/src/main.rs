/*!
 * RAIN.CHAT v2 - Professional Desktop AI IDE
 * 
 * A next-generation desktop IDE for AI development with native performance,
 * offline capabilities, and professional development tools.
 */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;

// Core modules
mod core;
mod ai;
mod ui;
mod config;
mod database;
mod performance;

// Import main components
use core::{
    editor::EditorEngine,
    project::ProjectManager,
    terminal::TerminalManager,
    debugger::DebuggerEngine,
    git::GitManager,
    lsp::LanguageServerManager,
};

use ai::{
    model_manager::ModelManager,
    chat::ChatEngine,
    context::ContextManager,
    assistant::CodeAssistant,
};

use config::AppConfig;
use database::Database;
use performance::PerformanceMonitor;

/// Main application state
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub database: Arc<RwLock<Database>>,
    pub projects: Arc<RwLock<ProjectManager>>,
    pub editor: Arc<RwLock<EditorEngine>>,
    pub terminal: Arc<RwLock<TerminalManager>>,
    pub debugger: Arc<RwLock<DebuggerEngine>>,
    pub git: Arc<RwLock<GitManager>>,
    pub lsp: Arc<RwLock<LanguageServerManager>>,
    pub ai_models: Arc<RwLock<ModelManager>>,
    pub chat: Arc<RwLock<ChatEngine>>,
    pub context: Arc<RwLock<ContextManager>>,
    pub assistant: Arc<RwLock<CodeAssistant>>,
    pub performance: Arc<RwLock<PerformanceMonitor>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            database: Arc::new(RwLock::new(Database::new())),
            projects: Arc::new(RwLock::new(ProjectManager::new())),
            editor: Arc::new(RwLock::new(EditorEngine::new())),
            terminal: Arc::new(RwLock::new(TerminalManager::new())),
            debugger: Arc::new(RwLock::new(DebuggerEngine::new())),
            git: Arc::new(RwLock::new(GitManager::new())),
            lsp: Arc::new(RwLock::new(LanguageServerManager::new())),
            ai_models: Arc::new(RwLock::new(ModelManager::new())),
            chat: Arc::new(RwLock::new(ChatEngine::new())),
            context: Arc::new(RwLock::new(ContextManager::new())),
            assistant: Arc::new(RwLock::new(CodeAssistant::new())),
            performance: Arc::new(RwLock::new(PerformanceMonitor::new())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("rain_chat_v2=debug,tauri=info")
        .init();

    info!("Starting RAIN.CHAT v2 - Professional Desktop AI IDE");

    // Initialize application state
    let app_state = AppState::default();

    // Initialize database
    {
        let mut db = app_state.database.write().await;
        db.initialize().await?;
    }

    // Build and run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .setup(|_app| {
            info!("RAIN.CHAT v2 initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Project management
            ui::project::test_project_connection,
            ui::project::open_project,
            ui::project::create_project,
            ui::project::get_project_structure,
            ui::project::get_recent_projects,
            
            // File operations
            ui::file::open_file,
            ui::file::save_file,
            ui::file::create_file,
            ui::file::delete_file,
            ui::file::list_directory,
            
            // Editor operations
            ui::editor::get_editor_content,
            ui::editor::set_editor_content,
            ui::editor::get_completions,
            
            // AI operations
            ui::ai::load_model,
            ui::ai::unload_model,
            ui::ai::get_available_models,
            ui::ai::chat_with_ai,
            ui::ai::get_code_suggestions,
            ui::ai::analyze_code,
            
            // Universal Model Loader
            ui::ai::discover_models,
            ui::ai::discover_embedding_models,
            ui::ai::load_best_model,
            ui::ai::load_model_by_name,
            ui::ai::generate_response,
            ui::ai::get_model_info,
            ui::ai::clear_conversation,
            ui::ai::reset_context,
            ui::ai::update_generation_settings,
            
            // Embedding Commands (EmbeddingGemma-300m)
            ui::ai::load_embedding_model,
            ui::ai::encode_text,
            ui::ai::compute_similarity,
            
            // Terminal operations
            ui::terminal::create_terminal,
            ui::terminal::execute_command,
            ui::terminal::get_terminal_output,
            
            // Debugger operations
            ui::debugger::start_debug_session,
            ui::debugger::set_breakpoint,
            ui::debugger::step_over,
            ui::debugger::continue_execution,
            
            // Git operations
            ui::git::get_git_status,
            ui::git::stage_changes,
            ui::git::commit_changes,
            ui::git::push_changes,
            
            // Settings
            ui::settings::get_settings,
            ui::settings::update_settings,
            
            // Performance monitoring
            ui::performance::get_performance_metrics,
            ui::performance::get_performance_history,
            ui::performance::get_system_info,
            ui::performance::should_update_performance,
            ui::performance::mark_performance_updated,
        ])
        .run(tauri::generate_context!())
        .expect("error while running RAIN.CHAT v2");

    Ok(())
}