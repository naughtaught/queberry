use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub id: i32,
    pub user_id: i32,
    pub preferred_theme: String,
    pub is_light_mode: bool,
    pub image_scaling: i32,
    pub autoplay: bool,
    pub volume: i32,
    pub completion_percent: i32,
    pub duration_display: String,
    pub preferred_audio_language: String,
    pub preferred_subtitle_language: String,
    pub subtitle_display: String,
    pub subtitle_margin: i64,
    pub audio_channel: String,
    pub default_resolver: String,
    pub file_size_limit: i32,
    pub disabled_plugins: String,
    pub season_completion_required: bool,
    pub enable_user_ratings: bool,
    pub open_app_fullscreen: bool,
    pub update_notification: bool,
    pub onscreen_keyboard_enabled: bool,
    pub excluded_resolutions: String,
    pub excluded_video_formats: String,
    pub excluded_audio_options: String,
    pub excluded_sources: String,
    pub indexer_sort_criteria: String,
    pub download_rate_limit: i32,
    pub screensaver_timeout: i32,
}

impl UserSettings {
    pub fn default_for_user(user_id: i32) -> Self {
        Self {
            id: 0,
            user_id,
            preferred_theme: "Default".to_string(),
            is_light_mode: false,
            image_scaling: 100,
            autoplay: true,
            volume: 30,
            completion_percent: 80,
            duration_display: "Duration".to_string(),
            preferred_audio_language: "Source".to_string(),
            preferred_subtitle_language: "en".to_string(),
            subtitle_display: "Auto".to_string(),
            subtitle_margin: 34,
            audio_channel: "5.1".to_string(),
            default_resolver: "torbox".to_string(),
            file_size_limit: 20,
            disabled_plugins: "[]".to_string(),
            season_completion_required: true,
            enable_user_ratings: true,
            open_app_fullscreen: false,
            update_notification: true,
            onscreen_keyboard_enabled: false,
            excluded_resolutions: "[]".to_string(),
            excluded_video_formats: "[]".to_string(),
            excluded_audio_options: "[]".to_string(),
            excluded_sources: "[]".to_string(),
            indexer_sort_criteria: "[]".to_string(),
            download_rate_limit: 0,
            screensaver_timeout: 30,
        }
    }

    pub fn to_frontend_json(&self) -> serde_json::Value {
        let mut value = serde_json::to_value(self).unwrap();

        if let serde_json::Value::Object(ref mut obj) = value {
            let fields_to_parse = [
                "disabled_plugins",
                "excluded_resolutions",
                "excluded_video_formats",
                "excluded_audio_options",
                "excluded_sources",
                "indexer_sort_criteria",
            ];

            for field in &fields_to_parse {
                if let Some(serde_json::Value::String(json_str)) = obj.get(*field) {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
                        obj.insert(field.to_string(), parsed);
                    }
                }
            }
        }

        value
    }
}
