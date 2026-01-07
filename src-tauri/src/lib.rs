mod commands;
pub mod constants;
mod plugin_system;
mod utils;

use commands::AppState;
use plugin_system::PluginManager;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let plugins_dir = utils::get_plugins_dir()?;
            let mut manager = PluginManager::new(plugins_dir);

            // Load all plugins
            if let Ok(plugins) = plugin_system::load_all_plugins() {
                for plugin in plugins {
                    manager.load_plugin(plugin);
                }
            }

            app.manage(AppState {
                plugin_manager: Mutex::new(manager),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_plugins,
            commands::call_plugin_method
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
