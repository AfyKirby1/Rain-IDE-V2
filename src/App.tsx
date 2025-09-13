import { useEffect, useState } from 'react';
import { Routes, Route } from 'react-router-dom';
import { Box, LinearProgress } from '@mui/material';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Components
import Workspace from './components/Workspace';
// import WelcomeScreen from './components/WelcomeScreen';
import StatusBar from './components/StatusBar';
import TitleBar from './components/TitleBar';

// Stores
import { useAppStore } from './stores/appStore';
import { useProjectStore } from './stores/projectStore';
import { useAIStore } from './stores/aiStore';

// Types
import type { Project } from './types/project';

function App() {
  const [isInitializing, setIsInitializing] = useState(true);
  const [initError, setInitError] = useState<string | null>(null);
  const [hasDiscoveredModels, setHasDiscoveredModels] = useState(false);
  
  const { 
    setTheme, 
    setSettings, 
    addRecentProject
  } = useAppStore();
  
  const { 
    setCurrentProject, 
    setProjects 
  } = useProjectStore();
  
  const { 
    setAvailableModels, 
    setCurrentModel,
    setDiscoveringModels,
    isDiscoveringModels
  } = useAIStore();

  useEffect(() => {
    initializeApp();
    setupEventListeners();
  }, []);

  const initializeApp = async () => {
    try {
      console.log('Initializing app...');
      // Load application settings
      console.log('Fetching settings...');
      const settings = await invoke('get_settings') as any;
      setSettings(settings);
      console.log('Settings loaded:', settings);
      
      // Load recent projects
      console.log('Fetching recent projects...');
      const recentProjects = await invoke('get_recent_projects', { limit: 10 }) as any[];
      setProjects(recentProjects);
      console.log('Recent projects loaded:', recentProjects);
      
      console.log('Initialization complete.');
      setIsInitializing(false);

      // Discover and load available AI models in background (non-blocking)
      if (!hasDiscoveredModels) {
        setHasDiscoveredModels(true);
        discoverModelsInBackground(settings);
      }
    } catch (error) {
      console.error('Failed to initialize app:', error);
      setInitError(error instanceof Error ? error.message : 'Unknown error');
      setIsInitializing(false);
    }
  };

  const discoverModelsInBackground = async (settings: any) => {
    try {
      setDiscoveringModels(true);
      console.log('🔍 Discovering models in background...');
      await invoke('discover_models');
      console.log('Model discovery complete.');
      
      console.log('Fetching available models...');
      const models = await invoke('get_available_models') as any[];
      setAvailableModels(models);
      console.log('Available models:', models);
      
      // Auto-load preferred model if configured
      const autoLoadModel = settings?.ai?.auto_load_model;
      if (autoLoadModel && models.length > 0) {
        const preferredModel = settings?.ai?.preferred_models?.[0];
        const modelToLoad = models.find((m: any) => m.name === preferredModel) || models[0];

        try {
          console.log(`Auto-loading model: ${modelToLoad.name}`);
          const ok: boolean = await invoke('load_model_by_name', { modelName: modelToLoad.name });
          if (ok) {
            setCurrentModel(modelToLoad);
            console.log('Model loaded successfully.');
          }
        } catch (error) {
          console.warn('Failed to auto-load model:', error);
        }
      }
    } catch (e) {
      console.warn('Model discovery failed:', e);
    } finally {
      setDiscoveringModels(false);
    }
  };

  const setupEventListeners = () => {
    // Listen for project events
    listen('project-opened', (event) => {
      const project = event.payload as Project;
      setCurrentProject(project);
      addRecentProject(project);
      // Note: setShowWelcome removed - now handled by Workspace component
    });

    // Listen for model events
    listen('model-loaded', (event) => {
      const model = event.payload as any;
      setCurrentModel(model);
    });

    // Listen for theme changes
    listen('theme-changed', (event) => {
      const theme = event.payload as 'dark' | 'light';
      setTheme(theme);
    });

    // Listen for file system events
    listen('file-changed', (event) => {
      // Handle file changes for live reload
      console.log('File changed:', event.payload);
    });

    // Listen for AI events
    listen('ai-response', (event) => {
      // Handle streaming AI responses
      console.log('AI response:', event.payload);
    });
  };

  if (isInitializing) {
    return (
      <Box
        sx={{
          height: '100vh',
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'center',
          alignItems: 'center',
          background: 'linear-gradient(135deg, #1e293b 0%, #0f172a 100%)',
        }}
      >
        <Box sx={{ mb: 4 }}>
          <img 
            src="/RAIN.png" 
            alt="RAIN.CHAT v2" 
            style={{ width: 120, height: 120 }}
          />
        </Box>
        <Box sx={{ width: 300, mb: 2 }}>
          <LinearProgress />
        </Box>
        <Box sx={{ color: 'text.secondary', fontSize: 14 }}>
          Initializing RAIN.CHAT v2...
        </Box>
        {initError && (
          <Box sx={{ color: 'error.main', fontSize: 12, mt: 2, textAlign: 'center' }}>
            Error: {initError}
          </Box>
        )}
      </Box>
    );
  }

  return (
    <Box sx={{ height: '100vh', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
      {/* Custom Title Bar */}
      <TitleBar />
      
      {/* Main Content */}
      <Box sx={{ flex: 1, display: 'flex', flexDirection: 'column', minHeight: 0, overflow: 'hidden' }} className="no-drag-region">
        <Routes>
          <Route path="/" element={<Workspace />} />
          <Route path="/workspace" element={<Workspace />} />
        </Routes>
      </Box>
      
      {/* Model Discovery Progress */}
      {isDiscoveringModels && (
        <Box sx={{ 
          backgroundColor: 'primary.main', 
          color: 'white', 
          px: 2, 
          py: 0.5, 
          fontSize: 12,
          display: 'flex',
          alignItems: 'center',
          gap: 1
        }}>
          <LinearProgress sx={{ color: 'white', width: 100 }} />
          🔍 Discovering AI models...
        </Box>
      )}
      
      {/* Status Bar */}
      <StatusBar />
    </Box>
  );
}

export default App;