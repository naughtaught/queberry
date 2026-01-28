use std::sync::Arc;

use crate::db::settings::SettingsManager;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::video_player::audio::AudioManager;
use crate::video_player::player::MpvPlayer;
use crate::video_player::subtitles::SubtitleManager;
use crate::video_player::types::{
    AudioTrackResponse, LoadVideoData, MessageResponse, SeekData, SetAudioChannel, SetTime,
    SetVolume, SubtitleTrackResponse, TogglePlayData,
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
pub fn close_video_player(state: State<'_, AppState>) -> ApiResponse<MessageResponse> {
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

    ApiResponse::ok(MessageResponse {
        message: "Video player closed successfully".to_string(),
    })
}

#[command]
pub fn set_audio_channel(
    state: State<'_, AppState>,
    channel: String,
) -> ApiResponse<SetAudioChannel> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    if let Err(e) = player.set_audio_channel(&channel) {
        return ApiResponse::error(500, format!("Failed to set audio channel: {}", e));
    }

    ApiResponse::ok(SetAudioChannel {
        message: "Video player audio channel changed successfully".to_string(),
        channel,
    })
}

#[command]
pub fn set_subtitle_track(
    state: State<'_, AppState>,
    subtitle_track_id: i64,
) -> ApiResponse<SubtitleTrackResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    match player.set_subtitle_track(subtitle_track_id) {
        Ok(_) => {
            let subtitle_manager = SubtitleManager::new(Arc::clone(&player.mpv));
            match subtitle_manager.get_current_subtitle_track() {
                Ok(current_subtitle_track) => ApiResponse::ok(SubtitleTrackResponse {
                    message: "Subtitle set successfully".to_string(),
                    current_subtitle_track,
                }),
                Err(e) => ApiResponse::error(500, format!("Failed to get current subtitle: {}", e)),
            }
        }
        Err(e) => ApiResponse::error(500, format!("Failed to set subtitle: {}", e)),
    }
}

#[command]
pub fn set_audio_track(
    state: State<'_, AppState>,
    audio_track_id: i64,
) -> ApiResponse<AudioTrackResponse> {
    let player_guard = match state.video_player.lock() {
        Ok(guard) => guard,
        Err(e) => return ApiResponse::error(500, format!("Failed to lock video player: {}", e)),
    };

    let player = match player_guard.as_ref() {
        Some(player) => player,
        None => return ApiResponse::error(404, "No player available".to_string()),
    };

    match player.set_audio_track(audio_track_id) {
        Ok(_) => {
            let audio_manager = AudioManager::new(Arc::clone(&player.mpv));
            match audio_manager.get_current_audio_track() {
                Ok(current_audio_track) => ApiResponse::ok(AudioTrackResponse {
                    message: "Audio track set successfully".to_string(),
                    current_audio_track,
                }),
                Err(e) => ApiResponse::error(500, format!("Failed to get current subtitle: {}", e)),
            }
        }
        Err(e) => ApiResponse::error(500, format!("Failed to set subtitle: {}", e)),
    }
}

#[command]
pub fn av_sync_adjust(state: State<'_, AppState>, value: f64) -> ApiResponse<MessageResponse> {
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

    ApiResponse::ok(MessageResponse {
        message: "Video player audio video sync changed successfully".to_string(),
    })
}

#[command]
pub fn center_speaker_level(state: State<'_, AppState>, value: i8) -> ApiResponse<MessageResponse> {
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

    ApiResponse::ok(MessageResponse {
        message: "Video player center speaker adjustment changed successfully".to_string(),
    })
}
