import React, { useState, useEffect, useRef } from 'react';
import {
  Box,
  Typography,
  IconButton,
  Tooltip,
  TextField,
  Chip,
} from '@mui/material';
import {
  PlayArrow,
  Stop,
  Clear,
  Settings,
  Add,
  Close,
  KeyboardArrowDown,
  KeyboardArrowUp,
} from '@mui/icons-material';

interface TerminalSession {
  id: string;
  name: string;
  active: boolean;
  history: string[];
  currentCommand: string;
}

const IntegratedTerminal: React.FC = () => {
  const [sessions, setSessions] = useState<TerminalSession[]>([
    {
      id: '1',
      name: 'Terminal 1',
      active: true,
      history: [
        'Welcome to RAIN.CHAT v2 Terminal',
        'Type commands below to get started...',
        '',
        'PS C:\\Users\\Ry\\Desktop\\RAIN.CHAT\\rain-chat-v2>',
      ],
      currentCommand: '',
    },
  ]);
  const [currentSession, setCurrentSession] = useState('1');
  const [commandInput, setCommandInput] = useState('');
  const terminalRef = useRef<HTMLDivElement>(null);

  const activeSession = sessions.find(s => s.id === currentSession);

  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [activeSession?.history]);

  const handleCommandSubmit = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      if (!activeSession) return;

      const newHistory = [...activeSession.history, `PS C:\\Users\\Ry\\Desktop\\RAIN.CHAT\\rain-chat-v2> ${commandInput}`];
      
      // Simulate command execution
      if (commandInput.trim() === 'clear') {
        newHistory.push('');
      } else if (commandInput.trim() === 'help') {
        newHistory.push('Available commands:');
        newHistory.push('  clear    - Clear the terminal');
        newHistory.push('  help     - Show this help message');
        newHistory.push('  npm      - Run npm commands');
        newHistory.push('  cargo    - Run cargo commands');
        newHistory.push('  ls       - List directory contents');
        newHistory.push('  pwd      - Print working directory');
        newHistory.push('');
      } else if (commandInput.trim() === 'ls') {
        newHistory.push('Directory of C:\\Users\\Ry\\Desktop\\RAIN.CHAT\\rain-chat-v2');
        newHistory.push('');
        newHistory.push('Mode                 LastWriteTime         Length Name');
        newHistory.push('----                 -------------         ------ ----');
        newHistory.push('d-----        12/19/2024   9:39 PM                src');
        newHistory.push('d-----        12/19/2024   9:39 PM                src-tauri');
        newHistory.push('-a----        12/19/2024   9:39 PM            1234 package.json');
        newHistory.push('-a----        12/19/2024   9:39 PM             567 README.md');
        newHistory.push('');
      } else if (commandInput.trim() === 'pwd') {
        newHistory.push('C:\\Users\\Ry\\Desktop\\RAIN.CHAT\\rain-chat-v2');
        newHistory.push('');
      } else if (commandInput.trim() === 'npm run tauri:dev') {
        newHistory.push('Starting Tauri development server...');
        newHistory.push('Running BeforeDevCommand (`npm run dev`)');
        newHistory.push('Running DevCommand (`cargo run --no-default-features --color always --`)');
        newHistory.push('Info Watching C:\\Users\\Ry\\Desktop\\RAIN.CHAT\\rain-chat-v2\\src-tauri for changes...');
        newHistory.push('');
      } else if (commandInput.trim()) {
        newHistory.push(`Command not found: ${commandInput}`);
        newHistory.push('Type "help" for available commands.');
        newHistory.push('');
      }

      setSessions(prev => 
        prev.map(s => 
          s.id === currentSession 
            ? { ...s, history: newHistory, currentCommand: '' }
            : s
        )
      );
      setCommandInput('');
    }
  };

  const addNewSession = () => {
    const newSession: TerminalSession = {
      id: Date.now().toString(),
      name: `Terminal ${sessions.length + 1}`,
      active: false,
      history: [
        'Welcome to RAIN.CHAT v2 Terminal',
        'Type commands below to get started...',
        '',
        'PS C:\\Users\\Ry\\Desktop\\RAIN.CHAT\\rain-chat-v2>',
      ],
      currentCommand: '',
    };
    setSessions(prev => [...prev, newSession]);
    setCurrentSession(newSession.id);
  };

  const switchSession = (sessionId: string) => {
    setSessions(prev => 
      prev.map(s => ({ ...s, active: s.id === sessionId }))
    );
    setCurrentSession(sessionId);
  };

  const closeSession = (sessionId: string) => {
    if (sessions.length <= 1) return;
    
    const newSessions = sessions.filter(s => s.id !== sessionId);
    setSessions(newSessions);
    
    if (currentSession === sessionId) {
      setCurrentSession(newSessions[0].id);
    }
  };

  const clearTerminal = () => {
    if (!activeSession) return;
    
    setSessions(prev => 
      prev.map(s => 
        s.id === currentSession 
          ? { ...s, history: ['PS C:\\Users\\Ry\\Desktop\\RAIN.CHAT\\rain-chat-v2>'] }
          : s
      )
    );
  };

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      {/* Terminal Tabs */}
      <Box
        sx={{
          height: 32,
          bgcolor: '#1e293b',
          borderBottom: '1px solid #475569',
          display: 'flex',
          alignItems: 'center',
          px: 1,
        }}
      >
        {sessions.map(session => (
          <Box
            key={session.id}
            sx={{
              display: 'flex',
              alignItems: 'center',
              gap: 1,
              px: 2,
              py: 0.5,
              bgcolor: session.active ? '#334155' : 'transparent',
              borderRadius: 1,
              mr: 1,
              cursor: 'pointer',
              '&:hover': { bgcolor: 'rgba(255, 255, 255, 0.05)' },
            }}
            onClick={() => switchSession(session.id)}
          >
            <Typography variant="caption" sx={{ color: '#e2e8f0', fontSize: 11 }}>
              {session.name}
            </Typography>
            {sessions.length > 1 && (
              <IconButton
                size="small"
                onClick={(e) => {
                  e.stopPropagation();
                  closeSession(session.id);
                }}
                sx={{ color: '#94a3b8', p: 0.5 }}
              >
                <Close fontSize="small" />
              </IconButton>
            )}
          </Box>
        ))}

        <IconButton
          size="small"
          onClick={addNewSession}
          sx={{ color: '#94a3b8', ml: 1 }}
        >
          <Add fontSize="small" />
        </IconButton>

        <Box sx={{ flex: 1 }} />

        {/* Terminal Actions */}
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
          <Tooltip title="Clear Terminal">
            <IconButton
              size="small"
              onClick={clearTerminal}
              sx={{ color: '#94a3b8' }}
            >
              <Clear fontSize="small" />
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

      {/* Terminal Output */}
      <Box
        ref={terminalRef}
        sx={{
          flex: 1,
          bgcolor: '#0f172a',
          overflow: 'auto',
          p: 1,
          fontFamily: 'Monaco, Consolas, "Courier New", monospace',
          fontSize: 13,
          lineHeight: 1.4,
        }}
      >
        {activeSession?.history.map((line, index) => (
          <Box
            key={index}
            sx={{
              color: line.startsWith('PS ') ? '#10b981' : '#e2e8f0',
              mb: 0.5,
              whiteSpace: 'pre-wrap',
            }}
          >
            {line}
          </Box>
        ))}
        
        {/* Command Input */}
        <Box sx={{ display: 'flex', alignItems: 'center', mt: 1 }}>
          <Typography
            sx={{
              color: '#10b981',
              fontFamily: 'Monaco, Consolas, "Courier New", monospace',
              fontSize: 13,
              mr: 1,
            }}
          >
            PS C:\Users\Ry\Desktop\RAIN.CHAT\rain-chat-v2&gt;
          </Typography>
          <Box
            sx={{
              flex: 1,
              position: 'relative',
            }}
          >
            <input
              type="text"
              value={commandInput}
              onChange={(e) => setCommandInput(e.target.value)}
              onKeyDown={handleCommandSubmit}
              style={{
                background: 'transparent',
                border: 'none',
                outline: 'none',
                color: '#e2e8f0',
                fontFamily: 'Monaco, Consolas, "Courier New", monospace',
                fontSize: 13,
                width: '100%',
              }}
              autoFocus
            />
            <Box
              sx={{
                position: 'absolute',
                right: 0,
                top: 0,
                width: 2,
                height: 16,
                bgcolor: '#e2e8f0',
                animation: 'blink 1s infinite',
                '@keyframes blink': {
                  '0%': { opacity: 1 },
                  '50%': { opacity: 0 },
                  '100%': { opacity: 1 },
                },
              }}
            />
          </Box>
        </Box>
      </Box>
    </Box>
  );
};

export default IntegratedTerminal;
