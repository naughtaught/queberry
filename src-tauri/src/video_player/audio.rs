use crate::constants::AUDIO_OFFSET_STEP;
use crate::errors::{AppError, Result};
use crate::video_player::types::AudioTrackInfo;
use libmpv2::Mpv;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AudioManager {
    mpv: Arc<Mutex<Mpv>>,
}

impl AudioManager {
    pub fn new(mpv: Arc<Mutex<Mpv>>) -> Self {
        Self { mpv }
    }

    pub async fn get_tracks(&self) -> Result<Vec<AudioTrackInfo>> {
        let mpv = self.mpv.lock().await;

        let track_count: i64 = mpv
            .get_property("track-list/count")
            .map_err(|e| AppError::Runtime(format!("Failed to get track count: {}", e)))?;

        let mut tracks = Vec::new();

        for i in 0..track_count {
            if let Ok(track_type) = mpv.get_property::<String>(&format!("track-list/{}/type", i)) {
                if track_type == "audio" {
                    if let Ok(id) = mpv.get_property::<i64>(&format!("track-list/{}/id", i)) {
                        let mut track = AudioTrackInfo::new(id);

                        if let Ok(lang) =
                            mpv.get_property::<String>(&format!("track-list/{}/lang", i))
                        {
                            track = track.with_lang(lang);
                        }

                        if let Ok(title) =
                            mpv.get_property::<String>(&format!("track-list/{}/title", i))
                        {
                            track = track.with_title(title);
                        }

                        tracks.push(track);
                    }
                }
            }
        }

        Ok(tracks)
    }

    pub async fn set_track(&self, track_id: i64) -> Result<()> {
        let mpv = self.mpv.lock().await;

        mpv.set_property("aid", track_id).map_err(|e| {
            AppError::Runtime(format!("Failed to set audio track ID {}: {}", track_id, e))
        })
    }

    pub async fn set_channels(&self, config: &str) -> Result<()> {
        let mpv = self.mpv.lock().await;

        let channels = Self::normalize_channel_config(config);

        if !Self::is_valid_channel_config(channels) {
            return Err(AppError::Validation(format!(
                "Invalid channel configuration: '{}'",
                config
            )));
        }

        let _ = mpv.command("af", &["remove", "loudnorm"]);

        mpv.set_property("audio-channels", channels).map_err(|e| {
            AppError::Runtime(format!(
                "Failed to set audio-channels to '{}': {}",
                channels, e
            ))
        })?;

        mpv.set_property("channels", channels).map_err(|e| {
            AppError::Runtime(format!("Failed to set channels to '{}': {}", channels, e))
        })?;

        if channels == "stereo" || channels == "2.1" {
            let _ = mpv.command("af", &["add", "loudnorm=I=-16:TP=-1.5:LRA=11"]);
        }

        Ok(())
    }

    fn is_valid_channel_config(config: &str) -> bool {
        matches!(
            config,
            "7.1"
                | "6.1"
                | "6.0"
                | "5.1"
                | "5.0"
                | "4.1"
                | "4.0"
                | "3.1"
                | "3.0"
                | "2.1"
                | "stereo"
                | "mono"
                | "auto"
        )
    }

    fn normalize_channel_config(config: &str) -> &str {
        match config {
            "7.1" => "7.1",
            "6.1" => "6.1",
            "6.0" => "6.0",
            "5.1" => "5.1",
            "5.0" => "5.0",
            "4.1" => "4.1",
            "4.0" | "quad" => "4.0",
            "3.1" => "3.1",
            "3.0" => "3.0",
            "2.1" => "2.1",
            "2.0" | "stereo" => "stereo",
            "mono" | "1.0" => "mono",
            "auto" | "original" => "auto",
            _ => config,
        }
    }

    pub async fn set_delay(&self, delay: f64) -> Result<()> {
        let mpv = self.mpv.lock().await;

        mpv.set_property("audio-delay", delay).map_err(|e| {
            AppError::Runtime(format!("Failed to set audio delay to {}: {}", delay, e))
        })
    }

    pub async fn get_delay(&self) -> Result<f64> {
        let mpv = self.mpv.lock().await;

        mpv.get_property("audio-delay")
            .map_err(|e| AppError::Runtime(format!("Failed to get audio delay: {}", e)))
    }

    pub async fn adjust_delay(&self, increase: bool) -> Result<f64> {
        let current = self.get_delay().await?;
        let new_delay = if increase {
            current + AUDIO_OFFSET_STEP
        } else {
            current - AUDIO_OFFSET_STEP
        };

        self.set_delay(new_delay).await?;
        Ok((new_delay * 100.0).round() / 100.0)
    }

    pub async fn apply_center_boost(&self, level: f64) -> Result<()> {
        if (level - 1.0).abs() < f64::EPSILON {
            return Ok(());
        }

        let mpv = self.mpv.lock().await;
        let cmd = format!("af set @pan c2={:.1}*c2", level);

        mpv.command("af", &["set", &cmd]).map_err(|e| {
            AppError::Runtime(format!(
                "Failed to apply center boost (level {}): {}",
                level, e
            ))
        })
    }

    pub fn find_best_track(
        tracks: &[AudioTrackInfo],
        preferred_lang: &str,
        content_lang: &str,
    ) -> Option<AudioTrackInfo> {
        let non_commentary: Vec<_> = tracks
            .iter()
            .filter(|t| !t.is_commentary())
            .cloned()
            .collect();

        if non_commentary.is_empty() {
            return tracks.first().cloned();
        }

        let target_lang = if preferred_lang.to_lowercase() == "source" {
            content_lang
        } else {
            preferred_lang
        };

        non_commentary
            .iter()
            .find(|t| {
                t.lang
                    .as_ref()
                    .map(|l| Self::language_matches(l, target_lang))
                    .unwrap_or(false)
            })
            .cloned()
            .or_else(|| non_commentary.first().cloned())
    }

    fn language_matches(lang1: &str, lang2: &str) -> bool {
        let base1 = Self::normalize_language(lang1);
        let base2 = Self::normalize_language(lang2);
        base1 == base2
    }

    fn normalize_language(lang: &str) -> String {
        lang.to_lowercase()
            .split('-')
            .next()
            .unwrap_or(lang)
            .to_string()
    }
}
