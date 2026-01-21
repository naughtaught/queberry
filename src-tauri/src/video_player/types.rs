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
pub struct Metadata {
    pub duration: f64,
}

#[derive(Serialize, Clone)]
pub struct VideoState {
    pub current_time: u64,
    pub cache_time: u64,
    pub cache_speed: u64,
    pub is_buffering: bool,
    pub buffering_percent: u64,
    pub is_paused: bool,
}

#[derive(Serialize, Clone)]
pub struct CompletionEvent {
    pub is_completed: bool,
}
