use crate::plugin_system::PluginManager;
use crate::plugin_system::{load_all_plugins, types::Plugin};
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub plugin_manager: Mutex<PluginManager>,
}

#[tauri::command]
pub fn get_plugins() -> Result<Vec<Plugin>, String> {
    load_all_plugins()
}

#[tauri::command]
pub fn call_plugin_method(
    state: State<'_, AppState>,
    plugin_name: String,
    method_name: String,
    args: Vec<Value>,
) -> Result<Value, String> {
    let manager = state.plugin_manager.lock().unwrap();
    manager.call_plugin_method(&plugin_name, &method_name, args)
}
