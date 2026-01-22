use crate::db::settings::SettingsManager;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::video_player::player::MpvPlayer;
use crate::video_player::types::{
    CloseVideoPlayer, LoadVideoData, SeekData, SetTime, SetVolume, TogglePlayData,
};
use crate::AppError;
use tauri::{command, AppHandle, State, WebviewWindow};

#[command]
pub fn load_video(
    app_state: tauri::State<AppState>,
    app: AppHandle,
    state: State<'_, AppState>,
    window: WebviewWindow,
    url: String,
    user_id: i32,
) -> ApiResponse<LoadVideoData> {
    let mut player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => {
            return ApiResponse::error(500, format!("Failed to lock video player: {}", e));
        }
    };

    let settings_manager: &SettingsManager = match app_state.get_settings_manager() {
        Some(manager) => manager,
        None => return ApiResponse::err(AppError::Runtime("Settings not initialized".to_string())),
    };

    let settings = match settings_manager.get_settings(user_id) {
        Ok(settings) => settings,
        Err(e) => return ApiResponse::err(e),
    };

    if player_guard.is_none() {
        let player = match MpvPlayer::new(window, app, &settings) {
            Ok(player) => player,
            Err(e) => {
                return ApiResponse::error(500, format!("Failed to create player: {}", e));
            }
        };
        *player_guard = Some(player);
    }

    if let Some(player) = player_guard.as_ref() {
        match player.load_file(url.clone()) {
            Ok(_) => (),
            Err(e) => {
                return ApiResponse::error(500, format!("Failed to load video: {}", e));
            }
        }
    }

    ApiResponse::ok(LoadVideoData {
        message: "Video loaded successfully".to_string(),
        url: url.clone(),
    })
}

#[command]
pub fn toggle_play(state: State<'_, AppState>, paused: bool) -> ApiResponse<TogglePlayData> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.toggle_play(paused) {
        return ApiResponse::error(500, format!("Failed to toggle play: {}", e));
    }

    ApiResponse::ok(TogglePlayData {
        message: "Play state toggled successfully".to_string(),
        paused: !paused,
    })
}

#[command]
pub fn seek(state: State<'_, AppState>, seek_amount: i8) -> ApiResponse<SeekData> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.seek(seek_amount) {
        return ApiResponse::error(500, format!("Failed to seek: {}", e));
    }

    ApiResponse::ok(SeekData {
        message: "Time adjusted successfully".to_string(),
        seek_amount,
    })
}

#[command]
pub fn set_time(state: State<'_, AppState>, time: f64) -> ApiResponse<SetTime> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_time(time) {
        return ApiResponse::error(500, format!("Failed to set time: {}", e));
    }

    ApiResponse::ok(SetTime {
        message: "Time adjusted successfully".to_string(),
        time,
    })
}

#[command]
pub fn set_volume(state: State<'_, AppState>, volume: f64) -> ApiResponse<SetVolume> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_volume(volume) {
        return ApiResponse::error(500, format!("Failed to set volume: {}", e));
    }

    ApiResponse::ok(SetVolume {
        message: "Volume adjusted successfully".to_string(),
        volume,
    })
}

#[command]
pub fn close_video_player(state: State<'_, AppState>) -> ApiResponse<CloseVideoPlayer> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.shutdown() {
        return ApiResponse::error(500, format!("Failed to close video player: {}", e));
    }

    ApiResponse::ok(CloseVideoPlayer {
        message: "Video player closed successfully".to_string(),
    })
}
