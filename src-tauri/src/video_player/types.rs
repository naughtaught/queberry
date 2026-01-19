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
    pub seek_amount: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct Metadata {
    pub duration: f64,
}
