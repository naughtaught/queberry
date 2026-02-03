#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
pub mod constants;
pub mod db;
pub mod errors;
mod plugin_system;
mod state;
mod utils;
mod video_player;

use crate::db::init_db;
use crate::plugin_system::init_plugins;
use crate::state::AppState;
pub use errors::{AppError, ErrorDetail, ErrorResponse};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let plugin_manager = match init_plugins::init_plugins() {
                Ok(manager) => manager,
                Err(e) => {
                    eprintln!("Failed to initialize plugins: {}", e);
                    return Err(e);
                }
            };

            let database = match init_db::init_db() {
                Ok(db) => {
                    println!("Database initialized successfully");
                    Some(db)
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {}", e);
                    // TODO ERROR HANDLING
                    None
                }
            };

            let app_state = AppState::new(plugin_manager, database);
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
            commands::video::add_playlist_item,
            commands::video::toggle_play,
            commands::video::seek,
            commands::video::next_playlist_item,
            commands::video::previous_playlist_item,
            commands::video::set_time,
            commands::video::set_volume,
            commands::video::close_video_player,
            commands::video::set_audio_channel,
            commands::video::set_audio_track,
            commands::video::set_subtitle_track,
            commands::video::av_sync_adjust,
            commands::video::center_speaker_level,
            commands::video::set_subtitle_margin,
            commands::video::toggle_shader,
            commands::db::update_user_settings,
            commands::db::get_user_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
