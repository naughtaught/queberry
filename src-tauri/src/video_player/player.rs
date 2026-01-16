use crate::errors::{AppError, Result};
use crate::video_player::config::MpvConfig;
use libmpv2::Mpv;
use std::sync::{Arc, Mutex};
use tauri::WebviewWindow;

/// Main MPV player structure
#[derive(Clone)]
pub struct MpvPlayer {
    pub(crate) mpv: Arc<Mutex<Mpv>>,
}

impl MpvPlayer {
    pub fn new(window: WebviewWindow) -> Result<Self> {
        #[cfg(target_os = "windows")]
        crate::video_player::platform::windows::init();
        #[cfg(target_os = "linux")]
        crate::video_player::platform::linux::init();
        #[cfg(target_os = "macos")]
        crate::video_player::platform::macos::init();

        log::info!("Creating MPV instance");

        let mpv = Mpv::new().map_err(|e| {
            AppError::Runtime(format!(
                "Failed to create mpv instance: {}. Please ensure libmpv is in your system PATH",
                e
            ))
        })?;

        let window_id = crate::video_player::platform::get_window_handle_id(&window)
            .ok_or_else(|| AppError::Runtime("Window handle not available".to_string()))?;

        let config = MpvConfig::new();
        config.set_window_id(&mpv, window_id)?;
        config.apply_to_mpv(&mpv)?;

        let mpv = Arc::new(Mutex::new(mpv));

        Ok(Self { mpv })
    }

    pub fn load_file(&self, file: String) -> Result<()> {
        log::info!("Loading file: {}", file);

        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("loadfile", &[&file, "append-play"])
            .map_err(|e| AppError::Runtime(format!("Failed to load file '{}': {}", file, e)))?;

        Ok(())
    }

    pub fn toggle_play(&self, paused: bool) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        if paused {
            mpv.set_property("pause", false)
                .map_err(|e| AppError::Runtime(format!("Failed to play: {}", e)))
        } else {
            mpv.set_property("pause", true)
                .map_err(|e| AppError::Runtime(format!("Failed to pause: {}", e)))
        }
    }
}

impl Drop for MpvPlayer {
    fn drop(&mut self) {
        log::info!("MpvPlayer dropped");
    }
}
