/*!
 * Configuration Module
 * 
 * Manages application settings, user preferences, and environment configuration.
 */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use dirs::config_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub editor: EditorConfig,
    pub ai: AIConfig,
    pub terminal: TerminalConfig,
    pub debugger: DebuggerConfig,
    pub git: GitConfig,
    pub ui: UIConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub workspace_path: Option<PathBuf>,
    pub recent_projects: Vec<PathBuf>,
    pub auto_save: bool,
    pub auto_save_interval: u64, // seconds
    pub backup_enabled: bool,
    pub telemetry_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub models_directory: PathBuf,
    pub preferred_models: Vec<String>,
    pub auto_load_model: bool,
    pub context_strategy: ContextStrategy,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextStrategy {
    File,
    Project,
    Selection,
    Smart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub default_shell: String,
    pub font_family: String,
    pub font_size: u32,
    pub cursor_blink: bool,
    pub cursor_style: CursorStyle,
    pub scrollback_lines: u32,
    pub confirm_on_exit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Line,
    Underline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggerConfig {
    pub auto_attach: bool,
    pub break_on_exceptions: bool,
    pub break_on_unhandled_exceptions: bool,
    pub show_return_value: bool,
    pub enable_step_filtering: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub auto_fetch: bool,
    pub fetch_interval: u64, // minutes
    pub show_inline_blame: bool,
    pub confirm_sync: bool,
    pub auto_stage_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: Theme,
    pub color_scheme: ColorScheme,
    pub sidebar_position: SidebarPosition,
    pub panel_position: PanelPosition,
    pub show_welcome_screen: bool,
    pub restore_tabs: bool,
    pub confirm_exit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    Default,
    HighContrast,
    Monokai,
    SolarizedDark,
    SolarizedLight,
    VSCodeDark,
    VSCodeLight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SidebarPosition {
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanelPosition {
    Bottom,
    Right,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            editor: EditorConfig::default(),
            ai: AIConfig::default(),
            terminal: TerminalConfig::default(),
            debugger: DebuggerConfig::default(),
            git: GitConfig::default(),
            ui: UIConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            workspace_path: None,
            recent_projects: Vec::new(),
            auto_save: true,
            auto_save_interval: 30,
            backup_enabled: true,
            telemetry_enabled: false, // Privacy first
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            font_family: "JetBrains Mono".to_string(),
            font_size: 14,
            tab_size: 4,
            insert_spaces: true,
            word_wrap: false,
            line_numbers: true,
            minimap: true,
            bracket_pair_colorization: true,
            format_on_save: true,
            format_on_paste: true,
            auto_closing_brackets: true,
            auto_closing_quotes: true,
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            models_directory: PathBuf::from("./models"),
            preferred_models: vec!["LFM2-VL-1.6B".to_string()],
            auto_load_model: true,
            context_strategy: ContextStrategy::Smart,
            max_context_length: 4096,
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 1024,
            streaming_enabled: true,
            code_completion_enabled: true,
            chat_enabled: true,
            documentation_generation: true,
            test_generation: true,
            refactoring_assistance: true,
        }
    }
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            default_shell: if cfg!(windows) {
                "powershell.exe".to_string()
            } else {
                "/bin/bash".to_string()
            },
            font_family: "Consolas".to_string(),
            font_size: 12,
            cursor_blink: true,
            cursor_style: CursorStyle::Block,
            scrollback_lines: 1000,
            confirm_on_exit: true,
        }
    }
}

impl Default for DebuggerConfig {
    fn default() -> Self {
        Self {
            auto_attach: false,
            break_on_exceptions: true,
            break_on_unhandled_exceptions: true,
            show_return_value: true,
            enable_step_filtering: true,
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            auto_fetch: true,
            fetch_interval: 5,
            show_inline_blame: false,
            confirm_sync: true,
            auto_stage_deleted: false,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            color_scheme: ColorScheme::Default,
            sidebar_position: SidebarPosition::Left,
            panel_position: PanelPosition::Bottom,
            show_welcome_screen: true,
            restore_tabs: true,
            confirm_exit: true,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(config_dir) = config_dir() {
            let rain_chat_dir = config_dir.join("com.rain-chat.rain-chat-v2");
            let config_file = rain_chat_dir.join("config.json");
            
            if config_file.exists() {
                let content = std::fs::read_to_string(config_file)?;
                let config: AppConfig = serde_json::from_str(&content)?;
                return Ok(config);
            }
        }
        
        Ok(Self::default())
    }
    
    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_dir) = config_dir() {
            let rain_chat_dir = config_dir.join("com.rain-chat.rain-chat-v2");
            std::fs::create_dir_all(&rain_chat_dir)?;
            
            let config_file = rain_chat_dir.join("config.json");
            let content = serde_json::to_string_pretty(self)?;
            std::fs::write(config_file, content)?;
        }
        
        Ok(())
    }
}