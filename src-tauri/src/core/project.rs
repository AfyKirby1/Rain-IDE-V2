use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManager {
    pub current_project: Option<Project>,
    pub recent_projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub project_type: ProjectType,
    pub settings: ProjectSettings,
    pub last_opened: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub auto_save: bool,
    pub format_on_save: bool,
    pub git_integration: bool,
    pub lsp_enabled: bool,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            auto_save: true,
            format_on_save: true,
            git_integration: true,
            lsp_enabled: true,
        }
    }
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            current_project: None,
            recent_projects: Vec::new(),
        }
    }

    pub fn open_project(&mut self, path: PathBuf) -> Result<Project, String> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let project_type = self.detect_project_type(&path);

        let project = Project {
            id: Uuid::new_v4(),
            name,
            path,
            project_type,
            settings: ProjectSettings::default(),
            last_opened: chrono::Utc::now(),
        };

        self.add_to_recent(&project);
        self.current_project = Some(project.clone());

        Ok(project)
    }

    pub fn create_project(&mut self, path: PathBuf, project_type: ProjectType) -> Result<Project, String> {
        // Create project directory if it doesn't exist
        if !path.exists() {
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        }

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let project = Project {
            id: Uuid::new_v4(),
            name,
            path,
            project_type,
            settings: ProjectSettings::default(),
            last_opened: chrono::Utc::now(),
        };

        self.add_to_recent(&project);
        self.current_project = Some(project.clone());

        Ok(project)
    }

    fn detect_project_type(&self, path: &PathBuf) -> ProjectType {
        if path.join("Cargo.toml").exists() {
            ProjectType::Rust
        } else if path.join("package.json").exists() {
            if path.join("tsconfig.json").exists() {
                ProjectType::TypeScript
            } else {
                ProjectType::JavaScript
            }
        } else if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() {
            ProjectType::Python
        } else {
            ProjectType::General
        }
    }

    fn add_to_recent(&mut self, project: &Project) {
        // Remove if already exists
        self.recent_projects.retain(|p| p.path != project.path);
        
        // Add to beginning
        self.recent_projects.insert(0, project.clone());
        
        // Keep only 10 most recent
        self.recent_projects.truncate(10);
    }
}