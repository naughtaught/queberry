use std::sync::Arc;

use crate::errors::{handle_command, handle_command_async, ApiResponse};
use crate::state::AppState;
use crate::video_player::audio::AudioManager;
use crate::video_player::player::MpvPlayer;
use crate::video_player::subtitles::SubtitleManager;
use crate::video_player::types::AudioTrackResponse;
use crate::video_player::types::{
    AddPlaylistItemRequest, LoadVideoRequest, SubtitleTrackResponse, VideoCommandResponse,
};
use crate::AppError;
use tauri::{command, AppHandle, State, WebviewWindow};

#[command]
pub async fn load_video(
    app_state: tauri::State<'_, AppState>,
    app: AppHandle,
    window: WebviewWindow,
    value: LoadVideoRequest,
) -> Result<ApiResponse<VideoCommandResponse>, AppError> {
    handle_command_async("load_video", async || {
        let LoadVideoRequest {
            url,
            user_id,
            video_language,
            progress,
        } = value;

        let settings_manager = app_state
            .get_settings_manager()
            .ok_or_else(|| AppError::Runtime("Settings not initialized".to_string()))?;

        let settings = settings_manager.get_settings(user_id).await?;

        let mut player_guard = app_state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = MpvPlayer::new(window, app, &settings, Some(video_language))
            .map_err(|e| AppError::Runtime(format!("Failed to create player: {}", e)))?;

        player
            .load_file(url.clone(), progress)
            .map_err(|e| AppError::Runtime(format!("Failed to load video: {}", e)))?;

        *player_guard = Some(player);

        Ok(VideoCommandResponse {
            value: serde_json::Value::String(url),
        })
    })
    .await
}

#[command]
pub fn toggle_play(state: State<'_, AppState>, value: bool) -> ApiResponse<VideoCommandResponse> {
    handle_command("toggle_play", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .toggle_play(value)
            .map_err(|e| AppError::Runtime(format!("Failed to toggle play: {}", e)))?;

        Ok(VideoCommandResponse {
            value: serde_json::Value::Bool(!value),
        })
    })
}

#[command]
pub fn seek(state: State<'_, AppState>, value: i8) -> ApiResponse<VideoCommandResponse> {
    handle_command("seek", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .seek(value)
            .map_err(|e| AppError::Runtime(format!("Failed to seek: {}", e)))?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn set_time(state: State<'_, AppState>, value: f64) -> ApiResponse<VideoCommandResponse> {
    handle_command("set_time", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .set_time(value)
            .map_err(|e| AppError::Runtime(format!("Failed to set time: {}", e)))?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn set_volume(state: State<'_, AppState>, value: f64) -> ApiResponse<VideoCommandResponse> {
    handle_command("set_volume", || {
        if !(0.0..=100.0).contains(&value) {
            return Err(AppError::Validation("Volume outside of bounds".to_string()));
        }

        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .set_volume(value)
            .map_err(|e| AppError::Runtime(format!("Failed to set volume: {}", e)))?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn close_video_player(state: State<'_, AppState>) -> ApiResponse<VideoCommandResponse> {
    handle_command("close_video_player", || {
        let mut player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        if let Some(player) = player_guard.as_ref() {
            player
                .shutdown()
                .map_err(|e| AppError::Runtime(format!("Failed to close video player: {}", e)))?;
        }

        *player_guard = None;

        Ok(VideoCommandResponse {
            value: serde_json::Value::Bool(true),
        })
    })
}

#[command]
pub fn set_audio_channel(
    state: State<'_, AppState>,
    value: String,
) -> ApiResponse<VideoCommandResponse> {
    handle_command("set_audio_channel", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .set_audio_channel(&value)
            .map_err(|e| AppError::Runtime(format!("Failed to set audio channel: {}", e)))?;

        Ok(VideoCommandResponse {
            value: serde_json::Value::String(value),
        })
    })
}

#[command]
pub fn set_subtitle_track(
    state: State<'_, AppState>,
    value: i64,
) -> ApiResponse<SubtitleTrackResponse> {
    handle_command("set_subtitle_track", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .set_subtitle_track(value)
            .map_err(|e| AppError::Runtime(format!("Failed to set subtitle: {}", e)))?;

        let subtitle_manager = SubtitleManager::new(Arc::clone(&player.mpv));
        let current_subtitle_track = subtitle_manager
            .get_current_subtitle_track()
            .map_err(|e| AppError::Runtime(format!("Failed to get current subtitle: {}", e)))?;

        Ok(SubtitleTrackResponse {
            value: current_subtitle_track,
        })
    })
}

#[command]
pub fn set_audio_track(state: State<'_, AppState>, value: i64) -> ApiResponse<AudioTrackResponse> {
    handle_command("set_audio_track", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .set_audio_track(value)
            .map_err(|e| AppError::Runtime(format!("Failed to set audio track: {}", e)))?;

        let audio_manager = AudioManager::new(Arc::clone(&player.mpv));
        let current_audio_track = audio_manager
            .get_current_audio_track()
            .map_err(|e| AppError::Runtime(format!("Failed to get current audio track: {}", e)))?;

        Ok(AudioTrackResponse {
            value: current_audio_track,
        })
    })
}

#[command]
pub fn av_sync_adjust(state: State<'_, AppState>, value: f64) -> ApiResponse<VideoCommandResponse> {
    handle_command("av_sync_adjust", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .av_sync_adjust(value)
            .map_err(|e| AppError::Runtime(format!("Failed to set av adjustment: {}", e)))?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn center_speaker_level(
    state: State<'_, AppState>,
    value: i8,
) -> ApiResponse<VideoCommandResponse> {
    handle_command("center_speaker_level", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player.center_speaker_level(value).map_err(|e| {
            AppError::Runtime(format!("Failed to set center speaker adjustment: {}", e))
        })?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn set_subtitle_pos(
    state: State<'_, AppState>,
    value: i64,
) -> ApiResponse<VideoCommandResponse> {
    handle_command("set_subtitle_pos", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .set_subtitle_pos(value)
            .map_err(|e| AppError::Runtime(format!("Failed to set subtitles: {}", e)))?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn get_subtitle_pos(state: State<'_, AppState>) -> ApiResponse<VideoCommandResponse> {
    handle_command("get_subtitle_pos", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        let pos = player
            .get_subtitle_pos()
            .map_err(|e| AppError::Runtime(format!("Failed to get subtitle position: {}", e)))?;

        Ok(VideoCommandResponse { value: pos.into() })
    })
}

#[command]
pub fn add_playlist_item(
    state: State<'_, AppState>,
    value: AddPlaylistItemRequest,
) -> ApiResponse<VideoCommandResponse> {
    handle_command("add_playlist_item", || {
        let AddPlaylistItemRequest { url } = value;

        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .add_playlist_item(url.clone())
            .map_err(|e| AppError::Runtime(format!("Failed to add playlist item: {}", e)))?;

        Ok(VideoCommandResponse {
            value: serde_json::Value::String(url),
        })
    })
}

#[command]
pub fn next_playlist_item(state: State<'_, AppState>) -> ApiResponse<VideoCommandResponse> {
    handle_command("next_playlist_item", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .next_playlist_item()
            .map_err(|e| AppError::Runtime(format!("Failed to play next playlist item: {}", e)))?;

        Ok(VideoCommandResponse {
            value: serde_json::Value::Null,
        })
    })
}

#[command]
pub fn previous_playlist_item(state: State<'_, AppState>) -> ApiResponse<VideoCommandResponse> {
    handle_command("previous_playlist_item", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player.previous_playlist_item().map_err(|e| {
            AppError::Runtime(format!("Failed to play previous playlist item: {}", e))
        })?;

        Ok(VideoCommandResponse {
            value: serde_json::Value::Null,
        })
    })
}

#[command]
pub fn toggle_shader(state: State<'_, AppState>, value: &str) -> ApiResponse<VideoCommandResponse> {
    handle_command("toggle_shader", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .toggle_shader(value)
            .map_err(|e| AppError::Runtime(format!("Failed to toggle shader: {}", e)))?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn set_subtitle_scaling(
    state: State<'_, AppState>,
    value: f64,
) -> ApiResponse<VideoCommandResponse> {
    handle_command("set_subtitle_scaling", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player
            .set_subtitle_scaling(value)
            .map_err(|e| AppError::Runtime(format!("Failed to scale subtitles: {}", e)))?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}

#[command]
pub fn subtitle_sync_adjust(
    state: State<'_, AppState>,
    value: f64,
) -> ApiResponse<VideoCommandResponse> {
    handle_command("subtitle_sync_adjust", || {
        let player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        let player = player_guard
            .as_ref()
            .ok_or_else(|| AppError::NotFound("No player available".to_string()))?;

        player.subtitle_sync_adjust(value).map_err(|e| {
            AppError::Runtime(format!("Failed to set subtitle sync adjustment: {}", e))
        })?;

        Ok(VideoCommandResponse {
            value: value.into(),
        })
    })
}
