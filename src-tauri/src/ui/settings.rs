/*!
 * Settings UI Commands
 * 
 * Tauri command handlers for settings management.
 */

use tauri::State;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// use crate::config::AppConfig; // Unused for now
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsInfo {
    pub general: GeneralSettingsInfo,
    pub editor: EditorSettingsInfo,
    pub ai: AISettingsInfo,
    pub terminal: TerminalSettingsInfo,
    pub debugger: DebuggerSettingsInfo,
    pub git: GitSettingsInfo,
    pub ui: UISettingsInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralSettingsInfo {
    pub workspace_path: Option<String>,
    pub recent_projects: Vec<String>,
    pub auto_save: bool,
    pub auto_save_interval: u64,
    pub backup_enabled: bool,
    pub telemetry_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorSettingsInfo {
    pub font_family: String,
    pub font_size: u32,
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub minimap: bool,
    pub bracket_pair_colorization: bool,
    pub format_on_save: bool,
    pub format_on_paste: bool,
    pub auto_closing_brackets: bool,
    pub auto_closing_quotes: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AISettingsInfo {
    pub models_directory: String,
    pub preferred_models: Vec<String>,
    pub auto_load_model: bool,
    pub context_strategy: String,
    pub max_context_length: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u32,
    pub streaming_enabled: bool,
    pub code_completion_enabled: bool,
    pub chat_enabled: bool,
    pub documentation_generation: bool,
    pub test_generation: bool,
    pub refactoring_assistance: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalSettingsInfo {
    pub default_shell: String,
    pub font_family: String,
    pub font_size: u32,
    pub cursor_blink: bool,
    pub cursor_style: String,
    pub scrollback_lines: u32,
    pub confirm_on_exit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebuggerSettingsInfo {
    pub auto_attach: bool,
    pub break_on_exceptions: bool,
    pub break_on_unhandled_exceptions: bool,
    pub show_return_value: bool,
    pub enable_step_filtering: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitSettingsInfo {
    pub auto_fetch: bool,
    pub fetch_interval: u64,
    pub show_inline_blame: bool,
    pub confirm_sync: bool,
    pub auto_stage_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UISettingsInfo {
    pub theme: String,
    pub color_scheme: String,
    pub sidebar_position: String,
    pub panel_position: String,
    pub show_welcome_screen: bool,
    pub restore_tabs: bool,
    pub confirm_exit: bool,
}

#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<SettingsInfo, String> {
    let config = state.config.read().await;
    
    Ok(SettingsInfo {
        general: GeneralSettingsInfo {
            workspace_path: config.general.workspace_path.as_ref().map(|p| p.to_string_lossy().to_string()),
            recent_projects: config.general.recent_projects.iter().map(|p| p.to_string_lossy().to_string()).collect(),
            auto_save: config.general.auto_save,
            auto_save_interval: config.general.auto_save_interval,
            backup_enabled: config.general.backup_enabled,
            telemetry_enabled: config.general.telemetry_enabled,
        },
        editor: EditorSettingsInfo {
            font_family: config.editor.font_family.clone(),
            font_size: config.editor.font_size,
            tab_size: config.editor.tab_size,
            insert_spaces: config.editor.insert_spaces,
            word_wrap: config.editor.word_wrap,
            line_numbers: config.editor.line_numbers,
            minimap: config.editor.minimap,
            bracket_pair_colorization: config.editor.bracket_pair_colorization,
            format_on_save: config.editor.format_on_save,
            format_on_paste: config.editor.format_on_paste,
            auto_closing_brackets: config.editor.auto_closing_brackets,
            auto_closing_quotes: config.editor.auto_closing_quotes,
        },
        ai: AISettingsInfo {
            models_directory: config.ai.models_directory.to_string_lossy().to_string(),
            preferred_models: config.ai.preferred_models.clone(),
            auto_load_model: config.ai.auto_load_model,
            context_strategy: format!("{:?}", config.ai.context_strategy),
            max_context_length: config.ai.max_context_length,
            temperature: config.ai.temperature,
            top_p: config.ai.top_p,
            max_tokens: config.ai.max_tokens,
            streaming_enabled: config.ai.streaming_enabled,
            code_completion_enabled: config.ai.code_completion_enabled,
            chat_enabled: config.ai.chat_enabled,
            documentation_generation: config.ai.documentation_generation,
            test_generation: config.ai.test_generation,
            refactoring_assistance: config.ai.refactoring_assistance,
        },
        terminal: TerminalSettingsInfo {
            default_shell: config.terminal.default_shell.clone(),
            font_family: config.terminal.font_family.clone(),
            font_size: config.terminal.font_size,
            cursor_blink: config.terminal.cursor_blink,
            cursor_style: format!("{:?}", config.terminal.cursor_style),
            scrollback_lines: config.terminal.scrollback_lines,
            confirm_on_exit: config.terminal.confirm_on_exit,
        },
        debugger: DebuggerSettingsInfo {
            auto_attach: config.debugger.auto_attach,
            break_on_exceptions: config.debugger.break_on_exceptions,
            break_on_unhandled_exceptions: config.debugger.break_on_unhandled_exceptions,
            show_return_value: config.debugger.show_return_value,
            enable_step_filtering: config.debugger.enable_step_filtering,
        },
        git: GitSettingsInfo {
            auto_fetch: config.git.auto_fetch,
            fetch_interval: config.git.fetch_interval,
            show_inline_blame: config.git.show_inline_blame,
            confirm_sync: config.git.confirm_sync,
            auto_stage_deleted: config.git.auto_stage_deleted,
        },
        ui: UISettingsInfo {
            theme: format!("{:?}", config.ui.theme),
            color_scheme: format!("{:?}", config.ui.color_scheme),
            sidebar_position: format!("{:?}", config.ui.sidebar_position),
            panel_position: format!("{:?}", config.ui.panel_position),
            show_welcome_screen: config.ui.show_welcome_screen,
            restore_tabs: config.ui.restore_tabs,
            confirm_exit: config.ui.confirm_exit,
        },
    })
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: SettingsInfo,
) -> Result<(), String> {
    let mut config = state.config.write().await;
    
    // Update general settings
    config.general.workspace_path = settings.general.workspace_path.map(|p| std::path::PathBuf::from(p));
    config.general.recent_projects = settings.general.recent_projects.into_iter().map(|p| std::path::PathBuf::from(p)).collect();
    config.general.auto_save = settings.general.auto_save;
    config.general.auto_save_interval = settings.general.auto_save_interval;
    config.general.backup_enabled = settings.general.backup_enabled;
    config.general.telemetry_enabled = settings.general.telemetry_enabled;
    
    // Update editor settings
    config.editor.font_family = settings.editor.font_family;
    config.editor.font_size = settings.editor.font_size;
    config.editor.tab_size = settings.editor.tab_size;
    config.editor.insert_spaces = settings.editor.insert_spaces;
    config.editor.word_wrap = settings.editor.word_wrap;
    config.editor.line_numbers = settings.editor.line_numbers;
    config.editor.minimap = settings.editor.minimap;
    config.editor.bracket_pair_colorization = settings.editor.bracket_pair_colorization;
    config.editor.format_on_save = settings.editor.format_on_save;
    config.editor.format_on_paste = settings.editor.format_on_paste;
    config.editor.auto_closing_brackets = settings.editor.auto_closing_brackets;
    config.editor.auto_closing_quotes = settings.editor.auto_closing_quotes;
    
    // Update AI settings
    config.ai.models_directory = std::path::PathBuf::from(settings.ai.models_directory);
    config.ai.preferred_models = settings.ai.preferred_models;
    config.ai.auto_load_model = settings.ai.auto_load_model;
    config.ai.max_context_length = settings.ai.max_context_length;
    config.ai.temperature = settings.ai.temperature;
    config.ai.top_p = settings.ai.top_p;
    config.ai.max_tokens = settings.ai.max_tokens;
    config.ai.streaming_enabled = settings.ai.streaming_enabled;
    config.ai.code_completion_enabled = settings.ai.code_completion_enabled;
    config.ai.chat_enabled = settings.ai.chat_enabled;
    config.ai.documentation_generation = settings.ai.documentation_generation;
    config.ai.test_generation = settings.ai.test_generation;
    config.ai.refactoring_assistance = settings.ai.refactoring_assistance;
    
    // Update terminal settings
    config.terminal.default_shell = settings.terminal.default_shell;
    config.terminal.font_family = settings.terminal.font_family;
    config.terminal.font_size = settings.terminal.font_size;
    config.terminal.cursor_blink = settings.terminal.cursor_blink;
    config.terminal.scrollback_lines = settings.terminal.scrollback_lines;
    config.terminal.confirm_on_exit = settings.terminal.confirm_on_exit;
    
    // Update debugger settings
    config.debugger.auto_attach = settings.debugger.auto_attach;
    config.debugger.break_on_exceptions = settings.debugger.break_on_exceptions;
    config.debugger.break_on_unhandled_exceptions = settings.debugger.break_on_unhandled_exceptions;
    config.debugger.show_return_value = settings.debugger.show_return_value;
    config.debugger.enable_step_filtering = settings.debugger.enable_step_filtering;
    
    // Update Git settings
    config.git.auto_fetch = settings.git.auto_fetch;
    config.git.fetch_interval = settings.git.fetch_interval;
    config.git.show_inline_blame = settings.git.show_inline_blame;
    config.git.confirm_sync = settings.git.confirm_sync;
    config.git.auto_stage_deleted = settings.git.auto_stage_deleted;
    
    // Update UI settings
    config.ui.show_welcome_screen = settings.ui.show_welcome_screen;
    config.ui.restore_tabs = settings.ui.restore_tabs;
    config.ui.confirm_exit = settings.ui.confirm_exit;
    
    // Save configuration
    config.save().await
        .map_err(|e| format!("Failed to save settings: {}", e))?;
    
    Ok(())
}


