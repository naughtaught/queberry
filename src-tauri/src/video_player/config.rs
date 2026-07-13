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

    pub fn apply_user_settings(&self, mpv: &Mpv) -> Result<()> {
        if let Some(settings) = &self.settings {
            if settings.volume >= 0 && settings.volume <= 100 {
                mpv.set_property("volume", settings.volume as i64)
                    .map_err(|e| AppError::Runtime(format!("Failed to set volume: {}", e)))?;
            }
            if !settings.audio_channel.is_empty() {
                audio::set_audio_channel_during_init(mpv, &settings.audio_channel)?;
            }
        }
        Ok(())
    }

    pub fn set_window_id(&self, mpv: &Mpv, window_id: i64) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            mpv.set_property("wid", window_id).map_err(|e| {
                AppError::Runtime(format!("Failed to set Windows window ID: {}", e))
            })?;
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
            ("cache-pause", "yes"),
            // Audio
            ("audio-file-auto", "no"),
            ("ao", "wasapi"),
            ("audio-buffer", "0.4"),
            // Hardware decoding
            ("hwdec", "auto-safe"),
            ("gpu-api", "auto"),
            ("gpu-context", "auto"),
            ("gpu-hwdec-interop", "auto"),
            // Threading
            ("demuxer-thread", "yes"),
            // Sync
            ("video-sync", "audio"),
            ("interpolation", "no"),
            ("video-sync-max-video-change", "1"),
            ("untimed", "no"),
            ("audio-stream-silence", "no"),
            ("audio-pitch-correction", "yes"),
            // Seeking behavior
            ("hr-seek", "yes"),
            ("reset-on-next-file", "video-aspect,vid,aid,sid"),
            ("vd-lavc-software-fallback", "yes"),
            ("ytdl", "no"),
        ]
        .iter()
        .cloned()
        .collect()
    }

    fn apply_optional_defaults(&self, initializer: &libmpv2::MpvInitializer) -> Result<()> {
        let defaults = self.get_optional_defaults();
        for (key, value) in &defaults {
            if let Err(e) = initializer.set_option(key, *value) {
                AppError::Config(format!("Failed to set mpv default '{}': {}", key, e))
                    .log("mpv_config");
            }
        }

        Ok(())
    }

    fn apply_hardcoded_params(&self, initializer: &libmpv2::MpvInitializer) -> Result<()> {
        let forced = self.get_hardcoded_params();
        for (key, value) in &forced {
            if let Err(e) = initializer.set_option(key, *value) {
                AppError::Config(format!(
                    "Failed to set mpv hardcoded param '{}': {}",
                    key, e
                ))
                .log("mpv_config");
            }
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
                "reconnect=1,reconnect_streamed=1,reconnect_delay_max=15",
            ),
            ("msg-level", "all=warn"),
            // ("terminal", "yes"),
            ("save-position-on-quit", "no"),
            ("stop-playback-on-init-failure", "no"),
        ]
        .iter()
        .cloned()
        .collect()
    }

    fn load_mpv_conf(&self, initializer: &libmpv2::MpvInitializer) {
        use crate::video_player::platform::get_mpv_conf_path;
        if let Some(conf_path) = get_mpv_conf_path() {
            if let Some(path_str) = conf_path.to_str() {
                if conf_path.exists() {
                    let abs_path = if conf_path.is_absolute() {
                        path_str.to_string()
                    } else {
                        match conf_path.canonicalize() {
                            Ok(abs) => abs.to_str().unwrap_or(path_str).to_string(),
                            Err(_) => path_str.to_string(),
                        }
                    };

                    if let Err(e) = initializer.load_config(&abs_path) {
                        AppError::Config(format!(
                            "Failed to load mpv config '{}': {}",
                            abs_path, e
                        ))
                        .log("mpv_config");
                    }
                }
            }
        }
    }

    pub fn apply_during_initialization(
        &self,
        initializer: &libmpv2::MpvInitializer,
    ) -> Result<(), libmpv2::Error> {
        let _ = self.apply_optional_defaults(initializer);

        self.load_mpv_conf(initializer);

        let _ = self.apply_hardcoded_params(initializer);

        Ok(())
    }
}
