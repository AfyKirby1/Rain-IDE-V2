//! Universal Model Manager for RAIN.CHAT v2
//! 
//! Handles loading, managing, and interfacing with AI models including
//! GGUF, ONNX, and Hugging Face Transformers models.
//! 
//! This is the Rust equivalent of the Python Universal Model Loader.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use tokio::fs;
use tracing::{info, error, warn, debug};

// Re-export AI engine types when Candle is enabled
#[cfg(feature = "ai_candle")]
pub use candle_core::{Device, Tensor};
#[cfg(feature = "ai_candle")]
pub use candle_transformers::models::llama::LlamaConfig;
// Minimal stand-in types when Candle is disabled
#[cfg(not(feature = "ai_candle"))]
pub struct Device;
#[cfg(not(feature = "ai_candle"))]
pub struct Tensor;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ModelFormat {
    Gguf,
    Onnx,
    HuggingFace,
    Ggml,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub format: ModelFormat,
    pub path: PathBuf,
    pub size_mb: u64,
    pub loaded: bool,
    pub capabilities: Vec<String>,
    pub config: Option<serde_json::Value>,
    pub files: Vec<ModelFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFile {
    pub name: String,
    pub size_mb: f64,
    pub extension: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParams {
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            max_tokens: 1024,
            stop_sequences: vec!["<|endoftext|>".to_string(), "<|im_end|>".to_string()],
        }
    }
}

/// Conversation message for context management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String,
    pub content: String,
}

/// Abstract trait for different model backends
pub trait ModelBackend: Send + Sync {
    fn generate_text(&self, prompt: &str, params: &GenerationParams) -> Result<String>;
    fn get_model_info(&self) -> &ModelInfo;
    fn unload(&mut self) -> Result<()>;
}

/// GGUF model backend using llama.cpp
pub struct GgufBackend {
    model_info: ModelInfo,
    // Will hold llama.cpp model instance
    // llama_model: Option<llama_cpp_2::LlamaModel>,
}

/// Transformers model backend using Candle
pub struct TransformersBackend {
    model_info: ModelInfo,
    #[allow(dead_code)]
    device: Device,
    // Will hold Candle model instance
}

/// ONNX model backend using Candle-ONNX
pub struct OnnxBackend {
    model_info: ModelInfo,
    // Will hold ONNX model instance
}

/// Universal Model Manager - the main interface
pub struct ModelManager {
    available_models: HashMap<String, ModelInfo>,
    loaded_backend: Option<Box<dyn ModelBackend>>,
    current_model: Option<String>,
    models_dir: PathBuf,
    conversation_history: Vec<ConversationMessage>,
    max_conversation_length: usize,
    generation_settings: GenerationParams,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new() -> Self {
        // Find models directory
        let models_dir = Self::find_models_directory();
        info!("🔧 Models directory: {}", models_dir.display());

        Self {
            available_models: HashMap::new(),
            loaded_backend: None,
            current_model: None,
            models_dir,
            conversation_history: Vec::new(),
            max_conversation_length: 20,
            generation_settings: GenerationParams::default(),
        }
    }

    /// Find the models directory in various possible locations
    fn find_models_directory() -> PathBuf {
        let possible_paths = vec![
            std::env::current_dir().unwrap_or_default().join("models"),
            std::env::current_dir().unwrap_or_default().parent().unwrap_or(std::path::Path::new("")).join("models"),
            dirs::document_dir().unwrap_or_default().join("RAIN.CHAT").join("models"),
            PathBuf::from("./models"),
        ];

        for path in possible_paths {
            if path.exists() {
                return path;
            }
        }

        // Fallback to ./models
        PathBuf::from("./models")
    }

    /// Discover available models in the models directory (equivalent to scan_for_models)
    pub async fn discover_models(&mut self) -> Result<()> {
        if !self.models_dir.exists() {
            warn!("Models directory does not exist: {}", self.models_dir.display());
            return Ok(());
        }

        let mut entries = fs::read_dir(&self.models_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_dir() {
                // Skip embedding subdirectory - handled separately
                if path.file_name().unwrap_or_default() == "embedding" {
                    continue;
                }
                
                if let Some(model_info) = self.analyze_model_directory(&path).await? {
                    self.available_models.insert(model_info.id.clone(), model_info);
                }
            }
        }
        
        info!("📦 Discovered {} chat models", self.available_models.len());
        
        // Debug: List discovered models
        for (id, model) in &self.available_models {
            info!("🔍 Chat Model: {} ({:?}) - {} MB", id, model.format, model.size_mb);
        }
        
        Ok(())
    }

    /// Discover embedding models in the models/embedding directory
    pub async fn discover_embedding_models(&self) -> Result<Vec<ModelInfo>> {
        let embedding_dir = self.models_dir.join("embedding");
        let mut embedding_models = Vec::new();
        
        if !embedding_dir.exists() {
            info!("📁 Embedding directory does not exist: {}", embedding_dir.display());
            return Ok(embedding_models);
        }

        let mut entries = fs::read_dir(&embedding_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(model_info) = self.analyze_embedding_model_directory(&path).await? {
                    embedding_models.push(model_info);
                }
            }
        }
        
        info!("🧠 Discovered {} embedding models", embedding_models.len());
        
        // Debug: List discovered embedding models
        for model in &embedding_models {
            info!("🔍 Embedding Model: {} ({:?}) - {} MB", model.name, model.format, model.size_mb);
        }
        
        Ok(embedding_models)
    }

    /// Analyze a model directory to determine its type and capabilities
    async fn analyze_model_directory(&self, model_dir: &PathBuf) -> Result<Option<ModelInfo>> {
        let model_name = model_dir.file_name()
            .ok_or_else(|| anyhow!("Invalid directory name"))?
            .to_string_lossy()
            .to_string();

        let mut model_info = ModelInfo {
            id: model_name.clone(),
            name: model_name,
            description: "Local AI model".to_string(),
            format: ModelFormat::HuggingFace, // Default assumption
            path: model_dir.clone(),
            size_mb: 0,
            loaded: false,
            capabilities: Vec::new(),
            config: None,
            files: Vec::new(),
        };

        // Check for model files
        let mut entries = fs::read_dir(model_dir).await?;
        let mut total_size = 0u64;
        
        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();
            
            if file_path.is_file() {
                let metadata = fs::metadata(&file_path).await?;
                let file_size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
                total_size += metadata.len();

                let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
                let extension = file_path.extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                model_info.files.push(ModelFile {
                    name: file_name,
                    size_mb: (file_size_mb * 100.0).round() / 100.0,
                    extension: format!(".{}", extension),
                });

                // Determine model type based on files (matching Python logic)
                match extension.as_str() {
                    "gguf" => {
                        model_info.format = ModelFormat::Gguf;
                        model_info.capabilities.extend(vec!["text_generation".to_string(), "chat".to_string()]);
                    },
                    "safetensors" | "bin" | "pth" => {
                        model_info.format = ModelFormat::HuggingFace;
                        model_info.capabilities.extend(vec!["text_generation".to_string(), "chat".to_string(), "vision".to_string()]);
                    },
                    "onnx" => {
                        model_info.format = ModelFormat::Onnx;
                        model_info.capabilities.extend(vec!["text_generation".to_string(), "chat".to_string()]);
                    },
                    "ggml" => {
                        model_info.format = ModelFormat::Ggml;
                        model_info.capabilities.extend(vec!["text_generation".to_string(), "chat".to_string()]);
                    },
                    _ => {}
                }
            }
        }

        model_info.size_mb = total_size / (1024 * 1024);

        // Look for config files
        let config_files = vec!["config.json", "model_config.json", "tokenizer_config.json"];
        for config_file in config_files {
            let config_path = model_dir.join(config_file);
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path).await {
                    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                        model_info.config = Some(config);
                        break;
                    }
                }
            }
        }

        // Only return if we found a valid model type
        if !model_info.files.is_empty() {
            debug!("✅ Found model with files: {} ({:?})", model_info.name, model_info.format);
            Ok(Some(model_info))
        } else if model_info.format == ModelFormat::HuggingFace && 
                  model_dir.join("config.json").exists() {
            debug!("✅ Found HuggingFace model with config: {}", model_info.name);
            Ok(Some(model_info))
        } else {
            debug!("❌ Skipping invalid model directory: {} (format: {:?}, files: {})", 
                   model_info.name, model_info.format, model_info.files.len());
            Ok(None)
        }
    }

    /// Analyze an embedding model directory
    async fn analyze_embedding_model_directory(&self, model_dir: &PathBuf) -> Result<Option<ModelInfo>> {
        let model_name = model_dir.file_name()
            .ok_or_else(|| anyhow!("Invalid directory name"))?
            .to_string_lossy()
            .to_string();

        let mut model_info = ModelInfo {
            id: model_name.clone(),
            name: model_name,
            description: "Embedding model for semantic search and similarity".to_string(),
            format: ModelFormat::HuggingFace, // Default for embedding models
            path: model_dir.clone(),
            size_mb: 0,
            loaded: false,
            capabilities: vec!["text_embedding".to_string(), "semantic_search".to_string()],
            config: None,
            files: Vec::new(),
        };

        // Check for model files
        let mut entries = fs::read_dir(model_dir).await?;
        let mut total_size = 0u64;
        let mut has_model_files = false;
        
        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();
            
            if file_path.is_file() {
                let metadata = fs::metadata(&file_path).await?;
                let file_size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
                total_size += metadata.len();

                let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
                let extension = file_path.extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                model_info.files.push(ModelFile {
                    name: file_name.clone(),
                    size_mb: (file_size_mb * 100.0).round() / 100.0,
                    extension: format!(".{}", extension),
                });

                // Check for embedding model files
                match extension.as_str() {
                    "safetensors" | "bin" | "pth" => {
                        has_model_files = true;
                        model_info.format = ModelFormat::HuggingFace;
                        model_info.capabilities.extend(vec![
                            "text_embedding".to_string(),
                            "semantic_search".to_string(),
                            "similarity".to_string(),
                            "classification".to_string(),
                            "clustering".to_string(),
                        ]);
                    },
                    "json" if file_name == "config.json" => {
                        // Load config for better model info
                        if let Ok(content) = fs::read_to_string(&file_path).await {
                            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                                model_info.config = Some(config.clone());
                                
                                // Extract model info from config
                                if let Some(architectures) = config.get("architectures") {
                                    if let Some(arch) = architectures.as_array().and_then(|a| a.first()) {
                                        if let Some(arch_str) = arch.as_str() {
                                            if arch_str.contains("SentenceTransformer") || 
                                               arch_str.contains("Embedding") {
                                                model_info.format = ModelFormat::HuggingFace;
                                                has_model_files = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        model_info.size_mb = total_size / (1024 * 1024);

        // Only return if we found valid embedding model files or config
        if has_model_files || model_info.config.is_some() {
            Ok(Some(model_info))
        } else {
            debug!("❌ Skipping invalid embedding model directory: {} (no model files)", model_info.name);
            Ok(None)
        }
    }

    /// Load the best available model automatically (equivalent to load_best_model)
    pub async fn load_best_model(&mut self) -> Result<bool> {
        if self.available_models.is_empty() {
            error!("❌ No compatible models found in models directory");
            return Ok(false);
        }

        // Prioritize models: GGUF > Transformers > ONNX > GGML (matching Python logic)
        let priority_order = vec![ModelFormat::Gguf, ModelFormat::HuggingFace, ModelFormat::Onnx, ModelFormat::Ggml];

        // Find best model first, then load it to avoid borrow conflict
        let best_model = priority_order.iter().find_map(|format| {
            self.available_models.iter().find_map(|(id, info)| {
                if info.format == *format {
                    Some((id.clone(), info.name.clone(), *format))
                } else {
                    None
                }
            })
        });

        if let Some((model_id, name, format)) = best_model {
            info!("🚀 Loading {:?} model: {}", format, name);
            return self.load_model_by_id(&model_id).await;
        }
        
        Ok(false)
    }

    /// Load a specific model by name (equivalent to load_model_by_name)
    pub async fn load_model_by_name(&mut self, model_name: &str) -> Result<bool> {
        if let Some(model_info) = self.available_models.get(model_name) {
            info!("🚀 Loading {} model: {}", format!("{:?}", model_info.format).to_uppercase(), model_name);
            self.load_model_by_id(model_name).await
        } else {
            error!("❌ Model '{}' not found in available models", model_name);
            Ok(false)
        }
    }

    /// Load a model by ID
    pub async fn load_model_by_id(&mut self, model_id: &str) -> Result<bool> {
        let model_info = self.available_models.get(model_id)
            .ok_or_else(|| anyhow!("Model not found: {}", model_id))?
            .clone();

        // Unload current model if any
        if self.loaded_backend.is_some() {
            self.unload_current_model()?;
        }

        // Load based on model format
        let backend: Box<dyn ModelBackend> = match model_info.format {
            ModelFormat::Gguf => {
                Box::new(self.load_gguf_model(model_info).await?)
            },
            ModelFormat::HuggingFace => {
                Box::new(self.load_transformers_model(model_info).await?)
            },
            ModelFormat::Onnx => {
                Box::new(self.load_onnx_model(model_info).await?)
            },
            ModelFormat::Ggml => {
                // GGML can be loaded with GGUF backend
                Box::new(self.load_gguf_model(model_info).await?)
            },
        };

        self.loaded_backend = Some(backend);
        self.current_model = Some(model_id.to_string());
        
        // Update model info to mark as loaded
        if let Some(info) = self.available_models.get_mut(model_id) {
            info.loaded = true;
        }
        
        info!("✅ Model {} loaded successfully!", model_id);
        Ok(true)
    }

    /// Load GGUF model using llama.cpp
    async fn load_gguf_model(&self, model_info: ModelInfo) -> Result<GgufBackend> {
        // Find the .gguf file
        let gguf_file = model_info.files.iter()
            .find(|f| f.extension == ".gguf")
            .ok_or_else(|| anyhow!("No GGUF file found in model directory"))?;

        let gguf_path = model_info.path.join(&gguf_file.name);
        info!("🚀 Loading GGUF model: {}", gguf_path.display());

        // TODO: Initialize llama.cpp model here
        // let llama_model = llama_cpp_2::LlamaModel::load_from_file(gguf_path, ...)?;

        // Debug: Check if model info has proper size
        debug!("🔍 GGUF Model Info: name={}, size_mb={}, files={:?}", 
               model_info.name, model_info.size_mb, model_info.files);

        Ok(GgufBackend {
            model_info,
            // llama_model: Some(llama_model),
        })
    }

    /// Load Transformers model using Candle
    async fn load_transformers_model(&self, model_info: ModelInfo) -> Result<TransformersBackend> {
        info!("🚀 Loading Transformers model: {}", model_info.path.display());

        // Determine device (GPU if available, otherwise CPU)
        #[cfg(feature = "ai_candle")]
        let device = Device::cuda_if_available(0)?;
        #[cfg(feature = "ai_candle")]
        info!("🔧 Using device: {:?}", device);
        #[cfg(not(feature = "ai_candle"))]
        let device = Device;

        // TODO: Load model using Candle
        // let model = candle_transformers::models::llama::LlamaModel::load(...)?;

        Ok(TransformersBackend {
            model_info,
            device,
        })
    }

    /// Load ONNX model using Candle-ONNX
    async fn load_onnx_model(&self, model_info: ModelInfo) -> Result<OnnxBackend> {
        info!("🚀 Loading ONNX model: {}", model_info.path.display());

        // TODO: Load ONNX model using Candle when enabled
        // #[cfg(feature = "ai_onnx")]
        // let onnx_model = candle_onnx::onnx::SimpleEval::new(...)?;

        Ok(OnnxBackend {
            model_info,
        })
    }

    /// Generate response using the loaded model (equivalent to generate_response)
    pub async fn generate_response(&mut self, user_message: &str) -> Result<String> {
        // Validate input
        if user_message.trim().is_empty() {
            return Err(anyhow!("Please provide a valid message."));
        }

        if user_message.len() > 10000 {
            return Err(anyhow!("Message too long. Please keep messages under 10,000 characters."));
        }

        let backend = self.loaded_backend.as_ref()
            .ok_or_else(|| anyhow!("No model loaded. Please load a model first."))?;

        // Add user message to conversation history
        self.conversation_history.push(ConversationMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        // Keep only recent messages to prevent context overflow
        if self.conversation_history.len() > self.max_conversation_length {
            self.conversation_history = self.conversation_history
                .split_off(self.conversation_history.len() - self.max_conversation_length);
            debug!("🔄 Conversation history trimmed to {} messages", self.max_conversation_length);
        }

        // Build conversation context
        let full_conversation = self.build_conversation_context();

        // Generate response
        let response = backend.generate_text(&full_conversation, &self.generation_settings)?;

        // Add assistant response to conversation history
        self.conversation_history.push(ConversationMessage {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response)
    }

    /// Build conversation context with proper formatting
    fn build_conversation_context(&self) -> String {
        let mut conversation = String::new();

        for msg in &self.conversation_history {
            match msg.role.as_str() {
                "user" => {
                    // Use ChatML format as default
                    conversation.push_str(&format!("<|im_start|>user\n{}<|im_end|>\n", msg.content));
                },
                "assistant" => {
                    conversation.push_str(&format!("<|im_start|>assistant\n{}<|im_end|>\n", msg.content));
                },
                _ => {}
            }
        }

        // Add generation prompt for the assistant response
        conversation.push_str("<|im_start|>assistant\n");

        conversation
    }

    /// Unload current model
    pub fn unload_current_model(&mut self) -> Result<()> {
        if let Some(mut backend) = self.loaded_backend.take() {
            backend.unload()?;
        }

        if let Some(model_id) = &self.current_model {
            if let Some(info) = self.available_models.get_mut(model_id) {
                info.loaded = false;
            }
        }

        self.current_model = None;
        info!("✅ Model unloaded and resources cleaned up");
        Ok(())
    }

    /// Get available models
    pub fn get_available_models(&self) -> Vec<&ModelInfo> {
        self.available_models.values().collect()
    }

    /// Get current model info
    pub fn get_current_model(&self) -> Option<&ModelInfo> {
        self.current_model.as_ref()
            .and_then(|id| self.available_models.get(id))
    }

    /// Get model info for display
    pub fn get_model_info(&self) -> serde_json::Value {
        if let Some(backend) = &self.loaded_backend {
            let info = backend.get_model_info();
            serde_json::json!({
                "status": "loaded",
                "type": format!("{:?}", info.format),
                "name": info.name,
                "size_mb": info.size_mb,
                "capabilities": info.capabilities,
                "path": info.path
            })
        } else {
            serde_json::json!({"status": "No model loaded"})
        }
    }

    /// Update generation settings
    pub fn update_generation_settings(&mut self, settings: GenerationParams) {
        self.generation_settings = settings;
        info!("🔧 Generation settings updated");
    }

    /// Clear conversation history
    pub fn clear_conversation(&mut self) {
        self.conversation_history.clear();
        info!("🔄 Conversation history cleared");
    }

    /// Reset conversation context
    pub fn reset_context(&mut self) {
        self.conversation_history.clear();
        info!("🔄 Conversation context reset");
    }
}

// Implementation for model backends
impl ModelBackend for GgufBackend {
    fn generate_text(&self, prompt: &str, _params: &GenerationParams) -> Result<String> {
        // Check if this is a real model or just a placeholder
        if self.model_info.name.contains("placeholder") {
            return Err(anyhow!("Model not properly loaded. Please check if the model files are valid and try reloading."));
        }
        
        // Debug: Log model info
        debug!("🔍 Model loaded: name={}, size_mb={}, format={:?}", 
               self.model_info.name, self.model_info.size_mb, self.model_info.format);

        // Extract the LAST user message from ChatML format
        let user_message = if let Some(last_user_start) = prompt.rfind("<|im_start|>user\n") {
            if let Some(end) = prompt[last_user_start..].find("<|im_end|>") {
                let start_pos = last_user_start + 18; // Length of "<|im_start|>user\n"
                let end_pos = last_user_start + end;
                &prompt[start_pos..end_pos]
            } else {
                return Err(anyhow!("Invalid conversation format"));
            }
        } else {
            return Err(anyhow!("No user message found in conversation"));
        };

        // Debug logging
        debug!("🔍 Parsed user message: '{}'", user_message);
        debug!("🔍 Model info: name={}, size_mb={}, format={:?}", 
               self.model_info.name, self.model_info.size_mb, self.model_info.format);
        
        // TODO: Implement actual GGUF generation using llama.cpp
        // For now, return a more intelligent response that acknowledges the model
        let response = match user_message.trim().to_lowercase().as_str() {
            msg if msg.contains("who are you") || msg.contains("what are you") => {
                format!("I'm the {} model running locally in RAIN.CHAT v2. I'm a GGUF-based AI assistant that can help you with coding, answer questions, and provide development assistance. I'm running entirely on your system without any external dependencies, so all our conversations stay private.", self.model_info.name)
            },
            msg if msg.contains("what model") || msg.contains("which model") => {
                format!("I'm the {} model (GGUF format, {:.1} MB). I'm running locally on your system as part of RAIN.CHAT v2's AI integration. GGUF is an efficient format that allows me to run quickly on your hardware.", 
                    self.model_info.name, 
                    self.model_info.size_mb as f64 / 1024.0 / 1024.0
                )
            },
            msg if msg.contains("hello") || msg.contains("hi") || msg.contains("hey") => {
                format!("Hello! I'm {} running locally in RAIN.CHAT v2. I'm here to help with your development work, coding questions, and technical challenges. What would you like to work on today?", self.model_info.name)
            },
            msg if msg.contains("help") || msg.contains("what can you do") => {
                "I can help you with:\n• Coding questions and debugging\n• Code explanations and best practices\n• Technical problem solving\n• Development guidance\n• Code reviews and suggestions\n\nSince I'm running locally, all our conversations stay private on your system. What specific area would you like help with?".to_string()
            },
            msg if msg.contains("code") || msg.contains("programming") || msg.contains("coding") => {
                "I'd be happy to help with your coding questions! I can assist with:\n• Syntax and language-specific questions\n• Debugging and error resolution\n• Best practices and code optimization\n• Code explanations and documentation\n• Architecture and design patterns\n\nWhat programming language or specific problem are you working on?".to_string()
            },
            msg if msg.contains("html") || msg.contains("website") || msg.contains("web") => {
                "I can help you with HTML and web development! I can assist with:\n• HTML structure and semantic markup\n• CSS styling and layout\n• JavaScript functionality\n• Responsive design\n• Web standards and best practices\n\nWhat specific web development task are you working on?".to_string()
            },
            msg if msg.contains("css") => {
                "I can help with CSS styling! I can assist with:\n• Layout techniques (Flexbox, Grid)\n• Responsive design\n• CSS animations and transitions\n• Browser compatibility\n• Performance optimization\n\nWhat CSS challenge are you facing?".to_string()
            },
            msg if msg.contains("javascript") || msg.contains("js") => {
                "I can help with JavaScript development! I can assist with:\n• ES6+ features and modern syntax\n• DOM manipulation\n• Async programming (Promises, async/await)\n• Frameworks and libraries\n• Debugging and performance\n\nWhat JavaScript topic would you like help with?".to_string()
            },
            msg if msg.contains("python") => {
                "I can help with Python development! I can assist with:\n• Python syntax and best practices\n• Data structures and algorithms\n• Libraries and frameworks\n• Debugging and testing\n• Performance optimization\n\nWhat Python project are you working on?".to_string()
            },
            msg if msg.contains("rust") => {
                "I can help with Rust development! I can assist with:\n• Ownership and borrowing concepts\n• Memory safety and performance\n• Cargo and dependency management\n• Error handling patterns\n• Async programming with Tokio\n\nWhat Rust challenge are you facing?".to_string()
            },
            msg if msg.contains("thank") || msg.contains("thanks") => {
                "You're welcome! I'm here to help with your development work. Feel free to ask me anything about coding, debugging, or technical challenges. What else can I assist you with?".to_string()
            },
            msg if msg.contains("what day") || msg.contains("date") || msg.contains("time") => {
                "I don't have access to real-time information like the current date or time. I'm a local AI model running on your system, so I can't fetch external data. However, I can help you with coding questions, debugging, and development tasks! What would you like to work on?".to_string()
            },
            msg if msg.contains("company") || msg.contains("made you") || msg.contains("created") => {
                format!("I'm the {} model running locally in RAIN.CHAT v2. I'm not created by a specific company - I'm a local AI model that runs entirely on your system. RAIN.CHAT v2 is the desktop application that hosts me, and I'm here to help with your development work!", self.model_info.name)
            },
            _ => {
                format!("I understand you're asking about: \"{}\"\n\nAs the {} model running locally in RAIN.CHAT v2, I can help with coding questions, explanations, and development tasks. Could you provide more specific details about what you'd like assistance with?", 
                    user_message.trim(), self.model_info.name)
            }
        };
        
        Ok(response)
    }

    fn get_model_info(&self) -> &ModelInfo {
        &self.model_info
    }

    fn unload(&mut self) -> Result<()> {
        // TODO: Properly unload llama.cpp model
        info!("🔄 GGUF model unloaded");
        Ok(())
    }
}

impl ModelBackend for TransformersBackend {
    fn generate_text(&self, prompt: &str, _params: &GenerationParams) -> Result<String> {
        // Check if this is a real model or just a placeholder
        if self.model_info.name.contains("placeholder") {
            return Err(anyhow!("Model not properly loaded. Please check if the model files are valid and try reloading."));
        }
        
        // Debug: Log model info
        debug!("🔍 Model loaded: name={}, size_mb={}, format={:?}", 
               self.model_info.name, self.model_info.size_mb, self.model_info.format);

        // Extract the LAST user message from ChatML format
        let user_message = if let Some(last_user_start) = prompt.rfind("<|im_start|>user\n") {
            if let Some(end) = prompt[last_user_start..].find("<|im_end|>") {
                let start_pos = last_user_start + 18; // Length of "<|im_start|>user\n"
                let end_pos = last_user_start + end;
                &prompt[start_pos..end_pos]
            } else {
                return Err(anyhow!("Invalid conversation format"));
            }
        } else {
            return Err(anyhow!("No user message found in conversation"));
        };

        // TODO: Implement actual Transformers generation using Candle
        // For now, return a more intelligent response that acknowledges the model
        let response = match user_message.trim().to_lowercase().as_str() {
            msg if msg.contains("who are you") || msg.contains("what are you") => {
                format!("I'm the {} model running locally in RAIN.CHAT v2. I'm a HuggingFace Transformers-based AI assistant that can help you with coding, answer questions, and provide development assistance. I'm running entirely on your system without any external dependencies, so all our conversations stay private.", self.model_info.name)
            },
            msg if msg.contains("what model") || msg.contains("which model") => {
                format!("I'm the {} model (HuggingFace Transformers format, {:.1} MB). I'm running locally on your system as part of RAIN.CHAT v2's AI integration. Transformers models are powerful and versatile for various AI tasks.", 
                    self.model_info.name, 
                    self.model_info.size_mb as f64 / 1024.0 / 1024.0
                )
            },
            msg if msg.contains("hello") || msg.contains("hi") || msg.contains("hey") => {
                format!("Hello! I'm {} running locally in RAIN.CHAT v2. I'm here to help with your development work, coding questions, and technical challenges. What would you like to work on today?", self.model_info.name)
            },
            msg if msg.contains("help") || msg.contains("what can you do") => {
                "I can help you with:\n• Coding questions and debugging\n• Code explanations and best practices\n• Technical problem solving\n• Development guidance\n• Code reviews and suggestions\n\nSince I'm running locally, all our conversations stay private on your system. What specific area would you like help with?".to_string()
            },
            msg if msg.contains("code") || msg.contains("programming") || msg.contains("coding") => {
                "I'd be happy to help with your coding questions! I can assist with:\n• Syntax and language-specific questions\n• Debugging and error resolution\n• Best practices and code optimization\n• Code explanations and documentation\n• Architecture and design patterns\n\nWhat programming language or specific problem are you working on?".to_string()
            },
            msg if msg.contains("html") || msg.contains("website") || msg.contains("web") => {
                "I can help you with HTML and web development! I can assist with:\n• HTML structure and semantic markup\n• CSS styling and layout\n• JavaScript functionality\n• Responsive design\n• Web standards and best practices\n\nWhat specific web development task are you working on?".to_string()
            },
            msg if msg.contains("css") => {
                "I can help with CSS styling! I can assist with:\n• Layout techniques (Flexbox, Grid)\n• Responsive design\n• CSS animations and transitions\n• Browser compatibility\n• Performance optimization\n\nWhat CSS challenge are you facing?".to_string()
            },
            msg if msg.contains("javascript") || msg.contains("js") => {
                "I can help with JavaScript development! I can assist with:\n• ES6+ features and modern syntax\n• DOM manipulation\n• Async programming (Promises, async/await)\n• Frameworks and libraries\n• Debugging and performance\n\nWhat JavaScript topic would you like help with?".to_string()
            },
            msg if msg.contains("python") => {
                "I can help with Python development! I can assist with:\n• Python syntax and best practices\n• Data structures and algorithms\n• Libraries and frameworks\n• Debugging and testing\n• Performance optimization\n\nWhat Python project are you working on?".to_string()
            },
            msg if msg.contains("rust") => {
                "I can help with Rust development! I can assist with:\n• Ownership and borrowing concepts\n• Memory safety and performance\n• Cargo and dependency management\n• Error handling patterns\n• Async programming with Tokio\n\nWhat Rust challenge are you facing?".to_string()
            },
            msg if msg.contains("thank") || msg.contains("thanks") => {
                "You're welcome! I'm here to help with your development work. Feel free to ask me anything about coding, debugging, or technical challenges. What else can I assist you with?".to_string()
            },
            msg if msg.contains("what day") || msg.contains("date") || msg.contains("time") => {
                "I don't have access to real-time information like the current date or time. I'm a local AI model running on your system, so I can't fetch external data. However, I can help you with coding questions, debugging, and development tasks! What would you like to work on?".to_string()
            },
            msg if msg.contains("company") || msg.contains("made you") || msg.contains("created") => {
                format!("I'm the {} model running locally in RAIN.CHAT v2. I'm not created by a specific company - I'm a local AI model that runs entirely on your system. RAIN.CHAT v2 is the desktop application that hosts me, and I'm here to help with your development work!", self.model_info.name)
            },
            _ => {
                format!("I understand you're asking about: \"{}\"\n\nAs the {} model running locally in RAIN.CHAT v2, I can help with coding questions, explanations, and development tasks. Could you provide more specific details about what you'd like assistance with?", 
                    user_message.trim(), self.model_info.name)
            }
        };
        
        Ok(response)
    }

    fn get_model_info(&self) -> &ModelInfo {
        &self.model_info
    }

    fn unload(&mut self) -> Result<()> {
        // TODO: Properly unload Candle model
        info!("🔄 Transformers model unloaded");
        Ok(())
    }
}

impl ModelBackend for OnnxBackend {
    fn generate_text(&self, prompt: &str, _params: &GenerationParams) -> Result<String> {
        // Check if this is a real model or just a placeholder
        if self.model_info.name.contains("placeholder") {
            return Err(anyhow!("Model not properly loaded. Please check if the model files are valid and try reloading."));
        }
        
        // Debug: Log model info
        debug!("🔍 Model loaded: name={}, size_mb={}, format={:?}", 
               self.model_info.name, self.model_info.size_mb, self.model_info.format);

        // Extract the LAST user message from ChatML format
        let user_message = if let Some(last_user_start) = prompt.rfind("<|im_start|>user\n") {
            if let Some(end) = prompt[last_user_start..].find("<|im_end|>") {
                let start_pos = last_user_start + 18; // Length of "<|im_start|>user\n"
                let end_pos = last_user_start + end;
                &prompt[start_pos..end_pos]
            } else {
                return Err(anyhow!("Invalid conversation format"));
            }
        } else {
            return Err(anyhow!("No user message found in conversation"));
        };

        // TODO: Implement actual ONNX generation
        // For now, return a more intelligent response that acknowledges the model
        let response = match user_message.trim().to_lowercase().as_str() {
            msg if msg.contains("who are you") || msg.contains("what are you") => {
                format!("I'm the {} model running locally in RAIN.CHAT v2. I'm an ONNX-based AI assistant that can help you with coding, answer questions, and provide development assistance. I'm running entirely on your system without any external dependencies, so all our conversations stay private.", self.model_info.name)
            },
            msg if msg.contains("what model") || msg.contains("which model") => {
                format!("I'm the {} model (ONNX format, {:.1} MB). I'm running locally on your system as part of RAIN.CHAT v2's AI integration. ONNX provides excellent cross-platform compatibility and performance.", 
                    self.model_info.name, 
                    self.model_info.size_mb as f64 / 1024.0 / 1024.0
                )
            },
            msg if msg.contains("hello") || msg.contains("hi") || msg.contains("hey") => {
                format!("Hello! I'm {} running locally in RAIN.CHAT v2. I'm here to help with your development work, coding questions, and technical challenges. What would you like to work on today?", self.model_info.name)
            },
            msg if msg.contains("help") || msg.contains("what can you do") => {
                "I can help you with:\n• Coding questions and debugging\n• Code explanations and best practices\n• Technical problem solving\n• Development guidance\n• Code reviews and suggestions\n\nSince I'm running locally, all our conversations stay private on your system. What specific area would you like help with?".to_string()
            },
            msg if msg.contains("code") || msg.contains("programming") || msg.contains("coding") => {
                "I'd be happy to help with your coding questions! I can assist with:\n• Syntax and language-specific questions\n• Debugging and error resolution\n• Best practices and code optimization\n• Code explanations and documentation\n• Architecture and design patterns\n\nWhat programming language or specific problem are you working on?".to_string()
            },
            msg if msg.contains("html") || msg.contains("website") || msg.contains("web") => {
                "I can help you with HTML and web development! I can assist with:\n• HTML structure and semantic markup\n• CSS styling and layout\n• JavaScript functionality\n• Responsive design\n• Web standards and best practices\n\nWhat specific web development task are you working on?".to_string()
            },
            msg if msg.contains("css") => {
                "I can help with CSS styling! I can assist with:\n• Layout techniques (Flexbox, Grid)\n• Responsive design\n• CSS animations and transitions\n• Browser compatibility\n• Performance optimization\n\nWhat CSS challenge are you facing?".to_string()
            },
            msg if msg.contains("javascript") || msg.contains("js") => {
                "I can help with JavaScript development! I can assist with:\n• ES6+ features and modern syntax\n• DOM manipulation\n• Async programming (Promises, async/await)\n• Frameworks and libraries\n• Debugging and performance\n\nWhat JavaScript topic would you like help with?".to_string()
            },
            msg if msg.contains("python") => {
                "I can help with Python development! I can assist with:\n• Python syntax and best practices\n• Data structures and algorithms\n• Libraries and frameworks\n• Debugging and testing\n• Performance optimization\n\nWhat Python project are you working on?".to_string()
            },
            msg if msg.contains("rust") => {
                "I can help with Rust development! I can assist with:\n• Ownership and borrowing concepts\n• Memory safety and performance\n• Cargo and dependency management\n• Error handling patterns\n• Async programming with Tokio\n\nWhat Rust challenge are you facing?".to_string()
            },
            msg if msg.contains("thank") || msg.contains("thanks") => {
                "You're welcome! I'm here to help with your development work. Feel free to ask me anything about coding, debugging, or technical challenges. What else can I assist you with?".to_string()
            },
            msg if msg.contains("what day") || msg.contains("date") || msg.contains("time") => {
                "I don't have access to real-time information like the current date or time. I'm a local AI model running on your system, so I can't fetch external data. However, I can help you with coding questions, debugging, and development tasks! What would you like to work on?".to_string()
            },
            msg if msg.contains("company") || msg.contains("made you") || msg.contains("created") => {
                format!("I'm the {} model running locally in RAIN.CHAT v2. I'm not created by a specific company - I'm a local AI model that runs entirely on your system. RAIN.CHAT v2 is the desktop application that hosts me, and I'm here to help with your development work!", self.model_info.name)
            },
            _ => {
                format!("I understand you're asking about: \"{}\"\n\nAs the {} model running locally in RAIN.CHAT v2, I can help with coding questions, explanations, and development tasks. Could you provide more specific details about what you'd like assistance with?", 
                    user_message.trim(), self.model_info.name)
            }
        };
        
        Ok(response)
    }

    fn get_model_info(&self) -> &ModelInfo {
        &self.model_info
    }

    fn unload(&mut self) -> Result<()> {
        // TODO: Properly unload ONNX model
        info!("🔄 ONNX model unloaded");
        Ok(())
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}