import React, { useState } from 'react';
import {
  Box,
  Typography,
  IconButton,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Chip,
  Tooltip,
  Divider,
  Paper,
  Button,
  CircularProgress,
} from '@mui/material';
import {
  SmartToy,
  PlayArrow,
  CheckCircle,
  Error,
  Refresh,
  Settings,
  Close,
} from '@mui/icons-material';
import { useAIStore } from '../stores/aiStore';
import { useModelStatusStore } from '../stores/modelStatusStore';

interface ModelPickerPanelProps {
  open: boolean;
  onClose: () => void;
}

const ModelPickerPanel: React.FC<ModelPickerPanelProps> = ({ open, onClose }) => {
  const {
    availableModels,
    currentModel,
    isModelLoading,
    discoverModels,
    loadModel,
  } = useAIStore();

  const {
    setModelStatus,
    setModelError,
    getModelStatus,
  } = useModelStatusStore();

  const [isDiscovering, setIsDiscovering] = useState(false);

  const handleModelSelect = async (modelName: string) => {
    setModelStatus(modelName, { 
      name: modelName, 
      type: 'chat', 
      status: 'loading' 
    });
    
    try {
      const success = await loadModel(modelName);
      
      if (success) {
        setModelStatus(modelName, { status: 'loaded' });
      } else {
        setModelError(modelName, 'Failed to load model');
      }
    } catch (error) {
      setModelError(modelName, String(error));
    }
  };

  const handleRefresh = async () => {
    setIsDiscovering(true);
    try {
      await discoverModels();
    } catch (error) {
      console.error('Failed to discover models:', error);
    } finally {
      setIsDiscovering(false);
    }
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
        return '🦙';
      case 'onnx':
        return '⚡';
      case 'huggingface':
        return '🤗';
      default:
        return '🤖';
    }
  };

  const getStatusIcon = (modelName: string) => {
    const status = getModelStatus(modelName)?.status;
    if (status === 'loading') {
      return <CircularProgress size={16} />;
    }
    if (status === 'loaded' || currentModel?.name === modelName) {
      return <CheckCircle sx={{ color: '#4CAF50', fontSize: 16 }} />;
    }
    if (status === 'error') {
      return <Error sx={{ color: '#F44336', fontSize: 16 }} />;
    }
    return <PlayArrow sx={{ color: '#9E9E9E', fontSize: 16 }} />;
  };

  return (
    <Drawer
      anchor="right"
      open={open}
      onClose={onClose}
      sx={{
        '& .MuiDrawer-paper': {
          width: 400,
          bgcolor: '#1a1a1a',
          borderLeft: '1px solid rgba(255, 255, 255, 0.1)',
        },
      }}
    >
      <Box sx={{ p: 2, borderBottom: '1px solid rgba(255, 255, 255, 0.1)' }}>
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <SmartToy sx={{ mr: 1, color: 'primary.main' }} />
            <Typography variant="h6" sx={{ color: 'text.primary', fontWeight: 600 }}>
              Model Picker
            </Typography>
          </Box>
          <Box>
            <Tooltip title="Refresh Models">
              <IconButton
                onClick={handleRefresh}
                size="small"
                disabled={isDiscovering}
                sx={{ mr: 1, color: 'text.secondary' }}
              >
                {isDiscovering ? <CircularProgress size={16} /> : <Refresh />}
              </IconButton>
            </Tooltip>
            <IconButton
              onClick={onClose}
              size="small"
              sx={{ color: 'text.secondary' }}
            >
              <Close />
            </IconButton>
          </Box>
        </Box>

        {/* Current Model Status */}
        {currentModel && (
          <Paper
            sx={{
              p: 2,
              mb: 2,
              bgcolor: 'rgba(59, 130, 246, 0.1)',
              border: '1px solid rgba(59, 130, 246, 0.3)',
            }}
          >
            <Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
              <CheckCircle sx={{ color: '#4CAF50', mr: 1, fontSize: 20 }} />
              <Typography variant="subtitle1" sx={{ color: 'text.primary', fontWeight: 600 }}>
                Active Model
              </Typography>
            </Box>
            <Typography variant="body2" sx={{ color: 'text.secondary' }}>
              {currentModel.name} ({formatSize(currentModel.size_mb)})
            </Typography>
            <Chip
              label={currentModel.format.toUpperCase()}
              size="small"
              sx={{
                mt: 1,
                bgcolor: '#3b82f6',
                color: 'white',
                fontSize: 10,
              }}
            />
          </Paper>
        )}
      </Box>

      <Box sx={{ flex: 1, overflow: 'auto' }}>
        <List sx={{ p: 0 }}>
          <ListItem sx={{ px: 2, py: 1 }}>
            <Typography variant="subtitle2" sx={{ color: 'text.secondary', fontWeight: 600 }}>
              Available Models ({availableModels.length})
            </Typography>
          </ListItem>
          
          {availableModels.length === 0 ? (
            <ListItem>
              <Box sx={{ textAlign: 'center', py: 4, width: '100%' }}>
                <Typography variant="body2" sx={{ color: 'text.secondary', mb: 2 }}>
                  No models found
                </Typography>
                <Button
                  variant="outlined"
                  size="small"
                  onClick={handleRefresh}
                  startIcon={<Refresh />}
                >
                  Discover Models
                </Button>
              </Box>
            </ListItem>
          ) : (
            availableModels.map((model) => {
              const isActive = currentModel?.name === model.name;
              const modelStatus = getModelStatus(model.name);
              const isLoading = modelStatus?.status === 'loading';
              
              return (
                <ListItem key={model.id} sx={{ px: 0 }}>
                  <ListItemButton
                    onClick={() => !isLoading && handleModelSelect(model.name)}
                    disabled={isLoading}
                    sx={{
                      mx: 1,
                      borderRadius: 1,
                      bgcolor: isActive ? 'rgba(59, 130, 246, 0.1)' : 'transparent',
                      border: isActive ? '1px solid rgba(59, 130, 246, 0.3)' : '1px solid transparent',
                      '&:hover': {
                        bgcolor: isActive ? 'rgba(59, 130, 246, 0.15)' : 'rgba(255, 255, 255, 0.05)',
                      },
                    }}
                  >
                    <ListItemIcon sx={{ minWidth: 40 }}>
                      <Typography sx={{ fontSize: 20 }}>
                        {getFormatIcon(model.format)}
                      </Typography>
                    </ListItemIcon>
                    
                    <ListItemText
                      primary={
                        <Typography
                          variant="body2"
                          sx={{
                            fontWeight: isActive ? 600 : 500,
                            color: isActive ? 'primary.main' : 'text.primary',
                            display: 'flex',
                            alignItems: 'center',
                            mb: 0.5,
                          }}
                        >
                          <Box component="span" sx={{ flexGrow: 1 }}>
                            {model.name}
                          </Box>
                          <Box component="span" sx={{ ml: 1 }}>
                            {getStatusIcon(model.name)}
                          </Box>
                        </Typography>
                      }
                      secondary={
                        <Typography variant="caption" sx={{ color: 'text.secondary', display: 'block' }}>
                          {formatSize(model.size_mb)} • {model.format.toUpperCase()}
                          {modelStatus?.error && (
                            <Typography variant="caption" sx={{ color: 'error.main', display: 'block' }}>
                              {modelStatus.error}
                            </Typography>
                          )}
                        </Typography>
                      }
                    />
                  </ListItemButton>
                </ListItem>
              );
            })
          )}
        </List>
      </Box>

      <Divider />
      
      <Box sx={{ p: 2 }}>
        <Button
          fullWidth
          variant="outlined"
          startIcon={<Settings />}
          sx={{ mb: 1 }}
        >
          Model Settings
        </Button>
        <Typography variant="caption" sx={{ color: 'text.secondary', textAlign: 'center', display: 'block' }}>
          {isModelLoading ? 'Loading model...' : 'Ready to switch models'}
        </Typography>
      </Box>
    </Drawer>
  );
};

export default ModelPickerPanel;
