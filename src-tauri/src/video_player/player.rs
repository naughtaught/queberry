use crate::db::types::UserSettings;
use crate::errors::{AppError, Result};
use crate::video_player::config::MpvConfig;
use crate::video_player::events::MpvEventHandler;
use crate::video_player::tracker::PlayerTracker;
use libmpv2::Mpv;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, WebviewWindow};

#[derive(Clone)]
pub struct MpvPlayer {
    pub(crate) mpv: Arc<Mutex<Mpv>>,
    tracker: PlayerTracker,
}

impl MpvPlayer {
    pub fn new(
        window: WebviewWindow,
        app_handle: AppHandle,
        settings: &UserSettings,
    ) -> Result<Self> {
        #[cfg(target_os = "windows")]
        crate::video_player::platform::windows::init();
        #[cfg(target_os = "linux")]
        crate::video_player::platform::linux::init();
        #[cfg(target_os = "macos")]
        crate::video_player::platform::macos::init();

        let mpv = Mpv::new().map_err(|e| {
            AppError::Runtime(format!(
                "Failed to create mpv instance: {}. Please ensure libmpv is in your system PATH",
                e
            ))
        })?;

        let window_id = crate::video_player::platform::get_window_handle_id(&window)
            .ok_or_else(|| AppError::Runtime("Window handle not available".to_string()))?;

        let config = MpvConfig::new(settings);
        config.set_window_id(&mpv, window_id)?;
        config.apply_to_mpv(&mpv)?;

        let mpv = Arc::new(Mutex::new(mpv));

        let tracker = PlayerTracker::new(
            Arc::clone(&mpv),
            app_handle.clone(),
            settings.completion_percent,
        );

        let event_logger = MpvEventHandler::new(Arc::clone(&mpv), app_handle.clone());
        event_logger.start();

        Ok(Self { mpv, tracker })
    }

    pub fn load_file(&self, file: String) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("loadfile", &[&file, "append-play"])
            .map_err(|e| AppError::Runtime(format!("Failed to load file '{}': {}", file, e)))?;

        self.tracker.start();

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

    pub fn seek(&self, seek_amount: i8) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("seek", &[&seek_amount.to_string(), "relative"])
            .map_err(|e| AppError::Runtime(format!("Failed to seek {}: {}", seek_amount, e)))?;

        Ok(())
    }

    pub fn set_time(&self, time: f64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("seek", &[&time.to_string(), "absolute"])
            .map_err(|e| AppError::Runtime(format!("Failed to seek {}: {}", time, e)))?;

        Ok(())
    }

    pub fn set_volume(&self, volume: f64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        let volume = volume.trunc();

        if volume <= 0.0 {
            mpv.set_property("mute", true)
                .map_err(|e| AppError::Runtime(format!("Failed to mute: {}", e)))?;
        } else {
            mpv.set_property("mute", false)
                .map_err(|e| AppError::Runtime(format!("Failed to unmute: {}", e)))?;

            mpv.set_property("volume", volume).map_err(|e| {
                AppError::Runtime(format!("Failed to set volume {}: {}", volume, e))
            })?;
        }

        Ok(())
    }

    pub fn shutdown(&self) -> Result<()> {
        self.tracker.stop();

        std::thread::sleep(std::time::Duration::from_millis(100));

        {
            let mpv = self
                .mpv
                .lock()
                .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

            mpv.command("stop", &[])
                .map_err(|e| AppError::Runtime(format!("Failed to clear playlist: {}", e)))?;
            mpv.command("playlist-clear", &[])
                .map_err(|e| AppError::Runtime(format!("Failed to clear playlist: {}", e)))?;
        }

        log::info!("MPV player shutdown completed");
        Ok(())
    }
}
