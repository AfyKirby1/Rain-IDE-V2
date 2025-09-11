# 🧠 EmbeddingGemma-300m Integration Guide

## 📋 **Model Overview**

**EmbeddingGemma-300m** is a state-of-the-art 300M parameter embedding model from Google that produces vector representations of text for search, retrieval, classification, and similarity tasks.

### **Key Features**
- **Size**: 300M parameters (303M params)
- **Dimensions**: 768 (with 512, 256, 128 options via MRL)
- **Languages**: 100+ spoken languages
- **Context Length**: 2048 tokens
- **Format**: Safetensors (F32)
- **Framework**: Sentence Transformers

### **Use Cases**
- Document retrieval and search
- Question answering systems
- Classification and clustering
- Semantic similarity
- Code retrieval
- Fact verification

---

## 🛠️ **Integration Plan**

### **Phase 1: Add Embedding Support to Rust Backend**

1. **Add Sentence Transformers Dependencies**
2. **Create Embedding Backend**
3. **Add Embedding Commands**
4. **Update Model Manager**

### **Phase 2: Frontend Integration**

1. **Add Embedding UI Components**
2. **Create Embedding Store**
3. **Add Embedding Commands**

### **Phase 3: Model Download System**

1. **Create Model Downloader**
2. **Add Progress Tracking**
3. **Integrate with Model Manager**

---

## 🔧 **Implementation Steps**

### **Step 1: Add Rust Dependencies**

```toml
# Add to src-tauri/Cargo.toml
[dependencies]
# ... existing dependencies

# Sentence Transformers support
candle-transformers = { version = "0.3", features = ["metal", "cuda"] }
tokenizers = "0.15"
hf-hub = "0.3"

# For embedding operations
ndarray = "0.15"
```

### **Step 2: Create Embedding Backend**

```rust
// src-tauri/src/ai/embedding_backend.rs
use candle_core::{Device, Tensor};
use candle_transformers::models::sentence_transformer::SentenceTransformer;
use anyhow::Result;

pub struct EmbeddingBackend {
    model: SentenceTransformer,
    device: Device,
    model_name: String,
}

impl EmbeddingBackend {
    pub async fn new(model_name: &str) -> Result<Self> {
        let device = Device::cuda_if_available(0)?;
        let model = SentenceTransformer::new(model_name, &device).await?;
        
        Ok(Self {
            model,
            device,
            model_name: model_name.to_string(),
        })
    }
    
    pub async fn encode_query(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.encode_query(text).await?;
        Ok(embeddings.to_vec())
    }
    
    pub async fn encode_document(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let embeddings = self.model.encode_document(texts).await?;
        Ok(embeddings.iter().map(|e| e.to_vec()).collect())
    }
    
    pub async fn similarity(&self, query: &[f32], documents: &[Vec<f32>]) -> Result<Vec<f32>> {
        // Compute cosine similarity
        let query_tensor = Tensor::new(query, &self.device)?;
        let mut similarities = Vec::new();
        
        for doc in documents {
            let doc_tensor = Tensor::new(doc, &self.device)?;
            let similarity = self.cosine_similarity(&query_tensor, &doc_tensor)?;
            similarities.push(similarity);
        }
        
        Ok(similarities)
    }
    
    fn cosine_similarity(&self, a: &Tensor, b: &Tensor) -> Result<f32> {
        let dot_product = a.matmul(b)?;
        let norm_a = a.norm_all()?;
        let norm_b = b.norm_all()?;
        Ok((dot_product / (norm_a * norm_b))?[0])
    }
}
```

### **Step 3: Add Embedding Commands**

```rust
// src-tauri/src/ui/embedding.rs
use tauri::State;
use crate::ai::embedding_backend::EmbeddingBackend;
use crate::AppState;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingRequest {
    pub text: String,
    pub task_type: String, // "query", "document", "classification", etc.
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
    let mut embedding_manager = state.embedding_models.write().await;
    embedding_manager.load_model(&model_name).await
        .map_err(|e| format!("Failed to load embedding model: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn encode_text(
    state: State<'_, AppState>,
    request: EmbeddingRequest,
) -> Result<EmbeddingResponse, String> {
    let embedding_manager = state.embedding_models.read().await;
    let embeddings = embedding_manager.encode_text(&request.text, &request.task_type).await
        .map_err(|e| format!("Failed to encode text: {}", e))?;
    
    Ok(EmbeddingResponse {
        embeddings,
        dimension: 768, // EmbeddingGemma default
        model_name: "google/embeddinggemma-300m".to_string(),
    })
}

#[tauri::command]
pub async fn compute_similarity(
    state: State<'_, AppState>,
    request: SimilarityRequest,
) -> Result<SimilarityResponse, String> {
    let embedding_manager = state.embedding_models.read().await;
    let similarities = embedding_manager.compute_similarity(&request.query, &request.documents).await
        .map_err(|e| format!("Failed to compute similarity: {}", e))?;
    
    // Rank documents by similarity
    let mut ranked: Vec<(usize, f32)> = similarities.iter().enumerate().map(|(i, &sim)| (i, sim)).collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    Ok(SimilarityResponse {
        similarities,
        ranked_documents: ranked,
    })
}
```

### **Step 4: Create Model Downloader**

```rust
// src-tauri/src/ai/model_downloader.rs
use hf_hub::api::tokio::Api;
use std::path::PathBuf;
use anyhow::Result;
use tracing::{info, error};

pub struct ModelDownloader {
    api: Api,
    models_dir: PathBuf,
}

impl ModelDownloader {
    pub fn new(models_dir: PathBuf) -> Self {
        let api = Api::new().unwrap();
        Self { api, models_dir }
    }
    
    pub async fn download_embedding_model(&self, model_name: &str) -> Result<PathBuf> {
        info!("📥 Downloading embedding model: {}", model_name);
        
        let repo = self.api.model(model_name.to_string());
        let model_path = self.models_dir.join("embedding").join(model_name);
        
        // Create directory if it doesn't exist
        tokio::fs::create_dir_all(&model_path).await?;
        
        // Download model files
        let files_to_download = vec![
            "config.json",
            "model.safetensors",
            "tokenizer.json",
            "tokenizer_config.json",
        ];
        
        for file in files_to_download {
            let local_file = model_path.join(file);
            if !local_file.exists() {
                info!("📥 Downloading {}...", file);
                let remote_file = repo.get(file).await?;
                tokio::fs::copy(remote_file, &local_file).await?;
                info!("✅ Downloaded {}", file);
            }
        }
        
        info!("🎉 Embedding model downloaded successfully: {}", model_path.display());
        Ok(model_path)
    }
    
    pub async fn is_model_downloaded(&self, model_name: &str) -> bool {
        let model_path = self.models_dir.join("embedding").join(model_name);
        model_path.join("config.json").exists() && 
        model_path.join("model.safetensors").exists()
    }
}
```

---

## 🎨 **Frontend Integration**

### **Step 1: Create Embedding Store**

```typescript
// src/stores/embeddingStore.ts
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface EmbeddingModel {
  name: string;
  downloaded: boolean;
  dimension: number;
  loaded: boolean;
}

interface EmbeddingState {
  models: EmbeddingModel[];
  currentModel: EmbeddingModel | null;
  isDownloading: boolean;
  isEncoding: boolean;
  
  // Actions
  loadModels: () => Promise<void>;
  downloadModel: (modelName: string) => Promise<void>;
  loadModel: (modelName: string) => Promise<void>;
  encodeText: (text: string, taskType: string) => Promise<number[]>;
  computeSimilarity: (query: string, documents: string[]) => Promise<{similarities: number[], ranked: Array<{index: number, score: number}>}>;
}

export const useEmbeddingStore = create<EmbeddingState>((set, get) => ({
  models: [],
  currentModel: null,
  isDownloading: false,
  isEncoding: false,
  
  loadModels: async () => {
    try {
      const models = await invoke('get_embedding_models');
      set({ models });
    } catch (error) {
      console.error('Failed to load embedding models:', error);
    }
  },
  
  downloadModel: async (modelName: string) => {
    set({ isDownloading: true });
    try {
      await invoke('download_embedding_model', { modelName });
      await get().loadModels(); // Refresh models list
    } catch (error) {
      console.error('Failed to download model:', error);
    } finally {
      set({ isDownloading: false });
    }
  },
  
  loadModel: async (modelName: string) => {
    try {
      await invoke('load_embedding_model', { modelName });
      const model = get().models.find(m => m.name === modelName);
      set({ currentModel: model || null });
    } catch (error) {
      console.error('Failed to load model:', error);
    }
  },
  
  encodeText: async (text: string, taskType: string) => {
    set({ isEncoding: true });
    try {
      const response = await invoke('encode_text', { 
        request: { text, task_type: taskType } 
      });
      return response.embeddings;
    } catch (error) {
      console.error('Failed to encode text:', error);
      throw error;
    } finally {
      set({ isEncoding: false });
    }
  },
  
  computeSimilarity: async (query: string, documents: string[]) => {
    try {
      const response = await invoke('compute_similarity', {
        request: { query, documents }
      });
      return {
        similarities: response.similarities,
        ranked: response.ranked_documents.map(([index, score]) => ({ index, score }))
      };
    } catch (error) {
      console.error('Failed to compute similarity:', error);
      throw error;
    }
  },
}));
```

### **Step 2: Create Embedding UI Components**

```typescript
// src/components/EmbeddingPanel.tsx
import React, { useState } from 'react';
import {
  Box,
  Typography,
  TextField,
  Button,
  Paper,
  Chip,
  CircularProgress,
  Alert,
} from '@mui/material';
import { useEmbeddingStore } from '../stores/embeddingStore';

const EmbeddingPanel: React.FC = () => {
  const {
    models,
    currentModel,
    isDownloading,
    isEncoding,
    downloadModel,
    loadModel,
    encodeText,
    computeSimilarity,
  } = useEmbeddingStore();

  const [inputText, setInputText] = useState('');
  const [taskType, setTaskType] = useState('query');
  const [embeddings, setEmbeddings] = useState<number[]>([]);
  const [documents, setDocuments] = useState<string[]>(['']);
  const [similarities, setSimilarities] = useState<number[]>([]);

  const handleEncode = async () => {
    if (!inputText.trim() || !currentModel) return;
    
    try {
      const result = await encodeText(inputText, taskType);
      setEmbeddings(result);
    } catch (error) {
      console.error('Encoding failed:', error);
    }
  };

  const handleSimilarity = async () => {
    if (!inputText.trim() || documents.length === 0 || !currentModel) return;
    
    try {
      const result = await computeSimilarity(inputText, documents.filter(d => d.trim()));
      setSimilarities(result.similarities);
    } catch (error) {
      console.error('Similarity computation failed:', error);
    }
  };

  return (
    <Paper sx={{ p: 3, mb: 3 }}>
      <Typography variant="h6" sx={{ mb: 2 }}>
        🧠 EmbeddingGemma-300m
      </Typography>
      
      {/* Model Status */}
      {currentModel ? (
        <Alert severity="success" sx={{ mb: 2 }}>
          Model loaded: {currentModel.name} ({currentModel.dimension}D)
        </Alert>
      ) : (
        <Alert severity="info" sx={{ mb: 2 }}>
          No embedding model loaded
        </Alert>
      )}
      
      {/* Model Management */}
      <Box sx={{ mb: 3 }}>
        {models.map((model) => (
          <Chip
            key={model.name}
            label={`${model.name} ${model.downloaded ? '✅' : '⬇️'}`}
            onClick={() => loadModel(model.name)}
            onDelete={!model.downloaded ? () => downloadModel(model.name) : undefined}
            color={currentModel?.name === model.name ? 'primary' : 'default'}
            sx={{ mr: 1, mb: 1 }}
          />
        ))}
      </Box>
      
      {/* Text Input */}
      <TextField
        fullWidth
        multiline
        rows={3}
        label="Text to encode"
        value={inputText}
        onChange={(e) => setInputText(e.target.value)}
        sx={{ mb: 2 }}
      />
      
      {/* Task Type Selection */}
      <TextField
        select
        label="Task Type"
        value={taskType}
        onChange={(e) => setTaskType(e.target.value)}
        sx={{ mb: 2, minWidth: 200 }}
      >
        <option value="query">Query</option>
        <option value="document">Document</option>
        <option value="classification">Classification</option>
        <option value="clustering">Clustering</option>
        <option value="similarity">Similarity</option>
      </TextField>
      
      {/* Action Buttons */}
      <Box sx={{ mb: 2 }}>
        <Button
          variant="contained"
          onClick={handleEncode}
          disabled={!inputText.trim() || !currentModel || isEncoding}
          sx={{ mr: 2 }}
        >
          {isEncoding ? <CircularProgress size={20} /> : 'Encode Text'}
        </Button>
        
        <Button
          variant="outlined"
          onClick={handleSimilarity}
          disabled={!inputText.trim() || !currentModel}
        >
          Compute Similarity
        </Button>
      </Box>
      
      {/* Results */}
      {embeddings.length > 0 && (
        <Box sx={{ mb: 2 }}>
          <Typography variant="subtitle2">Embeddings ({embeddings.length}D):</Typography>
          <Typography variant="body2" sx={{ fontFamily: 'monospace', fontSize: '0.8rem' }}>
            [{embeddings.slice(0, 10).map(n => n.toFixed(4)).join(', ')}
            {embeddings.length > 10 ? '...' : ''}]
          </Typography>
        </Box>
      )}
      
      {similarities.length > 0 && (
        <Box>
          <Typography variant="subtitle2">Similarities:</Typography>
          {similarities.map((sim, i) => (
            <Chip
              key={i}
              label={`Doc ${i + 1}: ${(sim * 100).toFixed(1)}%`}
              color={sim > 0.7 ? 'success' : sim > 0.5 ? 'warning' : 'default'}
              sx={{ mr: 1, mb: 1 }}
            />
          ))}
        </Box>
      )}
    </Paper>
  );
};

export default EmbeddingPanel;
```

---

## 🚀 **Quick Start Implementation**

### **Step 1: Add to Main App**

```rust
// src-tauri/src/main.rs
// Add to AppState
pub struct AppState {
    // ... existing fields
    pub embedding_models: Arc<RwLock<EmbeddingManager>>,
}

// Add to invoke_handler
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    // Embedding commands
    ui::embedding::load_embedding_model,
    ui::embedding::encode_text,
    ui::embedding::compute_similarity,
    ui::embedding::download_embedding_model,
])
```

### **Step 2: Add to Frontend**

```typescript
// src/App.tsx
import EmbeddingPanel from './components/EmbeddingPanel';

// Add to your main layout
<EmbeddingPanel />
```

---

## 📊 **Expected Results**

After implementation, you'll have:

- ✅ **Model Download**: One-click download of EmbeddingGemma-300m
- ✅ **Text Encoding**: Convert text to 768-dimensional vectors
- ✅ **Similarity Search**: Find similar documents
- ✅ **Multiple Task Types**: Query, document, classification, etc.
- ✅ **Real-time Processing**: Fast local inference
- ✅ **Progress Tracking**: Download and encoding progress

---

## 🔧 **Advanced Features**

### **Custom Prompts**
```rust
// Use EmbeddingGemma's specialized prompts
let prompt = match task_type {
    "query" => format!("task: search result | query: {}", text),
    "document" => format!("title: none | text: {}", text),
    "classification" => format!("task: classification | query: {}", text),
    _ => text.to_string(),
};
```

### **Batch Processing**
```rust
// Process multiple texts at once
let texts = vec!["text1".to_string(), "text2".to_string()];
let embeddings = model.encode_document(&texts).await?;
```

### **Dimension Reduction**
```rust
// Use Matryoshka Representation Learning for smaller embeddings
let embeddings_512 = embeddings[..512].to_vec(); // 512D
let embeddings_256 = embeddings[..256].to_vec(); // 256D
```

---

**This integration will give you powerful embedding capabilities for search, retrieval, and similarity tasks in your RAIN.CHAT v2 IDE!** 🎉

Would you like me to start implementing any specific part of this integration?
