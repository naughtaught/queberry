use crate::errors::{AppError, ErrorResponse};
use crate::plugin_system::loader::load_all_plugins;
use crate::state::AppState;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub async fn get_plugins(state: State<'_, AppState>) -> Result<ErrorResponse, AppError> {
    let plugins = if let Some(cache) = state.plugin_cache.as_ref() {
        crate::plugin_system::loader::load_all_plugins_cached(cache).await?
    } else {
        load_all_plugins()?
    };

    Ok(ErrorResponse::success(plugins))
}

#[tauri::command]
pub async fn call_plugin_method(
    state: State<'_, AppState>,
    plugin_name: String,
    method_name: String,
    args: Vec<Value>,
) -> Result<ErrorResponse, AppError> {
    let manager = state.plugin_manager.clone();

    let result: Value = manager
        .call_plugin_method(&plugin_name, &method_name, args)
        .await?;

    Ok(ErrorResponse::success(result))
}

#[tauri::command]
pub async fn unregister_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<ErrorResponse, AppError> {
    let manager = state.plugin_manager.clone();

    manager.unregister_plugin(&plugin_id);

    Ok(ErrorResponse::success(format!(
        "Plugin '{}' unregistered",
        plugin_id
    )))
}

#[tauri::command]
pub async fn unload_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<ErrorResponse, AppError> {
    let manager = state.plugin_manager.clone();

    manager.unload_plugin_from_runtime(&plugin_id);

    Ok(ErrorResponse::success(format!(
        "Plugin '{}' unloaded from runtime",
        plugin_id
    )))
}

#[tauri::command]
pub async fn refresh_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<ErrorResponse, AppError> {
    let manager = state.plugin_manager.clone();

    manager.refresh_plugin(&plugin_id).await?;

    Ok(ErrorResponse::success(format!(
        "Plugin '{}' refreshed from disk",
        plugin_id
    )))
}
