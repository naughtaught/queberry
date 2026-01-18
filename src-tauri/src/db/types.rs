use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub id: i32,
    pub user_id: i32,
    pub preferred_theme: String,
    pub is_light_mode: bool,
    pub image_size: i32,
    pub autoplay: bool,
    pub volume: i32,
    pub complete_percent: i32,
    pub duration_display: String,
    pub preferred_language: String,
    pub preferred_subtitle_language: String,
    pub subtitle_setting: String,
    pub audio_channels: String,
    pub preferred_resolver: String,
    pub size_limit: i32,
    pub disabled_plugins: String,
    pub stop_on_incomplete_season: bool,
    pub enable_user_ratings: bool,
    pub always_open_app_fullscreen: bool,
    pub update_notification: bool,
    pub keyboard_enabled: bool,
    pub exclude: String,
    pub sort: String,
    pub speed_limit: i32,
    pub screensaver: i32,
}

impl UserSettings {
    pub fn default_for_user(user_id: i32) -> Self {
        Self {
            id: 0,
            user_id,
            preferred_theme: "Default".to_string(),
            is_light_mode: false,
            image_size: 100,
            autoplay: true,
            volume: 30,
            complete_percent: 80,
            duration_display: "Duration".to_string(),
            preferred_language: "Source".to_string(),
            preferred_subtitle_language: "en".to_string(),
            subtitle_setting: "Auto".to_string(),
            audio_channels: "2.0".to_string(),
            preferred_resolver: "torbox".to_string(),
            size_limit: 20,
            disabled_plugins: "[]".to_string(),
            stop_on_incomplete_season: true,
            enable_user_ratings: true,
            always_open_app_fullscreen: false,
            update_notification: true,
            keyboard_enabled: false,
            exclude: "[]".to_string(),
            sort: "[]".to_string(),
            speed_limit: 0,
            screensaver: 30,
        }
    }

    // Helper to parse JSON fields
    pub fn disabled_plugins_list(&self) -> Vec<String> {
        serde_json::from_str(&self.disabled_plugins).unwrap_or_default()
    }

    pub fn excluded_items(&self) -> Vec<String> {
        serde_json::from_str(&self.exclude).unwrap_or_default()
    }

    pub fn sort_preferences(&self) -> Vec<String> {
        serde_json::from_str(&self.sort).unwrap_or_default()
    }
}
