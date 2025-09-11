import React from 'react';
import {
  Fab,
  Badge,
  Tooltip,
} from '@mui/material';
import {
  SmartToy,
} from '@mui/icons-material';
import { useAIStore } from '../stores/aiStore';

interface FloatingModelButtonProps {
  onOpenModelPicker: () => void;
}

const FloatingModelButton: React.FC<FloatingModelButtonProps> = ({ onOpenModelPicker }) => {
  const { currentModel, isModelLoading } = useAIStore();

  return (
    <Tooltip title={currentModel ? `Current: ${currentModel.name}` : 'Select AI Model'}>
      <Fab
        color="primary"
        onClick={onOpenModelPicker}
        disabled={isModelLoading}
        sx={{
          position: 'fixed',
          bottom: 24,
          right: 24,
          zIndex: 1000,
          bgcolor: '#3b82f6',
          '&:hover': {
            bgcolor: '#2563eb',
          },
          boxShadow: '0 8px 32px rgba(59, 130, 246, 0.3)',
        }}
      >
        <Badge
          badgeContent={currentModel ? 1 : 0}
          color="success"
          sx={{
            '& .MuiBadge-badge': {
              bgcolor: '#4CAF50',
              color: 'white',
              fontSize: '0.7rem',
              width: 16,
              height: 16,
              minWidth: 16,
            },
          }}
        >
          <SmartToy />
        </Badge>
      </Fab>
    </Tooltip>
  );
};

export default FloatingModelButton;
