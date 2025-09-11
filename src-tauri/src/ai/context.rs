/*!
 * Context Manager Module
 * 
 * Manages context for AI interactions including file content, project structure,
 * and intelligent context selection.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextManager {
    pub context_cache: HashMap<String, ContextItem>,
    pub context_strategies: HashMap<String, ContextStrategy>,
    pub max_context_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    pub id: String,
    pub content: String,
    pub metadata: ContextMetadata,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    pub source_type: ContextSourceType,
    pub file_path: Option<PathBuf>,
    pub language: Option<String>,
    pub size: usize,
    pub relevance_score: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextSourceType {
    File,
    Directory,
    Project,
    Selection,
    Documentation,
    Error,
    Log,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextStrategy {
    pub name: String,
    pub description: String,
    pub max_files: usize,
    pub include_dependencies: bool,
    pub include_tests: bool,
    pub include_documentation: bool,
    pub relevance_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRequest {
    pub query: String,
    pub current_file: Option<PathBuf>,
    pub project_root: Option<PathBuf>,
    pub strategy: String,
    pub max_tokens: usize,
    pub include_selection: bool,
    pub selection_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextResponse {
    pub context_items: Vec<ContextItem>,
    pub total_tokens: usize,
    pub strategy_used: String,
    pub relevance_scores: HashMap<String, f32>,
}

impl Default for ContextManager {
    fn default() -> Self {
        Self {
            context_cache: HashMap::new(),
            context_strategies: Self::get_default_strategies(),
            max_context_length: 4096,
        }
    }
}

impl ContextManager {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_default_strategies() -> HashMap<String, ContextStrategy> {
        let mut strategies = HashMap::new();

        strategies.insert("file".to_string(), ContextStrategy {
            name: "File Context".to_string(),
            description: "Include only the current file content".to_string(),
            max_files: 1,
            include_dependencies: false,
            include_tests: false,
            include_documentation: false,
            relevance_threshold: 0.0,
        });

        strategies.insert("project".to_string(), ContextStrategy {
            name: "Project Context".to_string(),
            description: "Include relevant files from the entire project".to_string(),
            max_files: 20,
            include_dependencies: true,
            include_tests: true,
            include_documentation: true,
            relevance_threshold: 0.3,
        });

        strategies.insert("smart".to_string(), ContextStrategy {
            name: "Smart Context".to_string(),
            description: "Intelligently select the most relevant context".to_string(),
            max_files: 10,
            include_dependencies: true,
            include_tests: false,
            include_documentation: true,
            relevance_threshold: 0.5,
        });

        strategies.insert("selection".to_string(), ContextStrategy {
            name: "Selection Context".to_string(),
            description: "Include only the selected code and minimal surrounding context".to_string(),
            max_files: 3,
            include_dependencies: false,
            include_tests: false,
            include_documentation: false,
            relevance_threshold: 0.0,
        });

        strategies
    }

    pub fn get_context(&mut self, request: ContextRequest) -> Result<ContextResponse> {
        let strategy = self.context_strategies.get(&request.strategy)
            .ok_or_else(|| anyhow::anyhow!("Unknown context strategy: {}", request.strategy))?;

        let mut context_items = Vec::new();
        let mut _total_tokens = 0;
        let mut relevance_scores = HashMap::new();

        // Add selection content if provided
        if request.include_selection {
            if let Some(selection) = request.selection_content {
                let selection_item = ContextItem {
                    id: "selection".to_string(),
                    content: selection.clone(),
                    metadata: ContextMetadata {
                        source_type: ContextSourceType::Selection,
                        file_path: request.current_file.clone(),
                        language: self.detect_language(&request.current_file),
                        size: selection.len(),
                        relevance_score: 1.0,
                        tags: vec!["selection".to_string()],
                    },
                    last_accessed: chrono::Utc::now(),
                    access_count: 1,
                };
                context_items.push(selection_item);
                _total_tokens += self.estimate_tokens(&selection);
            }
        }

        // Add current file content
        if let Some(current_file) = &request.current_file {
            if let Some(file_content) = self.get_file_content(current_file)? {
                let relevance = self.calculate_relevance(&file_content, &request.query);
                relevance_scores.insert(current_file.to_string_lossy().to_string(), relevance);

                if relevance >= strategy.relevance_threshold {
                    let file_item = ContextItem {
                        id: current_file.to_string_lossy().to_string(),
                        content: file_content,
                        metadata: ContextMetadata {
                            source_type: ContextSourceType::File,
                            file_path: Some(current_file.clone()),
                            language: self.detect_language(&Some(current_file.clone())),
                            size: 0, // Will be set below
                            relevance_score: relevance,
                            tags: vec!["current_file".to_string()],
                        },
                        last_accessed: chrono::Utc::now(),
                        access_count: 1,
                    };
                    context_items.push(file_item);
                }
            }
        }

        // Add project context if strategy requires it
        if strategy.max_files > 1 {
            if let Some(project_root) = &request.project_root {
                let project_files = self.get_relevant_project_files(project_root, &request.query, strategy)?;
                
                for (file_path, content) in project_files {
                    let relevance = self.calculate_relevance(&content, &request.query);
                    relevance_scores.insert(file_path.to_string_lossy().to_string(), relevance);

                    if relevance >= strategy.relevance_threshold && context_items.len() < strategy.max_files {
                        let file_item = ContextItem {
                            id: file_path.to_string_lossy().to_string(),
                            content,
                            metadata: ContextMetadata {
                                source_type: ContextSourceType::File,
                                file_path: Some(file_path.clone()),
                                language: self.detect_language(&Some(file_path)),
                                size: 0, // Will be set below
                                relevance_score: relevance,
                                tags: vec!["project_file".to_string()],
                            },
                            last_accessed: chrono::Utc::now(),
                            access_count: 1,
                        };
                        context_items.push(file_item);
                    }
                }
            }
        }

        // Sort by relevance and limit tokens
        context_items.sort_by(|a, b| b.metadata.relevance_score.partial_cmp(&a.metadata.relevance_score).unwrap());
        
        let mut final_items = Vec::new();
        let mut final_tokens = 0;

        for item in context_items {
            let item_tokens = self.estimate_tokens(&item.content);
            if final_tokens + item_tokens <= request.max_tokens {
                final_items.push(item);
                final_tokens += item_tokens;
            } else {
                break;
            }
        }

        Ok(ContextResponse {
            context_items: final_items,
            total_tokens: final_tokens,
            strategy_used: request.strategy,
            relevance_scores,
        })
    }

    fn get_file_content(&self, file_path: &PathBuf) -> Result<Option<String>> {
        if file_path.exists() {
            let content = std::fs::read_to_string(file_path)?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    fn get_relevant_project_files(&self, project_root: &PathBuf, query: &str, strategy: &ContextStrategy) -> Result<Vec<(PathBuf, String)>> {
        let mut relevant_files = Vec::new();
        
        if !project_root.exists() {
            return Ok(relevant_files);
        }

        let mut entries = Vec::new();
        self.collect_files(project_root, &mut entries, strategy)?;

        for file_path in entries {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let relevance = self.calculate_relevance(&content, query);
                if relevance >= strategy.relevance_threshold {
                    relevant_files.push((file_path, content));
                }
            }
        }

        // Sort by relevance and limit
        relevant_files.sort_by(|a, b| {
            let relevance_a = self.calculate_relevance(&a.1, query);
            let relevance_b = self.calculate_relevance(&b.1, query);
            relevance_b.partial_cmp(&relevance_a).unwrap()
        });

        relevant_files.truncate(strategy.max_files);
        Ok(relevant_files)
    }

    fn collect_files(&self, dir: &PathBuf, entries: &mut Vec<PathBuf>, strategy: &ContextStrategy) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // Skip certain file types
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    match ext {
                        "log" | "tmp" | "cache" | "lock" => continue,
                        _ => {}
                    }
                }

                // Skip large files
                if let Ok(metadata) = std::fs::metadata(&path) {
                    if metadata.len() > 1024 * 1024 { // 1MB limit
                        continue;
                    }
                }

                entries.push(path);
            } else if path.is_dir() {
                // Skip certain directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    match dir_name {
                        "node_modules" | "target" | ".git" | "dist" | "build" => continue,
                        _ => {}
                    }
                }

                self.collect_files(&path, entries, strategy)?;
            }
        }

        Ok(())
    }

    fn calculate_relevance(&self, content: &str, query: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let content_lower = content.to_lowercase();
        
        let mut score = 0.0;
        
        // Exact phrase match
        if content_lower.contains(&query_lower) {
            score += 1.0;
        }
        
        // Word matches
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let content_words: Vec<&str> = content_lower.split_whitespace().collect();
        
        for query_word in &query_words {
            let matches = content_words.iter().filter(|&&word| word.contains(query_word)).count();
            score += matches as f32 * 0.1;
        }
        
        // Function/class name matches
        let function_pattern = Regex::new(r"\b(fn|function|def|class|struct|interface)\s+(\w+)").unwrap();
        for cap in function_pattern.captures_iter(content) {
            if let Some(name) = cap.get(2) {
                if query_lower.contains(&name.as_str().to_lowercase()) {
                    score += 0.5;
                }
            }
        }
        
        // Normalize score
        (score / 10.0).min(1.0)
    }

    fn detect_language(&self, file_path: &Option<PathBuf>) -> Option<String> {
        if let Some(path) = file_path {
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                match extension {
                    "rs" => Some("rust".to_string()),
                    "js" => Some("javascript".to_string()),
                    "ts" => Some("typescript".to_string()),
                    "tsx" => Some("typescript".to_string()),
                    "jsx" => Some("javascript".to_string()),
                    "py" => Some("python".to_string()),
                    "html" => Some("html".to_string()),
                    "css" => Some("css".to_string()),
                    "json" => Some("json".to_string()),
                    "toml" => Some("toml".to_string()),
                    "yaml" | "yml" => Some("yaml".to_string()),
                    "md" => Some("markdown".to_string()),
                    _ => Some("text".to_string()),
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Simple token estimation: roughly 4 characters per token
        (text.len() as f32 / 4.0).ceil() as usize
    }

    pub fn cache_context(&mut self, id: String, content: String, metadata: ContextMetadata) {
        let item = ContextItem {
            id: id.clone(),
            content,
            metadata,
            last_accessed: chrono::Utc::now(),
            access_count: 1,
        };
        self.context_cache.insert(id, item);
    }

    pub fn get_cached_context(&mut self, id: &str) -> Option<&ContextItem> {
        if let Some(item) = self.context_cache.get_mut(id) {
            item.last_accessed = chrono::Utc::now();
            item.access_count += 1;
        }
        self.context_cache.get(id)
    }

    pub fn clear_cache(&mut self) {
        self.context_cache.clear();
    }

    pub fn get_cache_stats(&self) -> (usize, usize) {
        let total_items = self.context_cache.len();
        let total_size: usize = self.context_cache.values().map(|item| item.content.len()).sum();
        (total_items, total_size)
    }
}