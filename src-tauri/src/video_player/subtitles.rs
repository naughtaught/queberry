use crate::db::types::UserSettings;
use crate::errors::{AppError, Result};
use crate::utils::language::language_matches;
use crate::video_player::types::{CaptionType, SubtitleTrackInfo};
use libmpv2::Mpv;
use std::sync::{Arc, Mutex};

pub struct SubtitleManager {
    mpv: Arc<Mutex<Mpv>>,
}

impl SubtitleManager {
    pub fn new(mpv: Arc<Mutex<Mpv>>) -> Self {
        Self { mpv }
    }

    pub fn get_current_subtitle_track(&self) -> Result<Option<SubtitleTrackInfo>, AppError> {
        let mpv_guard = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV mutex: {}", e)))?;

        let sub_id: i64 = mpv_guard.get_property("sub").unwrap_or(-1);

        if sub_id == -1 {
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

            if track_id == sub_id {
                let type_prop = format!("track-list/{}/type", i);
                let track_type: String = match mpv_guard.get_property(&type_prop) {
                    Ok(ttype) => ttype,
                    Err(_) => continue,
                };

                if track_type == "sub" {
                    return Ok(Some(self.extract_subtitle_track_info(&mpv_guard, i)?));
                }
            }
        }

        Ok(None)
    }

    pub fn get_all_subtitle_tracks(&self) -> Result<Vec<SubtitleTrackInfo>, AppError> {
        let mpv_guard = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV mutex: {}", e)))?;

        let track_count: i64 = mpv_guard
            .get_property("track-list/count")
            .unwrap_or_default();

        let mut subtitles = Vec::new();

        for i in 0..track_count {
            let type_prop = format!("track-list/{}/type", i);
            let track_type: String = match mpv_guard.get_property(&type_prop) {
                Ok(ttype) => ttype,
                Err(_) => continue,
            };

            if track_type == "sub" {
                match self.extract_subtitle_track_info(&mpv_guard, i) {
                    Ok(info) => subtitles.push(info),
                    Err(e) => log::warn!("Failed to extract subtitle info for track {}: {}", i, e),
                }
            }
        }

        Ok(subtitles)
    }

    fn extract_subtitle_track_info(
        &self,
        mpv: &Mpv,
        track_index: i64,
    ) -> Result<SubtitleTrackInfo, AppError> {
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

        let caption_type = Self::detect_caption_type(&title, &codec);

        Ok(SubtitleTrackInfo {
            id: if id == -1 { None } else { Some(id) },
            lang,
            title,
            caption_type,
        })
    }

    fn detect_caption_type(title: &str, codec: &str) -> CaptionType {
        let lower_title = title.to_lowercase();
        let lower_codec = codec.to_lowercase();

        if lower_title.contains("commentary") || lower_codec.contains("commentary") {
            CaptionType::Commentary
        } else if lower_title.contains("forced") || lower_codec.contains("forced") {
            CaptionType::Forced
        } else if lower_title.contains("[cc]")
            || lower_title.contains("closed caption")
            || lower_codec.contains("cc")
        {
            CaptionType::Cc
        } else if lower_title.contains("sdh")
            || lower_title.contains("hard of hearing")
            || lower_title.contains("hearing impaired")
        {
            CaptionType::Sdh
        } else {
            CaptionType::Normal
        }
    }

    pub fn set_subtitle_track(&self, track_id: Option<i64>) -> Result<(), AppError> {
        let mpv_guard = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV mutex: {}", e)))?;

        match track_id {
            Some(id) => {
                mpv_guard.set_property("sub", id).map_err(|e| {
                    AppError::Runtime(format!("Failed to set subtitle track: {}", e))
                })?;

                mpv_guard
                    .set_property("sub-visibility", "yes")
                    .map_err(|e| AppError::Runtime(format!("Failed to show subtitles: {}", e)))?;
            }
            None => {
                mpv_guard.set_property("sub", "no").map_err(|e| {
                    AppError::Runtime(format!("Failed to disable subtitles: {}", e))
                })?;
            }
        }

        Ok(())
    }

    fn find_best_subtitle_track(
        &self,
        subtitle_tracks: &[SubtitleTrackInfo],
        video_language: &str,
        user_settings: &UserSettings,
    ) -> Result<Option<SubtitleTrackInfo>, AppError> {
        let filtered: Vec<SubtitleTrackInfo> = subtitle_tracks
            .iter()
            .filter(|track| !track.title.to_lowercase().contains("commentary"))
            .cloned()
            .collect();

        if filtered.is_empty() {
            return Ok(None);
        }

        if video_language.is_empty() {
            return Ok(None);
        }

        let preferred_subtitle_language = &user_settings.preferred_subtitle_language;

        match user_settings.subtitle_display.to_lowercase().as_str() {
            "off" => Ok(None),
            "on" => {
                let candidates: Vec<SubtitleTrackInfo> = filtered
                    .iter()
                    .filter(|track| language_matches(&track.lang, preferred_subtitle_language))
                    .cloned()
                    .collect();

                if candidates.is_empty() {
                    Ok(None)
                } else {
                    let best_track = candidates
                        .into_iter()
                        .min_by_key(|track| self.score_subtitle_track(track))
                        .unwrap();
                    Ok(Some(best_track))
                }
            }
            "auto" => {
                if language_matches(video_language, preferred_subtitle_language) {
                    let forced_track = filtered
                        .iter()
                        .find(|track| {
                            language_matches(&track.lang, preferred_subtitle_language)
                                && track.caption_type == CaptionType::Forced
                        })
                        .cloned();

                    Ok(forced_track)
                } else {
                    let candidates: Vec<SubtitleTrackInfo> = filtered
                        .iter()
                        .filter(|track| language_matches(&track.lang, preferred_subtitle_language))
                        .cloned()
                        .collect();

                    if candidates.is_empty() {
                        Ok(None)
                    } else {
                        let best_track = candidates
                            .into_iter()
                            .min_by_key(|track| self.score_subtitle_track(track))
                            .unwrap();
                        Ok(Some(best_track))
                    }
                }
            }
            _ => Ok(None),
        }
    }

    fn score_subtitle_track(&self, track: &SubtitleTrackInfo) -> i32 {
        let mut score = 0;

        if track.caption_type == CaptionType::Sdh {
            score += 2;
        }

        if track.caption_type == CaptionType::Forced {
            score += 1;
        }

        score
    }

    pub fn auto_select_subtitle_track(
        &self,
        video_language: &str,
        user_settings: &UserSettings,
    ) -> Result<(), AppError> {
        let all_subtitles = self.get_all_subtitle_tracks()?;

        let best_track =
            self.find_best_subtitle_track(&all_subtitles, video_language, user_settings)?;

        let track_id = best_track.and_then(|track| track.id);
        self.set_subtitle_track(track_id)
    }
}
