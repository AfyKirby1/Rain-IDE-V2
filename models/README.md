# Models Directory

This directory contains AI model files for RAIN.CHAT v2.

## Structure

- `embedding/` - Embedding models for semantic search
- `gemma-3-270m-it/` - Gemma 3 270M instruction-tuned model
- `LFM2-350M/` - LFM2 350M model
- `LFM2-VL-1.6B-Q8_0/` - LFM2 Vision-Language 1.6B model
- `Qwen2.5-0.5B-Instruct-Gensyn-Swarm-tall_mammalian_caribou/` - Qwen2.5 0.5B model

## Model Files

The actual model files (`.gguf`, `.safetensors`, etc.) are not included in this repository due to their large size. 

To use the models:
1. Download the model files to their respective directories
2. Ensure the model files match the expected names in the configuration
3. The application will automatically detect and load available models

## Supported Formats

- GGUF (recommended for local inference)
- SafeTensors
- ONNX
- PyTorch (.pt, .pth)
- HuggingFace Transformers

## Security Note

All model files are excluded from version control to:
- Reduce repository size
- Protect proprietary model weights
- Comply with model licensing requirements
