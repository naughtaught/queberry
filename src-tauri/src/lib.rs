#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
mod commands;
pub mod constants;
pub mod db;
pub mod downloads;
pub mod errors;
mod plugin_system;
mod state;
mod utils;
mod video_player;

use std::sync::Arc;

use crate::plugin_system::init_plugins;
use crate::state::AppState;
use crate::{db::init_db, utils::avatar_dir::get_avatars_dir};
pub use errors::{AppError, ErrorDetail};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("errors.log".to_string()),
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                ])
                .level(log::LevelFilter::Warn)
                .level_for("app", log::LevelFilter::Info)
                .level_for("tao", log::LevelFilter::Error)
                .level_for("cranelift_codegen", log::LevelFilter::Error)
                .level_for("cranelift_wasm", log::LevelFilter::Error)
                .level_for("wasmtime", log::LevelFilter::Warn)
                .level_for("wasmtime_cranelift", log::LevelFilter::Warn)
                .level_for("wasmtime_runtime", log::LevelFilter::Warn)
                .level_for("wasmtime_jit", log::LevelFilter::Warn)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(2))
                .max_file_size(10_000_000) // 10MB
                .build(),
        )
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_prevent_default::debug())
        .setup(move |app| {
            log::info!("=== Application starting up ===");

            log::logger().flush();

            let avatars_dir = get_avatars_dir()?;
            if std::fs::read_dir(&avatars_dir)
                .map(|mut d| d.next().is_none())
                .unwrap_or(true)
            {
                let defaults = app.path().resource_dir()?.join("avatars");
                if defaults.exists() {
                    for entry in std::fs::read_dir(defaults)?.filter_map(|e| e.ok()) {
                        let dest = avatars_dir.join(entry.file_name());
                        std::fs::copy(entry.path(), dest).ok();
                    }
                }
            }

            let (database, plugin_manager) = match tauri::async_runtime::block_on(async {
                let database = init_db::init_db(app.handle()).await?;
                let database = Arc::new(database);
                let plugin_manager = init_plugins::init_plugins(Arc::clone(&database)).await?;
                Ok::<_, Box<dyn std::error::Error>>((database, plugin_manager))
            }) {
                Ok(result) => result,
                Err(e) => {
                    AppError::Runtime(format!("Failed to initialize: {}", e)).log("startup");
                    return Err(e);
                }
            };

            let app_state = AppState::new(plugin_manager, Some(database));
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
            commands::video::subtitle_sync_adjust,
            commands::video::center_speaker_level,
            commands::video::set_subtitle_pos,
            commands::video::get_subtitle_pos,
            commands::video::set_subtitle_scaling,
            commands::video::toggle_shader,
            commands::db::update_user_settings,
            commands::db::get_user_settings,
            commands::db::get_all_users,
            commands::db::get_user,
            commands::db::get_user_by_postgres_id,
            commands::db::create_user,
            commands::db::update_user,
            commands::db::delete_user,
            commands::db::delete_key,
            commands::db::update_key,
            commands::db::create_key,
            commands::db::get_keys,
            commands::db::get_users_blacklisted,
            commands::db::create_blacklisted_hash,
            commands::db::delete_users_blacklisted,
            commands::db::update_global_settings,
            commands::db::get_global_settings,
            commands::db::get_user_content_ratings,
            commands::db::upsert_user_content_ratings,
            commands::db::delete_user_content_ratings,
            commands::db::verify_pin,
            commands::db::get_user_keyboard_shortcuts,
            commands::db::upsert_user_keyboard_shortcuts,
            commands::db::create_transfer,
            commands::db::list_transfers,
            commands::db::delete_transfer,
            commands::db::upsert_transfer,
            commands::db::delete_local_media_by_directory,
            commands::db::scan_local_folder,
            commands::db::find_local_media,
            commands::db::update_local_media,
            commands::db::get_editable_local_media,
            commands::api::api_reset_token,
            commands::api::api_register,
            commands::api::api_login,
            commands::api::api_delete_user,
            commands::api::api_reset_user_media_state,
            commands::api::api_clear_user_episode_groups,
            commands::api::api_upsert_user_media,
            commands::api::api_delete_watched_episodes,
            commands::api::api_fetch_media_item,
            commands::api::api_fetch_recent_releases,
            commands::api::api_fetch_season_data,
            commands::api::api_fetch_trending,
            commands::api::api_fetch_up_next,
            commands::api::api_upsert_watched_episodes,
            commands::api::api_delete_watched_episode_ids,
            commands::api::api_fetch_data,
            commands::api::api_verify_token,
            commands::api::api_delete_user_media,
            commands::api::api_search_media,
            commands::api::api_request_media,
            commands::api::api_get_media_requests,
            commands::api::api_delete_media_request,
            commands::api::api_get_blacklist_entry,
            commands::api::api_reset_user_data,
            commands::api::api_update_user,
            commands::api::api_fetch_random_media,
            commands::api::api_fetch_random_backdrop,
            commands::api::api_fetch_related_media,
            commands::api::api_fetch_collections,
            commands::avatars::get_avatars,
            commands::avatars::get_avatars_dir_path,
            commands::app::show_window,
            commands::app::log_error,
            commands::app::get_log_path,
            commands::downloads::download_file,
            commands::downloads::cancel_download,
            commands::downloads::process_download_queue,
            commands::downloads::get_all_downloads,
            commands::downloads::clear_completed_downloads,
            commands::downloads::retry_download,
            commands::downloads::cleanup_downloads_on_login,
            commands::downloads::clear_completed_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
