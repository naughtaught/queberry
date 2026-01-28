use serde::Serialize;

#[derive(Serialize)]
pub struct LoadVideoData {
    pub message: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct TogglePlayData {
    pub message: String,
    pub paused: bool,
}

#[derive(Serialize)]
pub struct SeekData {
    pub message: String,
    pub seek_amount: i8,
}

#[derive(Serialize)]
pub struct SetTime {
    pub message: String,
    pub time: f64,
}

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

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompletionEvent {
    pub is_completed: bool,
}

#[derive(Serialize)]
pub struct SetVolume {
    pub message: String,
    pub volume: f64,
}

#[derive(Serialize)]
pub struct SetAudioChannel {
    pub message: String,
    pub channel: String,
}

#[derive(Serialize)]
pub struct SubtitleTrackResponse {
    pub message: String,
    pub current_subtitle_track: Option<SubtitleTrackInfo>,
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
    pub message: String,
    pub current_audio_track: Option<AudioTrackInfo>,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}
