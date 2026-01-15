// commands/video.rs
use crate::errors::{AppError, Result};
use crate::state::AppState; // Use the unified AppState
use crate::video_player::player::MpvPlayer;
use crate::video_player::types::{AudioTrackInfo, SubtitleTrackInfo};
use serde::Deserialize;
use tauri::{command, Emitter, State, WebviewWindow};

#[command]
pub async fn load_video(
    state: State<'_, AppState>,
    window: WebviewWindow,
    url: String, // Accept just a string instead of a struct
) -> Result<serde_json::Value, AppError> {
    // Get user settings
    let settings = {
        let settings_guard = state
            .user_settings
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock user settings: {}", e)))?;

        settings_guard.clone()
    };

    // Create player
    let player = MpvPlayer::new(window, settings, state.mpv_config_path.clone())
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to create player: {}", e)))?;

    // Load the video
    {
        let mpv = player.mpv.lock().await;
        mpv.command("loadfile", &[&url, "append-play"])
            .map_err(|e| AppError::Runtime(format!("Failed to load video: {}", e)))?;
    }

    // Store player in state
    {
        let mut player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock video player: {}", e)))?;

        *player_guard = Some(player);
    }

    Ok(serde_json::json!({
        "success": true,
        "message": "Video loaded successfully"
    }))
}

#[derive(Deserialize)]
pub struct TogglePlayRequest {
    state: bool,
}

#[command]
pub async fn toggle_play(
    state: State<'_, AppState>,
    window: WebviewWindow,
    request: TogglePlayRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    player_clone
        .toggle_play(request.state)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to toggle play: {}", e)))?;

    window
        .emit("VideoPlayStateChanged", request.state)
        .map_err(|e| AppError::Runtime(format!("Failed to emit event: {}", e)))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct SeekRequest {
    time: f64,
    #[serde(default)]
    absolute: bool,
}

// #[command]
// pub async fn seek_video(state: State<'_, AppState>, request: SeekRequest) -> Result<(), AppError> {
//     // Clone the player while holding the lock briefly
//     let player_clone = {
//         let player = state
//             .video_player
//             .lock()
//             .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

//         player
//             .as_ref()
//             .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
//             .clone()
//     }; // Lock released here

//     // Use the clone without holding the lock
//     player_clone
//         .seek(request.time, request.absolute)
//         .await
//         .map_err(|e| AppError::Runtime(format!("Failed to seek: {}", e)))?;

//     Ok(())
// }

#[derive(Deserialize)]
pub struct SetVolumeRequest {
    volume: f64,
}

#[command]
pub async fn set_video_volume(
    state: State<'_, AppState>,
    request: SetVolumeRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    player_clone
        .set_volume(request.volume)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to set volume: {}", e)))?;

    Ok(())
}

#[command]
pub async fn get_audio_tracks(state: State<'_, AppState>) -> Result<Vec<AudioTrackInfo>, AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    let tracks = player_clone
        .audio_manager()
        .get_tracks()
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to get audio tracks: {}", e)))?;

    Ok(tracks)
}

#[command]
pub async fn get_subtitle_tracks(
    state: State<'_, AppState>,
) -> Result<Vec<SubtitleTrackInfo>, AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    let tracks = player_clone
        .subtitle_manager()
        .get_tracks()
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to get subtitle tracks: {}", e)))?;

    Ok(tracks)
}

#[derive(Deserialize)]
pub struct SetAudioTrackRequest {
    #[serde(rename = "trackId")]
    track_id: i64,
}

#[command]
pub async fn set_audio_track(
    state: State<'_, AppState>,
    request: SetAudioTrackRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    player_clone
        .audio_manager()
        .set_track(request.track_id)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to set audio track: {}", e)))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct SetSubtitleTrackRequest {
    #[serde(rename = "trackId")]
    track_id: i64,
}

#[command]
pub async fn set_subtitle_track(
    state: State<'_, AppState>,
    request: SetSubtitleTrackRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    player_clone
        .subtitle_manager()
        .set_track(request.track_id)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to set subtitle track: {}", e)))?;

    Ok(())
}

#[command]
pub async fn turn_off_subtitle_track(state: State<'_, AppState>) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    player_clone
        .subtitle_manager()
        .disable()
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to disable subtitle: {}", e)))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct SetSpeakerConfigRequest {
    #[serde(rename = "audioChannels")]
    audio_channels: String,
}

#[command]
pub async fn set_speaker_configuration(
    state: State<'_, AppState>,
    request: SetSpeakerConfigRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    player_clone
        .audio_manager()
        .set_channels(&request.audio_channels)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to set speaker config: {}", e)))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct ChangeSubtitleSizeRequest {
    increase: bool,
}

#[command]
pub async fn change_subtitle_size(
    state: State<'_, AppState>,
    window: WebviewWindow,
    request: ChangeSubtitleSizeRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    let new_size = player_clone
        .subtitle_manager()
        .adjust_size(request.increase)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to adjust subtitle size: {}", e)))?;

    window
        .emit(
            "SubtitleSizeChanged",
            serde_json::json!({ "size": new_size }),
        )
        .map_err(|e| AppError::Runtime(format!("Failed to emit subtitle size change: {}", e)))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct AudioSyncRequest {
    increase: bool,
}

#[command]
pub async fn audio_sync_correction(
    state: State<'_, AppState>,
    window: WebviewWindow,
    request: AudioSyncRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    let new_offset = player_clone
        .audio_manager()
        .adjust_delay(request.increase)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to adjust audio sync: {}", e)))?;

    window
        .emit(
            "AudioOffsetChanged",
            serde_json::json!({ "offset": new_offset }),
        )
        .map_err(|e| AppError::Runtime(format!("Failed to emit audio offset change: {}", e)))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct SubtitleSyncRequest {
    increase: bool,
}

#[command]
pub async fn subtitle_sync_correction(
    state: State<'_, AppState>,
    window: WebviewWindow,
    request: SubtitleSyncRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    let new_offset = player_clone
        .subtitle_manager()
        .adjust_delay(request.increase)
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to adjust subtitle sync: {}", e)))?;

    window
        .emit(
            "SubtitleOffsetChanged",
            serde_json::json!({ "offset": new_offset }),
        )
        .map_err(|e| AppError::Runtime(format!("Failed to emit subtitle offset change: {}", e)))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct AddToPlaylistRequest {
    #[serde(rename = "mediaData")]
    media_data: serde_json::Value,
    file: String,
    language: String,
    title: String,
    id: i64,
    #[serde(rename = "seasonNumber")]
    season_number: i32,
    #[serde(rename = "episodeNumber")]
    episode_number: i32,
}

#[command]
pub async fn add_to_playlist(
    state: State<'_, AppState>,
    request: AddToPlaylistRequest,
) -> Result<(), AppError> {
    // Clone the player while holding the lock briefly
    let player_clone = {
        let player = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player
            .as_ref()
            .ok_or_else(|| AppError::NotFound("Player not initialized".to_string()))?
            .clone()
    }; // Lock released here

    // Use the clone without holding the lock
    player_clone
        .add_to_playlist(
            request.media_data,
            request.file,
            request.language,
            request.title,
            request.id,
            request.season_number,
            request.episode_number,
        )
        .await
        .map_err(|e| AppError::Runtime(format!("Failed to add to playlist: {}", e)))?;

    Ok(())
}

#[command]
pub async fn close_video_player(state: State<'_, AppState>) -> Result<(), AppError> {
    // Take the player out of the mutex while holding the lock
    let player_option = {
        let mut player_guard = state
            .video_player
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock player: {}", e)))?;

        player_guard.take()
    }; // Lock released here

    // Use the player without holding the lock
    if let Some(player) = player_option {
        player
            .shutdown()
            .await
            .map_err(|e| AppError::Runtime(format!("Failed to shutdown player: {}", e)))?;
    }

    Ok(())
}
