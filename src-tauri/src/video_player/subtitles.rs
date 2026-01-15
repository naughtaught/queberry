use crate::constants::{
    SUBTITLE_DEFAULT_POSITION, SUBTITLE_OFFSET, SUBTITLE_OFFSET_STEP, SUBTITLE_SIZE_MAX,
    SUBTITLE_SIZE_MIN, SUBTITLE_SIZE_STEP,
};
use crate::errors::{AppError, Result};
use crate::video_player::types::{SubtitleSetting, SubtitleTrackInfo};
use libmpv2::Mpv;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct SubtitleManager {
    mpv: Arc<Mutex<Mpv>>,
}

impl SubtitleManager {
    pub fn new(mpv: Arc<Mutex<Mpv>>) -> Self {
        Self { mpv }
    }

    pub async fn get_tracks(&self) -> Result<Vec<SubtitleTrackInfo>> {
        let mpv = self.mpv.lock().await;

        let track_count: i64 = mpv
            .get_property("track-list/count")
            .map_err(|e| AppError::Runtime(format!("Failed to get subtitle track count: {}", e)))?;

        let mut tracks = Vec::new();

        for i in 0..track_count {
            if let Ok(track_type) = mpv.get_property::<String>(&format!("track-list/{}/type", i)) {
                if track_type == "sub" {
                    if let Ok(id) = mpv.get_property::<i64>(&format!("track-list/{}/id", i)) {
                        let mut track = SubtitleTrackInfo::new(id);

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

                        if let Ok(forced) =
                            mpv.get_property::<bool>(&format!("track-list/{}/forced", i))
                        {
                            track = track.with_forced(forced);
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

        mpv.set_property("sid", track_id).map_err(|e| {
            AppError::Runtime(format!(
                "Failed to set subtitle track ID {}: {}",
                track_id, e
            ))
        })?;

        if track_id > 0 {
            let pos = SUBTITLE_DEFAULT_POSITION - SUBTITLE_OFFSET;
            let _ = mpv.set_property("sub-pos", pos);
        }

        Ok(())
    }

    pub async fn disable(&self) -> Result<()> {
        self.set_track(0).await
    }

    pub async fn set_position(&self, position: i64) -> Result<()> {
        let mpv = self.mpv.lock().await;

        mpv.set_property("sub-pos", position).map_err(|e| {
            AppError::Runtime(format!(
                "Failed to set subtitle position to {}: {}",
                position, e
            ))
        })
    }

    pub async fn toggle_shift(&self, shift_up: bool) -> Result<i64> {
        let position = if shift_up {
            SUBTITLE_DEFAULT_POSITION - SUBTITLE_OFFSET
        } else {
            SUBTITLE_DEFAULT_POSITION
        };

        self.set_position(position).await?;
        Ok(position)
    }

    pub async fn set_scale(&self, scale: f64) -> Result<()> {
        let mpv = self.mpv.lock().await;

        let clamped_scale = scale.clamp(SUBTITLE_SIZE_MIN, SUBTITLE_SIZE_MAX);

        mpv.set_property("sub-scale", clamped_scale).map_err(|e| {
            AppError::Runtime(format!(
                "Failed to set subtitle scale to {}: {}",
                clamped_scale, e
            ))
        })?;

        let margin = if clamped_scale < 1.0 {
            40 + ((1.0 - clamped_scale) * 10.0) as i64
        } else {
            40
        };

        let _ = mpv.set_property("sub-margin-y", margin);

        Ok(())
    }

    pub async fn get_scale(&self) -> Result<f64> {
        let mpv = self.mpv.lock().await;

        mpv.get_property("sub-scale")
            .map_err(|e| AppError::Runtime(format!("Failed to get subtitle scale: {}", e)))
    }

    pub async fn adjust_size(&self, increase: bool) -> Result<f64> {
        let current = self.get_scale().await?;

        let new_scale = if increase {
            (current + SUBTITLE_SIZE_STEP).min(SUBTITLE_SIZE_MAX)
        } else {
            (current - SUBTITLE_SIZE_STEP).max(SUBTITLE_SIZE_MIN)
        };

        self.set_scale(new_scale).await?;
        Ok((new_scale * 100.0).round() / 100.0)
    }

    pub async fn set_delay(&self, delay: f64) -> Result<()> {
        let mpv = self.mpv.lock().await;

        mpv.set_property("sub-delay", delay).map_err(|e| {
            AppError::Runtime(format!("Failed to set subtitle delay to {}: {}", delay, e))
        })
    }

    pub async fn get_delay(&self) -> Result<f64> {
        let mpv = self.mpv.lock().await;

        mpv.get_property("sub-delay")
            .map_err(|e| AppError::Runtime(format!("Failed to get subtitle delay: {}", e)))
    }

    pub async fn adjust_delay(&self, increase: bool) -> Result<f64> {
        let current = self.get_delay().await?;

        let new_delay = if increase {
            current + SUBTITLE_OFFSET_STEP
        } else {
            current - SUBTITLE_OFFSET_STEP
        };

        self.set_delay(new_delay).await?;
        Ok((new_delay * 100.0).round() / 100.0)
    }

    pub fn find_best_track(
        tracks: &[SubtitleTrackInfo],
        setting: SubtitleSetting,
        preferred_lang: &str,
        content_lang: &str,
    ) -> Option<SubtitleTrackInfo> {
        let non_commentary: Vec<_> = tracks
            .iter()
            .filter(|t| !t.is_commentary())
            .cloned()
            .collect();

        if non_commentary.is_empty() {
            return None;
        }

        match setting {
            SubtitleSetting::Off => None,
            SubtitleSetting::On => Self::find_preferred_subtitle(&non_commentary, preferred_lang),
            SubtitleSetting::Auto => {
                Self::find_auto_subtitle(&non_commentary, preferred_lang, content_lang)
            }
        }
    }

    fn find_preferred_subtitle(
        tracks: &[SubtitleTrackInfo],
        preferred_lang: &str,
    ) -> Option<SubtitleTrackInfo> {
        let mut candidates: Vec<_> = tracks
            .iter()
            .filter(|t| {
                t.lang
                    .as_ref()
                    .map(|l| Self::language_matches(l, preferred_lang))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if candidates.is_empty() {
            return None;
        }

        candidates.sort_by_key(|t| t.priority_score());
        candidates.first().cloned()
    }

    fn find_auto_subtitle(
        tracks: &[SubtitleTrackInfo],
        preferred_lang: &str,
        content_lang: &str,
    ) -> Option<SubtitleTrackInfo> {
        if Self::language_matches(content_lang, preferred_lang) {
            tracks
                .iter()
                .find(|t| {
                    t.forced
                        && t.lang
                            .as_ref()
                            .map(|l| Self::language_matches(l, preferred_lang))
                            .unwrap_or(false)
                })
                .cloned()
        } else {
            Self::find_preferred_subtitle(tracks, preferred_lang)
        }
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
