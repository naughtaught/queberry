use crate::video_player::MpvPlayer;
use crate::AppState;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
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

        // Get window handle ID
        let window_id = get_window_handle_id(window);

        match MpvPlayer::new(window_id) {
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

fn get_window_handle_id(window: &WebviewWindow) -> Option<i64> {
    #[cfg(windows)]
    {
        if let Ok(handle) = window.window_handle() {
            match handle.as_raw() {
                RawWindowHandle::Win32(handle) => {
                    // hwnd.get() returns isize directly, not Option<isize>
                    let hwnd = handle.hwnd.get();
                    println!("Got window HWND: {:?}", hwnd);
                    return Some(hwnd as i64);
                }
                _ => {
                    eprintln!("Unsupported window handle type");
                }
            }
        } else {
            eprintln!("Failed to get window handle");
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(handle) = window.window_handle() {
            match handle.as_raw() {
                RawWindowHandle::Xlib(handle) => {
                    println!("Got X11 window ID: {}", handle.window);
                    return Some(handle.window as i64);
                }
                RawWindowHandle::Wayland(handle) => {
                    eprintln!("Wayland windows need special handling");
                }
                _ => {
                    eprintln!("Unsupported window handle type");
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(handle) = window.window_handle() {
            match handle.as_raw() {
                RawWindowHandle::AppKit(handle) => {
                    println!("Got macOS NSView pointer");
                    return Some(handle.ns_view.as_ptr() as i64);
                }
                _ => {
                    eprintln!("Unsupported window handle type");
                }
            }
        }
    }

    None
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
