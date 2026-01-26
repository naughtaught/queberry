use crate::db::types::UserSettings;
use crate::errors::{AppError, Result};
use crate::utils::language::language_matches;
use crate::video_player::types::AudioTrackInfo;
use libmpv2::Mpv;
use std::sync::{Arc, Mutex};

pub struct AudioManager {
    mpv: Arc<Mutex<Mpv>>,
}

impl AudioManager {
    pub fn new(mpv: Arc<Mutex<Mpv>>) -> Self {
        Self { mpv }
    }

    pub fn get_current_audio_track(&self) -> Result<Option<AudioTrackInfo>, AppError> {
        let mpv_guard = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV mutex: {}", e)))?;

        let audio_id: i64 = mpv_guard.get_property("aid").unwrap_or(-1);

        if audio_id == -1 {
            return Ok(None);
        }

        let track_count: i64 = mpv_guard
            .get_property("track-list/count")
            .unwrap_or_default();

        for i in 0..track_count {
            let id_prop = format!("track-list/{}/id", i);
            let track_id: i64 = match mpv_guard.get_property(&id_prop) {
                Ok(id) => id,
                Err(_) => continue,
            };

            if track_id == audio_id {
                let type_prop = format!("track-list/{}/type", i);
                let track_type: String = match mpv_guard.get_property(&type_prop) {
                    Ok(ttype) => ttype,
                    Err(_) => continue,
                };

                if track_type == "audio" {
                    return Ok(Some(self.extract_audio_track_info(&mpv_guard, i)?));
                }
            }
        }

        Ok(None)
    }

    pub fn get_all_audio_tracks(&self) -> Result<Vec<AudioTrackInfo>, AppError> {
        let mpv_guard = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV mutex: {}", e)))?;

        let track_count: i64 = mpv_guard
            .get_property("track-list/count")
            .unwrap_or_default();

        let mut audio_tracks = Vec::new();

        for i in 0..track_count {
            let type_prop = format!("track-list/{}/type", i);
            let track_type: String = match mpv_guard.get_property(&type_prop) {
                Ok(ttype) => ttype,
                Err(_) => continue,
            };

            if track_type == "audio" {
                match self.extract_audio_track_info(&mpv_guard, i) {
                    Ok(info) => audio_tracks.push(info),
                    Err(e) => log::warn!("Failed to extract audio info for track {}: {}", i, e),
                }
            }
        }

        Ok(audio_tracks)
    }

    fn extract_audio_track_info(
        &self,
        mpv: &Mpv,
        track_index: i64,
    ) -> Result<AudioTrackInfo, AppError> {
        let id_prop = format!("track-list/{}/id", track_index);
        let id: i64 = mpv.get_property(&id_prop).unwrap_or(-1);

        let title_prop = format!("track-list/{}/title", track_index);
        let title: String = mpv
            .get_property(&title_prop)
            .unwrap_or_else(|_| "".to_string());

        let lang_prop = format!("track-list/{}/lang", track_index);
        let lang: String = mpv
            .get_property(&lang_prop)
            .unwrap_or_else(|_| "".to_string());

        let codec_prop = format!("track-list/{}/codec", track_index);
        let codec: String = mpv
            .get_property(&codec_prop)
            .unwrap_or_else(|_| "".to_string());

        let channels_prop = format!("track-list/{}/demux-channel-count", track_index);
        let channels: Option<i64> = mpv.get_property(&channels_prop).ok();

        let sample_rate_prop = format!("track-list/{}/demux-samplerate", track_index);
        let sample_rate: Option<i64> = mpv.get_property(&sample_rate_prop).ok();

        let bitrate_prop = format!("track-list/{}/demux-bitrate", track_index);
        let bitrate: Option<i64> = mpv.get_property(&bitrate_prop).ok();

        let default_prop = format!("track-list/{}/default", track_index);
        let default: bool = mpv.get_property(&default_prop).unwrap_or(false);

        Ok(AudioTrackInfo {
            id: if id == -1 { None } else { Some(id) },
            lang,
            title,
            codec,
            channels,
            sample_rate,
            bitrate,
            default,
        })
    }

    pub fn set_audio_track(&self, track_id: Option<i64>) -> Result<(), AppError> {
        let mpv_guard = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV mutex: {}", e)))?;

        match track_id {
            Some(id) => {
                mpv_guard
                    .set_property("aid", id)
                    .map_err(|e| AppError::Runtime(format!("Failed to set audio track: {}", e)))?;
            }
            None => {
                mpv_guard
                    .set_property("aid", "no")
                    .map_err(|e| AppError::Runtime(format!("Failed to disable audio: {}", e)))?;
            }
        }

        Ok(())
    }

    fn find_best_audio_track(
        &self,
        audio_tracks: &[AudioTrackInfo],
        video_language: &str,
        user_settings: &UserSettings,
    ) -> Result<Option<AudioTrackInfo>, AppError> {
        let filtered: Vec<AudioTrackInfo> = audio_tracks
            .iter()
            .filter(|track| !track.title.to_lowercase().contains("commentary"))
            .cloned()
            .collect();

        if filtered.is_empty() {
            return Ok(None);
        }

        match user_settings
            .preferred_audio_language
            .to_lowercase()
            .as_str()
        {
            "source" => {
                for track in &filtered {
                    if language_matches(&track.lang, video_language) {
                        return Ok(Some(track.clone()));
                    }
                }

                if let Some(default_track) = filtered.iter().find(|t| t.default) {
                    return Ok(Some(default_track.clone()));
                }
                Ok(filtered.first().cloned())
            }

            preferred_lang => {
                let candidates: Vec<AudioTrackInfo> = filtered
                    .iter()
                    .filter(|track| language_matches(&track.lang, preferred_lang))
                    .cloned()
                    .collect();

                if !candidates.is_empty() {
                    let best_track = candidates
                        .into_iter()
                        .max_by_key(|track| self.score_audio_track(track))
                        .unwrap();
                    return Ok(Some(best_track));
                }

                if let Some(default_track) = filtered.iter().find(|t| t.default) {
                    return Ok(Some(default_track.clone()));
                }

                let english_candidates: Vec<AudioTrackInfo> = filtered
                    .iter()
                    .filter(|track| {
                        language_matches(&track.lang, "en")
                            || track.lang.to_lowercase().contains("en")
                    })
                    .cloned()
                    .collect();

                if !english_candidates.is_empty() {
                    return Ok(english_candidates.first().cloned());
                }

                Ok(filtered.first().cloned())
            }
        }
    }

    fn score_audio_track(&self, track: &AudioTrackInfo) -> i32 {
        let mut score = 0;

        if let Some(channels) = track.channels {
            score += channels as i32;
        }

        if let Some(sample_rate) = track.sample_rate {
            score += (sample_rate / 1000) as i32; // Convert to kHz for scoring
        }

        if let Some(bitrate) = track.bitrate {
            score += (bitrate / 1000) as i32; // Convert to kbps for scoring
        }

        if track.default {
            score += 5;
        }

        score
    }

    pub fn auto_select_audio_track(
        &self,
        video_language: &str,
        user_settings: &UserSettings,
    ) -> Result<(), AppError> {
        let all_audio_tracks = self.get_all_audio_tracks()?;

        let best_track =
            self.find_best_audio_track(&all_audio_tracks, video_language, user_settings)?;

        let track_id = best_track.and_then(|track| track.id);
        self.set_audio_track(track_id)
    }
}

pub fn set_audio_channel(mpv: &Mpv, audio_channel: &str) -> Result<()> {
    let normalized_channel = match audio_channel.to_lowercase().as_str() {
        "7.1" => "7.1",
        "6.1" => "6.1",
        "6.0" => "6.0",
        "5.1" => "5.1",
        "5.0" => "5.0",
        "4.1" => "4.1",
        "4.0" => "4.0",
        "3.1" => "3.1",
        "3.0" => "3.0",
        "2.1" => "2.1",
        "2.0" | "stereo" => "stereo",
        "auto" | "original" => "auto",
        _ => {
            return Err(AppError::Runtime(format!(
                "Unsupported audio channel: {}",
                audio_channel
            )));
        }
    };

    let _ = mpv.command("no-osd", &["af", "remove", "loudnorm"]);

    mpv.set_property("audio-channels", normalized_channel)
        .map_err(|e| {
            AppError::Runtime(format!(
                "Failed to set audio channel to {}: {}",
                normalized_channel, e
            ))
        })?;

    if normalized_channel == "stereo" || normalized_channel == "2.1" {
        if let Err(e) = mpv.command("no-osd", &["af", "add", "loudnorm=I=-16:TP=-1.5:LRA=11"]) {
            log::debug!("Could not add loudnorm filter during initialization: {}", e);
        }
    }

    Ok(())
}
