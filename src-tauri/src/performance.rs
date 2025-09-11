use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use sysinfo::{System, Pid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: u64,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub memory_total: u64,
    pub memory_percentage: f32,
    pub process_cpu: f32,
    pub process_memory: u64,
    pub process_threads: usize,
    pub system_load: f32,
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceHistory {
    pub cpu_history: Vec<f32>,
    pub memory_history: Vec<f32>,
    pub process_cpu_history: Vec<f32>,
    pub process_memory_history: Vec<u64>,
    pub timestamps: Vec<u64>,
}

pub struct PerformanceMonitor {
    system: Arc<Mutex<System>>,
    history: Arc<Mutex<VecDeque<PerformanceMetrics>>>,
    max_history_size: usize,
    last_update: Arc<Mutex<Instant>>,
    update_interval: Duration,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system: Arc::new(Mutex::new(system)),
            history: Arc::new(Mutex::new(VecDeque::new())),
            max_history_size: 100, // Keep last 100 measurements
            last_update: Arc::new(Mutex::new(Instant::now())),
            update_interval: Duration::from_secs(1), // Update every second
        }
    }

    pub fn get_current_metrics(&self) -> Result<PerformanceMetrics, String> {
        let mut system = self.system.lock().map_err(|e| format!("Failed to lock system: {}", e))?;
        system.refresh_all();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Get system-wide metrics
        let cpu_usage = system.global_cpu_info().cpu_usage();
        let memory_usage = system.used_memory();
        let memory_total = system.total_memory();
        let memory_percentage = if memory_total > 0 {
            (memory_usage as f32 / memory_total as f32) * 100.0
        } else {
            0.0
        };

        // Get process-specific metrics
        let current_pid = std::process::id();
        let process = system.process(Pid::from_u32(current_pid));
        
        let (process_cpu, process_memory, process_threads) = if let Some(process) = process {
            (process.cpu_usage(), process.memory(), 1) // Simplified thread count
        } else {
            (0.0, 0, 0)
        };

        // Calculate system load (simplified)
        let system_load = System::load_average().one as f32;

        // Get uptime
        let uptime = System::uptime();

        let metrics = PerformanceMetrics {
            timestamp,
            cpu_usage,
            memory_usage,
            memory_total,
            memory_percentage,
            process_cpu,
            process_memory,
            process_threads,
            system_load,
            uptime,
        };

        // Add to history
        self.add_to_history(metrics.clone())?;

        Ok(metrics)
    }

    fn add_to_history(&self, metrics: PerformanceMetrics) -> Result<(), String> {
        let mut history = self.history.lock().map_err(|e| format!("Failed to lock history: {}", e))?;
        
        history.push_back(metrics);
        
        // Keep only the last max_history_size entries
        while history.len() > self.max_history_size {
            history.pop_front();
        }

        Ok(())
    }

    pub fn get_history(&self) -> Result<PerformanceHistory, String> {
        let history = self.history.lock().map_err(|e| format!("Failed to lock history: {}", e))?;
        
        let mut cpu_history = Vec::new();
        let mut memory_history = Vec::new();
        let mut process_cpu_history = Vec::new();
        let mut process_memory_history = Vec::new();
        let mut timestamps = Vec::new();

        for metrics in history.iter() {
            cpu_history.push(metrics.cpu_usage);
            memory_history.push(metrics.memory_percentage);
            process_cpu_history.push(metrics.process_cpu);
            process_memory_history.push(metrics.process_memory);
            timestamps.push(metrics.timestamp);
        }

        Ok(PerformanceHistory {
            cpu_history,
            memory_history,
            process_cpu_history,
            process_memory_history,
            timestamps,
        })
    }

    pub fn should_update(&self) -> bool {
        let last_update = self.last_update.lock().unwrap();
        last_update.elapsed() >= self.update_interval
    }

    pub fn mark_updated(&self) {
        let mut last_update = self.last_update.lock().unwrap();
        *last_update = Instant::now();
    }

    pub fn get_system_info(&self) -> Result<SystemInfo, String> {
        let system = self.system.lock().map_err(|e| format!("Failed to lock system: {}", e))?;
        
        Ok(SystemInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            cpu_count: system.cpus().len(),
            total_memory: system.total_memory(),
            total_swap: system.total_swap(),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub host_name: String,
    pub cpu_count: usize,
    pub total_memory: u64,
    pub total_swap: u64,
    pub kernel_version: String,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
