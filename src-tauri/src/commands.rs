use crate::errors::{AppError, ErrorResponse};
use crate::plugin_system::loader::load_all_plugins;
use crate::plugin_system::PluginManager;
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub plugin_manager: Mutex<PluginManager>,
}

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
