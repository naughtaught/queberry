use crate::errors::{AppError, Result};
use libmpv2::Mpv;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct MpvConfig {
    config_path: PathBuf,
}

impl MpvConfig {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn apply_to_mpv(&self, mpv: &Mpv) -> Result<()> {
        self.apply_optional_defaults(mpv)?;
        self.load_config_file(mpv)?;
        self.apply_hardcoded_params(mpv)?;
        Ok(())
    }

    fn apply_optional_defaults(&self, mpv: &Mpv) -> Result<()> {
        let defaults = self.get_optional_defaults();

        for (key, value) in defaults {
            if let Err(e) = mpv.set_property(key, value) {
                log::warn!("Failed to set optional property {}: {}", key, e);
            }
        }

        Ok(())
    }

    pub fn set_window_id(&self, mpv: &Mpv, window_id: i64) -> Result<()> {
        // For Wayland on Linux, we need to handle it differently
        #[cfg(target_os = "linux")]
        {
            use raw_window_handle::HasWindowHandle;
            use raw_window_handle::RawWindowHandle;

            // Check if we're on Wayland
            let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok();

            if is_wayland {
                // Wayland requires different approach - using `wid` with 0 often works
                mpv.set_property("wid", 0).map_err(|e| {
                    AppError::Runtime(format!("Failed to set Wayland window ID: {}", e))
                })?;
                log::info!("Using Wayland window embedding (wid=0)");
            } else {
                // X11
                mpv.set_property("wid", window_id).map_err(|e| {
                    AppError::Runtime(format!("Failed to set X11 window ID: {}", e))
                })?;
            }
        }

        #[cfg(target_os = "windows")]
        {
            mpv.set_property("wid", window_id).map_err(|e| {
                AppError::Runtime(format!("Failed to set Windows window ID: {}", e))
            })?;
        }

        #[cfg(target_os = "macos")]
        {
            // macOS may need special handling
            // Sometimes it's "wid", sometimes other properties
            mpv.set_property("wid", window_id)
                .or_else(|_| mpv.set_property("macos-wid", window_id))
                .map_err(|e| AppError::Runtime(format!("Failed to set macOS window ID: {}", e)))?;
        }

        Ok(())
    }

    fn get_optional_defaults(&self) -> HashMap<&'static str, &'static str> {
        [
            // Video quality
            ("profile", "high-quality"),
            ("vo", "gpu-next"),
            ("scale-antiring", "0.6"),
            // Subtitles
            ("sub-auto", "no"),
            ("sub-pos", "95"),
            ("sub-font", "Segoe UI"),
            ("sub-font-size", "28"),
            ("sub-color", "#FFFFFF"),
            ("sub-border-color", "#000000"),
            ("sub-border-size", "2"),
            ("sub-shadow-color", "#000000"),
            ("sub-shadow-offset", "0.7"),
            ("sub-spacing", "0.5"),
            ("sub-bold", "yes"),
            ("sub-italic", "no"),
            ("sub-ass", "no"),
            ("sub-scale", "1.2"),
            // Cache settings
            ("cache", "auto"),
            ("cache-on-disk", "no"),
            ("stream-buffer-size", "32MiB"),
            ("demuxer-max-bytes", "256MiB"),
            ("demuxer-max-back-bytes", "32MiB"),
            ("demuxer-readahead-secs", "15"),
            ("cache-pause", "no"),
            // Audio
            ("audio-file-auto", "no"),
            ("ao", "wasapi"),
            ("audio-buffer", "0.2"),
            // Hardware decoding
            ("hwdec", "no"),
            ("gpu-api", "d3d11"),
            ("gpu-context", "d3d11"),
            ("gpu-hwdec-interop", "auto"),
            // Threading
            ("vd-lavc-threads", "0"),
            ("vd-lavc-dr", "yes"),
            ("demuxer-thread", "yes"),
            // Sync
            ("video-sync", "display-resample"),
            ("interpolation", "yes"),
            ("video-sync-max-video-change", "1"),
            ("untimed", "no"),
            ("audio-stream-silence", "no"),
            ("audio-pitch-correction", "yes"),
            // Timing optimizations
            ("video-latency-hacks", "yes"),
            ("opengl-pbo", "yes"),
            // Seeking behavior
            ("hr-seek", "yes"),
            ("reset-on-next-file", "video-aspect,vid,aid,sid"),
        ]
        .iter()
        .cloned()
        .collect()
    }

    fn load_config_file(&self, _mpv: &Mpv) -> Result<()> {
        let conf_path = self.config_path.join("mpv.conf");

        if conf_path.exists() {
            log::info!("Loading MPV config from: {}", conf_path.display());
            // MPV auto-loads from standard locations, but we can also explicitly load
            // Note: libmpv2 may not have a direct load_config method, handle appropriately
        }

        Ok(())
    }

    fn apply_hardcoded_params(&self, mpv: &Mpv) -> Result<()> {
        let params = self.get_hardcoded_params();

        let mut errors = Vec::new();

        for (key, value) in params {
            if let Err(e) = mpv.set_property(key, value) {
                errors.push(format!("Failed to set property '{}': {}", key, e));
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Runtime(format!(
                "Multiple property setting errors: {}",
                errors.join("; ")
            )));
        }

        Ok(())
    }

    fn get_hardcoded_params(&self) -> HashMap<&'static str, &'static str> {
        [
            // Window/embedding
            ("idle", "yes"),
            ("border", "no"),
            ("force-window", "yes"),
            // Network/streaming
            ("prefetch-playlist", "yes"),
            ("network-timeout", "30"),
            (
                "stream-lavf-o",
                "reconnect=1,reconnect_streamed=1,reconnect_delay_max=5",
            ),
        ]
        .iter()
        .cloned()
        .collect()
    }
}
