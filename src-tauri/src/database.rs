/*!
 * Database Module
 * 
 * Manages persistent storage for RAIN.CHAT v2 using SQLite.
 * Stores project metadata, chat history, settings, and user data.
 */

use sqlx::{SqlitePool, Row};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;
use anyhow::Result;

pub struct Database {
    pool: Option<SqlitePool>,
    db_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRecord {
    pub id: String,
    pub name: String,
    pub path: String,
    pub project_type: String,
    pub language: String,
    pub created_at: i64,
    pub last_opened: i64,
    pub settings: String, // JSON
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRecord {
    pub id: String,
    pub session_id: String,
    pub role: String, // user, assistant, system
    pub content: String,
    pub timestamp: i64,
    pub model_used: Option<String>,
    pub context_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub id: String,
    pub project_id: String,
    pub path: String,
    pub language: String,
    pub size: i64,
    pub last_modified: i64,
    pub content_hash: String,
}

impl Database {
    pub fn new() -> Self {
        // Try multiple fallback locations for the database
        let db_path = Self::get_database_path();

        Self {
            pool: None,
            db_path,
        }
    }

    fn get_database_path() -> PathBuf {
        // For now, use current directory to avoid permission issues
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let db_path = current_dir.join("rain_chat_v2.db");
        
        eprintln!("Database path chosen: {:?}", db_path);
        eprintln!("Current working directory: {:?}", current_dir);
        
        // Test if we can write to this location
        if let Ok(_test_file) = std::fs::File::create(current_dir.join("test_write.tmp")) {
            std::fs::remove_file(current_dir.join("test_write.tmp")).ok();
            eprintln!("Write test successful");
        } else {
            eprintln!("Write test failed in current directory");
        }
        
        db_path
    }

    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing database at: {:?}", self.db_path);
        
        // Ensure parent directory exists
        if let Some(parent) = self.db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                anyhow::anyhow!("Failed to create database directory {:?}: {}", parent, e)
            })?;
        }
        
        let database_url = format!("sqlite:{}", self.db_path.display());
        eprintln!("Connecting to database URL: {}", database_url);
        
        // Try to connect with more specific error handling
        let pool = match SqlitePool::connect(&database_url).await {
            Ok(pool) => {
                eprintln!("Database connection successful");
                pool
            }
            Err(e) => {
                eprintln!("Database connection failed: {}", e);
                
                // Try creating an empty file first
                if let Err(file_err) = std::fs::File::create(&self.db_path) {
                    eprintln!("Failed to create database file: {}", file_err);
                }
                
                // Try connecting again
                SqlitePool::connect(&database_url).await
                    .map_err(|e2| anyhow::anyhow!("Failed to connect to database after file creation: {}", e2))?
            }
        };
        
        // Create tables
        self.create_tables(&pool).await?;
        
        self.pool = Some(pool);
        info!("Database initialized successfully");
        
        Ok(())
    }

    async fn create_tables(&self, pool: &SqlitePool) -> Result<()> {
        // Projects table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                project_type TEXT NOT NULL,
                language TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                last_opened INTEGER NOT NULL,
                settings TEXT NOT NULL DEFAULT '{}'
            )"
        )
        .execute(pool)
        .await?;

        // Chat history table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS chat_history (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                model_used TEXT,
                context_type TEXT
            )"
        )
        .execute(pool)
        .await?;

        // Files table for project file metadata
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS files (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                path TEXT NOT NULL,
                language TEXT NOT NULL,
                size INTEGER NOT NULL,
                last_modified INTEGER NOT NULL,
                content_hash TEXT NOT NULL,
                FOREIGN KEY(project_id) REFERENCES projects(id)
            )"
        )
        .execute(pool)
        .await?;

        // Settings table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            )"
        )
        .execute(pool)
        .await?;

        // Model cache table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS model_cache (
                model_path TEXT PRIMARY KEY,
                model_type TEXT NOT NULL,
                size_mb REAL NOT NULL,
                last_used INTEGER NOT NULL,
                load_time_ms INTEGER NOT NULL,
                capabilities TEXT NOT NULL
            )"
        )
        .execute(pool)
        .await?;

        // Create indexes for better performance
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_projects_last_opened ON projects(last_opened DESC)"
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_chat_session ON chat_history(session_id, timestamp)"
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_files_project ON files(project_id)"
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // Project operations
    pub async fn insert_project(&self, project: &ProjectRecord) -> Result<()> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                "INSERT OR REPLACE INTO projects 
                (id, name, path, project_type, language, created_at, last_opened, settings) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
            )
            .bind(&project.id)
            .bind(&project.name)
            .bind(&project.path)
            .bind(&project.project_type)
            .bind(&project.language)
            .bind(project.created_at)
            .bind(project.last_opened)
            .bind(&project.settings)
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn get_recent_projects(&self, limit: usize) -> Result<Vec<ProjectRecord>> {
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                "SELECT id, name, path, project_type, language, created_at, last_opened, settings 
                 FROM projects ORDER BY last_opened DESC LIMIT ?1"
            )
            .bind(limit as i64)
            .fetch_all(pool)
            .await?;

            let projects = rows.iter().map(|row| ProjectRecord {
                id: row.get("id"),
                name: row.get("name"),
                path: row.get("path"),
                project_type: row.get("project_type"),
                language: row.get("language"),
                created_at: row.get("created_at"),
                last_opened: row.get("last_opened"),
                settings: row.get("settings"),
            }).collect();

            return Ok(projects);
        }
        Ok(Vec::new())
    }

    // Chat operations
    pub async fn insert_chat_message(&self, message: &ChatRecord) -> Result<()> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                "INSERT INTO chat_history 
                (id, session_id, role, content, timestamp, model_used, context_type) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            )
            .bind(&message.id)
            .bind(&message.session_id)
            .bind(&message.role)
            .bind(&message.content)
            .bind(message.timestamp)
            .bind(&message.model_used)
            .bind(&message.context_type)
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn get_chat_history(&self, session_id: &str, limit: usize) -> Result<Vec<ChatRecord>> {
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                "SELECT id, session_id, role, content, timestamp, model_used, context_type 
                 FROM chat_history WHERE session_id = ?1 
                 ORDER BY timestamp DESC LIMIT ?2"
            )
            .bind(session_id)
            .bind(limit as i64)
            .fetch_all(pool)
            .await?;

            let mut messages: Vec<ChatRecord> = rows.iter().map(|row| ChatRecord {
                id: row.get("id"),
                session_id: row.get("session_id"),
                role: row.get("role"),
                content: row.get("content"),
                timestamp: row.get("timestamp"),
                model_used: row.get("model_used"),
                context_type: row.get("context_type"),
            }).collect();

            messages.reverse(); // Return in chronological order
            return Ok(messages);
        }
        Ok(Vec::new())
    }

    // Settings operations
    pub async fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        if let Some(pool) = &self.pool {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            sqlx::query(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)"
            )
            .bind(key)
            .bind(value)
            .bind(timestamp)
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        if let Some(pool) = &self.pool {
            let row = sqlx::query("SELECT value FROM settings WHERE key = ?1")
                .bind(key)
                .fetch_optional(pool)
                .await?;
            
            if let Some(row) = row {
                return Ok(Some(row.get("value")));
            }
        }
        Ok(None)
    }

    pub async fn cleanup_old_data(&self, days: u64) -> Result<()> {
        if let Some(pool) = &self.pool {
            let cutoff = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64 - (days * 24 * 60 * 60) as i64;

            // Clean old chat messages
            sqlx::query("DELETE FROM chat_history WHERE timestamp < ?1")
                .bind(cutoff)
                .execute(pool)
                .await?;

            info!("Cleaned up old data older than {} days", days);
        }
        Ok(())
    }
}