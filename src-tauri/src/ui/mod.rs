/*!
 * UI Module
 * 
 * Contains Tauri command handlers for frontend communication.
 */

pub mod project;
pub mod file;
pub mod editor;
pub mod ai;
pub mod terminal;
pub mod debugger;
pub mod git;
pub mod settings;
pub mod performance;

// Re-export command functions for main.rs
pub use project::{open_project, create_project, get_project_structure, get_recent_projects};
pub use file::{open_file, save_file, create_file, delete_file, list_directory};
pub use editor::{
    get_editor_content, set_editor_content, get_completions
};
pub use ai::{
    load_model, unload_model, get_available_models, chat_with_ai, get_code_suggestions,
    analyze_code, discover_models, discover_embedding_models, load_best_model,
    load_model_by_name, generate_response, get_model_info, clear_conversation,
    reset_context, update_generation_settings, load_embedding_model, encode_text,
    compute_similarity
};
pub use terminal::{
    create_terminal, execute_command, get_terminal_output
};
pub use debugger::{
    start_debug_session, set_breakpoint, step_over, continue_execution
};
pub use git::{
    get_git_status, stage_changes, commit_changes, push_changes
};
pub use settings::{
    get_settings, update_settings
};
pub use performance::{
    get_performance_metrics, get_performance_history, get_system_info,
    should_update_performance, mark_performance_updated
};