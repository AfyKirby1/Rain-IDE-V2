import React, { useState, useEffect } from 'react';
import { Box, Paper, IconButton, Tooltip, Slide } from '@mui/material';
import {
  FolderOpen,
  Code,
  Terminal, 
  Chat, 
  ChevronLeft,
  ChevronRight,
  KeyboardArrowUp,
  KeyboardArrowDown,
  SmartToy,
} from '@mui/icons-material';
import { useAIStore } from '../stores/aiStore';
import FileExplorer from './FileExplorer';
import CodeEditor from './CodeEditor';
import IntegratedTerminal from './IntegratedTerminal';
import AIChatPanel from './AIChatPanel';
import ModelPickerPanel from './ModelPickerPanel';
import FloatingModelButton from './FloatingModelButton';

interface PanelState {
  fileExplorer: boolean;
  codeEditor: boolean;
  terminal: boolean;
  aiChat: boolean;
}

const IDELayout: React.FC = () => {
  const { currentModel } = useAIStore();
  
  console.log('IDELayout: Component rendered');
  const [panels, setPanels] = useState<PanelState>({
    fileExplorer: true,
    codeEditor: true,
    terminal: true,
    aiChat: true,
  });

  const [sidebarWidth] = useState(280);
  const [bottomHeight, setBottomHeight] = useState(200);
  const [modelPickerOpen, setModelPickerOpen] = useState(false);

  const togglePanel = (panel: keyof PanelState) => {
    setPanels(prev => ({ ...prev, [panel]: !prev[panel] }));
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Only handle shortcuts when not typing in input fields
      if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
        return;
      }

      const { ctrlKey, metaKey, shiftKey, key } = event;
      const isCtrlOrCmd = ctrlKey || metaKey;

      if (isCtrlOrCmd && !shiftKey) {
        switch (key.toLowerCase()) {
          case 'b':
            event.preventDefault();
            togglePanel('fileExplorer');
            break;
          case 'j':
            event.preventDefault();
            togglePanel('terminal');
            break;
          case 'l':
            event.preventDefault();
            togglePanel('aiChat');
            break;
          case 'm':
            event.preventDefault();
            setModelPickerOpen(true);
            break;
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  const getPanelIcon = (panel: keyof PanelState) => {
    const icons = {
      fileExplorer: FolderOpen,
      codeEditor: Code,
      terminal: Terminal,
      aiChat: Chat,
    };
    const Icon = icons[panel];
    return <Icon fontSize="small" />;
  };

  return (
    <Box
      sx={{
        height: '100%',
        maxHeight: '100%',
        display: 'flex',
        flexDirection: 'column',
        bgcolor: '#0f172a',
        color: 'white',
        overflow: 'hidden',
      }}
    >
      {/* Top Toolbar */}
      <Box
        sx={{
          height: 40,
          background: 'linear-gradient(90deg, #1e293b 0%, #334155 100%)',
          display: 'flex',
          alignItems: 'center',
          px: 2,
          borderBottom: '1px solid #475569',
        }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Tooltip title="File Explorer (Ctrl+B)">
            <IconButton
              size="small"
              onClick={() => togglePanel('fileExplorer')}
              sx={{
                color: panels.fileExplorer ? '#60a5fa' : '#94a3b8',
                '&:hover': { bgcolor: 'rgba(255, 255, 255, 0.1)' }
              }}
            >
              {getPanelIcon('fileExplorer')}
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Code Editor">
            <IconButton
              size="small"
              onClick={() => togglePanel('codeEditor')}
              sx={{
                color: panels.codeEditor ? '#60a5fa' : '#94a3b8',
                '&:hover': { bgcolor: 'rgba(255, 255, 255, 0.1)' }
              }}
            >
              {getPanelIcon('codeEditor')}
            </IconButton>
          </Tooltip>
          
          <Tooltip title="AI Chat (Ctrl+L)">
            <IconButton
              size="small"
              onClick={() => togglePanel('aiChat')}
              sx={{
                color: panels.aiChat ? '#60a5fa' : '#94a3b8',
                '&:hover': { bgcolor: 'rgba(255, 255, 255, 0.1)' }
              }}
            >
              {getPanelIcon('aiChat')}
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Terminal (Ctrl+J)">
            <IconButton
              size="small"
              onClick={() => togglePanel('terminal')}
              sx={{
                color: panels.terminal ? '#60a5fa' : '#94a3b8',
                '&:hover': { bgcolor: 'rgba(255, 255, 255, 0.1)' }
              }}
            >
              {getPanelIcon('terminal')}
            </IconButton>
          </Tooltip>

          <Tooltip title="Model Picker (Ctrl+M)">
            <IconButton
              size="small"
              onClick={() => setModelPickerOpen(true)}
              sx={{
                color: currentModel ? '#10b981' : '#94a3b8',
                '&:hover': { bgcolor: 'rgba(255, 255, 255, 0.1)' }
              }}
            >
              <SmartToy />
            </IconButton>
          </Tooltip>
          
        </Box>

        <Box sx={{ flex: 1 }} />

        {/* AI Model Status */}
        {currentModel && (
          <Box
            sx={{
              display: 'flex',
              alignItems: 'center',
              gap: 1,
              px: 2,
              py: 0.5,
              bgcolor: 'rgba(16, 185, 129, 0.1)',
              borderRadius: 1,
              border: '1px solid rgba(16, 185, 129, 0.3)',
            }}
          >
            <Box
              sx={{
                width: 8,
                height: 8,
                borderRadius: '50%',
                bgcolor: '#10b981',
                animation: 'pulse 2s infinite',
                '@keyframes pulse': {
                  '0%': { opacity: 1 },
                  '50%': { opacity: 0.5 },
                  '100%': { opacity: 1 },
                },
              }}
            />
            <Box sx={{ fontSize: 12, color: '#10b981', fontWeight: 500 }}>
              {currentModel.name}
            </Box>
          </Box>
        )}
      </Box>

      {/* Main Content Area */}
      <Box sx={{ flex: 1, display: 'flex', overflow: 'hidden', minHeight: 0 }}>
        {/* Left Sidebar - File Explorer */}
        {panels.fileExplorer && (
          <Slide direction="right" in={panels.fileExplorer} timeout={300}>
            <Paper
              elevation={0}
              sx={{
                width: sidebarWidth,
                minWidth: 200,
                maxWidth: 400,
                bgcolor: '#1e293b',
                borderRight: '1px solid #475569',
                display: 'flex',
                flexDirection: 'column',
              }}
            >
            <Box
              sx={{
                height: 32,
                px: 2,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                borderBottom: '1px solid #475569',
                bgcolor: '#334155',
              }}
            >
              <Box sx={{ fontSize: 12, fontWeight: 600, color: '#e2e8f0' }}>
                EXPLORER
              </Box>
              <IconButton
                size="small"
                onClick={() => togglePanel('fileExplorer')}
                sx={{ color: '#94a3b8', p: 0.5 }}
              >
                <ChevronLeft fontSize="small" />
              </IconButton>
            </Box>
            <Box sx={{ flex: 1, overflow: 'auto' }}>
              <FileExplorer />
            </Box>
            </Paper>
          </Slide>
        )}

        {/* Center Area - Code Editor */}
        <Box sx={{ flex: 1, display: 'flex', flexDirection: 'column', minHeight: 0, overflow: 'hidden' }}>
          {panels.codeEditor && (
            <Paper
              elevation={0}
              sx={{
                flex: 1,
                bgcolor: '#0f172a',
                borderRight: '1px solid #475569',
                display: 'flex',
                flexDirection: 'column',
                minHeight: 300,
              }}
            >
              <Box
                sx={{
                  height: 32,
                  px: 2,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between',
                  borderBottom: '1px solid #475569',
                  bgcolor: '#1e293b',
                }}
              >
                <Box sx={{ fontSize: 12, fontWeight: 600, color: '#e2e8f0' }}>
                  EDITOR
                </Box>
                <IconButton
                  size="small"
                  onClick={() => togglePanel('codeEditor')}
                  sx={{ color: '#94a3b8', p: 0.5 }}
                >
                  <KeyboardArrowUp fontSize="small" />
                </IconButton>
              </Box>
              <Box sx={{ flex: 1, overflow: 'hidden' }}>
                <CodeEditor />
              </Box>
            </Paper>
          )}

          {/* Bottom Area - Terminal and Performance */}
          <Box sx={{ display: 'flex', flexDirection: 'column' }}>
            {panels.terminal && (
              <Paper
                elevation={0}
                sx={{
                  height: bottomHeight,
                  minHeight: 120,
                  maxHeight: 400,
                  bgcolor: '#0f172a',
                  borderTop: '1px solid #475569',
                  display: 'flex',
                  flexDirection: 'column',
                }}
              >
                <Box
                  sx={{
                    height: 32,
                    px: 2,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    borderBottom: '1px solid #475569',
                    bgcolor: '#1e293b',
                  }}
                >
                  <Box sx={{ fontSize: 12, fontWeight: 600, color: '#e2e8f0' }}>
                    TERMINAL
                  </Box>
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                    <IconButton
                      size="small"
                      onClick={() => setBottomHeight(prev => Math.max(120, prev - 20))}
                      sx={{ color: '#94a3b8', p: 0.5 }}
                    >
                      <KeyboardArrowDown fontSize="small" />
                    </IconButton>
                    <IconButton
                      size="small"
                      onClick={() => setBottomHeight(prev => Math.min(400, prev + 20))}
                      sx={{ color: '#94a3b8', p: 0.5 }}
                    >
                      <KeyboardArrowUp fontSize="small" />
                    </IconButton>
                    <IconButton
                      size="small"
                      onClick={() => togglePanel('terminal')}
                      sx={{ color: '#94a3b8', p: 0.5 }}
                    >
                      <KeyboardArrowDown fontSize="small" />
                    </IconButton>
                  </Box>
                </Box>
                <Box sx={{ flex: 1, overflow: 'hidden' }}>
                  <IntegratedTerminal />
                </Box>
              </Paper>
            )}
            
          </Box>
        </Box>

        {/* Right Sidebar - AI Chat */}
        {panels.aiChat && (
          <Slide direction="left" in={panels.aiChat} timeout={300}>
            <Paper
              elevation={0}
              sx={{
                width: 320,
                minWidth: 280,
                maxWidth: 500,
                height: '100%',
                maxHeight: '100%',
                bgcolor: '#1e293b',
                borderLeft: '1px solid #475569',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'hidden',
              }}
            >
            <Box
              sx={{
                height: 32,
                px: 2,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                borderBottom: '1px solid #475569',
                bgcolor: '#334155',
              }}
            >
              <Box sx={{ fontSize: 12, fontWeight: 600, color: '#e2e8f0' }}>
                AI CHAT
              </Box>
              <IconButton
                size="small"
                onClick={() => togglePanel('aiChat')}
                sx={{ color: '#94a3b8', p: 0.5 }}
              >
                <ChevronRight fontSize="small" />
              </IconButton>
            </Box>
            <Box sx={{ flex: 1, overflow: 'hidden' }}>
              <AIChatPanel onOpenModelPicker={() => setModelPickerOpen(true)} />
            </Box>
            </Paper>
          </Slide>
        )}
      </Box>

      {/* Model Picker Panel */}
      <ModelPickerPanel
        open={modelPickerOpen}
        onClose={() => setModelPickerOpen(false)}
      />

      {/* Floating Model Button */}
      <FloatingModelButton
        onOpenModelPicker={() => setModelPickerOpen(true)}
      />
    </Box>
  );
};

export default IDELayout;
