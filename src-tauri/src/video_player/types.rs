use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleTrackInfo {
    pub id: Option<i64>,
    pub lang: String,
    pub title: String,
    pub caption_type: CaptionType,
}

#[derive(Serialize, Clone, PartialEq)]
pub enum CaptionType {
    Normal,
    #[serde(rename = "SDH")]
    Sdh,
    #[serde(rename = "CC")]
    Cc,
    Forced,
    Commentary,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub title: String,
    pub duration: f64,
    pub audio_channel: String,
    pub subtitle_tracks: Vec<SubtitleTrackInfo>,
    pub current_subtitle_track: Option<SubtitleTrackInfo>,
    pub audio_tracks: Vec<AudioTrackInfo>,
    pub current_audio_track: Option<AudioTrackInfo>,
    pub av_sync: f64,
    pub subtitle_margin: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoState {
    pub current_time: u64,
    pub cache_time: u64,
    pub cache_speed: u64,
    pub is_buffering: bool,
    pub buffering_percent: u64,
    pub is_paused: bool,
}

#[derive(Serialize)]
pub struct SubtitleTrackResponse {
    pub value: Option<SubtitleTrackInfo>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrackInfo {
    pub id: Option<i64>,
    pub lang: String,
    pub title: String,
    pub codec: String,
    pub channels: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
    pub default: bool,
}

#[derive(Serialize)]
pub struct AudioTrackResponse {
    pub value: Option<AudioTrackInfo>,
}

#[derive(Serialize)]
pub struct VideoCommandResponse {
    pub value: Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] // Optional: matches JS naming convention
pub struct LoadVideoRequest {
    pub url: String,
    pub user_id: i32,
}
