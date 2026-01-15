use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::AppError;

/// Audio track information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioTrackInfo {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl AudioTrackInfo {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            lang: None,
            title: None,
        }
    }

    pub fn with_lang(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn is_commentary(&self) -> bool {
        self.title
            .as_ref()
            .map(|t| t.to_lowercase().contains("commentary"))
            .unwrap_or(false)
    }
}

/// Subtitle track information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubtitleTrackInfo {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub forced: bool,
    #[serde(default)]
    pub sdh: bool,
}

impl SubtitleTrackInfo {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            lang: None,
            title: None,
            forced: false,
            sdh: false,
        }
    }

    pub fn with_lang(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        let title_upper = title.to_uppercase();
        self.sdh = title_upper.contains("SDH");
        self.title = Some(title);
        self
    }

    pub fn with_forced(mut self, forced: bool) -> Self {
        self.forced = forced;
        self
    }

    pub fn is_commentary(&self) -> bool {
        self.title
            .as_ref()
            .map(|t| t.to_lowercase().contains("commentary"))
            .unwrap_or(false)
    }

    /// Calculate priority score for subtitle selection (lower is better)
    pub fn priority_score(&self) -> i32 {
        let mut score = 0;
        if self.sdh {
            score += 2;
        }
        if self.forced {
            score += 1;
        }
        score
    }
}

/// Shader information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderInfo {
    pub name: String,
    pub file: String,
    pub path: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl ShaderInfo {
    pub fn none() -> Self {
        Self {
            name: "None".to_string(),
            file: String::new(),
            path: String::new(),
            is_active: true,
        }
    }

    pub fn new(name: String, file: String, path: String) -> Self {
        Self {
            name,
            file,
            path,
            is_active: false,
        }
    }
}

/// Playlist item metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItemMetadata {
    pub id: i64,
    #[serde(rename = "seasonNumber")]
    pub season_number: i32,
    #[serde(rename = "episodeNumber")]
    pub episode_number: i32,
    #[serde(rename = "filePath")]
    pub file_path: String,
    pub title: String,
    pub duration: f64,
    #[serde(rename = "currentAudioTrack")]
    pub current_audio_track: Option<AudioTrackInfo>,
    #[serde(rename = "currentSubtitleTrack")]
    pub current_subtitle_track: Option<SubtitleTrackInfo>,
    #[serde(rename = "audioTracks")]
    pub audio_tracks: Vec<AudioTrackInfo>,
    #[serde(rename = "subtitleTracks")]
    pub subtitle_tracks: Vec<SubtitleTrackInfo>,
    #[serde(rename = "speakerConfiguration")]
    pub speaker_configuration: String,
    pub language: String,
    #[serde(rename = "mediaData")]
    pub media_data: serde_json::Value,
    #[serde(rename = "availableShaders")]
    pub available_shaders: Vec<ShaderInfo>,
}

impl PlaylistItemMetadata {
    pub fn new(
        id: i64,
        season_number: i32,
        episode_number: i32,
        file_path: String,
        title: String,
        language: String,
        speaker_configuration: String,
        media_data: serde_json::Value,
        available_shaders: Vec<ShaderInfo>,
    ) -> Self {
        Self {
            id,
            season_number,
            episode_number,
            file_path,
            title,
            duration: 0.0,
            current_audio_track: None,
            current_subtitle_track: None,
            audio_tracks: Vec::new(),
            subtitle_tracks: Vec::new(),
            speaker_configuration,
            language,
            media_data,
            available_shaders,
        }
    }
}

/// User settings for the player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub volume: i32,
    #[serde(rename = "completePercent")]
    pub complete_percent: i32,
    #[serde(rename = "subtitleSetting")]
    pub subtitle_setting: SubtitleSetting,
    #[serde(rename = "preferredLanguage")]
    pub preferred_language: String,
    #[serde(rename = "preferredSubtitleLanguage")]
    pub preferred_subtitle_language: String,
    #[serde(rename = "audioChannels")]
    pub audio_channels: String,
    #[serde(rename = "speedLimit")]
    pub speed_limit: i32,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            volume: 70,
            complete_percent: 90,
            subtitle_setting: SubtitleSetting::Auto,
            preferred_language: "eng".to_string(),
            preferred_subtitle_language: "eng".to_string(),
            audio_channels: "5.1".to_string(),
            speed_limit: 100,
        }
    }
}

/// Subtitle setting mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubtitleSetting {
    Off,
    On,
    Auto,
}

impl FromStr for SubtitleSetting {
    type Err = AppError; // Or any error type you prefer

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            "auto" => Ok(Self::Auto),
            _ => Ok(Self::Auto), // Or return an error: Err(AppError::Validation(...))
        }
    }
}

/// Language information
#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    pub code: String,
    pub code2: String,
    pub name: String,
    #[serde(rename = "nativeName")]
    pub native_name: String,
}

/// Buffering information
#[derive(Debug, Clone, Serialize)]
pub struct BufferingInfo {
    #[serde(rename = "currentTime")]
    pub current_time: f64,
    pub buffered: f64,
}

/// Playback navigation state
#[derive(Debug, Clone, Serialize)]
pub struct NavigationState {
    #[serde(rename = "isPrevious")]
    pub is_previous: bool,
    #[serde(rename = "isNext")]
    pub is_next: bool,
}
