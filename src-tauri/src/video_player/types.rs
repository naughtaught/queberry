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
