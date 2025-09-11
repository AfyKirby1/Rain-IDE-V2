import React from 'react';
import { Box, Typography, IconButton } from '@mui/material';
import { Close, Minimize, CropSquare } from '@mui/icons-material';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const TitleBar: React.FC = () => {
  const webviewWindow = getCurrentWebviewWindow();
  const handleMinimize = () => webviewWindow.minimize();
  const handleMaximize = () => webviewWindow.toggleMaximize();
  const handleClose = () => webviewWindow.close();

  return (
    <Box
      data-tauri-drag-region
      className="drag-region"
      sx={{
        height: 32,
        background: 'linear-gradient(90deg, #1e293b 0%, #334155 100%)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        borderBottom: '1px solid #475569',
        cursor: 'move',
      }}
    >
      <Box sx={{ display: 'flex', alignItems: 'center', px: 2 }} data-tauri-drag-region className="drag-region">
        <Typography variant="body2" sx={{ color: 'text.secondary', fontSize: 12 }}>
          RAIN.CHAT v2 - Professional AI IDE
        </Typography>
      </Box>

      <Box sx={{ display: 'flex' }} className="no-drag-region">
        <IconButton
          size="small"
          onClick={handleMinimize}
          data-tauri-drag-region="false"
          className="no-drag-region"
          sx={{
            color: 'text.secondary',
            borderRadius: 0,
            width: 32,
            height: 32,
            '&:hover': { background: 'rgba(255, 255, 255, 0.1)' }
          }}
        >
          <Minimize fontSize="small" />
        </IconButton>
        <IconButton
          size="small"
          onClick={handleMaximize}
          data-tauri-drag-region="false"
          className="no-drag-region"
          sx={{
            color: 'text.secondary',
            borderRadius: 0,
            width: 32,
            height: 32,
            '&:hover': { background: 'rgba(255, 255, 255, 0.1)' }
          }}
        >
          <CropSquare fontSize="small" />
        </IconButton>
        <IconButton
          size="small"
          onClick={handleClose}
          data-tauri-drag-region="false"
          className="no-drag-region"
          sx={{
            color: 'text.secondary',
            borderRadius: 0,
            width: 32,
            height: 32,
            '&:hover': { background: '#ef4444', color: 'white' }
          }}
        >
          <Close fontSize="small" />
        </IconButton>
      </Box>
    </Box>
  );
};

export default TitleBar;