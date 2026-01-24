use crate::db::types::UserSettings;
use crate::errors::{AppError, Result};
use crate::video_player::audio;
use libmpv2::Mpv;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct MpvConfig {
    settings: Option<UserSettings>,
}

impl MpvConfig {
    pub fn new(settings: &UserSettings) -> Self {
        Self {
            settings: Some(settings.clone()),
        }
    }

    pub fn apply_to_mpv(&self, mpv: &Mpv) -> Result<()> {
        self.apply_user_settings(mpv)?;
        self.apply_optional_defaults(mpv)?;
        self.apply_hardcoded_params(mpv)?;
        Ok(())
    }

    fn apply_user_settings(&self, mpv: &Mpv) -> Result<()> {
        if let Some(settings) = &self.settings {
            if settings.volume >= 0 && settings.volume <= 100 {
                mpv.set_property("volume", settings.volume as i64)
                    .map_err(|e| AppError::Runtime(format!("Failed to set volume: {}", e)))?;
            }
            if !settings.audio_channels.is_empty() {
                audio::set_audio_channel(mpv, &settings.audio_channels)?;
            }
        }
        Ok(())
    }

    pub fn set_window_id(&self, mpv: &Mpv, window_id: i64) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            use raw_window_handle::HasWindowHandle;
            use raw_window_handle::RawWindowHandle;

            let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok();

            if is_wayland {
                mpv.set_property("wid", 0).map_err(|e| {
                    AppError::Runtime(format!("Failed to set Wayland window ID: {}", e))
                })?;
                log::info!("Using Wayland window embedding (wid=0)");
            } else {
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
            // macOS TODO
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

    fn apply_optional_defaults(&self, mpv: &Mpv) -> Result<()> {
        let defaults = self.get_optional_defaults();

        for (key, value) in defaults {
            if let Err(e) = mpv.set_property(key, value) {
                log::warn!("Failed to set optional property {}: {}", key, e);
            }
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
