import React, { useState } from 'react';
import {
  Box,
  Typography,
  Button,
  Card,
  CardContent,
  List,
  ListItem,
  ListItemText,
  ListItemIcon,
  Divider,
  CircularProgress,
} from '@mui/material';
import {
  FolderOpen,
  CreateNewFolder,
  History,
  SmartToy,
} from '@mui/icons-material';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useAppStore } from '../stores/appStore';
import { useProjectStore } from '../stores/projectStore';

const WelcomeScreen: React.FC = () => {
  const { recentProjects } = useAppStore();
  const { setCurrentProject } = useProjectStore();
  const [isOpeningProject, setIsOpeningProject] = useState(false);

  const handleOpenProject = async () => {
    try {
      setIsOpeningProject(true);
      
      // Test connection first
      console.log('Testing connection...');
      const testResult = await invoke('test_project_connection') as string;
      console.log('Connection test result:', testResult);
      
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected && typeof selected === 'string') {
        // Add timeout to prevent infinite hang
        const timeoutPromise = new Promise((_, reject) => 
          setTimeout(() => reject(new Error('Project opening timed out after 10 seconds')), 10000)
        );
        
        const openProjectPromise = invoke('open_project', { path: selected });
        const result = await Promise.race([openProjectPromise, timeoutPromise]) as any;
        setCurrentProject(result);
      }
    } catch (error) {
      console.error('Failed to open project:', error);
      // You could add a toast notification here
    } finally {
      setIsOpeningProject(false);
    }
  };

  const handleCreateProject = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected && typeof selected === 'string') {
        const result = await invoke('create_project', {
          path: selected,
          project_type: 'general'
        }) as any;
        
        setCurrentProject(result);
      }
    } catch (error) {
      console.error('Failed to create project:', error);
    }
  };

  return (
    <Box
      sx={{
        height: '100%',
        background: 'linear-gradient(135deg, #1e293b 0%, #0f172a 100%)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        p: 4,
      }}
    >
      <Box sx={{ maxWidth: 800, width: '100%' }}>
        {/* Header */}
        <Box sx={{ textAlign: 'center', mb: 6 }}>
          <Typography variant="h2" sx={{ fontWeight: 'bold', mb: 2, color: '#3b82f6' }}>
            RAIN.CHAT v2
          </Typography>
          <Typography variant="h5" sx={{ color: 'text.secondary', mb: 1 }}>
            Professional Desktop AI IDE
          </Typography>
          <Typography variant="body1" sx={{ color: 'text.disabled' }}>
            Build, debug, and deploy with AI-powered development tools
          </Typography>
        </Box>

        {/* Action Cards */}
        <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: 3 }}>
          {/* Quick Actions */}
          <Card sx={{ bgcolor: 'background.paper', border: '1px solid #475569' }}>
            <CardContent>
              <Typography variant="h6" sx={{ mb: 3, display: 'flex', alignItems: 'center' }}>
                <SmartToy sx={{ mr: 1, color: '#10b981' }} />
                Get Started
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Button
                  variant="contained"
                  startIcon={isOpeningProject ? <CircularProgress size={20} /> : <FolderOpen />}
                  onClick={handleOpenProject}
                  disabled={isOpeningProject}
                  sx={{ justifyContent: 'flex-start' }}
                >
                  {isOpeningProject ? 'Opening...' : 'Open Project'}
                </Button>
                <Button
                  variant="outlined"
                  startIcon={<CreateNewFolder />}
                  onClick={handleCreateProject}
                  sx={{ justifyContent: 'flex-start' }}
                >
                  Create New Project
                </Button>
              </Box>
            </CardContent>
          </Card>

          {/* Recent Projects */}
          <Card sx={{ bgcolor: 'background.paper', border: '1px solid #475569' }}>
            <CardContent>
              <Typography variant="h6" sx={{ mb: 2, display: 'flex', alignItems: 'center' }}>
                <History sx={{ mr: 1, color: '#f59e0b' }} />
                Recent Projects
              </Typography>
              {recentProjects.length > 0 ? (
                <List dense>
                  {recentProjects.slice(0, 5).map((project, index) => (
                    <React.Fragment key={project.id}>
                      <ListItem
                        button
                        onClick={async () => {
                          try {
                            console.log('Opening recent project:', project.path);
                            const result = await invoke('open_project', { path: project.path }) as any;
                            console.log('Recent project opened:', result);
                            setCurrentProject(result);
                          } catch (error) {
                            console.error('Failed to open recent project:', error);
                          }
                        }}
                        sx={{
                          borderRadius: 1,
                          '&:hover': { bgcolor: 'action.hover' }
                        }}
                      >
                        <ListItemIcon>
                          <FolderOpen fontSize="small" />
                        </ListItemIcon>
                        <ListItemText
                          primary={project.name}
                          secondary={project.path}
                          secondaryTypographyProps={{ noWrap: true, fontSize: 12 }}
                        />
                      </ListItem>
                      {index < Math.min(recentProjects.length - 1, 4) && <Divider />}
                    </React.Fragment>
                  ))}
                </List>
              ) : (
                <Typography variant="body2" sx={{ color: 'text.disabled', fontStyle: 'italic' }}>
                  No recent projects
                </Typography>
              )}
            </CardContent>
          </Card>
        </Box>

        {/* Features */}
        <Box sx={{ mt: 6, textAlign: 'center' }}>
          <Typography variant="body2" sx={{ color: 'text.disabled' }}>
            AI-powered code completion • Local model inference • Integrated debugger • Git integration • LSP support
          </Typography>
        </Box>
      </Box>
    </Box>
  );
};

export default WelcomeScreen;