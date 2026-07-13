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
    let plugin_name_clone = plugin_name.clone();
    let method_name_clone = method_name.clone();
    let args_clone = args.clone();

    let result = tokio::task::spawn_blocking(move || {
        let mut manager = manager
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

        let rt = tokio::runtime::Handle::current();
        rt.block_on(manager.call_plugin_method(&plugin_name_clone, &method_name_clone, args_clone))
    })
    .await
    .map_err(|e| AppError::Runtime(format!("Plugin execution task failed: {}", e)))?;

    result.map(ErrorResponse::success)
}

#[tauri::command]
pub async fn unregister_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<ErrorResponse, AppError> {
    let manager = state.plugin_manager.clone();
    let plugin_id_clone = plugin_id.clone();

    tokio::task::spawn_blocking(move || {
        let mut manager = manager
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

        manager.unregister_plugin(&plugin_id_clone);
        Ok::<_, AppError>(())
    })
    .await
    .map_err(|e| AppError::Runtime(format!("Task join error: {}", e)))??;

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
    let plugin_id_clone = plugin_id.clone();

    tokio::task::spawn_blocking(move || {
        let mut manager = manager
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

        manager.unload_plugin(&plugin_id_clone)
    })
    .await
    .map_err(|e| AppError::Runtime(format!("Task join error: {}", e)))??;

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
    let plugin_id_clone = plugin_id.clone();

    tokio::task::spawn_blocking(move || {
        let mut manager = manager
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock plugin manager: {}", e)))?;

        let rt = tokio::runtime::Handle::current();
        rt.block_on(manager.refresh_plugin(&plugin_id_clone))
    })
    .await
    .map_err(|e| AppError::Runtime(format!("Task join error: {}", e)))??;

    Ok(ErrorResponse::success(format!(
        "Plugin '{}' refreshed from disk",
        plugin_id
    )))
}
