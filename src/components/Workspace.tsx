import React from 'react';
import { Box } from '@mui/material';
import { useProjectStore } from '../stores/projectStore';
import IDELayout from './IDELayout';
import WelcomeScreen from './WelcomeScreen';

const Workspace: React.FC = () => {
  const { currentProject } = useProjectStore();

  return (
    <Box
      sx={{
        height: '100%',
        maxHeight: '100%',
        display: 'flex',
        flexDirection: 'column',
        bgcolor: 'background.default',
        overflow: 'hidden',
      }}
    >
      {currentProject ? (
        <IDELayout />
      ) : (
        <WelcomeScreen />
      )}
    </Box>
  );
};

export default Workspace;