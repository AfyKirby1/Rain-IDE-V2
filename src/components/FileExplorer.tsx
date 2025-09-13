import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Collapse,
  IconButton,
  Tooltip,
} from '@mui/material';
import {
  Folder,
  FolderOpen,
  InsertDriveFile,
  Code,
  Image,
  Description,
  DataObject,
  ChevronRight,
  KeyboardArrowDown,
  Add,
  Refresh,
} from '@mui/icons-material';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../stores/projectStore';
import { useEditorStore } from '../stores/editorStore';

interface FileNode {
  name: string;
  path: string;
  is_directory: boolean;
  children?: FileNode[];
  expanded?: boolean;
}

const FileExplorer: React.FC = () => {
  const { currentProject } = useProjectStore();
  const { openFile, createUntitledFile } = useEditorStore();
  const [fileTree, setFileTree] = useState<FileNode[]>([]);
  const [loading, setLoading] = useState(false);
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (currentProject) {
      loadFileTree();
    }
  }, [currentProject]);

  const loadFileTree = async () => {
    if (!currentProject) return;
    
    setLoading(true);
    try {
      const files = await invoke('list_directory', { 
        directoryPath: currentProject.path 
      }) as FileNode[];
      setFileTree(files);
    } catch (error) {
      console.error('Failed to load file tree:', error);
    } finally {
      setLoading(false);
    }
  };

  const toggleNode = (nodePath: string) => {
    const newExpanded = new Set(expandedNodes);
    if (newExpanded.has(nodePath)) {
      newExpanded.delete(nodePath);
    } else {
      newExpanded.add(nodePath);
    }
    setExpandedNodes(newExpanded);
  };

  const handleFileClick = async (node: FileNode) => {
    if (!node.is_directory) {
      try {
        console.log('Opening file:', node.path);
        await openFile(node.path);
        console.log('File opened successfully');
      } catch (error) {
        console.error('Failed to open file:', error);
      }
    } else {
      toggleNode(node.path);
    }
  };

  const handleNewFile = () => {
    // Create an untitled file immediately (like Cursor)
    createUntitledFile('text');
    console.log('Created untitled file');
  };

  const getFileIcon = (fileName: string, is_directory: boolean) => {
    // Safety check for undefined fileName
    if (!fileName) {
      return <Description fontSize="small" />;
    }
    
    if (is_directory) {
      return expandedNodes.has(fileName) ? <FolderOpen fontSize="small" /> : <Folder fontSize="small" />;
    }

    const extension = fileName.split('.').pop()?.toLowerCase();
    switch (extension) {
      case 'ts':
      case 'tsx':
      case 'js':
      case 'jsx':
        return <Code fontSize="small" />;
      case 'json':
        return <DataObject fontSize="small" />;
      case 'md':
        return <Description fontSize="small" />;
      case 'png':
      case 'jpg':
      case 'jpeg':
      case 'gif':
      case 'svg':
        return <Image fontSize="small" />;
      default:
        return <InsertDriveFile fontSize="small" />;
    }
  };

  const renderFileNode = (node: FileNode, level: number = 0) => {
    const isExpanded = expandedNodes.has(node.path);
    const hasChildren = node.children && node.children.length > 0;

    return (
      <Box key={node.path}>
        <ListItem
          disablePadding
          sx={{
            pl: level * 2,
            '&:hover': { bgcolor: 'rgba(255, 255, 255, 0.05)' },
          }}
        >
          <ListItemButton
            onClick={() => handleFileClick(node)}
            sx={{
              py: 0.5,
              minHeight: 32,
            }}
          >
            <ListItemIcon sx={{ minWidth: 32, color: '#94a3b8' }}>
              {node.is_directory && hasChildren ? (
                isExpanded ? <KeyboardArrowDown fontSize="small" /> : <ChevronRight fontSize="small" />
              ) : (
                <Box sx={{ width: 16 }} />
              )}
            </ListItemIcon>
            <ListItemIcon sx={{ minWidth: 24, color: '#94a3b8' }}>
              {getFileIcon(node.name, node.is_directory)}
            </ListItemIcon>
            <ListItemText
              primary={
                <Typography
                  variant="body2"
                  sx={{
                    fontSize: 13,
                    color: node.is_directory ? '#e2e8f0' : '#cbd5e1',
                    fontFamily: 'Monaco, Consolas, "Courier New", monospace',
                  }}
                >
                  {node.name}
                </Typography>
              }
            />
          </ListItemButton>
        </ListItem>
        
        {node.is_directory && hasChildren && (
          <Collapse in={isExpanded} timeout="auto" unmountOnExit>
            <List component="div" disablePadding>
              {node.children?.map(child => renderFileNode(child, level + 1))}
            </List>
          </Collapse>
        )}
      </Box>
    );
  };

  if (!currentProject) {
    return (
      <Box
        sx={{
          p: 3,
          textAlign: 'center',
          color: '#94a3b8',
        }}
      >
        <Folder sx={{ fontSize: 48, mb: 2, opacity: 0.5 }} />
        <Typography variant="body2">
          No project open
        </Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      {/* Toolbar */}
      <Box
        sx={{
          p: 1,
          borderBottom: '1px solid #475569',
          display: 'flex',
          gap: 0.5,
        }}
      >
        <Tooltip title="Refresh">
          <IconButton
            size="small"
            onClick={loadFileTree}
            disabled={loading}
            sx={{ color: '#94a3b8' }}
          >
            <Refresh fontSize="small" />
          </IconButton>
        </Tooltip>
        <Tooltip title="New File">
          <IconButton
            size="small"
            onClick={handleNewFile}
            sx={{ color: '#94a3b8' }}
          >
            <Add fontSize="small" />
          </IconButton>
        </Tooltip>
      </Box>

      {/* File Tree */}
      <Box sx={{ flex: 1, overflow: 'auto' }}>
        {loading ? (
          <Box sx={{ p: 2, textAlign: 'center' }}>
            <Typography variant="body2" sx={{ color: '#94a3b8' }}>
              Loading...
            </Typography>
          </Box>
        ) : (
          <List dense>
            {fileTree.map(node => renderFileNode(node))}
          </List>
        )}
      </Box>
    </Box>
  );
};

export default FileExplorer;
