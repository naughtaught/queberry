use crate::video_player::MpvPlayer;
use crate::AppState;
use serde::Serialize;
use tauri::{command, State, WebviewWindow};

#[derive(Serialize, Clone)]
pub struct PlayerResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> PlayerResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

fn ensure_player_initialized(
    state: &tauri::State<'_, AppState>,
    window: &WebviewWindow,
) -> Result<(), String> {
    let mut player_guard = state
        .video_player
        .lock()
        .map_err(|e| format!("Failed to lock video player mutex: {}", e))?;

    if player_guard.is_none() {
        println!("Initializing MPV player...");

        match MpvPlayer::new(Some(window)) {
            Ok(player) => {
                *player_guard = Some(player);
                println!("MPV player initialized successfully");
                Ok(())
            }
            Err(e) => Err(format!("Failed to initialize MPV player: {}", e)),
        }
    } else {
        Ok(())
    }
}

#[command]
pub async fn load_video(
    state: State<'_, AppState>,
    window: WebviewWindow,
    url: String,
) -> Result<PlayerResponse<()>, String> {
    if let Err(e) = ensure_player_initialized(&state, &window) {
        return Ok(PlayerResponse::error(e));
    }

    let player_guard = state
        .video_player
        .lock()
        .map_err(|e| format!("Failed to lock video player mutex: {}", e))?;

    match &*player_guard {
        Some(player) => match player.load_file(&url) {
            Ok(_) => Ok(PlayerResponse::success(())),
            Err(e) => Ok(PlayerResponse::error(e)),
        },
        None => Ok(PlayerResponse::error(
            "Video player not initialized".to_string(),
        )),
    }
}

#[command]
pub async fn play_video(
    state: State<'_, AppState>,
    window: WebviewWindow,
) -> Result<PlayerResponse<()>, String> {
    if let Err(e) = ensure_player_initialized(&state, &window) {
        return Ok(PlayerResponse::error(e));
    }

    let player_guard = state
        .video_player
        .lock()
        .map_err(|e| format!("Failed to lock video player mutex: {}", e))?;

    match &*player_guard {
        Some(player) => match player.play() {
            Ok(_) => Ok(PlayerResponse::success(())),
            Err(e) => Ok(PlayerResponse::error(e)),
        },
        None => Ok(PlayerResponse::error(
            "Video player not initialized".to_string(),
        )),
    }
}
