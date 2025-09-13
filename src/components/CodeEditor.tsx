import React, { useState, useEffect, useRef } from 'react';
import { Box, Typography, IconButton, Tooltip, Chip, TextField } from '@mui/material';
import { 
  PlayArrow, 
  // Stop, 
  BugReport, 
  FormatListBulleted,
  Settings,
  // Fullscreen,
  Close,
  Save,
  // SaveAlt
} from '@mui/icons-material';
import { useEditorStore } from '../stores/editorStore';

const CodeEditor: React.FC = () => {
  const { 
    openTabs, 
    activeTabId, 
    closeTab, 
    setActiveTab, 
    updateTabContent, 
    saveFile 
  } = useEditorStore();
  
  const [isRunning, setIsRunning] = useState(false);
  const editorRef = useRef<HTMLDivElement>(null);

  // Get current active tab
  const activeTab = openTabs.find(tab => tab.id === activeTabId);
  
  // Debug logging
  useEffect(() => {
    console.log('CodeEditor: openTabs updated:', openTabs.length, 'tabs');
    console.log('CodeEditor: activeTabId:', activeTabId);
    console.log('CodeEditor: activeTab:', activeTab);
  }, [openTabs, activeTabId, activeTab]);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey || e.metaKey) {
        switch (e.key) {
          case 'n':
            e.preventDefault();
            const { createUntitledFile } = useEditorStore.getState();
            createUntitledFile('text');
            break;
          case 's':
            e.preventDefault();
            if (activeTab) {
              handleSave();
            }
            break;
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [activeTab]);

  const handleRun = () => {
    setIsRunning(true);
    // Simulate running
    setTimeout(() => setIsRunning(false), 2000);
  };

  const handleDebug = () => {
    console.log('Starting debug session...');
  };

  const handleFormat = () => {
    console.log('Formatting code...');
  };

  const handleSave = async () => {
    if (activeTab) {
      try {
        await saveFile(activeTab.id);
      } catch (error) {
        console.error('Failed to save file:', error);
      }
    }
  };

  const handleContentChange = (content: string) => {
    if (activeTab) {
      updateTabContent(activeTab.id, content);
    }
  };

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      {/* Editor Tabs */}
      <Box
        sx={{
          height: 32,
          bgcolor: '#1e293b',
          borderBottom: '1px solid #475569',
          display: 'flex',
          alignItems: 'center',
          px: 1,
          overflow: 'auto',
        }}
      >
        {openTabs.length > 0 ? (
          openTabs.map((tab) => (
            <Box
              key={tab.id}
              sx={{
                display: 'flex',
                alignItems: 'center',
                gap: 1,
                px: 2,
                py: 0.5,
                bgcolor: tab.id === activeTabId ? '#334155' : 'transparent',
                borderRadius: 1,
                mr: 1,
                cursor: 'pointer',
                '&:hover': { bgcolor: '#334155' },
              }}
              onClick={() => setActiveTab(tab.id)}
            >
              <Typography variant="caption" sx={{ color: '#e2e8f0', fontSize: 11 }}>
                {tab.isUntitled ? 'Untitled' : tab.fileName}{tab.isDirty ? ' *' : ''}
              </Typography>
              <IconButton
                size="small"
                onClick={(e) => {
                  e.stopPropagation();
                  closeTab(tab.id);
                }}
                sx={{ color: '#94a3b8', p: 0.5 }}
              >
                <Close fontSize="small" />
              </IconButton>
            </Box>
          ))
        ) : (
          <Box
            sx={{
              display: 'flex',
              alignItems: 'center',
              gap: 1,
              px: 2,
              py: 0.5,
              bgcolor: '#334155',
              borderRadius: 1,
              mr: 1,
            }}
          >
            <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 11 }}>
              No file open
            </Typography>
          </Box>
        )}

        <Box sx={{ flex: 1 }} />

        {/* Editor Actions */}
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
          <Chip
            label={activeTab?.language?.toUpperCase() || 'TEXT'}
            size="small"
            sx={{
              height: 20,
              fontSize: 10,
              bgcolor: '#475569',
              color: '#e2e8f0',
              '& .MuiChip-label': { px: 1 },
            }}
          />
          
          <Tooltip title={activeTab?.isUntitled ? "Save As..." : "Save File"}>
            <IconButton
              size="small"
              onClick={handleSave}
              disabled={!activeTab}
              sx={{ color: activeTab?.isDirty || activeTab?.isUntitled ? '#10b981' : '#94a3b8' }}
            >
              <Save fontSize="small" />
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Run Code">
            <IconButton
              size="small"
              onClick={handleRun}
              disabled={isRunning}
              sx={{ color: isRunning ? '#10b981' : '#94a3b8' }}
            >
              <PlayArrow fontSize="small" />
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Debug">
            <IconButton
              size="small"
              onClick={handleDebug}
              sx={{ color: '#94a3b8' }}
            >
              <BugReport fontSize="small" />
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Format">
            <IconButton
              size="small"
              onClick={handleFormat}
              sx={{ color: '#94a3b8' }}
            >
              <FormatListBulleted fontSize="small" />
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Settings">
            <IconButton
              size="small"
              sx={{ color: '#94a3b8' }}
            >
              <Settings fontSize="small" />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      {/* Editor Content */}
      <Box
        ref={editorRef}
        sx={{
          flex: 1,
          bgcolor: '#0f172a',
          overflow: 'auto',
          position: 'relative',
        }}
      >
        {activeTab ? (
          <TextField
            multiline
            fullWidth
            value={activeTab.content}
            onChange={(e) => handleContentChange(e.target.value)}
            sx={{
              flex: 1,
              '& .MuiInputBase-root': {
                height: '100%',
                alignItems: 'flex-start',
                fontFamily: 'Monaco, Consolas, "Courier New", monospace',
                fontSize: 14,
                lineHeight: 1.6,
                color: '#e2e8f0',
              },
              '& .MuiInputBase-input': {
                height: '100% !important',
                overflow: 'auto !important',
                padding: 3,
                color: '#e2e8f0',
              },
              '& .MuiOutlinedInput-notchedOutline': {
                border: 'none',
              },
              '& .MuiInputBase-root:hover .MuiOutlinedInput-notchedOutline': {
                border: 'none',
              },
              '& .MuiInputBase-root.Mui-focused .MuiOutlinedInput-notchedOutline': {
                border: 'none',
              },
            }}
          />
        ) : (
          <Box
            sx={{
              height: '100%',
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
              p: 4,
              textAlign: 'center',
            }}
          >
            <Box
              sx={{
                width: 80,
                height: 80,
                borderRadius: 2,
                bgcolor: 'rgba(59, 130, 246, 0.1)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                mb: 3,
              }}
            >
              <Typography variant="h3" sx={{ color: '#3b82f6' }}>
                📝
              </Typography>
            </Box>
            
            <Typography variant="h6" sx={{ color: '#e2e8f0', mb: 2 }}>
              Code Editor
            </Typography>
            
            <Typography variant="body2" sx={{ color: '#94a3b8', mb: 3, maxWidth: 400 }}>
              Open a file from the explorer to start coding. The Monaco Editor will provide
              syntax highlighting, IntelliSense, and all the features you expect from a modern IDE.
            </Typography>
            
            <Box
              sx={{
                display: 'flex',
                gap: 2,
                flexWrap: 'wrap',
                justifyContent: 'center',
              }}
            >
              <Chip
                label="TypeScript"
                size="small"
                sx={{ bgcolor: '#1e40af', color: 'white' }}
              />
              <Chip
                label="JavaScript"
                size="small"
                sx={{ bgcolor: '#f59e0b', color: 'white' }}
              />
              <Chip
                label="Python"
                size="small"
                sx={{ bgcolor: '#10b981', color: 'white' }}
              />
              <Chip
                label="Rust"
                size="small"
                sx={{ bgcolor: '#f97316', color: 'white' }}
              />
            </Box>
          </Box>
        )}
      </Box>

      {/* Status Bar */}
      <Box
        sx={{
          height: 24,
          bgcolor: '#1e293b',
          borderTop: '1px solid #475569',
          display: 'flex',
          alignItems: 'center',
          px: 2,
          gap: 2,
        }}
      >
        <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 11 }}>
          Line 1, Column 1
        </Typography>
        <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 11 }}>
          UTF-8
        </Typography>
        <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 11 }}>
          {activeTab?.language || 'TEXT'}
        </Typography>
        {isRunning && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
            <Box
              sx={{
                width: 6,
                height: 6,
                borderRadius: '50%',
                bgcolor: '#10b981',
                animation: 'pulse 1s infinite',
                '@keyframes pulse': {
                  '0%': { opacity: 1 },
                  '50%': { opacity: 0.5 },
                  '100%': { opacity: 1 },
                },
              }}
            />
            <Typography variant="caption" sx={{ color: '#10b981', fontSize: 11 }}>
              Running...
            </Typography>
          </Box>
        )}
      </Box>
    </Box>
  );
};

export default CodeEditor;
