// commands/plugin.rs
use crate::errors::{AppError, ErrorResponse};
use crate::plugin_system::loader::load_all_plugins;
use crate::state::AppState; // Use the unified state
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub fn get_plugins() -> Result<ErrorResponse, AppError> {
    load_all_plugins().map(ErrorResponse::success)
}

#[tauri::command]
pub fn call_plugin_method(
    state: State<'_, AppState>,
    plugin_name: String,
    method_name: String,
    args: Vec<Value>,
) -> Result<ErrorResponse, AppError> {
    let mut manager = state
        .plugin_manager
        .lock()
        .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

    manager
        .call_plugin_method(&plugin_name, &method_name, args)
        .map(ErrorResponse::success)
}

#[tauri::command]
pub fn unregister_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<ErrorResponse, AppError> {
    let mut manager = state
        .plugin_manager
        .lock()
        .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

    manager.unregister_plugin(&plugin_id);
    manager.unload_plugin(&plugin_id)?;

    Ok(ErrorResponse::success(format!(
        "Plugin '{}' unregistered",
        plugin_id
    )))
}

#[tauri::command]
pub fn unload_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<ErrorResponse, AppError> {
    let mut manager = state
        .plugin_manager
        .lock()
        .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

    manager.unload_plugin(&plugin_id)?;

    Ok(ErrorResponse::success(format!(
        "Plugin '{}' unloaded from runtime",
        plugin_id
    )))
}

#[tauri::command]
pub fn refresh_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<ErrorResponse, AppError> {
    let mut manager = state
        .plugin_manager
        .lock()
        .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

    manager.refresh_plugin(&plugin_id)?;

    Ok(ErrorResponse::success(format!(
        "Plugin '{}' refreshed from disk",
        plugin_id
    )))
}
