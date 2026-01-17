// lib.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
pub mod constants;
pub mod errors;
mod plugin_system;
mod state;
mod utils;
mod video_player;

use crate::plugin_system::loader::load_all_plugins;
use crate::plugin_system::PluginManager;
use crate::state::AppState;
pub use errors::{AppError, ErrorDetail, ErrorResponse};
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

            let mut plugin_manager = PluginManager::new(plugins_dir);

            for plugin in load_all_plugins()? {
                if let Err(e) = plugin_manager.register_plugin(plugin) {
                    eprintln!("Failed to register plugin: {}", e);
                }
            }

            let app_state = AppState::new(plugin_manager);
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_plugins,
            commands::call_plugin_method,
            commands::unregister_plugin,
            commands::unload_plugin,
            commands::refresh_plugin,
            commands::video::load_video,
            commands::video::toggle_play,
            commands::video::seek,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
