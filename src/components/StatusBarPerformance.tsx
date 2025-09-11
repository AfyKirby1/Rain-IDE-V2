import React, { useState, useEffect } from 'react';
import { Box, Typography, Chip } from '@mui/material';
import { invoke } from '@tauri-apps/api/core';

interface PerformanceMetrics {
  cpu_usage: number;
  memory_percentage: number;
  process_cpu: number;
  process_memory: number;
}

const StatusBarPerformance: React.FC = () => {
  const [metrics, setMetrics] = useState<PerformanceMetrics | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const fetchMetrics = async () => {
    try {
      setIsLoading(true);
      const metricsData = await invoke<PerformanceMetrics>('get_performance_metrics');
      setMetrics(metricsData);
    } catch (error) {
      console.error('Failed to fetch performance metrics:', error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchMetrics();
    const interval = setInterval(fetchMetrics, 3000); // Update every 3 seconds
    return () => clearInterval(interval);
  }, []);

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

  if (!metrics) {
    return (
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
        <Typography variant="caption" sx={{ color: '#6b7280', fontSize: 10 }}>
          Loading...
        </Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
      {/* CPU Usage */}
      <Chip
        label={`CPU: ${metrics.cpu_usage.toFixed(0)}%`}
        size="small"
        sx={{
          height: 16,
          fontSize: 9,
          bgcolor: getCpuColor(metrics.cpu_usage),
          color: 'white',
          '& .MuiChip-label': { px: 0.5 }
        }}
      />
      
      {/* Memory Usage */}
      <Chip
        label={`RAM: ${metrics.memory_percentage.toFixed(0)}%`}
        size="small"
        sx={{
          height: 16,
          fontSize: 9,
          bgcolor: getMemoryColor(metrics.memory_percentage),
          color: 'white',
          '& .MuiChip-label': { px: 0.5 }
        }}
      />
      
      {/* Process CPU */}
      <Chip
        label={`App: ${metrics.process_cpu.toFixed(0)}%`}
        size="small"
        sx={{
          height: 16,
          fontSize: 9,
          bgcolor: '#3b82f6',
          color: 'white',
          '& .MuiChip-label': { px: 0.5 }
        }}
      />
      
      {/* Status indicator */}
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
    </Box>
  );
};

export default StatusBarPerformance;
