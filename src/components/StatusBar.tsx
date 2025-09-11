import React from 'react';
import { Box, Typography, Chip, CircularProgress } from '@mui/material';
import { useAIStore } from '../stores/aiStore';
import { useProjectStore } from '../stores/projectStore';
import { useModelStatusStore } from '../stores/modelStatusStore';
import StatusBarPerformance from './StatusBarPerformance';

const StatusBar: React.FC = () => {
  const { currentModel, isModelLoading, isDiscoveringModels } = useAIStore();
  const { currentProject } = useProjectStore();
  const { getAllModelStatuses } = useModelStatusStore();
  
  // Check for any models currently loading
  const loadingModels = getAllModelStatuses().filter(status => status.status === 'loading');
  const hasLoadingModels = loadingModels.length > 0;

  return (
    <Box
      sx={{
        height: 24,
        background: '#1e293b',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        px: 2,
        borderTop: '1px solid #475569',
      }}
    >
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        {currentProject && (
          <Typography variant="caption" sx={{ color: 'text.secondary' }}>
            {currentProject.name}
          </Typography>
        )}
      </Box>

      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        {/* Performance Monitor */}
        <StatusBarPerformance />
        
        {/* AI Model Status */}
        {currentModel && !isModelLoading && !hasLoadingModels && (
          <Chip
            label={`AI: ${currentModel.name}`}
            size="small"
            sx={{
              height: 16,
              fontSize: 10,
              bgcolor: '#10b981',
              color: 'white',
              '& .MuiChip-label': { px: 1 }
            }}
          />
        )}
        
        {/* Loading Status */}
        {(isModelLoading || hasLoadingModels || isDiscoveringModels) && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <CircularProgress size={12} sx={{ color: '#60a5fa' }} />
            <Typography variant="caption" sx={{ color: '#60a5fa', fontSize: 10 }}>
              {isDiscoveringModels ? '🔍 Discovering models...' : 
               hasLoadingModels ? `🔄 Loading ${loadingModels[0].name}...` :
               '🔄 Loading model...'}
            </Typography>
          </Box>
        )}
        
        <Typography variant="caption" sx={{ color: 'text.secondary' }}>
          {isModelLoading || hasLoadingModels || isDiscoveringModels ? 'Loading...' : 'Ready'}
        </Typography>
      </Box>
    </Box>
  );
};

export default StatusBar;