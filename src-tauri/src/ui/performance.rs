use tauri::State;
use crate::performance::{PerformanceMetrics, PerformanceHistory, SystemInfo};
use crate::AppState;

/// Get current performance metrics
#[tauri::command]
pub async fn get_performance_metrics(
    state: State<'_, AppState>,
) -> Result<PerformanceMetrics, String> {
    let monitor = state.performance.read().await;
    monitor.get_current_metrics()
}

/// Get performance history
#[tauri::command]
pub async fn get_performance_history(
    state: State<'_, AppState>,
) -> Result<PerformanceHistory, String> {
    let monitor = state.performance.read().await;
    monitor.get_history()
}

/// Get system information
#[tauri::command]
pub async fn get_system_info(
    state: State<'_, AppState>,
) -> Result<SystemInfo, String> {
    let monitor = state.performance.read().await;
    monitor.get_system_info()
}

/// Check if performance monitor should update
#[tauri::command]
pub async fn should_update_performance(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let monitor = state.performance.read().await;
    Ok(monitor.should_update())
}

/// Mark performance monitor as updated
#[tauri::command]
pub async fn mark_performance_updated(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let monitor = state.performance.read().await;
    monitor.mark_updated();
    Ok(())
}
