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
use crate::state::AppState; // Use the unified state from state.rs
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
            let mpv_config_path = utils::get_mpv_config_dir()?;

            let mut plugin_manager = PluginManager::new(plugins_dir);

            for plugin in load_all_plugins()? {
                if let Err(e) = plugin_manager.register_plugin(plugin) {
                    eprintln!("Failed to register plugin: {}", e);
                }
            }

            // Create the unified app state
            let app_state = AppState::new(plugin_manager, mpv_config_path);
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
            commands::video::seek_video,
            commands::video::set_video_volume,
            commands::video::get_audio_tracks,
            commands::video::get_subtitle_tracks,
            commands::video::set_audio_track,
            commands::video::set_subtitle_track,
            commands::video::turn_off_subtitle_track,
            commands::video::set_speaker_configuration,
            commands::video::change_subtitle_size,
            commands::video::audio_sync_correction,
            commands::video::subtitle_sync_correction,
            commands::video::add_to_playlist,
            commands::video::close_video_player,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
