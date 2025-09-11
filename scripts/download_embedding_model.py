#!/usr/bin/env python3
"""
Download EmbeddingGemma-300m model for RAIN.CHAT v2
This script downloads the model files needed for local inference
"""

import os
import sys
import json
from pathlib import Path
from huggingface_hub import hf_hub_download, snapshot_download
import argparse

def download_embedding_model(model_name="google/embeddinggemma-300m", output_dir="models/embedding"):
    """Download EmbeddingGemma model files"""
    
    # Create output directory
    output_path = Path(output_dir) / model_name.split("/")[-1]
    output_path.mkdir(parents=True, exist_ok=True)
    
    print(f"📥 Downloading {model_name} to {output_path}")
    
    # Files needed for Sentence Transformers
    files_to_download = [
        "config.json",
        "model.safetensors", 
        "tokenizer.json",
        "tokenizer_config.json",
        "special_tokens_map.json",
        "sentence_bert_config.json"
    ]
    
    downloaded_files = []
    
    for file_name in files_to_download:
        try:
            print(f"📥 Downloading {file_name}...")
            local_path = hf_hub_download(
                repo_id=model_name,
                filename=file_name,
                local_dir=output_path,
                local_dir_use_symlinks=False
            )
            downloaded_files.append(file_name)
            print(f"✅ Downloaded {file_name}")
        except Exception as e:
            print(f"⚠️  Could not download {file_name}: {e}")
    
    # Create model info file
    model_info = {
        "name": model_name.split("/")[-1],
        "repo_id": model_name,
        "type": "embedding",
        "framework": "sentence_transformers",
        "dimension": 768,
        "max_tokens": 2048,
        "languages": 100,
        "downloaded_files": downloaded_files,
        "capabilities": [
            "text_embedding",
            "semantic_search", 
            "similarity",
            "classification",
            "clustering",
            "retrieval"
        ],
        "prompt_templates": {
            "query": "task: search result | query: {text}",
            "document": "title: {title} | text: {text}",
            "classification": "task: classification | query: {text}",
            "clustering": "task: clustering | query: {text}",
            "similarity": "task: sentence similarity | query: {text}",
            "code_retrieval": "task: code retrieval | query: {text}"
        }
    }
    
    # Save model info
    info_path = output_path / "model_info.json"
    with open(info_path, 'w') as f:
        json.dump(model_info, f, indent=2)
    
    print(f"✅ Model download complete!")
    print(f"📁 Files saved to: {output_path}")
    print(f"📊 Downloaded {len(downloaded_files)} files")
    print(f"📋 Model info saved to: {info_path}")
    
    return output_path

def create_rust_integration():
    """Create Rust integration files"""
    
    print("🔧 Creating Rust integration files...")
    
    # Create embedding backend
    embedding_backend_code = '''// src-tauri/src/ai/embedding_backend.rs
use candle_core::{Device, Tensor};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModelInfo {
    pub name: String,
    pub repo_id: String,
    pub dimension: usize,
    pub max_tokens: usize,
    pub capabilities: Vec<String>,
    pub prompt_templates: std::collections::HashMap<String, String>,
}

pub struct EmbeddingBackend {
    model_info: EmbeddingModelInfo,
    device: Device,
    // TODO: Add actual model loading when sentence-transformers is available
}

impl EmbeddingBackend {
    pub fn new(model_info: EmbeddingModelInfo) -> Result<Self> {
        let device = Device::cuda_if_available(0)?;
        
        Ok(Self {
            model_info,
            device,
        })
    }
    
    pub async fn encode_text(&self, text: &str, task_type: &str) -> Result<Vec<f32>> {
        // Get prompt template
        let template = self.model_info.prompt_templates
            .get(task_type)
            .unwrap_or(&"task: search result | query: {text}".to_string());
        
        let prompt = template.replace("{text}", text);
        
        // TODO: Implement actual embedding generation
        // For now, return a mock embedding
        let mock_embedding = vec![0.1; self.model_info.dimension];
        
        println!("🔤 Encoding text with prompt: {}", prompt);
        println!("📊 Generated {}-dimensional embedding", self.model_info.dimension);
        
        Ok(mock_embedding)
    }
    
    pub async fn compute_similarity(&self, query: &str, documents: &[String]) -> Result<Vec<f32>> {
        let query_embedding = self.encode_text(query, "query").await?;
        
        // Mock similarity computation
        let similarities = documents.iter()
            .enumerate()
            .map(|(i, _)| 0.5 + (i as f32 * 0.1).sin())
            .collect();
        
        println!("🔍 Computed similarities for {} documents", documents.len());
        
        Ok(similarities)
    }
    
    pub fn get_model_info(&self) -> &EmbeddingModelInfo {
        &self.model_info
    }
}
'''
    
    # Create Tauri commands
    tauri_commands_code = '''// src-tauri/src/ui/embedding.rs
use tauri::State;
use serde::{Deserialize, Serialize};
use crate::ai::embedding_backend::{EmbeddingBackend, EmbeddingModelInfo};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub text: String,
    pub task_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub embeddings: Vec<f32>,
    pub dimension: usize,
    pub model_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarityRequest {
    pub query: String,
    pub documents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarityResponse {
    pub similarities: Vec<f32>,
    pub ranked_documents: Vec<(usize, f32)>,
}

#[tauri::command]
pub async fn load_embedding_model(
    state: State<'_, AppState>,
    model_name: String,
) -> Result<(), String> {
    // TODO: Implement actual model loading
    println!("📥 Loading embedding model: {}", model_name);
    Ok(())
}

#[tauri::command]
pub async fn encode_text(
    state: State<'_, AppState>,
    request: EmbeddingRequest,
) -> Result<EmbeddingResponse, String> {
    // TODO: Use actual embedding backend
    let mock_embeddings = vec![0.1; 768];
    
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
    let similarities = vec![0.5, 0.7, 0.3];
    let ranked = vec![(1, 0.7), (0, 0.5), (2, 0.3)];
    
    Ok(SimilarityResponse {
        similarities,
        ranked_documents: ranked,
    })
}
'''
    
    # Write files
    backend_path = Path("src-tauri/src/ai/embedding_backend.rs")
    commands_path = Path("src-tauri/src/ui/embedding.rs")
    
    backend_path.parent.mkdir(parents=True, exist_ok=True)
    commands_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(backend_path, 'w') as f:
        f.write(embedding_backend_code)
    
    with open(commands_path, 'w') as f:
        f.write(tauri_commands_code)
    
    print(f"✅ Created {backend_path}")
    print(f"✅ Created {commands_path}")

def main():
    parser = argparse.ArgumentParser(description="Download EmbeddingGemma model")
    parser.add_argument("--model", default="google/embeddinggemma-300m", 
                       help="Model name to download")
    parser.add_argument("--output", default="models/embedding",
                       help="Output directory")
    parser.add_argument("--create-rust", action="store_true",
                       help="Create Rust integration files")
    
    args = parser.parse_args()
    
    try:
        # Download model
        output_path = download_embedding_model(args.model, args.output)
        
        # Create Rust integration if requested
        if args.create_rust:
            create_rust_integration()
        
        print("\n🎉 Setup complete!")
        print(f"📁 Model files: {output_path}")
        print("🔧 Next steps:")
        print("1. Add embedding commands to main.rs")
        print("2. Create frontend components")
        print("3. Test the integration")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
