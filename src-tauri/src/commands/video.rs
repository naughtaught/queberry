use std::sync::Arc;

use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::video_player::audio::AudioManager;
use crate::video_player::player::MpvPlayer;
use crate::video_player::subtitles::SubtitleManager;
use crate::video_player::types::{
    AddPlaylistItemRequest, LoadVideoRequest, SubtitleTrackResponse, VideoCommandResponse,
};
use crate::AppError;
use crate::{db::settings::SettingsManager, video_player::types::AudioTrackResponse};
use tauri::{command, AppHandle, State, WebviewWindow};

#[command]
pub fn load_video(
    app_state: tauri::State<AppState>,
    app: AppHandle,
    state: State<'_, AppState>,
    window: WebviewWindow,
    value: LoadVideoRequest,
) -> ApiResponse<VideoCommandResponse> {
    let LoadVideoRequest { url, user_id } = value;

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

    let player = match MpvPlayer::new(window, app, &settings) {
        Ok(player) => player,
        Err(e) => {
            return ApiResponse::error(500, format!("Failed to create player: {}", e));
        }
    };

    match player.load_file(url.clone()) {
        Ok(_) => (),
        Err(e) => {
            return ApiResponse::error(500, format!("Failed to load video: {}", e));
        }
    };

    *player_guard = Some(player);

    ApiResponse::ok(VideoCommandResponse {
        value: serde_json::Value::String(url.clone()),
    })
}

#[command]
pub fn toggle_play(state: State<'_, AppState>, value: bool) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.toggle_play(value) {
        return ApiResponse::error(500, format!("Failed to toggle play: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: serde_json::Value::Bool(!value),
    })
}

#[command]
pub fn seek(state: State<'_, AppState>, value: i8) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.seek(value) {
        return ApiResponse::error(500, format!("Failed to seek: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn set_time(state: State<'_, AppState>, value: f64) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_time(value) {
        return ApiResponse::error(500, format!("Failed to set time: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn set_volume(state: State<'_, AppState>, value: f64) -> ApiResponse<VideoCommandResponse> {
    if !(0.0..=100.0).contains(&value) {
        return ApiResponse::error(500, "Volume outside of bounds:".to_string());
    };

    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_volume(value) {
        return ApiResponse::error(500, format!("Failed to set volume: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn close_video_player(state: State<'_, AppState>) -> ApiResponse<VideoCommandResponse> {
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

    ApiResponse::ok(VideoCommandResponse {
        value: serde_json::Value::Bool(true),
    })
}

#[command]
pub fn set_audio_channel(
    state: State<'_, AppState>,
    value: String,
) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_audio_channel(&value) {
        return ApiResponse::error(500, format!("Failed to set audio channel: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: serde_json::Value::String(value),
    })
}

#[command]
pub fn set_subtitle_track(
    state: State<'_, AppState>,
    value: i64,
) -> ApiResponse<SubtitleTrackResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    match player.set_subtitle_track(value) {
        Ok(_) => {
            let subtitle_manager = SubtitleManager::new(Arc::clone(&player.mpv));
            match subtitle_manager.get_current_subtitle_track() {
                Ok(current_subtitle_track) => ApiResponse::ok(SubtitleTrackResponse {
                    value: current_subtitle_track,
                }),
                Err(e) => ApiResponse::error(500, format!("Failed to get current subtitle: {}", e)),
            }
        }
        Err(e) => ApiResponse::error(500, format!("Failed to set subtitle: {}", e)),
    }
}

#[command]
pub fn set_audio_track(state: State<'_, AppState>, value: i64) -> ApiResponse<AudioTrackResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    match player.set_audio_track(value) {
        Ok(_) => {
            let audio_manager = AudioManager::new(Arc::clone(&player.mpv));
            match audio_manager.get_current_audio_track() {
                Ok(current_audio_track) => ApiResponse::ok(AudioTrackResponse {
                    value: current_audio_track,
                }),
                Err(e) => ApiResponse::error(500, format!("Failed to get current subtitle: {}", e)),
            }
        }
        Err(e) => ApiResponse::error(500, format!("Failed to set subtitle: {}", e)),
    }
}

#[command]
pub fn av_sync_adjust(state: State<'_, AppState>, value: f64) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.av_sync_adjust(value) {
        return ApiResponse::error(500, format!("Failed to set av adjustment: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn center_speaker_level(
    state: State<'_, AppState>,
    value: i8,
) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.center_speaker_level(value) {
        return ApiResponse::error(
            500,
            format!("Failed to set center speaker adjustment: {}", e),
        );
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn set_subtitle_margin(
    state: State<'_, AppState>,
    value: i64,
) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_subtitle_margin(value) {
        return ApiResponse::error(500, format!("Failed to shift subtitles: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn add_playlist_item(
    state: State<'_, AppState>,
    value: AddPlaylistItemRequest,
) -> ApiResponse<VideoCommandResponse> {
    let AddPlaylistItemRequest { url } = value;

    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    if let Some(player) = player_guard.as_ref() {
        match player.add_playlist_item(url.clone()) {
            Ok(_) => (),
            Err(e) => {
                return ApiResponse::error(500, format!("Failed to add playlist item: {}", e));
            }
        }
    } else {
        return ApiResponse::error(404, "No player available".to_string());
    }

    ApiResponse::ok(VideoCommandResponse {
        value: serde_json::Value::String(url.clone()),
    })
}

#[command]
pub fn next_playlist_item(state: State<'_, AppState>) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.next_playlist_item() {
        return ApiResponse::error(500, format!("Failed to play next playlist item: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: serde_json::Value::Null,
    })
}

#[command]
pub fn previous_playlist_item(state: State<'_, AppState>) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.previous_playlist_item() {
        return ApiResponse::error(500, format!("Failed to play previous playlist item: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: serde_json::Value::Null,
    })
}

#[command]
pub fn toggle_shader(state: State<'_, AppState>, value: &str) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.toggle_shader(value) {
        return ApiResponse::error(500, format!("Failed to toggle shader: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn set_subtitle_scaling(
    state: State<'_, AppState>,
    value: f64,
) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_subtitle_scaling(value) {
        return ApiResponse::error(500, format!("Failed to scale subtitles: {}", e));
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}

#[command]
pub fn subtitle_sync_adjust(
    state: State<'_, AppState>,
    value: f64,
) -> ApiResponse<VideoCommandResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.subtitle_sync_adjust(value) {
        return ApiResponse::error(
            500,
            format!("Failed to set subtitle sync adjustment: {}", e),
        );
    }

    ApiResponse::ok(VideoCommandResponse {
        value: value.into(),
    })
}
