/*!
 * AI UI Commands
 * 
 * Tauri command handlers for AI operations.
 */

use tauri::State;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::ai::chat::{MessageRole, MessageMetadata};
use crate::ai::context::{ContextRequest, ContextResponse};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub format: String,
    pub path: String,
    pub size_mb: u64,
    pub loaded: bool,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: i64,
    pub metadata: MessageMetadata,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub session_id: String,
    pub message: String,
    pub context_type: String,
    pub include_file_context: bool,
    pub include_project_context: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message_id: String,
    pub content: String,
    pub metadata: MessageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeSuggestion {
    pub id: String,
    pub suggestion_type: String,
    pub title: String,
    pub description: String,
    pub code: String,
    pub language: String,
    pub confidence: f32,
    pub position: CodePosition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePosition {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub file_path: String,
    pub issues: Vec<CodeIssue>,
    pub metrics: CodeMetrics,
    pub suggestions: Vec<CodeSuggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeIssue {
    pub id: String,
    pub issue_type: String,
    pub severity: String,
    pub message: String,
    pub position: CodePosition,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub lines_of_code: u32,
    pub cyclomatic_complexity: u32,
    pub maintainability_index: f32,
    pub test_coverage: f32,
    pub documentation_coverage: f32,
}

#[tauri::command]
pub async fn load_model(
    state: State<'_, AppState>,
    model_id: String,
) -> Result<(), String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.load_model_by_id(&model_id).await
        .map_err(|e| format!("Failed to load model: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn unload_model(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.unload_current_model()
        .map_err(|e| format!("Failed to unload model: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn get_available_models(
    state: State<'_, AppState>,
) -> Result<Vec<ModelInfo>, String> {
    let model_manager = state.ai_models.read().await;
    let models = model_manager.get_available_models();
    
    let model_infos: Vec<ModelInfo> = models.into_iter().map(|model| ModelInfo {
        id: model.id.clone(),
        name: model.name.clone(),
        description: model.description.clone(),
        format: format!("{:?}", model.format),
        path: model.path.to_string_lossy().to_string(),
        size_mb: model.size_mb,
        loaded: model.loaded,
        capabilities: model.capabilities.clone(),
    }).collect();
    
    Ok(model_infos)
}

#[tauri::command]
pub async fn chat_with_ai(
    state: State<'_, AppState>,
    request: ChatRequest,
) -> Result<ChatResponse, String> {
    let mut chat_engine = state.chat.write().await;
    let mut context_manager = state.context.write().await;
    
    // Add user message to chat
    let _user_message_id = chat_engine.add_message(
        &request.session_id,
        MessageRole::User,
        request.message.clone(),
        MessageMetadata {
            model_used: None,
            tokens_used: None,
            generation_time_ms: None,
            context_files: vec![],
            code_blocks: vec![],
        },
    ).map_err(|e| format!("Failed to add user message: {}", e))?;
    
    // Get context if requested
    let context = if request.include_file_context || request.include_project_context {
        let context_request = ContextRequest {
            query: request.message.clone(),
            current_file: None, // Would be set based on current editor state
            project_root: None, // Would be set based on current project
            strategy: "smart".to_string(),
            max_tokens: 2048,
            include_selection: false,
            selection_content: None,
        };
        
        context_manager.get_context(context_request)
            .map_err(|e| format!("Failed to get context: {}", e))?
    } else {
        ContextResponse {
            context_items: vec![],
            total_tokens: 0,
            strategy_used: "none".to_string(),
            relevance_scores: std::collections::HashMap::new(),
        }
    };
    
    // Generate AI response (placeholder)
    let ai_response = format!("AI response to: {}", request.message);
    
    // Add AI response to chat
    let ai_message_id = chat_engine.add_message(
        &request.session_id,
        MessageRole::Assistant,
        ai_response.clone(),
        MessageMetadata {
            model_used: Some("mock-model".to_string()),
            tokens_used: Some(100),
            generation_time_ms: Some(500),
            context_files: context.context_items.iter().map(|item| item.id.clone()).collect(),
            code_blocks: vec![],
        },
    ).map_err(|e| format!("Failed to add AI message: {}", e))?;
    
    Ok(ChatResponse {
        message_id: ai_message_id,
        content: ai_response,
        metadata: MessageMetadata {
            model_used: Some("mock-model".to_string()),
            tokens_used: Some(100),
            generation_time_ms: Some(500),
            context_files: context.context_items.iter().map(|item| item.id.clone()).collect(),
            code_blocks: vec![],
        },
    })
}

#[tauri::command]
pub async fn get_code_suggestions(
    state: State<'_, AppState>,
    file_path: String,
    line: u32,
    column: u32,
) -> Result<Vec<CodeSuggestion>, String> {
    let mut assistant = state.assistant.write().await;
    let path = PathBuf::from(&file_path);
    
    let position = crate::ai::assistant::CodePosition { 
        line, 
        column, 
                end_line: Some(line),
        end_column: Some(column), 
        file_path: path.clone() 
    };
    let context = "mock context"; // Would get actual context from editor
    
    let suggestions = assistant.get_completions(&path, position, context).await
        .map_err(|e| format!("Failed to get code suggestions: {}", e))?;
    
    let code_suggestions: Vec<CodeSuggestion> = suggestions.into_iter().map(|suggestion| CodeSuggestion {
        id: suggestion.id,
        suggestion_type: format!("{:?}", suggestion.suggestion_type),
        title: suggestion.title,
        description: suggestion.description,
        code: suggestion.code,
        language: suggestion.language,
        confidence: suggestion.confidence,
        position: CodePosition {
            file_path: suggestion.position.file_path.to_string_lossy().to_string(),
            line: suggestion.position.line,
            column: suggestion.position.column,
            end_line: suggestion.position.end_line,
            end_column: suggestion.position.end_column,
        },
    }).collect();
    
    Ok(code_suggestions)
}

#[tauri::command]
pub async fn analyze_code(
    state: State<'_, AppState>,
    file_path: String,
    content: String,
) -> Result<CodeAnalysis, String> {
    let assistant = state.assistant.read().await;
    let path = PathBuf::from(&file_path);
    
    let analysis = assistant.analyze_code(&path, &content).await
        .map_err(|e| format!("Failed to analyze code: {}", e))?;
    
    let issues: Vec<CodeIssue> = analysis.issues.into_iter().map(|issue| CodeIssue {
        id: issue.id,
        issue_type: format!("{:?}", issue.issue_type),
        severity: format!("{:?}", issue.severity),
        message: issue.message,
        position: CodePosition {
            file_path: issue.position.file_path.to_string_lossy().to_string(),
            line: issue.position.line,
            column: issue.position.column,
            end_line: issue.position.end_line,
            end_column: issue.position.end_column,
        },
        fix_suggestion: issue.fix_suggestion,
    }).collect();
    
    let suggestions: Vec<CodeSuggestion> = analysis.suggestions.into_iter().map(|suggestion| CodeSuggestion {
        id: suggestion.id,
        suggestion_type: format!("{:?}", suggestion.suggestion_type),
        title: suggestion.title,
        description: suggestion.description,
        code: suggestion.code,
        language: suggestion.language,
        confidence: suggestion.confidence,
        position: CodePosition {
            file_path: suggestion.position.file_path.to_string_lossy().to_string(),
            line: suggestion.position.line,
            column: suggestion.position.column,
            end_line: suggestion.position.end_line,
            end_column: suggestion.position.end_column,
        },
    }).collect();
    
    Ok(CodeAnalysis {
        file_path: analysis.file_path.to_string_lossy().to_string(),
        issues,
        metrics: CodeMetrics {
            lines_of_code: analysis.metrics.lines_of_code,
            cyclomatic_complexity: analysis.metrics.cyclomatic_complexity,
            maintainability_index: analysis.metrics.maintainability_index,
            test_coverage: analysis.metrics.test_coverage,
            documentation_coverage: analysis.metrics.documentation_coverage,
        },
        suggestions,
    })
}




// New Universal Model Loader commands

#[tauri::command]
pub async fn discover_models(
    state: State<'_, AppState>,
) -> Result<Vec<ModelInfo>, String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.discover_models().await
        .map_err(|e| format!("Failed to discover models: {}", e))?;
    
    // Return the discovered models
    let models = model_manager.get_available_models();
    let model_infos: Vec<ModelInfo> = models.into_iter().map(|model| ModelInfo {
        id: model.id.clone(),
        name: model.name.clone(),
        description: model.description.clone(),
        format: format!("{:?}", model.format),
        path: model.path.to_string_lossy().to_string(),
        size_mb: model.size_mb,
        loaded: model.loaded,
        capabilities: model.capabilities.clone(),
    }).collect();
    
    Ok(model_infos)
}

#[tauri::command]
pub async fn discover_embedding_models(
    state: State<'_, AppState>,
) -> Result<Vec<ModelInfo>, String> {
    let embedding_models = {
        let model_manager = state.ai_models.read().await;
        model_manager.discover_embedding_models().await
            .map_err(|e| format!("Failed to discover embedding models: {}", e))?
    };
    
    let model_infos: Vec<ModelInfo> = embedding_models.into_iter().map(|model| ModelInfo {
        id: model.id.clone(),
        name: model.name.clone(),
        description: model.description.clone(),
        format: format!("{:?}", model.format),
        path: model.path.to_string_lossy().to_string(),
        size_mb: model.size_mb,
        loaded: model.loaded,
        capabilities: model.capabilities.clone(),
    }).collect();
    
    Ok(model_infos)
}

#[tauri::command]
pub async fn load_best_model(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.load_best_model().await
        .map_err(|e| format!("Failed to load best model: {}", e))
}

#[tauri::command]
pub async fn load_model_by_name(
    state: State<'_, AppState>,
    model_name: String,
) -> Result<bool, String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.load_model_by_name(&model_name).await
        .map_err(|e| format!("Failed to load model by name: {}", e))
}

#[tauri::command]
pub async fn generate_response(
    state: State<'_, AppState>,
    message: String,
) -> Result<String, String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.generate_response(&message).await
        .map_err(|e| format!("Failed to generate response: {}", e))
}

#[tauri::command]
pub async fn get_model_info(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let model_manager = state.ai_models.read().await;
    Ok(model_manager.get_model_info())
}

#[tauri::command]
pub async fn clear_conversation(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.clear_conversation();
    Ok(())
}

#[tauri::command]
pub async fn reset_context(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut model_manager = state.ai_models.write().await;
    model_manager.reset_context();
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationSettings {
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
}

#[tauri::command]
pub async fn update_generation_settings(
    state: State<'_, AppState>,
    settings: GenerationSettings,
) -> Result<(), String> {
    let mut model_manager = state.ai_models.write().await;
    let params = crate::ai::GenerationParams {
        temperature: settings.temperature,
        top_p: settings.top_p,
        top_k: settings.top_k,
        max_tokens: settings.max_tokens,
        stop_sequences: settings.stop_sequences,
    };
    model_manager.update_generation_settings(params);
    Ok(())
}

// Embedding commands for EmbeddingGemma-300m
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingRequest {
    pub text: String,
    pub task_type: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingResponse {
    pub embeddings: Vec<f32>,
    pub dimension: usize,
    pub model_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SimilarityRequest {
    pub query: String,
    pub documents: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SimilarityResponse {
    pub similarities: Vec<f32>,
    pub ranked_documents: Vec<(usize, f32)>,
}

#[tauri::command]
pub async fn load_embedding_model(
    state: State<'_, AppState>,
    model_name: String,
) -> Result<(), String> {
    // TODO: Implement actual embedding model loading
    println!("📥 Loading embedding model: {}", model_name);
    info!("📥 Loading embedding model: {}", model_name);
    Ok(())
}

#[tauri::command]
pub async fn encode_text(
    state: State<'_, AppState>,
    request: EmbeddingRequest,
) -> Result<EmbeddingResponse, String> {
    // TODO: Implement actual embedding generation
    // For now, return a mock embedding based on the text
    let mock_embeddings = generate_mock_embedding(&request.text, &request.task_type);
    
    info!("🔤 Encoded text with task type '{}': {} chars", 
          request.task_type, request.text.len());
    
    Ok(EmbeddingResponse {
        embeddings: mock_embeddings,
        dimension: 768,
        model_name: "google/embeddinggemma-300m".to_string(),
    })
}

#[tauri::command]
pub async fn compute_similarity(
    state: State<'_, AppState>,
    request: SimilarityRequest,
) -> Result<SimilarityResponse, String> {
    // TODO: Implement actual similarity computation
    // For now, generate mock similarities
    let similarities = generate_mock_similarities(&request.query, &request.documents);
    
    // Rank documents by similarity
    let mut ranked: Vec<(usize, f32)> = similarities.iter().enumerate().map(|(i, &sim)| (i, sim)).collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    info!("🔍 Computed similarities for {} documents", request.documents.len());
    
    Ok(SimilarityResponse {
        similarities,
        ranked_documents: ranked,
    })
}

// Helper function to generate mock embeddings
fn generate_mock_embedding(text: &str, task_type: &str) -> Vec<f32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Create a deterministic "embedding" based on text content
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    task_type.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Generate 768-dimensional vector based on hash
    let mut embedding = Vec::with_capacity(768);
    for i in 0..768 {
        let seed = hash.wrapping_add(i as u64);
        let value = ((seed % 10000) as f32 / 10000.0 - 0.5) * 2.0; // Normalize to [-1, 1]
        embedding.push(value);
    }
    
    // Normalize the vector
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for val in &mut embedding {
            *val /= norm;
        }
    }
    
    embedding
}

// Helper function to generate mock similarities
fn generate_mock_similarities(query: &str, documents: &[String]) -> Vec<f32> {
    documents.iter().enumerate().map(|(i, doc)| {
        // Simple similarity based on text length and position
        let length_sim = 1.0 - (query.len() as f32 - doc.len() as f32).abs() / (query.len() as f32 + doc.len() as f32);
        let position_sim = 1.0 - (i as f32 * 0.1);
        (length_sim + position_sim) / 2.0
    }).collect()
}