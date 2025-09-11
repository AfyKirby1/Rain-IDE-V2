import React, { useState, useEffect, useRef } from 'react';
import {
  Box,
  Typography,
  LinearProgress,
  Chip,
  IconButton,
  Tooltip,
  Grid,
  Card,
  CardContent,
  Divider,
} from '@mui/material';
import {
  Memory,
  Refresh,
  Monitor,
  Computer,
} from '@mui/icons-material';
import { invoke } from '@tauri-apps/api/core';

interface PerformanceMetrics {
  timestamp: number;
  cpu_usage: number;
  memory_usage: number;
  memory_total: number;
  memory_percentage: number;
  process_cpu: number;
  process_memory: number;
  process_threads: number;
  system_load: number;
  uptime: number;
}

interface SystemInfo {
  os_name: string;
  os_version: string;
  host_name: string;
  cpu_count: number;
  total_memory: number;
  total_swap: number;
  kernel_version: string;
}

const PerformancePanel: React.FC = () => {
  const [metrics, setMetrics] = useState<PerformanceMetrics | null>(null);
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [lastUpdate, setLastUpdate] = useState<Date | null>(null);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  const fetchMetrics = async () => {
    try {
      setIsLoading(true);
      const [metricsData, systemData] = await Promise.all([
        invoke<PerformanceMetrics>('get_performance_metrics'),
        invoke<SystemInfo>('get_system_info'),
      ]);
      
      setMetrics(metricsData);
      setSystemInfo(systemData);
      setLastUpdate(new Date());
    } catch (error) {
      console.error('Failed to fetch performance metrics:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const formatBytes = (bytes: number): string => {
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
  };

  const formatUptime = (seconds: number): string => {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    
    if (days > 0) return `${days}d ${hours}h ${minutes}m`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const getCpuColor = (usage: number): string => {
    if (usage < 30) return '#10b981'; // green
    if (usage < 60) return '#f59e0b'; // yellow
    if (usage < 80) return '#f97316'; // orange
    return '#ef4444'; // red
  };

  const getMemoryColor = (percentage: number): string => {
    if (percentage < 50) return '#10b981'; // green
    if (percentage < 70) return '#f59e0b'; // yellow
    if (percentage < 85) return '#f97316'; // orange
    return '#ef4444'; // red
  };

  useEffect(() => {
    fetchMetrics();
    
    // Set up auto-refresh every 2 seconds
    intervalRef.current = setInterval(fetchMetrics, 2000);
    
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, []);

  if (!metrics || !systemInfo) {
    return (
      <Box
        sx={{
          height: '100%',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          flexDirection: 'column',
          p: 3,
        }}
      >
        <Box
          sx={{
            width: 60,
            height: 60,
            borderRadius: 2,
            bgcolor: 'rgba(59, 130, 246, 0.1)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            mb: 2,
          }}
        >
          <Monitor sx={{ fontSize: 32, color: '#3b82f6' }} />
        </Box>
        <Typography variant="h6" sx={{ color: '#e2e8f0', mb: 1, textAlign: 'center' }}>
          Performance Monitor
        </Typography>
        <Typography variant="body2" sx={{ color: '#94a3b8', textAlign: 'center' }}>
          Loading system metrics...
        </Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', p: 2 }}>
      {/* Header */}
      <Box
        sx={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          mb: 2,
        }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Monitor sx={{ color: '#60a5fa' }} />
          <Typography variant="h6" sx={{ color: '#e2e8f0', fontSize: 14, fontWeight: 600 }}>
            Performance Monitor
          </Typography>
        </Box>
        
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          {lastUpdate && (
            <Typography variant="caption" sx={{ color: '#6b7280', fontSize: 10 }}>
              {lastUpdate.toLocaleTimeString()}
            </Typography>
          )}
          <Tooltip title="Refresh">
            <IconButton
              size="small"
              onClick={fetchMetrics}
              disabled={isLoading}
              sx={{ color: '#94a3b8' }}
            >
              <Refresh fontSize="small" />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      {/* System Info */}
      <Card sx={{ mb: 2, bgcolor: '#1e293b', border: '1px solid #475569' }}>
        <CardContent sx={{ p: 2 }}>
          <Typography variant="subtitle2" sx={{ color: '#e2e8f0', mb: 1, fontSize: 12 }}>
            System Information
          </Typography>
          <Grid container spacing={1}>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                OS: {systemInfo.os_name} {systemInfo.os_version}
              </Typography>
            </Grid>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                CPU Cores: {systemInfo.cpu_count}
              </Typography>
            </Grid>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                Total RAM: {formatBytes(systemInfo.total_memory)}
              </Typography>
            </Grid>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                Uptime: {formatUptime(metrics.uptime)}
              </Typography>
            </Grid>
          </Grid>
        </CardContent>
      </Card>

      {/* CPU Usage */}
      <Card sx={{ mb: 2, bgcolor: '#1e293b', border: '1px solid #475569' }}>
        <CardContent sx={{ p: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <Computer sx={{ fontSize: 16, color: getCpuColor(metrics.cpu_usage) }} />
              <Typography variant="subtitle2" sx={{ color: '#e2e8f0', fontSize: 12 }}>
                CPU Usage
              </Typography>
            </Box>
            <Chip
              label={`${metrics.cpu_usage.toFixed(1)}%`}
              size="small"
              sx={{
                bgcolor: getCpuColor(metrics.cpu_usage),
                color: 'white',
                fontSize: 10,
                height: 20,
              }}
            />
          </Box>
          <LinearProgress
            variant="determinate"
            value={metrics.cpu_usage}
            sx={{
              height: 6,
              borderRadius: 3,
              bgcolor: 'rgba(255, 255, 255, 0.1)',
              '& .MuiLinearProgress-bar': {
                bgcolor: getCpuColor(metrics.cpu_usage),
              },
            }}
          />
        </CardContent>
      </Card>

      {/* Memory Usage */}
      <Card sx={{ mb: 2, bgcolor: '#1e293b', border: '1px solid #475569' }}>
        <CardContent sx={{ p: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <Memory sx={{ fontSize: 16, color: getMemoryColor(metrics.memory_percentage) }} />
              <Typography variant="subtitle2" sx={{ color: '#e2e8f0', fontSize: 12 }}>
                Memory Usage
              </Typography>
            </Box>
            <Chip
              label={`${metrics.memory_percentage.toFixed(1)}%`}
              size="small"
              sx={{
                bgcolor: getMemoryColor(metrics.memory_percentage),
                color: 'white',
                fontSize: 10,
                height: 20,
              }}
            />
          </Box>
          <LinearProgress
            variant="determinate"
            value={metrics.memory_percentage}
            sx={{
              height: 6,
              borderRadius: 3,
              bgcolor: 'rgba(255, 255, 255, 0.1)',
              '& .MuiLinearProgress-bar': {
                bgcolor: getMemoryColor(metrics.memory_percentage),
              },
            }}
          />
          <Typography variant="caption" sx={{ color: '#6b7280', fontSize: 10, mt: 0.5, display: 'block' }}>
            {formatBytes(metrics.memory_usage)} / {formatBytes(metrics.memory_total)}
          </Typography>
        </CardContent>
      </Card>

      {/* Process Info */}
      <Card sx={{ mb: 2, bgcolor: '#1e293b', border: '1px solid #475569' }}>
        <CardContent sx={{ p: 2 }}>
          <Typography variant="subtitle2" sx={{ color: '#e2e8f0', mb: 1, fontSize: 12 }}>
            RAIN.CHAT Process
          </Typography>
          <Grid container spacing={1}>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                CPU: {metrics.process_cpu.toFixed(1)}%
              </Typography>
            </Grid>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                Memory: {formatBytes(metrics.process_memory)}
              </Typography>
            </Grid>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                Threads: {metrics.process_threads}
              </Typography>
            </Grid>
            <Grid item xs={6}>
              <Typography variant="caption" sx={{ color: '#94a3b8', fontSize: 10 }}>
                Load: {metrics.system_load.toFixed(2)}
              </Typography>
            </Grid>
          </Grid>
        </CardContent>
      </Card>

      {/* Status */}
      <Box sx={{ mt: 'auto', pt: 2 }}>
        <Divider sx={{ borderColor: '#475569', mb: 1 }} />
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <Typography variant="caption" sx={{ color: '#6b7280', fontSize: 10 }}>
            {isLoading ? 'Updating...' : 'Real-time monitoring'}
          </Typography>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
            <Box
              sx={{
                width: 6,
                height: 6,
                borderRadius: '50%',
                bgcolor: isLoading ? '#f59e0b' : '#10b981',
                animation: isLoading ? 'pulse 1s infinite' : 'none',
                '@keyframes pulse': {
                  '0%': { opacity: 1 },
                  '50%': { opacity: 0.5 },
                  '100%': { opacity: 1 },
                },
              }}
            />
            <Typography variant="caption" sx={{ color: '#6b7280', fontSize: 10 }}>
              {isLoading ? 'Syncing' : 'Active'}
            </Typography>
          </Box>
        </Box>
      </Box>
    </Box>
  );
};

export default PerformancePanel;
