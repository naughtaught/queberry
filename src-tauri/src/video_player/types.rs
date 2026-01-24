use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct LoadVideoData {
    pub message: String,
    pub url: String,
}

#[derive(Serialize, Clone)]
pub struct TogglePlayData {
    pub message: String,
    pub paused: bool,
}

#[derive(Serialize, Clone)]
pub struct SeekData {
    pub message: String,
    pub seek_amount: i8,
}

#[derive(Serialize, Clone)]
pub struct SetTime {
    pub message: String,
    pub time: f64,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub title: String,
    pub duration: f64,
    pub audio_channel: String,
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

#[derive(Serialize, Clone)]
pub struct SetVolume {
    pub message: String,
    pub volume: f64,
}

#[derive(Serialize, Clone)]
pub struct CloseVideoPlayer {
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct SetAudioChannel {
    pub message: String,
    pub channel: String,
}
