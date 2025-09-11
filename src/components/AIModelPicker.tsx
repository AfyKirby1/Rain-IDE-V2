import React, { useEffect, useState } from 'react';
import {
  Box,
  Typography,
  FormControl,
  Select,
  MenuItem,
  Chip,
  Button,
  CircularProgress,
  Paper,
  IconButton,
  Tooltip,
  Alert,
} from '@mui/material';
import {
  SmartToy,
  Memory,
  Refresh,
  PlayArrow,
} from '@mui/icons-material';
import { useAIStore } from '../stores/aiStore';

const AIModelPicker: React.FC = () => {
  const {
    availableModels,
    currentModel,
    isModelLoading,
    discoverModels,
    loadModel,
    loadBestModel,
  } = useAIStore();

  const [selectedModel, setSelectedModel] = useState<string>('');

  useEffect(() => {
    // Auto-discover models on component mount
    discoverModels();
  }, [discoverModels]);

  useEffect(() => {
    if (currentModel) {
      setSelectedModel(currentModel.name);
    }
  }, [currentModel]);

  const handleModelChange = async (modelName: string) => {
    setSelectedModel(modelName);
    if (modelName) {
      try {
        const success = await loadModel(modelName);
        if (!success) {
          console.error('Failed to load model:', modelName);
          // Reset selection if loading failed
          setSelectedModel(currentModel?.name || '');
        }
      } catch (error) {
        console.error('Error loading model:', error);
        setSelectedModel(currentModel?.name || '');
      }
    }
  };

  const handleLoadBest = async () => {
    await loadBestModel();
  };

  const handleRefresh = async () => {
    await discoverModels();
  };

  const formatSize = (sizeMb: number) => {
    if (sizeMb < 1024) {
      return `${sizeMb.toFixed(1)} MB`;
    }
    return `${(sizeMb / 1024).toFixed(1)} GB`;
  };

  const getFormatIcon = (format: string) => {
    switch (format.toLowerCase()) {
      case 'gguf':
        return '🦙'; // Llama for GGUF
      case 'onnx':
        return '⚡'; // Lightning for ONNX
      case 'huggingface':
        return '🤗'; // Hugging Face emoji
      default:
        return '🤖'; // Robot for others
    }
  };

  const getFormatColor = (format: string) => {
    switch (format.toLowerCase()) {
      case 'gguf':
        return '#4CAF50'; // Green
      case 'onnx':
        return '#FF9800'; // Orange
      case 'huggingface':
        return '#2196F3'; // Blue
      default:
        return '#9E9E9E'; // Gray
    }
  };

  return (
    <Paper
      elevation={2}
      sx={{
        p: 3,
        mb: 3,
        background: 'linear-gradient(135deg, #1a1a1a 0%, #2d2d2d 100%)',
        border: '1px solid rgba(255, 255, 255, 0.1)',
        borderRadius: 2,
      }}
    >
      {/* Header */}
      <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
        <SmartToy sx={{ mr: 1, color: 'primary.main' }} />
        <Typography variant="h6" sx={{ color: 'text.primary', flexGrow: 1 }}>
          AI Model Manager
        </Typography>
        <Tooltip title="Refresh Models">
          <IconButton
            onClick={handleRefresh}
            size="small"
            sx={{ color: 'text.secondary' }}
          >
            <Refresh />
          </IconButton>
        </Tooltip>
      </Box>

      {/* Model Selection */}
      <Box sx={{ display: 'flex', gap: 2, mb: 2, alignItems: 'center' }}>
        <FormControl size="small" sx={{ minWidth: 250, flexGrow: 1 }}>
          <Select
            value={selectedModel}
            onChange={(e) => handleModelChange(e.target.value)}
            displayEmpty
            disabled={isModelLoading}
            sx={{
              bgcolor: 'rgba(255, 255, 255, 0.05)',
              '& .MuiOutlinedInput-notchedOutline': {
                borderColor: 'rgba(255, 255, 255, 0.2)',
              },
              '&:hover .MuiOutlinedInput-notchedOutline': {
                borderColor: 'rgba(255, 255, 255, 0.3)',
              },
              '& .MuiSelect-select': {
                color: 'text.primary',
              },
            }}
          >
            <MenuItem value="" disabled>
              <Typography sx={{ color: 'text.disabled' }}>
                {availableModels.length === 0 ? 'No models found' : 'Select a model...'}
              </Typography>
            </MenuItem>
            {availableModels.map((model) => (
              <MenuItem key={model.id} value={model.name}>
                <Box sx={{ display: 'flex', alignItems: 'center', width: '100%' }}>
                  <Typography sx={{ mr: 1 }}>
                    {getFormatIcon(model.format)}
                  </Typography>
                  <Box sx={{ flexGrow: 1 }}>
                    <Typography variant="body2" sx={{ fontWeight: 500 }}>
                      {model.name}
                    </Typography>
                    <Typography variant="caption" sx={{ color: 'text.secondary' }}>
                      {model.format.toUpperCase()} • {formatSize(model.size_mb)}
                    </Typography>
                  </Box>
                  {model.loaded && (
                    <Chip
                      label="Loaded"
                      size="small"
                      color="success"
                      variant="outlined"
                      sx={{ ml: 1 }}
                    />
                  )}
                </Box>
              </MenuItem>
            ))}
          </Select>
        </FormControl>

        <Button
          variant="outlined"
          onClick={handleLoadBest}
          disabled={isModelLoading || availableModels.length === 0}
          startIcon={isModelLoading ? <CircularProgress size={16} /> : <PlayArrow />}
          sx={{
            borderColor: 'primary.main',
            color: 'primary.main',
            '&:hover': {
              borderColor: 'primary.light',
              bgcolor: 'rgba(25, 118, 210, 0.1)',
            },
          }}
        >
          Load Best
        </Button>
      </Box>

      {/* Current Model Status */}
      {currentModel && (
        <Box
          sx={{
            p: 2,
            borderRadius: 1,
            bgcolor: 'rgba(76, 175, 80, 0.1)',
            border: '1px solid rgba(76, 175, 80, 0.3)',
            mb: 2,
          }}
        >
          <Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
            <Box
              sx={{
                width: 8,
                height: 8,
                borderRadius: '50%',
                bgcolor: '#4CAF50',
                mr: 1,
                animation: 'pulse 2s infinite',
                '@keyframes pulse': {
                  '0%': { opacity: 1 },
                  '50%': { opacity: 0.5 },
                  '100%': { opacity: 1 },
                },
              }}
            />
            <Typography variant="body2" sx={{ fontWeight: 500, color: '#4CAF50' }}>
              Model Loaded: {currentModel.name}
            </Typography>
          </Box>
          
          <Box sx={{ display: 'flex', gap: 1, flexWrap: 'wrap' }}>
            <Chip
              icon={<Memory />}
              label={formatSize(currentModel.size_mb)}
              size="small"
              variant="outlined"
              sx={{ borderColor: getFormatColor(currentModel.format) }}
            />
            <Chip
              label={currentModel.format.toUpperCase()}
              size="small"
              sx={{
                bgcolor: getFormatColor(currentModel.format),
                color: 'white',
                fontWeight: 500,
              }}
            />
            {currentModel.capabilities.map((capability) => (
              <Chip
                key={capability}
                label={capability.replace('_', ' ')}
                size="small"
                variant="outlined"
                sx={{ 
                  borderColor: 'rgba(255, 255, 255, 0.3)',
                  color: 'text.secondary',
                }}
              />
            ))}
          </Box>

          {currentModel.description && (
            <Typography variant="caption" sx={{ color: 'text.secondary', mt: 1, display: 'block' }}>
              {currentModel.description}
            </Typography>
          )}
        </Box>
      )}

      {/* No Models Alert */}
      {availableModels.length === 0 && !isModelLoading && (
        <Alert 
          severity="info" 
          sx={{ 
            bgcolor: 'rgba(33, 150, 243, 0.1)',
            color: 'text.primary',
            border: '1px solid rgba(33, 150, 243, 0.3)',
          }}
        >
          No AI models found. Place models in your <code>models/</code> directory and click refresh.
          <br />
          <Typography variant="caption" sx={{ mt: 1, display: 'block' }}>
            Supported formats: GGUF (.gguf), ONNX (.onnx), HuggingFace (.safetensors, .bin)
          </Typography>
        </Alert>
      )}

      {/* Loading State */}
      {isModelLoading && (
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', py: 2 }}>
          <CircularProgress size={24} sx={{ mr: 2 }} />
          <Typography variant="body2" sx={{ color: 'text.secondary' }}>
            Loading model...
          </Typography>
        </Box>
      )}
    </Paper>
  );
};

export default AIModelPicker;
